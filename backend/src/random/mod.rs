use base64;
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

pub fn generate_secure_random_bytes_as_base64(num_bytes: usize) -> String {
    let bytes = generate_secure_random_bytes(num_bytes);
    base64::encode(&bytes)
}

pub fn generate_secure_random_bytes(num_bytes: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(num_bytes);
    bytes.resize(num_bytes, 0);
    let mut rng = StdRng::from_entropy();
    rng.try_fill_bytes(&mut bytes).unwrap_or_else(
        |err| log::error!("Failed to generate secure random bytes: {}", err)
    );
    bytes
}
