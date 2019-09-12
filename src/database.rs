use std::io::Write;
use std::fs::{self, File};
use std::path::Path;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::git_serialisable::GitSerialisable;

#[derive(Debug)]
pub struct Database<'a> {
    pub db_path: &'a Path
}

impl<'a> Database<'a> {
    pub fn store<T: GitSerialisable>(&self, object: T) -> String {
        let serialised_object = object.serialise();

        self.write_object(object.oid(), &serialised_object);

        object.oid()
    }

    fn write_object(&self, oid: String, content: &[u8]) {
        let object_path = self.db_path
            .join(&oid[..2])
            .join(&oid[2..]);
        let dirname = object_path
            .parent().unwrap();
        let temp_path = &dirname
            .join(generate_temp_name());

        fs::create_dir_all(dirname)
            .expect("could not create object dir");
        let mut file = File::create(temp_path)
            .expect("could not create temp file");

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(content)
            .expect("couldn't encode stream");

        let encoded_data_bytes: &[u8] = &encoder.finish().expect("");
        file.write_all(encoded_data_bytes)
            .expect("could not write to temp file");

        fs::rename(temp_path, object_path)
            .expect("could not rename temp to object");
    }
}

fn generate_temp_name() -> String {
    let base_string = String::from("tmp_obj");
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .collect();

    vec![base_string, rand_string].join("_")
}
