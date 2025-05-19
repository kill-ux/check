pub enum AccessLevel {}

pub struct User {}

impl User {
  pub fn new(name: String, level: AccessLevel) -> User {}
  pub fn send_name(&self) -> Option<&str> {}
}

pub fn check_user_name(user: &User) -> (bool, &str) {

}
