use std::fmt;

#[derive(Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub time: String,
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}> {}", self.name, self.email, self.time)
    }
}
