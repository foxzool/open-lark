use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(timestamp: &str, nonce: &str, secret: &str, signature: &str) -> bool {
    use base64::engine::Engine;
    let content = format!("{}{}", timestamp, nonce);
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(content.as_bytes());
    let computed = base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes());
    computed == signature
}
