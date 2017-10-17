pub enum InputErrorKind {
    Password(Vec<PasswordErrorKind>),
    Email(Vec<EmailErrorKind>),
    Authenticator,
    PermissionName,
}

impl ToString for InputErrorKind {
    fn to_string(&self) -> String {
        match *self {
            InputErrorKind::Password(ref err_vec) => {
                let messages: Vec<String> = err_vec.iter().map(|p| p.to_string()).collect();

                messages.join(", ")
            }
            InputErrorKind::Email(ref err_vec) => {
                let messages: Vec<String> = err_vec.iter().map(|m| m.to_string()).collect();

                messages.join(", ")
            }
            InputErrorKind::Authenticator => "Invalid authentication message format".to_string(),
            InputErrorKind::PermissionName => "Invalid permission name".to_string(),
        }
    }
}

pub enum PasswordErrorKind {
    NoLowercase,
    NoNumber,
    NoSymbol,
    NoUppercase,
    TooShort,
}

impl ToString for PasswordErrorKind {
    fn to_string(&self) -> String {
        match *self {
            PasswordErrorKind::NoLowercase => {
                "Password must contain at least one lowercase letter".to_string()
            }
            PasswordErrorKind::NoNumber => "Password must contain at least one number".to_string(),
            PasswordErrorKind::NoSymbol => "Password must contain at least one symbol".to_string(),
            PasswordErrorKind::NoUppercase => {
                "Password must contain at least one uppercase letter".to_string()
            }
            PasswordErrorKind::TooShort => "Password must be at least 8 characters".to_string(),
        }
    }
}

pub enum EmailErrorKind {
    Blank,
    InvalidFormat,
}

impl ToString for EmailErrorKind {
    fn to_string(&self) -> String {
        match *self {
            EmailErrorKind::Blank => "Email field must not be blank".to_string(),
            EmailErrorKind::InvalidFormat => "Invalid email format".to_string(),
        }
    }
}
