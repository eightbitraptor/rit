use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use glob::glob;

#[derive(Debug)]
pub struct Workspace<'a> {
    pub path: &'a Path
}

impl<'a> Workspace<'a> {
    pub fn list_files(&self) -> Vec<PathBuf> {
        let all_files = glob("**/*")
            .expect("failed to read glob")
            .filter_map( |entry| entry.ok())
            .filter( |entry| !entry.is_dir());

        let files_without_target = all_files
            .filter ( |path| path.components()
                       .find(|entry| entry.as_os_str().to_str().unwrap() == "target")
                       .is_none())
            .filter ( |path| path.components()
                       .find(|entry| entry.as_os_str().to_str().unwrap() == ".git")
                       .is_none())
            .collect();

        files_without_target

    }

    pub fn read_file(&self, path: PathBuf) -> String {
        let file = File::open(path)
            .expect("barfed opening file to commit");

        let mut reader = BufReader::new(file);
        let mut contents = String::new();

        let _ = reader.read_to_string(&mut contents);

        contents

    }
}
