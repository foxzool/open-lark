//! Webhook 模块输入校验工具。

use openlark_core::SDKResult;

/// 校验 webhook URL 非空。
pub fn validate_webhook_url(url: &str) -> SDKResult<()> {
    if url.trim().is_empty() {
        return Err(openlark_core::CoreError::validation_msg(
            "Webhook URL cannot be empty",
        ));
    }
    Ok(())
}

/// 校验消息内容非空。
pub fn validate_message_content(content: &str) -> SDKResult<()> {
    if content.trim().is_empty() {
        return Err(openlark_core::CoreError::validation_msg(
            "Message content cannot be empty",
        ));
    }
    Ok(())
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_webhook_url_empty() {
        assert!(validate_webhook_url("").is_err());
        assert!(validate_webhook_url("   ").is_err());
    }

    #[test]
    fn test_validate_webhook_url_valid() {
        assert!(validate_webhook_url("https://example.com/webhook").is_ok());
    }

    #[test]
    fn test_validate_message_content_empty() {
        assert!(validate_message_content("").is_err());
        assert!(validate_message_content("   ").is_err());
    }

    #[test]
    fn test_validate_message_content_valid() {
        assert!(validate_message_content("Hello, World!").is_ok());
    }
}
