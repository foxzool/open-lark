//! Document AI 集成测试

use openlark_ai::document_ai::document_ai::v1::recognize::*;
use openlark_ai::prelude::*;

/// 测试简历解析请求构建器
#[test]
fn test_resume_parse_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request = create_resume_parse(config).file_token("test_file_token".to_string());

    // 验证构建器创建成功 - body() 方法返回请求体
    let body = request.body();
    assert_eq!(body.file_token, "test_file_token");
}

/// 测试身份证识别请求构建器
#[test]
fn test_id_card_recognize_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request = create_id_card_recognize(config).file_token("test_file_token".to_string());

    let body = request.body();
    assert_eq!(body.file_token, "test_file_token");
}

/// 测试银行卡识别请求构建器
#[test]
fn test_bank_card_recognize_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request = create_bank_card_recognize(config).file_token("test_file_token".to_string());

    let body = request.body();
    assert_eq!(body.file_token, "test_file_token");
}

/// 测试营业执照识别请求构建器
#[test]
fn test_business_license_recognize_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request =
        create_business_license_recognize(config).file_token("test_file_token".to_string());

    let body = request.body();
    assert_eq!(body.file_token, "test_file_token");
}

/// 测试增值税发票识别请求构建器
#[test]
fn test_vat_invoice_recognize_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request = create_vat_invoice_recognize(config).file_token("test_file_token".to_string());

    let body = request.body();
    assert_eq!(body.file_token, "test_file_token");
}
