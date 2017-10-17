use argon2::{self, Config, ThreadMode, Variant, Version};
use errors::Error;

// hash encodes plaintext with a salt vector slice.  Returns encoded string or
// fails with [argon2::Error](https://docs.rs/rust-argon2/0.3.0/argon2/enum.Error.html)
pub fn hash(password: &str, salt: Vec<u8>) -> Result<String, Error> {
    // cfg is Argon2id hybrid version. It follows the Argon2i approach
    // for the first pass over memory and the Argon2d approach for subsequent passes.
    //
    // For optimal security, cost parameters should be fine tuned to machine
    // CPU and memory specs.
    //
    // ref: https://github.com/P-H-C/phc-winner-argon2/blob/master/argon2-specs.pdf
    let cfg = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 512,
        time_cost: 100,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 28,
    };

    let argon2id = argon2::hash_encoded(password.as_bytes(), &salt.as_ref(), &cfg)?;

    Ok(argon2id)
}

// verifies hash_encoded method output as true. Returns DecodingFail if failure.
pub fn verify(encoded_hash: &str, password: &[u8]) -> Result<bool, Error> {
    let is_valid = argon2::verify_encoded(encoded_hash, password)?;

    Ok(is_valid)
}

// generate_salt creates salt vector of a usize parameter.
pub fn generate_salt(size: usize) -> Vec<u8> {
    nonce(size).as_bytes().to_vec()
}

// nonce takes a usize parameter for length
fn nonce(take: usize) -> String {
    use rand::{self, Rng};

    rand::thread_rng()
        .gen_ascii_chars()
        .take(take)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::generate_salt;
    use super::hash;
    use super::verify;

    const PWD: &str = "Passw0rd!";
    const INCORRECT_PWD: &str = "Inc0rrect!";

    #[test]
    fn gen_salt() {
        let salt = generate_salt(24);
        assert_eq!(24, salt.len());
    }

    #[test]
    fn hash_and_validate() {
        let salt = generate_salt(16);
        let digest = hash(&PWD, salt).unwrap();
        let verified = verify(&digest, &PWD.as_bytes()).unwrap();
        let should_eq_false = verify(&digest, &INCORRECT_PWD.as_bytes()).unwrap();
        assert_eq!(verified, true);
        assert_eq!(should_eq_false, false)
    }
}
