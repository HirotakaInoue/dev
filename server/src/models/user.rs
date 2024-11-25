pub struct User {
    pub id: String,
    pub password: String,
}

impl User {
    pub fn new(id: &str, password: &str) -> Self {
        User {
            id: id.to_string(),
            password: password.to_string(),
        }
    }
}
