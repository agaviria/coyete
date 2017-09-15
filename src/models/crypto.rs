use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use rocket::http::Status;

// crypto constants
const SALT_SIZE: usize = 32;
const HASH_SIZE: usize = 32;
const MIN_PWD_SIZE: usize = 6;
// authentication guard constants
const AUTH_PREFIX: &'static str = "x-auth ";
const AUTH_SEPERATOR: &'static str = ":";


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

#[derive(Debug, Serialize, Deserialize)]
pub struct Cipher {
    pub salt: Vec<u8>,
    pub hash: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassThruValidator {
    value: String,
}
// PassThruValidator under the hood implements a helper which places a minimum
// string parameter of $MIN_PWD_SIZE. In adherence to best security practices.
impl PassThruValidator {
    pub fn new(value: &str) -> Result<PassThruValidator, Error> {
        if value.len() < MIN_PWD_SIZE {
            return Err(Error::PassThruValidator(MIN_PWD_SIZE));
        }
        Ok(PassThruValidator { value: value.to_string() })
    }
}

pub struct HashEncoder;

impl HashEncoder {
    pub fn generate(sub: &str, credential: &PassThruValidator) -> Result<Cipher, Error> {
        use argon2::{self, Config, ThreadMode, Variant, Version};
        // Argon2id is a hybrid version. It follows the Argon2i approach for the
        // first pass over memory and the Argon2d approach for subsequent passes.
        //
        // TODO:
        // * Add argon2id_config Config to settings.toml
        // * Include argon2::verify_encode() to integration tests
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

        let salt = generate_salt();
        let hash = argon2::hash_encoded(&credential.value, &salt, &argon_config);
        Ok(Cipher {
            salt: salt,
            hash: hash.to_vec(),
        })
    }
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
