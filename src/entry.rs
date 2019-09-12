use std::path::PathBuf;

#[derive(Debug)]
pub struct Entry {
    pub name: PathBuf,
    pub oid: String
}
