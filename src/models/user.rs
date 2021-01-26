use std::fmt;

pub struct UserModel {
    pub name: String,
    pub image: String,
    pub status: String,
}

impl fmt::Display for UserModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} image: {}", self.name, self.status, self.image)
    }
}
