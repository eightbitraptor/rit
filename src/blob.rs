use crate::git_serialisable::GitSerialisable;

#[derive(Debug)]
pub struct Blob {
    pub data: String
}

impl GitSerialisable for Blob {
    fn o_type(&self) -> Vec<u8> {
        "blob".as_bytes().to_vec()
    }

    fn encode(&self) -> Vec<u8> {
        self.data.as_bytes().to_vec()
    }
}
