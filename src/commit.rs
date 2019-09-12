use crate::git_serialisable::GitSerialisable;

#[derive(Debug)]
pub struct Commit {
    pub tree: String,
    pub author: String,
    pub message: String,
}

impl GitSerialisable for Commit {
    fn o_type(&self) -> Vec<u8> {
        "commit".as_bytes().to_vec()
    }

    fn encode(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();

        let mut tree_line = String::from("tree ");
        tree_line.push_str(&self.tree);
        tree_line.push('\n');

        let mut author_line = String::from("author ");
        author_line.push_str(&self.author);
        author_line.push('\n');

        let mut committer_line = String::from("committer ");
        committer_line.push_str(&self.author);
        committer_line.push('\n');
        committer_line.push('\n');

        encoded.append(&mut tree_line.as_bytes().to_vec());
        encoded.append(&mut author_line.as_bytes().to_vec());
        encoded.append(&mut committer_line.as_bytes().to_vec());
        encoded.append(&mut self.message.as_bytes().to_vec());
        encoded
    }
}
