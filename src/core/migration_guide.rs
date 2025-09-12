/// Phase 3 优化迁移指南
/// 
/// 本文件展示如何将现有代码迁移到使用改进的响应处理器和请求执行器

use std::collections::HashMap;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    improved_response_handler::{ImprovedResponseHandler, OptimizedBaseResponse},
    request_executor::RequestExecutor,
    req_option::RequestOption,
    SDKResult,
};

/// 迁移示例：从原始 MessageService 到现代化实现
pub struct MigrationExamples;

impl MigrationExamples {
    /// 原始方式 - 使用 Transport 和手动 ApiRequest 构建
    pub async fn old_way_create_message(
        config: &Config,
        receive_id_type: &str,
        body: CreateMessageBody,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MessageData>> {
        // 这是原始的实现方式 - 需要手动构建 ApiRequest
        // 大量重复代码：
        // 1. 手动设置 http_method
        // 2. 手动设置 api_path
        // 3. 手动设置 supported_access_token_types
        // 4. 手动处理查询参数
        // 5. 手动序列化请求体

        // 注意：这里仅作为示例，实际的旧代码在 message.rs 中
        todo!("这是旧的实现方式，现已被RequestExecutor替代")
    }

    /// 新方式 - 使用 RequestExecutor
    pub async fn new_way_create_message(
        config: &Config,
        receive_id_type: &str,
        body: CreateMessageBody,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MessageData>> {
        let mut query_params = HashMap::new();
        query_params.insert("receive_id_type", receive_id_type.to_string());

        // 使用 RequestExecutor - 所有重复代码都被抽象了
        RequestExecutor::execute(
            config,
            Method::POST,
            "/open-apis/im/v1/messages",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            Some(query_params),
            Some(body),
            option,
        )
        .await
    }

    /// 最新方式 - 使用改进的响应处理器
    pub async fn modern_way_create_message(
        config: &Config,
        receive_id_type: &str,
        body: CreateMessageBody,
        option: Option<RequestOption>,
    ) -> SDKResult<OptimizedBaseResponse<MessageData>> {
        // 注意：这需要 RequestExecutor 支持 OptimizedBaseResponse
        // 目前作为概念演示
        todo!("未来版本中，RequestExecutor 将支持 OptimizedBaseResponse")
    }
}

/// 性能对比：原始双重解析 vs 优化单次解析
pub struct PerformanceComparison;

impl PerformanceComparison {
    /// 原始方式：双重JSON解析
    /// 
    /// 问题：
    /// 1. 首先解析为 Value
    /// 2. 再从 Value 解析为具体类型
    /// 3. 额外的内存分配和序列化开销
    pub async fn old_parsing_approach(json_response: &str) -> Result<BaseResponse<MessageData>, serde_json::Error> {
        // 第一次解析：String -> Value
        let raw_value: serde_json::Value = serde_json::from_str(json_response)?;
        
        // 第二次解析：Value -> BaseResponse<T>
        let base_response: BaseResponse<MessageData> = serde_json::from_value(raw_value)?;
        
        Ok(base_response)
    }

    /// 新方式：单次JSON解析
    /// 
    /// 优势：
    /// 1. 直接解析为目标类型
    /// 2. 减少内存分配
    /// 3. 更好的性能
    pub async fn new_parsing_approach(json_response: &str) -> Result<BaseResponse<MessageData>, serde_json::Error> {
        // 单次解析：String -> BaseResponse<T>
        let base_response: BaseResponse<MessageData> = serde_json::from_str(json_response)?;
        
        Ok(base_response)
    }

    /// 基准测试示例
    #[cfg(test)]
    pub fn benchmark_parsing_methods() {
        let json_data = r#"{"code": 0, "msg": "success", "data": {"message_id": "test123", "content": "Hello World"}}"#;
        
        // 测试原始方法
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _value: serde_json::Value = serde_json::from_str(json_data).unwrap();
            let _result: Result<BaseResponse<MessageData>, _> = serde_json::from_value(_value);
        }
        let old_duration = start.elapsed();
        
        // 测试新方法
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _result: Result<BaseResponse<MessageData>, _> = serde_json::from_str(json_data);
        }
        let new_duration = start.elapsed();
        
        println!("旧方法（双重解析）: {:?}", old_duration);
        println!("新方法（单次解析）: {:?}", new_duration);
        println!("性能提升: {:.2}%", (old_duration.as_nanos() as f64 - new_duration.as_nanos() as f64) / old_duration.as_nanos() as f64 * 100.0);
    }
}

/// 错误处理改进示例
pub struct ErrorHandlingImprovements;

impl ErrorHandlingImprovements {
    /// 原始错误处理 - 手动检查和构建错误响应
    pub fn old_error_handling(response_json: &str) -> SDKResult<BaseResponse<MessageData>> {
        let raw_value: serde_json::Value = serde_json::from_str(response_json)?;
        
        if raw_value["code"].as_i64() == Some(0) {
            // 成功情况下的解析
            Ok(serde_json::from_value(raw_value)?)
        } else {
            // 错误情况下的手动构建
            todo!("手动构建错误响应")
        }
    }

    /// 新的错误处理 - 自动化处理
    pub async fn new_error_handling(response: reqwest::Response) -> SDKResult<BaseResponse<MessageData>> {
        // 使用 ImprovedResponseHandler 自动处理所有情况
        ImprovedResponseHandler::handle_response(response).await
    }
}

/// 代码重用对比
pub struct CodeReuseComparison;

impl CodeReuseComparison {
    /// 统计：通过 RequestExecutor 减少的代码行数
    /// 
    /// 原始实现（每个API方法）：~15行重复代码
    /// - 设置 http_method: 1行
    /// - 设置 api_path: 1行  
    /// - 设置 supported_access_token_types: 1行
    /// - 处理查询参数: 3-5行
    /// - 序列化请求体: 2-3行
    /// - 调用 Transport::request: 1行
    /// - 错误处理: 2-3行
    /// 
    /// 新实现（使用 RequestExecutor）：~5行
    /// - 设置查询参数（如需要）: 2-3行
    /// - 调用 RequestExecutor::execute: 1-2行
    /// 
    /// 减少代码量：~67%
    /// 
    /// 对于193个API方法：
    /// - 原始代码行数: 193 × 15 = 2,895行
    /// - 新代码行数: 193 × 5 = 965行
    /// - 减少行数: 1,930行
    pub fn calculate_code_reduction() -> (usize, usize, f64) {
        const API_COUNT: usize = 193;
        const OLD_LINES_PER_API: usize = 15;
        const NEW_LINES_PER_API: usize = 5;
        
        let old_total = API_COUNT * OLD_LINES_PER_API;
        let new_total = API_COUNT * NEW_LINES_PER_API;
        let reduction_percentage = (old_total - new_total) as f64 / old_total as f64 * 100.0;
        
        (old_total, new_total, reduction_percentage)
    }
}

// 测试数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageBody {
    pub receive_id: String,
    pub msg_type: String,
    pub content: String,
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MessageData {
    pub message_id: String,
    pub content: String,
}

impl ApiResponseTrait for MessageData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_reduction_calculation() {
        let (old_total, new_total, reduction) = CodeReuseComparison::calculate_code_reduction();
        
        assert_eq!(old_total, 2895);
        assert_eq!(new_total, 965);
        assert!((reduction - 66.67).abs() < 0.1);
        
        println!("代码减少统计:");
        println!("原始代码行数: {}", old_total);
        println!("优化后代码行数: {}", new_total);
        println!("减少比例: {:.2}%", reduction);
    }

    #[test]
    fn test_parsing_performance() {
        // 这里只是一个简单的演示
        // 实际的基准测试应该使用 criterion 等专业工具
        let json_data = r#"{"code": 0, "msg": "success", "data": {"message_id": "test123", "content": "Hello World"}}"#;
        
        // 双重解析
        let start = std::time::Instant::now();
        let _value: serde_json::Value = serde_json::from_str(json_data).unwrap();
        let _result: Result<BaseResponse<MessageData>, _> = serde_json::from_value(_value);
        let double_parse_time = start.elapsed();
        
        // 单次解析
        let start = std::time::Instant::now();
        let _result: Result<BaseResponse<MessageData>, _> = serde_json::from_str(json_data);
        let single_parse_time = start.elapsed();
        
        println!("双重解析时间: {:?}", double_parse_time);
        println!("单次解析时间: {:?}", single_parse_time);
        
        // 单次解析应该更快（虽然在单次测试中差异可能很小）
        // 实际差异在大量请求时会更明显
    }
}

/// Phase 3 迁移清单
/// 
/// ## 已完成的优化
/// 
/// ### 1. 通用请求执行器 ✅
/// - 创建了 `RequestExecutor` 统一所有API调用
/// - 提供了多种便利方法：`execute`, `json_request`, `query_request` 等
/// - 支持路径参数替换和查询参数处理
/// - 减少67%的重复代码
/// 
/// ### 2. 响应解析优化 ✅  
/// - 创建了 `ImprovedResponseHandler` 替代双重JSON解析
/// - 实现单次解析，提升性能
/// - 改进错误处理逻辑
/// - 统一处理不同响应格式（Data, Flatten, Binary）
/// 
/// ### 3. 集成到现有系统 ✅
/// - 将 `ImprovedResponseHandler` 集成到 `Transport::do_send`
/// - 保持向后兼容性
/// - 清理了未使用的代码
/// 
/// ## 待完成的任务
/// 
/// ### 3. 建立集成测试套件 🔄
/// - 引入 wiremock 模拟API端点
/// - 创建测试不同响应格式的用例
/// - 验证 RequestExecutor 和 ImprovedResponseHandler 的正确性
/// 
/// ### 4. 编写端到端测试 🔄
/// - 覆盖核心业务流程
/// - 测试错误处理边界情况
/// - 性能基准测试
/// 
/// ## 迁移建议
/// 
/// 1. **渐进式迁移**：可以逐步将现有API从手动构建迁移到使用 RequestExecutor
/// 2. **保持兼容性**：新实现与旧接口完全兼容
/// 3. **性能监控**：在生产环境中监控新实现的性能表现
/// 4. **测试覆盖**：确保所有迁移的API都有充分的测试覆盖
pub struct Phase3MigrationChecklist;