//! AES-256-GCM 加解密模块
//!
//! 用于加密登录请求和解密响应，防止明文传输被抓包

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use std::fmt;

/// Nonce 长度（12 字节，GCM 推荐值）
const NONCE_LENGTH: usize = 12;

/// AuthTag 长度（16 字节）
const AUTH_TAG_LENGTH: usize = 16;

/// 共享加密密钥（32 字节 = 256 位）
/// 重要：客户端和服务端必须使用相同的密钥！
/// 十六进制密钥：5ee88f388e79950a48e7f84f42676d5fa9701549844354427374f20cf1e35d63
const ENCRYPTION_KEY: &[u8; 32] = &[
    0x5e, 0xe8, 0x8f, 0x38, 0x8e, 0x79, 0x95, 0x0a, 0x48, 0xe7, 0xf8, 0x4f, 0x42, 0x67, 0x6d, 0x5f,
    0xa9, 0x70, 0x15, 0x49, 0x84, 0x43, 0x54, 0x42, 0x73, 0x74, 0xf2, 0x0c, 0xf1, 0xe3, 0x5d, 0x63,
];

/// 加密错误类型
#[derive(Debug)]
pub enum CryptoError {
    /// 密钥长度错误
    InvalidKeyLength,
    /// Base64 解码失败
    Base64DecodeError,
    /// 密文格式错误（长度不足）
    InvalidCiphertext,
    /// 解密失败（认证标签不匹配）
    DecryptionFailed,
    /// 加密失败
    EncryptionFailed,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::InvalidKeyLength => write!(f, "密钥长度必须为 32 字节"),
            CryptoError::Base64DecodeError => write!(f, "Base64 解码失败"),
            CryptoError::InvalidCiphertext => write!(f, "密文格式错误：长度不足"),
            CryptoError::DecryptionFailed => write!(f, "解密失败：数据可能被篡改"),
            CryptoError::EncryptionFailed => write!(f, "加密失败"),
        }
    }
}

impl std::error::Error for CryptoError {}


/// AES-256-GCM 加密
///
/// 输入：明文字符串
/// 输出：Base64(Nonce + Ciphertext + AuthTag)
///
/// # 示例
/// ```
/// let encrypted = encrypt("hello world").unwrap();
/// ```
pub fn encrypt(plaintext: &str) -> Result<String, CryptoError> {
    encrypt_with_key(plaintext, ENCRYPTION_KEY)
}

/// 使用指定密钥进行 AES-256-GCM 加密
pub fn encrypt_with_key(plaintext: &str, key: &[u8; 32]) -> Result<String, CryptoError> {
    // 创建加密器
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidKeyLength)?;

    // 生成随机 Nonce
    let mut nonce_bytes = [0u8; NONCE_LENGTH];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 加密（结果包含 ciphertext + auth_tag）
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| CryptoError::EncryptionFailed)?;

    // 组合：Nonce + Ciphertext（已包含 AuthTag）
    let mut combined = Vec::with_capacity(NONCE_LENGTH + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    // Base64 编码
    Ok(BASE64.encode(&combined))
}

/// AES-256-GCM 解密
///
/// 输入：Base64 编码的密文
/// 输出：明文字符串
///
/// # 示例
/// ```
/// let decrypted = decrypt(encrypted_text).unwrap();
/// ```
pub fn decrypt(ciphertext_b64: &str) -> Result<String, CryptoError> {
    decrypt_with_key(ciphertext_b64, ENCRYPTION_KEY)
}

/// 使用指定密钥进行 AES-256-GCM 解密
pub fn decrypt_with_key(ciphertext_b64: &str, key: &[u8; 32]) -> Result<String, CryptoError> {
    // Base64 解码
    let combined = BASE64
        .decode(ciphertext_b64)
        .map_err(|_| CryptoError::Base64DecodeError)?;

    // 检查最小长度（Nonce + AuthTag）
    let min_length = NONCE_LENGTH + AUTH_TAG_LENGTH;
    if combined.len() < min_length {
        return Err(CryptoError::InvalidCiphertext);
    }

    // 提取 Nonce 和密文
    let nonce = Nonce::from_slice(&combined[..NONCE_LENGTH]);
    let ciphertext = &combined[NONCE_LENGTH..];

    // 创建解密器
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidKeyLength)?;

    // 解密
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed)?;

    String::from_utf8(plaintext).map_err(|_| CryptoError::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 基本加解密测试
    #[test]
    fn test_encrypt_decrypt_basic() {
        let original = "Hello, 世界! 🎉";
        let encrypted = encrypt(original).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }

    /// 空字符串测试
    #[test]
    fn test_encrypt_decrypt_empty() {
        let original = "";
        let encrypted = encrypt(original).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }

    /// 长字符串测试
    #[test]
    fn test_encrypt_decrypt_long() {
        let original = "a".repeat(10000);
        let encrypted = encrypt(&original).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }

    /// 无效 Base64 测试
    #[test]
    fn test_decrypt_invalid_base64() {
        let result = decrypt("not-valid-base64!!!");
        assert!(matches!(result, Err(CryptoError::Base64DecodeError)));
    }

    /// 密文过短测试
    #[test]
    fn test_decrypt_short_ciphertext() {
        let short = BASE64.encode(&[0u8; 10]); // 小于 28 字节
        let result = decrypt(&short);
        assert!(matches!(result, Err(CryptoError::InvalidCiphertext)));
    }

    /// 篡改密文测试
    #[test]
    fn test_decrypt_tampered() {
        let encrypted = encrypt("test").unwrap();
        let mut bytes = BASE64.decode(&encrypted).unwrap();
        // 篡改密文中间部分
        if bytes.len() > 20 {
            bytes[20] ^= 0xFF;
        }
        let tampered = BASE64.encode(&bytes);
        let result = decrypt(&tampered);
        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }

    /// 相同明文产生不同密文测试
    #[test]
    fn test_different_ciphertext() {
        let plaintext = "same text";
        let encrypted1 = encrypt(plaintext).unwrap();
        let encrypted2 = encrypt(plaintext).unwrap();
        assert_ne!(encrypted1, encrypted2, "相同明文应产生不同密文");
    }
}
