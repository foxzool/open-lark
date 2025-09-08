
//! Workplace模块兼容性参考测试
//! 
//! 此文件包含改进前后应该保持兼容的API使用模式

use open_lark::prelude::*;
use open_lark::service::workplace::models::*;

/// 测试改进前的API使用方式
#[tokio::test]
async fn test_workplace_api_before_improvement() {
    let client = create_test_client();
    
    // 原始请求构建方式应该继续工作
    let request = AccessDataSearchRequest {
        page_token: Some("token123".to_string()),
        page_size: Some(20),
        start_time: Some(1609459200),
        end_time: Some(1640995200),
        user_id: Some("user123".to_string()),
        department_id: None,
        access_type: Some("view".to_string()),
    };
    
    // API调用方式 - 这里我们期望返回类型改变但调用方式保持相同
    let result = client.workplace.workplace_access_data
        .search(request, None)
        .await;
        
    // 错误处理模式应该保持相同
    match result {
        Ok(data) => {
            // 改进后：直接获得业务数据 
            println!("搜索结果: {:?}", data);
        },
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
}

/// 测试改进后的API使用方式（应该向前兼容）
#[tokio::test] 
async fn test_workplace_api_after_improvement() {
    let client = create_test_client();
    
    // 新的Builder模式应该工作
    let request = AccessDataSearchRequest::builder()
        .page_size(20)
        .time_range(1609459200, 1640995200)
        .user_filter("user123")
        .access_type_filter("view")
        .build();
        
    // API调用保持相同
    let result = client.workplace.workplace_access_data
        .search(request, None)
        .await;
        
    // 错误处理保持相同  
    match result {
        Ok(data) => {
            // 改进后：直接获得业务数据，无需.data访问
            println!("搜索结果: {:?}", data);
        },
        Err(e) => {
            println!("搜索失败: {:?}", e);
        }
    }
}

/// 测试混合使用模式（向后兼容性关键测试）
#[tokio::test]
async fn test_mixed_usage_patterns() {
    let client = create_test_client();
    
    // 旧的直接构造方式
    let old_request = AccessDataSearchRequest {
        page_size: Some(10),
        user_id: Some("user456".to_string()),
        ..Default::default()
    };
    
    // 新的Builder方式
    let new_request = AccessDataSearchRequest::builder()
        .page_size(10)
        .user_filter("user456")
        .build();
        
    // 两种请求对象应该产生相同结果
    let result1 = client.workplace.workplace_access_data
        .search(old_request, None)
        .await;
    let result2 = client.workplace.workplace_access_data
        .search(new_request, None)  
        .await;
        
    // 验证结果一致性
    assert_eq!(result1.is_ok(), result2.is_ok());
}

fn create_test_client() -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .with_app_type(AppType::SelfBuild)
        .build()
}
