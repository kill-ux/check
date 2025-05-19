pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

pub struct User {
    pub name: String,
    pub acess_level: AccessLevel,
}

impl User {
    pub fn new(name: String, acess_level: AccessLevel) -> User {
        Self { name, acess_level }
    }
    pub fn send_name(&self) -> Option<&str> {
        match self.acess_level {
            AccessLevel::Guest => None,
            _ => Some(&self.name),
        }
    }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    match user.send_name() {
        Some(name) => (true, name),
        _ => (false, "ERROR: User is guest"),
    }
}
