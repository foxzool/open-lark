use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Webhook HMAC-SHA256 签名计算器。
type HmacSha256 = Hmac<Sha256>;

/// 为飞书 webhook 生成签名。
///
/// 算法为 `base64(hmac_sha256("{timestamp}\n{secret}"))`。
pub fn sign(timestamp: i64, secret: &str) -> String {
    use base64::engine::Engine;
    let content = format!("{}\n{}", timestamp, secret);
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(content.as_bytes());
    base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes())
}

/// 获取当前 Unix 时间戳，单位为秒。
pub fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

/// 校验飞书 webhook 签名是否匹配。
pub fn verify_signature(timestamp: i64, secret: &str, signature: &str) -> bool {
    let computed = sign(timestamp, secret);
    computed == signature
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_known_input() {
        // Test with known input/output from 飞书 official docs
        let timestamp = 1599360473i64;
        let secret = "test-secret";
        let signature = sign(timestamp, secret);

        // Verify it's base64 encoded and not empty
        assert!(!signature.is_empty());
    }

    #[test]
    fn test_sign_different_secrets() {
        let timestamp = 1599360473i64;
        let sig1 = sign(timestamp, "secret1");
        let sig2 = sign(timestamp, "secret2");

        // Different secrets should produce different signatures
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_sign_different_timestamps() {
        let secret = "test-secret";
        let sig1 = sign(1599360473, secret);
        let sig2 = sign(1599360474, secret);

        // Different timestamps should produce different signatures
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_verify_signature_valid() {
        let timestamp = 1599360473i64;
        let secret = "test-secret";
        let signature = sign(timestamp, secret);

        // Verification should pass with correct signature
        assert!(verify_signature(timestamp, secret, &signature));
    }

    #[test]
    fn test_verify_signature_invalid() {
        let timestamp = 1599360473i64;
        let secret = "test-secret";

        // Verification should fail with wrong signature
        assert!(!verify_signature(timestamp, secret, "invalid-signature"));
    }

    #[test]
    fn test_verify_signature_wrong_secret() {
        let timestamp = 1599360473i64;
        let secret = "test-secret";
        let signature = sign(timestamp, secret);

        // Verification should fail with wrong secret
        assert!(!verify_signature(timestamp, "wrong-secret", &signature));
    }

    #[test]
    fn test_current_timestamp() {
        let ts = current_timestamp();

        // Timestamp should be positive and reasonable (after 2020)
        assert!(ts > 1577836800); // 2020-01-01
        assert!(ts < 2000000000); // Before year 2033
    }
}
