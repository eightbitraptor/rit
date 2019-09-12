extern crate clap;
extern crate glob;
extern crate crypto;
extern crate rand;
extern crate flate2;
extern crate chrono;

use clap::{Arg, App, SubCommand};
use chrono::prelude::*;

mod author;
mod blob;
mod commit;
mod database;
mod entry;
mod git_serialisable;
mod tree;
mod workspace;

use std::path::Path;
use std::io::{self, Read, Write};
use std::fs::OpenOptions;

use crate::git_serialisable::GitSerialisable;

fn main() {
    let dirname_argname = "DIRECTORY";

    let arg_matches = App::new("Rit - a Git implementation in Rust")
        .subcommand(SubCommand::with_name("init")
                    .arg(Arg::with_name(dirname_argname)
                         .required(true)
                         .index(1)))
        .subcommand(SubCommand::with_name("commit"))
        .get_matches();


    match arg_matches.subcommand() {
        ("init", Some(match_value)) => {
            let root_path = Path::new(match_value.value_of(dirname_argname).unwrap())
                .canonicalize()
                .expect("Path does not exist");

            let git_path = root_path.join(".git");
            let base_git_dirnames = ["objects", "refs"];

            for dirname in base_git_dirnames.iter() {
                std::fs::create_dir_all(git_path.join(dirname))
                    .expect("Could not create directory aborting");
            }

            println!("Initialized empty Rit repository in {}", &root_path.display());
        },
        ("commit", Some(_match_value)) => {
            println!("got arg commit");
            let repo_path = std::env::current_dir()
                .expect("Cannot stat current directory");
            let db_path = repo_path.join(".git/objects");

            let workspace = workspace::Workspace {
                path: repo_path.as_path()
            };
            let database = database::Database {
                db_path: db_path.as_path()
            };

            let mut entries = Vec::new();
            for file in workspace.list_files() {
                let data = workspace.read_file(file.clone());
                let blob = blob::Blob { data: data };

                let oid = blob.oid().clone();
                let _ = database.store(blob);

                entries.push(
                    entry::Entry { name: file, oid: oid }
                );
            }

            let tree = tree::Tree { entries: entries };
            println!("tree: {}", &tree.oid());

            let stored_tree = database.store(tree);

            let name = std::env::var("GIT_AUTHOR_NAME")
                .expect("GIT_AUTHOR_NAME not set in environment");
            let email =std::env::var("GIT_AUTHOR_EMAIL")
                .expect("GIT_AUTHOR_EMAIL not set in environment");
            let author = author::Author { name: name, email: email, time: Local::now().format("%s %z").to_string() };

            println!("{}", author.to_string());

            let mut message = String::new();
            io::stdin().read_to_string(&mut message)
                .expect("could not read from STDIN");

            let commit = commit::Commit {
                tree: stored_tree,
                author: author.to_string(),
                message: message
            };
            println!("{:x?}", commit.encode());
            let commit_id = database.store(commit);

            let mut head_file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(repo_path.join(".git/HEAD"))
                .expect("could not open head file for writing");

            head_file.write_all(&commit_id.into_bytes())
                .expect("could not write head file");
        },
        _ => {
            println!("Matched Nothing");
        }
    }
}
