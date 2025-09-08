
//! Builder模式兼容性测试
//! 
//! 测试Builder模式的向后兼容性和新功能

use open_lark::prelude::*;
use open_lark::service::workplace::models::*;

/// 测试Builder模式基本功能
#[tokio::test]
async fn test_builder_basic_functionality() {
    // 测试Builder基本链式调用
    let request = AccessDataSearchRequest::builder()
        .page_size(50)
        .start_time(1609459200)
        .end_time(1640995200)
        .build();
        
    assert_eq!(request.page_size, Some(50));
    assert_eq!(request.start_time, Some(1609459200));
    assert_eq!(request.end_time, Some(1640995200));
}

/// 测试Builder复合方法
#[tokio::test]
async fn test_builder_compound_methods() {
    // 测试复合设置方法
    let request = AccessDataSearchRequest::builder()
        .time_range(1609459200, 1640995200)  // 复合方法
        .pagination(Some("token".to_string()), Some(20))  // 复合方法
        .build();
        
    assert_eq!(request.start_time, Some(1609459200));
    assert_eq!(request.end_time, Some(1640995200));
    assert_eq!(request.page_token, Some("token".to_string()));
    assert_eq!(request.page_size, Some(20));
}

/// 测试Builder便捷方法
#[tokio::test]
async fn test_builder_convenience_methods() {
    // 测试语义化的便捷方法
    let request = AccessDataSearchRequest::builder()
        .user_filter("user123")  // 便捷方法
        .department_filter("dept456")  // 便捷方法
        .access_type_filter("view")  // 便捷方法
        .build();
        
    assert_eq!(request.user_id, Some("user123".to_string()));
    assert_eq!(request.department_id, Some("dept456".to_string()));
    assert_eq!(request.access_type, Some("view".to_string()));
}

/// 测试Builder与传统方式等价性
#[tokio::test]
async fn test_builder_equivalence_with_traditional() {
    // 传统构造方式
    let traditional = AccessDataSearchRequest {
        page_token: Some("token".to_string()),
        page_size: Some(25),
        start_time: Some(1609459200),
        end_time: Some(1640995200),
        user_id: Some("user789".to_string()),
        department_id: None,
        access_type: Some("edit".to_string()),
    };
    
    // Builder构造方式  
    let builder_made = AccessDataSearchRequest::builder()
        .page_token("token")
        .page_size(25)
        .time_range(1609459200, 1640995200)
        .user_filter("user789")
        .access_type_filter("edit")
        .build();
        
    // 应该完全等价
    assert_eq!(traditional.page_token, builder_made.page_token);
    assert_eq!(traditional.page_size, builder_made.page_size);
    assert_eq!(traditional.start_time, builder_made.start_time);
    assert_eq!(traditional.end_time, builder_made.end_time);
    assert_eq!(traditional.user_id, builder_made.user_id);
    assert_eq!(traditional.department_id, builder_made.department_id);
    assert_eq!(traditional.access_type, builder_made.access_type);
}

/// 测试Builder类型转换
#[tokio::test]
async fn test_builder_type_conversions() {
    // 测试Into<String>转换
    let request1 = AccessDataSearchRequest::builder()
        .user_filter("string_literal")  // &str
        .build();
        
    let request2 = AccessDataSearchRequest::builder()
        .user_filter("owned_string".to_string())  // String
        .build();
        
    assert_eq!(request1.user_id, Some("string_literal".to_string()));
    assert_eq!(request2.user_id, Some("owned_string".to_string()));
}
