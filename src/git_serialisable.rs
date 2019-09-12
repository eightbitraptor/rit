use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub trait GitSerialisable {
    fn o_type(&self) -> Vec<u8>;

    fn oid(&self) -> String {
        let mut hasher = Sha1::new();

        let input = unsafe {
            String::from_utf8_unchecked(self.serialise())
        };

        hasher.input_str(input.as_str());
        hasher.result_str()
    }

    fn encode(&self) -> Vec<u8>;

    fn serialise(&self) -> Vec<u8> {
        // encode the data contained in this object
        let mut o_bytes = self.encode();

        let str_len = (o_bytes.len() as usize).to_string();

        // object type followed by a space
        let mut o_type = self.o_type().clone();
        o_type.push(32);

        // length of the content in bytes followed by the nul byte
        let mut o_len = str_len.as_bytes().to_vec();
        o_len.push(0);

        o_type.append(&mut o_len);
        o_type.append(&mut o_bytes);

        o_type
    }
}
