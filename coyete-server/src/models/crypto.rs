use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use rocket::http::Status;
use argon2::{self, Config, ThreadMode, Variant, Version};
use api::response::{self, APIResponse};

// crypto constants
const SALT_SIZE: usize = 32;
const HASH_SIZE: usize = 32;
const MIN_PWD_SIZE: usize = 6;
// authentication guard constants
const AUTH_PREFIX: &'static str = "x-auth ";
const AUTH_SEPERATOR: &'static str = ".";


#[derive(Serialize)]
pub struct AuthorizationBearer(pub String);

impl<'a 'r> FromRequest<'a, 'r> for AuthorizationBearer {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<AuthorizationBearer, ()> {
        let keys: Vec<_> = req.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        };

        let mut key: String = keys[0].to_string();
        if !key.starts_with(BASIC_AUTH_PREFIX) {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        key = key.replace(AUTH_SEPERATOR, "");
        let decoded = String::from_utf8(decode(&key).unwrap()).unwrap();
        let auth: Vec<String> = decoded
            .split(AUTH_SEPERATOR)
            .map(|s| s.to_string())
            .collect();

        return Outcome::Success(AuthorizationBearer(auth.to_string()));
    }
}

pub fn hash_password<'a>(password: &'a str, salt: &'a str) -> String {
    // argon_config is an Argon2id hybrid version. It follows the Argon2i approach
    // for the first pass over memory and the Argon2d approach for subsequent passes.
    let argon_config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 256,
        time_cost: 3,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };

    argon2::hash_encoded(&password, &salt, &argon_config)
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect()
}

// generate_salt creates salt vector of 32 bytes
pub fn generate_salt() -> Vec<u8> {
    nonce(32).as_bytes().to_vec();
}

// nonce takes a usize parameter for length
fn nonce(take: usize) -> String {
    use rand::{self, Rng};

    rand::thread_rng()
        .gen_ascii_chars()
        .take(take)
        .collect::<String>()
}
