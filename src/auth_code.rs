use rand::RngCore;
use rand::rngs::OsRng;

pub struct AuthCode {
    pub key: String,
    pub secret: String,
    pub secret_hash: String,
}

pub fn create_auth_code() -> AuthCode {
    let secret_bytes = random_bytes(16);
    // has the secret bytes, and if that fails, just use an invalid hash
    let secret_hash = hash_bytes(secret_bytes.clone());

    AuthCode {
        key: bytes_to_hex(random_bytes(16)),
        secret: bytes_to_hex(secret_bytes),
        secret_hash,
    }
}

fn random_bytes(len: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; len];
    OsRng.fill_bytes(&mut buffer);
    buffer
}

fn bytes_to_hex(buffer: Vec<u8>) -> String {
    buffer.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .unwrap_or_else(|_| Vec::new())
}



fn hash_bytes(bytes: Vec<u8>) -> String {
    bcrypt::hash(bytes, 12).unwrap_or_else(|_| "INVALID".to_string())
}
