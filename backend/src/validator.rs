use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct UserSerializer {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[serde(skip_deserializing)]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginSerializer {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UserSettingsSerializer {
    #[validate(email(message = "Invalid email"))]
    pub email: Option<String>,
    pub password: Option<String>,
    #[validate(length(min = "6", max = "24", message = "Passwords has to be a minimum of (6) and max of (24) characters"))]
    pub new_password: Option<String>,
}
