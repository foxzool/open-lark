//! 加密工具模块

use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};
use tracing::debug;

type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;

/// 加密工具
pub struct CryptoUtils;

impl CryptoUtils {
    /// SHA256哈希
    pub fn sha256_hash(data: &[u8]) -> String {
        debug!("Computing SHA256 hash for {} bytes", data.len());
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// SHA256哈希（字符串输入）
    pub fn sha256_hash_string(data: &str) -> String {
        Self::sha256_hash(data.as_bytes())
    }

    /// SHA512哈希
    pub fn sha512_hash(data: &[u8]) -> String {
        debug!("Computing SHA512 hash for {} bytes", data.len());
        let mut hasher = Sha512::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// SHA512哈希（字符串输入）
    pub fn sha512_hash_string(data: &str) -> String {
        Self::sha512_hash(data.as_bytes())
    }

    /// HMAC-SHA256签名
    pub fn hmac_sha256_sign(key: &[u8], data: &[u8]) -> Result<String, crate::error::AuthError> {
        debug!("Signing {} bytes with HMAC-SHA256", data.len());

        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| crate::error::AuthError::CryptoError(format!("Invalid key: {}", e)))?;
        mac.update(data);
        let result = mac.finalize();

        Ok(hex::encode(result.into_bytes()))
    }

    /// HMAC-SHA256签名（字符串输入）
    pub fn hmac_sha256_sign_string(
        key: &str,
        data: &str,
    ) -> Result<String, crate::error::AuthError> {
        Self::hmac_sha256_sign(key.as_bytes(), data.as_bytes())
    }

    /// HMAC-SHA256验证
    pub fn hmac_sha256_verify(
        key: &[u8],
        data: &[u8],
        signature: &str,
    ) -> Result<bool, crate::error::AuthError> {
        debug!("Verifying HMAC-SHA256 signature");

        let expected_signature = Self::hmac_sha256_sign(key, data)?;
        let decoded_signature = hex::decode(signature).map_err(|e| {
            crate::error::AuthError::CryptoError(format!("Invalid signature format: {}", e))
        })?;

        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| crate::error::AuthError::CryptoError(format!("Invalid key: {}", e)))?;
        mac.update(data);

        Ok(mac.verify_slice(&decoded_signature).is_ok())
    }

    /// HMAC-SHA256验证（字符串输入）
    pub fn hmac_sha256_verify_string(
        key: &str,
        data: &str,
        signature: &str,
    ) -> Result<bool, crate::error::AuthError> {
        Self::hmac_sha256_verify(key.as_bytes(), data.as_bytes(), signature)
    }

    /// HMAC-SHA512签名
    pub fn hmac_sha512_sign(key: &[u8], data: &[u8]) -> Result<String, crate::error::AuthError> {
        debug!("Signing {} bytes with HMAC-SHA512", data.len());

        let mut mac = HmacSha512::new_from_slice(key)
            .map_err(|e| crate::error::AuthError::CryptoError(format!("Invalid key: {}", e)))?;
        mac.update(data);
        let result = mac.finalize();

        Ok(hex::encode(result.into_bytes()))
    }

    /// HMAC-SHA512签名（字符串输入）
    pub fn hmac_sha512_sign_string(
        key: &str,
        data: &str,
    ) -> Result<String, crate::error::AuthError> {
        Self::hmac_sha512_sign(key.as_bytes(), data.as_bytes())
    }

    /// HMAC-SHA512验证
    pub fn hmac_sha512_verify(
        key: &[u8],
        data: &[u8],
        signature: &str,
    ) -> Result<bool, crate::error::AuthError> {
        debug!("Verifying HMAC-SHA512 signature");

        let expected_signature = Self::hmac_sha512_sign(key, data)?;
        let decoded_signature = hex::decode(signature).map_err(|e| {
            crate::error::AuthError::CryptoError(format!("Invalid signature format: {}", e))
        })?;

        let mut mac = HmacSha512::new_from_slice(key)
            .map_err(|e| crate::error::AuthError::CryptoError(format!("Invalid key: {}", e)))?;
        mac.update(data);

        Ok(mac.verify_slice(&decoded_signature).is_ok())
    }

    /// HMAC-SHA512验证（字符串输入）
    pub fn hmac_sha512_verify_string(
        key: &str,
        data: &str,
        signature: &str,
    ) -> Result<bool, crate::error::AuthError> {
        Self::hmac_sha512_verify(key.as_bytes(), data.as_bytes(), signature)
    }

    /// Base64编码
    pub fn base64_encode(data: &[u8]) -> String {
        debug!("Encoding {} bytes to Base64", data.len());
        general_purpose::STANDARD.encode(data)
    }

    /// Base64编码（字符串输入）
    pub fn base64_encode_string(data: &str) -> String {
        Self::base64_encode(data.as_bytes())
    }

    /// Base64解码
    pub fn base64_decode(data: &str) -> Result<Vec<u8>, crate::error::AuthError> {
        debug!("Decoding Base64 data");
        general_purpose::STANDARD.decode(data).map_err(|e| {
            crate::error::AuthError::CryptoError(format!("Base64 decode error: {}", e))
        })
    }

    /// 生成随机字符串
    pub fn generate_random_string(length: usize) -> String {
        use rand::{thread_rng, Rng};
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789";
        let mut rng = thread_rng();

        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// 生成随机字节
    pub fn generate_random_bytes(length: usize) -> Vec<u8> {
        use rand::{thread_rng, RngCore};
        let mut rng = thread_rng();
        let mut bytes = vec![0u8; length];
        rng.fill_bytes(&mut bytes);
        bytes
    }

    /// 生成安全随机字符串（包含特殊字符）
    pub fn generate_secure_random_string(length: usize) -> String {
        use rand::{thread_rng, Rng};
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789\
                                 !@#$%^&*()_+-=[]{}|;:,.<>?";
        let mut rng = thread_rng();

        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// 安全比较两个字符串（防止时序攻击）
    pub fn secure_compare(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (byte_a, byte_b) in a.bytes().zip(b.bytes()) {
            result |= byte_a ^ byte_b;
        }

        result == 0
    }

    /// 安全比较两个字节数组（防止时序攻击）
    pub fn secure_compare_bytes(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (byte_a, byte_b) in a.iter().zip(b.iter()) {
            result |= byte_a ^ byte_b;
        }

        result == 0
    }

    /// 派生密钥（基于密码）
    pub fn derive_key_from_password(password: &str, salt: &[u8], iterations: u32) -> Vec<u8> {
        use pbkdf2::pbkdf2_hmac;
        use sha2::Sha256;

        debug!("Deriving key from password with {} iterations", iterations);
        let mut key = vec![0u8; 32]; // 256-bit key
        pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, iterations, &mut key);
        key
    }

    /// 生成盐值
    pub fn generate_salt(length: usize) -> Vec<u8> {
        Self::generate_random_bytes(length)
    }

    /// 常量时间比较（用于令牌验证）
    pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (byte_a, byte_b) in a.iter().zip(b.iter()) {
            result |= byte_a ^ byte_b;
        }

        result == 0
    }
}

/// 密码哈希器
pub struct PasswordHasher {
    iterations: u32,
    salt_length: usize,
}

impl PasswordHasher {
    /// 创建新的密码哈希器
    pub fn new(iterations: u32, salt_length: usize) -> Self {
        Self {
            iterations,
            salt_length,
        }
    }

    /// 哈希密码
    pub fn hash_password(&self, password: &str) -> Result<String, crate::error::AuthError> {
        debug!("Hashing password with {} iterations", self.iterations);

        let salt = CryptoUtils::generate_salt(self.salt_length);
        let key = CryptoUtils::derive_key_from_password(password, &salt, self.iterations);

        let salt_b64 = CryptoUtils::base64_encode(&salt);
        let key_b64 = CryptoUtils::base64_encode(&key);

        Ok(format!("${}:${}:${}", self.iterations, salt_b64, key_b64))
    }

    /// 验证密码
    pub fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> Result<bool, crate::error::AuthError> {
        debug!("Verifying password hash");

        let parts: Vec<&str> = hash.split(':').collect();
        if parts.len() != 3 {
            return Err(crate::error::AuthError::CryptoError(
                "Invalid hash format".to_string(),
            ));
        }

        let iterations = parts[0]
            .parse::<u32>()
            .map_err(|_| crate::error::AuthError::CryptoError("Invalid iterations".to_string()))?;
        let salt = CryptoUtils::base64_decode(parts[1])?;
        let expected_key = CryptoUtils::base64_decode(parts[2])?;

        let derived_key = CryptoUtils::derive_key_from_password(password, &salt, iterations);

        Ok(CryptoUtils::constant_time_compare(
            &derived_key,
            &expected_key,
        ))
    }
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new(100000, 16) // 100k iterations, 16-byte salt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let data = b"hello world";
        let hash = CryptoUtils::sha256_hash(data);

        // 验证哈希长度
        assert_eq!(hash.len(), 64); // SHA256 produces 32 bytes = 64 hex chars

        // 验证一致性
        let hash2 = CryptoUtils::sha256_hash(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_hmac_sha256() {
        let key = b"secret_key";
        let data = b"message";

        let signature = CryptoUtils::hmac_sha256_sign(key, data).unwrap();
        let is_valid = CryptoUtils::hmac_sha256_verify(key, data, &signature).unwrap();

        assert!(is_valid);

        // 测试错误数据
        let wrong_data = b"wrong_message";
        let is_invalid = CryptoUtils::hmac_sha256_verify(key, wrong_data, &signature).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_base64_encode_decode() {
        let data = b"hello world";
        let encoded = CryptoUtils::base64_encode(data);
        let decoded = CryptoUtils::base64_decode(&encoded).unwrap();

        assert_eq!(data.to_vec(), decoded);
    }

    #[test]
    fn test_random_string_generation() {
        let s1 = CryptoUtils::generate_random_string(10);
        let s2 = CryptoUtils::generate_random_string(10);

        assert_eq!(s1.len(), 10);
        assert_eq!(s2.len(), 10);
        assert_ne!(s1, s2); // 应该不同
    }

    #[test]
    fn test_secure_compare() {
        let a = "hello";
        let b = "hello";
        let c = "world";

        assert!(CryptoUtils::secure_compare(a, b));
        assert!(!CryptoUtils::secure_compare(a, c));
        assert!(!CryptoUtils::secure_compare("short", "longer"));
    }

    #[test]
    fn test_password_hasher() {
        let hasher = PasswordHasher::default();
        let password = "test_password_123";

        let hash = hasher.hash_password(password).unwrap();
        assert!(!hash.is_empty());
        assert!(hash.contains(':'));

        let is_valid = hasher.verify_password(password, &hash).unwrap();
        assert!(is_valid);

        let is_invalid = hasher.verify_password("wrong_password", &hash).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_key_derivation() {
        let password = "test_password";
        let salt = b"some_salt";
        let iterations = 1000;

        let key1 = CryptoUtils::derive_key_from_password(password, salt, iterations);
        let key2 = CryptoUtils::derive_key_from_password(password, salt, iterations);

        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32); // 256-bit key
    }

    #[test]
    fn test_constant_time_compare() {
        let a = b"hello";
        let b = b"hello";
        let c = b"world";

        assert!(CryptoUtils::constant_time_compare(a, b));
        assert!(!CryptoUtils::constant_time_compare(a, c));
        assert!(!CryptoUtils::constant_time_compare(b"short", b"longer"));
    }
}
