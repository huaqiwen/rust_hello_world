
pub struct User {
    pub username: String,
    pub email: String,
    sign_in_count: u32,
    active: bool,
}

impl User {
    pub fn new(username: String, email: String) -> User {
        // invalid email, panics
        if !email.contains("@") {
            panic!("Invalid email address.")
        }

        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    pub fn get_sign_in_count(&self) -> u32 {
        self.sign_in_count
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn sign_in(&mut self) {
        self.sign_in_count += 1;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}
