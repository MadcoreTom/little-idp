use rand::RngCore;
use rand::rngs::OsRng;

pub struct AuthCode {
    pub key: String,
    pub secret: String,
    pub secret_bytes: Vec<u8>,
}

pub fn create_auth_code() -> AuthCode {
    let secret_bytes = random_bytes(16);
    AuthCode {
        key: bytes_to_hex(random_bytes(16)),
        secret: bytes_to_hex(secret_bytes.clone()),
        secret_bytes: secret_bytes,
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
