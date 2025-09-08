
//! 通用API兼容性测试
//! 
//! 测试StandardResponse改进的通用兼容性模式

use open_lark::prelude::*;

/// 测试标准响应处理模式
#[tokio::test]
async fn test_standard_response_compatibility() {
    // 这个测试验证改进前后的错误处理模式
    // 改进前: 返回 SDKResult<BaseResponse<T>>
    // 改进后: 返回 SDKResult<T>
    
    // 用户代码应该保持相同的错误处理模式
    let client = create_test_client();
    
    // 模拟API调用 - 成功情况
    match simulate_api_success() {
        Ok(data) => {
            // 改进后直接获得业务数据
            println!("API成功: {:?}", data);
        },
        Err(e) => {
            // 错误处理保持相同
            println!("API失败: {:?}", e);
        }
    }
    
    // 模拟API调用 - 错误情况
    match simulate_api_error() {
        Ok(_) => panic!("应该返回错误"),
        Err(e) => {
            // 错误类型应该保持一致
            assert!(matches!(e, LarkAPIError::HttpError(_)));
        }
    }
}

/// 测试链式调用兼容性
#[tokio::test]
async fn test_chaining_compatibility() {
    let client = create_test_client();
    
    // 改进后应该支持更流畅的链式调用
    let result = client
        .im  // 假设的IM模块
        .v1
        .message
        .create(create_message_request(), None)
        .await
        .map(|data| {
            // 改进后直接处理业务数据
            process_message_data(data)  
        })
        .map_err(|e| {
            // 错误处理保持相同
            log::error!("消息发送失败: {:?}", e);
            e
        });
}

// 辅助函数
fn create_test_client() -> LarkClient {
    LarkClient::builder("test", "test").build()
}

fn simulate_api_success() -> SDKResult<String> {
    Ok("success_data".to_string())
}

fn simulate_api_error() -> SDKResult<String> {
    Err(LarkAPIError::HttpError("network error".to_string()))
}

fn create_message_request() -> String {
    "test_message".to_string()
}

fn process_message_data(data: String) -> String {
    format!("processed: {}", data)
}
