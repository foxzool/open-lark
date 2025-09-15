/// 文件上传验证集成测试
///
/// 测试文件上传服务中的验证功能，包括：
/// - IM文件上传验证
/// - Drive文件上传验证
/// - 文件名验证
/// - 文件大小验证
/// - 文件类型验证

use open_lark::{
    core::{
        validation::{validate_file_name, validate_file_extension, validate_upload_file, ValidationResult},
        SDKResult,
    },
    service::cloud_docs::drive::v1::files::UploadAllRequestBuilder,
    service::im::v1::file::FileUploadBuilder,
};

#[test]
fn test_im_file_upload_builder_validation() {
    // 测试正常情况
    let file_data = b"Hello, World!".to_vec();
    let result = FileUploadBuilder::new()
        .file_type("image")
        .file_name("test.jpg")
        .file_data(file_data.clone())
        .build();

    assert!(result.is_ok());
    
    let request = result.unwrap();
    assert_eq!(request.file_type, "image");
    assert_eq!(request.file_name, "test.jpg");
    assert_eq!(request.file_data, file_data);
}

#[test]
fn test_im_file_upload_builder_empty_file_type() {
    let file_data = b"Hello, World!".to_vec();
    let result = FileUploadBuilder::new()
        .file_name("test.jpg")
        .file_data(file_data)
        .build();

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("file_type is required"));
}

#[test]
fn test_im_file_upload_builder_empty_file_name() {
    let file_data = b"Hello, World!".to_vec();
    let result = FileUploadBuilder::new()
        .file_type("image")
        .file_name("")
        .file_data(file_data)
        .build();

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Invalid file_name"));
}

#[test]
fn test_im_file_upload_builder_empty_file_data() {
    let result = FileUploadBuilder::new()
        .file_type("image")
        .file_name("test.jpg")
        .file_data(Vec::new())
        .build();

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("file_data cannot be empty"));
}

#[test]
fn test_im_file_upload_builder_invalid_file_name() {
    let file_data = b"Hello, World!".to_vec();
    let result = FileUploadBuilder::new()
        .file_type("image")
        .file_name("test/file.jpg") // 包含非法字符
        .file_data(file_data)
        .build();

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Invalid file_name"));
}

#[test]
fn test_im_file_upload_builder_too_large_file() {
    // 创建一个超过IM限制的大文件
    let file_data = vec![0u8; 21 * 1024 * 1024]; // 21MB
    let result = FileUploadBuilder::new()
        .file_type("image")
        .file_name("large.jpg")
        .file_data(file_data)
        .build();

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("File validation failed"));
}

#[test]
fn test_im_file_upload_builder_validate_trait() {
    let builder = FileUploadBuilder::new()
        .file_type("image")
        .file_name("test.jpg")
        .file_data(b"Hello, World!".to_vec());

    let validation_result = builder.validate();
    assert!(validation_result.is_valid());
}

#[test]
fn test_drive_file_upload_builder_validation() {
    let file_data = b"Hello, Drive!".to_vec();
    let request = UploadAllRequestBuilder::default()
        .file_name("document.pdf")
        .parent_type("explorer")
        .parent_node("folder_token")
        .size(file_data.len() as i32)
        .file(file_data.clone())
        .build();

    // 验证构建成功
    assert_eq!(request.file_name, "document.pdf");
    assert_eq!(request.parent_type, "explorer");
    assert_eq!(request.parent_node, "folder_token");
    assert_eq!(request.size, file_data.len() as i32);
    assert_eq!(request.api_req.file, file_data);
}

#[test]
fn test_drive_file_upload_builder_missing_required_fields() {
    // 测试缺少必填字段的情况
    let request = UploadAllRequestBuilder::default()
        .file_name("") // 空文件名
        .parent_type("explorer")
        .parent_node("folder_token")
        .size(100)
        .file(b"data".to_vec())
        .build();

    // 应该返回一个带有空body的请求
    assert!(request.api_req.body.is_empty());
}

#[test]
fn test_drive_file_upload_builder_invalid_size() {
    let request = UploadAllRequestBuilder::default()
        .file_name("test.txt")
        .parent_type("explorer")
        .parent_node("folder_token")
        .size(-1) // 无效大小
        .file(b"data".to_vec())
        .build();

    // 应该返回一个带有空body的请求
    assert!(request.api_req.body.is_empty());
}

#[test]
fn test_drive_file_upload_builder_validate_trait() {
    let builder = UploadAllRequestBuilder::default()
        .file_name("document.pdf")
        .parent_type("explorer")
        .parent_node("folder_token")
        .size(100)
        .file(b"file content".to_vec());

    let validation_result = builder.validate();
    assert!(validation_result.is_valid());
}

#[test]
fn test_drive_file_upload_builder_validate_trait_missing_field() {
    let builder = UploadAllRequestBuilder::default()
        .file_name("") // 空文件名
        .parent_type("explorer")
        .parent_node("folder_token")
        .size(100)
        .file(b"file content".to_vec());

    let validation_result = builder.validate();
    assert!(!validation_result.is_valid());
    assert!(validation_result.error().unwrap().contains("file_name is required"));
}

#[test]
fn test_file_name_validation() {
    // 测试正常文件名
    let (name, result) = validate_file_name("test_document.pdf");
    assert_eq!(name, "test_document.pdf");
    assert!(result.is_valid());

    // 测试空文件名
    let (name, result) = validate_file_name("");
    assert!(name.is_empty());
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("cannot be empty"));

    // 测试过长文件名
    let long_name = "a".repeat(300);
    let (name, result) = validate_file_name(&long_name);
    assert!(name.is_empty());
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("exceeds maximum length"));

    // 测试非法字符
    let (name, result) = validate_file_name("test/file.txt");
    assert!(name.is_empty());
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("invalid character"));

    // 测试保留文件名
    let (name, result) = validate_file_name("CON.txt");
    assert!(name.is_empty());
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("reserved file name"));
}

#[test]
fn test_file_extension_validation() {
    // 测试允许的扩展名
    let allowed_types = vec!["jpg", "png", "pdf", "doc", "docx"];
    
    let (ext, result) = validate_file_extension("image.jpg", &allowed_types);
    assert_eq!(ext, Some("jpg".to_string()));
    assert!(result.is_valid());

    // 测试不允许的扩展名
    let (ext, result) = validate_file_extension("script.exe", &allowed_types);
    assert_eq!(ext, None);
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("not allowed"));

    // 测试无扩展名
    let (ext, result) = validate_file_extension("noextension", &allowed_types);
    assert_eq!(ext, None);
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("must have an extension"));

    // 测试空扩展名
    let (ext, result) = validate_file_extension("file.", &allowed_types);
    assert_eq!(ext, None);
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("cannot be empty"));
}

#[test]
fn test_upload_file_validation() {
    // 测试正常文件
    let file_data = b"Hello, World!".to_vec();
    let result = validate_upload_file(&file_data, "test.txt", false);
    assert!(result.is_valid());

    // 测试过大文件
    let large_file = vec![0u8; 101 * 1024 * 1024]; // 101MB
    let result = validate_upload_file(&large_file, "large.bin", false);
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("exceeds maximum size"));

    // 测试IM上传大小限制
    let im_large_file = vec![0u8; 21 * 1024 * 1024]; // 21MB
    let result = validate_upload_file(&im_large_file, "large.jpg", true);
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("exceeds maximum size"));

    // 测试无效文件名
    let file_data = b"test".to_vec();
    let result = validate_upload_file(&file_data, "test/file.txt", false);
    assert!(!result.is_valid());
    assert!(result.error().unwrap().contains("invalid character"));
}

#[test]
fn test_im_file_upload_builder_unvalidated() {
    // 测试无验证构建方法（向后兼容）
    let file_data = b"Test content".to_vec();
    let request = FileUploadBuilder::new()
        .file_type("") // 空文件类型通常会失败
        .file_name("") // 空文件名通常会失败
        .file_data(Vec::new()) // 空文件数据通常会失败
        .build_unvalidated();

    // 无验证构建应该总是成功
    assert_eq!(request.file_type, "");
    assert_eq!(request.file_name, "");
    assert!(request.file_data.is_empty());
}