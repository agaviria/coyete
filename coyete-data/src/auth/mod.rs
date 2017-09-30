pub mod models;

// mod security {
//     use argon2::{self, Config, ThreadMode, Variant, Version};

//     pub fn hash_password(password: &str, salt: Vec<u8>) -> String {
//         // argon_config is an Argon2id hybrid version. It follows the Argon2i approach
//         // for the first pass over memory and the Argon2d approach for subsequent passes.
//         let argon_config = Config {
//             variant: Variant::Argon2id,
//             version: Version::Version13,
//             mem_cost: 256,
//             time_cost: 3,
//             lanes: 4,
//             thread_mode: ThreadMode::Parallel,
//             secret: &[],
//             ad: &[],
//             hash_length: 32,
//         };

//         let argon2id = argon2::hash_encoded(password.as_bytes(), &salt.as_ref(), &argon_config)
//             .unwrap();

//         argon2id
//     }

//     // generate_salt creates salt vector of 32 bytes
//     pub fn generate_salt() -> Vec<u8> {
//         nonce(32).as_bytes().to_vec()
//     }

//     // nonce takes a usize parameter for length
//     fn nonce(take: usize) -> String {
//         use rand::{self, Rng};

//         rand::thread_rng()
//             .gen_ascii_chars()
//             .take(take)
//             .collect::<String>()
//     }
// }
