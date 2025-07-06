use rand::RngCore;
use rand::rngs::OsRng;

/*
 * The idea here is that an auth code has a component that helps you find the "row" in the table,
 * and a secret component that gets compared against the hash stored in that "row"
 */

const SEPARATOR: char = '.';

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

pub struct DecodedAuthCode {
    pub key: String,
    pub secret_bytes: Vec<u8>,
}

pub fn parse_auth_code(auth_code: &str) -> DecodedAuthCode {
    let parts: Vec<&str> = auth_code.splitn(2, SEPARATOR).collect();
    let key = parts[0].to_string();
    let secret_hex = parts[1];
    DecodedAuthCode {
        key,
        secret_bytes: hex_to_bytes(secret_hex),
    }
}

pub fn verify_auth_code(auth_code: DecodedAuthCode, key: &str, hash: &str) -> bool {
    if !key.eq(auth_code.key.as_str()) {
        return false;
    }
    bcrypt::verify(auth_code.secret_bytes, hash).unwrap_or_else(|_| false)
}

pub fn format_auth_code(auth_code: &AuthCode) -> String {
    format!("{}{}{}", auth_code.key, SEPARATOR, auth_code.secret)
}

fn hash_bytes(bytes: Vec<u8>) -> String {
    bcrypt::hash(bytes, 12).unwrap_or_else(|_| "INVALID".to_string())
}

fn random_bytes(len: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; len];
    OsRng.fill_bytes(&mut buffer);
    buffer
}

fn bytes_to_hex(buffer: Vec<u8>) -> String {
    buffer.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .unwrap_or_else(|_| Vec::new())
}
