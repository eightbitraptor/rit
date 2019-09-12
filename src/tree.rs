use crate::entry;
use crate::git_serialisable::GitSerialisable;

#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<entry::Entry>
}

impl GitSerialisable for Tree {
    fn o_type(&self) -> Vec<u8> {
        "tree".as_bytes().to_vec()
    }

    fn encode(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();
        for entry in &self.entries {
            // push the file mode followed by a space
            for byte in "100644 ".as_bytes() {
                encoded.push(*byte)
            }

            // push the nul-byte terminated filename
            let mut e_name_bytes = entry.name.to_str()
                .expect("failed converting name to str")
                .to_string()
                .into_bytes();
            encoded.append(&mut e_name_bytes);
            encoded.push(0);

            // push the oid of the entry: 2 hex digits = 1 byte
            let oid = entry.oid.as_str();
            let mut e_oid_bytes: Vec<u8> = (0..oid.len())
                .step_by(2)
                .map(|i| {
                    u8::from_str_radix(&oid[i..i + 2], 16)
                        .expect("oid is not valid hex")
                })
                .collect();
            encoded.append(&mut e_oid_bytes);
        }

        encoded
    }
}
