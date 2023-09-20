pub struct UsernameUserLoginPayload {
    pub username: String,
    pub password: String,
}

pub struct EmailUserLoginPayload {
    pub email: String,
    pub password: String,
}

pub struct PhoneNumberUserLoginPayload {
    pub phone_number: String,
    pub password: String,
}