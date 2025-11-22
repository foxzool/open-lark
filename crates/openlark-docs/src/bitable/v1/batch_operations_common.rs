//! 批量操作通用模块
//!
//! 提供批量操作的通用类型和功能，减少代码重复。

use serde::{Deserialize, Serialize};

/// 批量操作的通用参数
#[derive(Clone, Debug)]
pub struct BatchCommonParams {
    /// 用户 ID 类型
    #[serde(skip)]
    pub user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    #[serde(skip)]
    pub client_token: Option<String>,
}

impl BatchCommonParams {
    /// 创建新的批量操作通用参数
    pub fn new() -> Self {
        Self {
            user_id_type: None,
            client_token: None,
        }
    }

    /// 设置用户 ID 类型
    pub fn with_user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置客户端令牌，用于幂等操作
    pub fn with_client_token(mut self, client_token: impl ToString) -> Self {
        self.client_token = Some(client_token.to_string());
        self
    }
}

/// 批量操作的基础请求体结构
#[derive(Serialize, Debug)]
pub struct BatchCommonBody<T> {
    /// 具体的操作数据
    pub data: T,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 客户端令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl<T> BatchCommonBody<T> {
    /// 创建新的批量操作请求体
    pub fn new(data: T) -> Self {
        Self {
            data,
            user_id_type: None,
            client_token: None,
        }
    }

    /// 添加通用参数
    pub fn with_common_params(mut self, params: BatchCommonParams) -> Self {
        self.user_id_type = params.user_id_type;
        self.client_token = params.client_token;
        self
    }
}

/// 批量操作结果的基础结构
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BatchOperationResult {
    /// 操作是否成功
    pub success: bool,
    /// 错误信息（如果操作失败）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl BatchOperationResult {
    /// 创建成功的操作结果
    pub fn success() -> Self {
        Self {
            success: true,
            error: None,
        }
    }

    /// 创建失败的操作结果
    pub fn failure(error: impl ToString) -> Self {
        Self {
            success: false,
            error: Some(error.to_string()),
        }
    }
}

/// 批量操作的标准响应类型
#[derive(Clone, Debug)]
pub struct StandardBatchResponse<T> {
    /// 操作结果列表
    pub results: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_batch_common_params() {
        let params = BatchCommonParams::new()
            .with_user_id_type("open_id")
            .with_client_token("550e8400-e29b-41d4-a716-446655440000");

        assert_eq!(params.user_id_type, Some("open_id".to_string()));
        assert_eq!(params.client_token, Some("550e8400-e29b-41d4-a716-446655440000".to_string()));
    }

    #[test]
    fn test_batch_common_body() {
        let data = vec!["item1", "item2"];
        let params = BatchCommonParams::new()
            .with_user_id_type("user_id");

        let body = BatchCommonBody::new(data.clone())
            .with_common_params(params);

        assert_eq!(body.data, data);
        assert_eq!(body.user_id_type, Some("user_id".to_string()));
        assert!(body.client_token.is_none());
    }

    #[test]
    fn test_batch_operation_result() {
        let success_result = BatchOperationResult::success();
        assert!(success_result.success);
        assert!(success_result.error.is_none());

        let failure_result = BatchOperationResult::failure("权限不足");
        assert!(!failure_result.success);
        assert_eq!(failure_result.error, Some("权限不足".to_string()));
    }

    #[test]
    fn test_batch_operation_result_serialization() {
        let result = BatchOperationResult::failure("测试错误");
        let serialized = serde_json::to_value(&result).unwrap();
        let expected = json!({
            "success": false,
            "error": "测试错误"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_standard_batch_response() {
        let results = vec![
            BatchOperationResult::success(),
            BatchOperationResult::failure("第二个失败"),
        ];

        let response = StandardBatchResponse { results };
        assert_eq!(response.results.len(), 2);
        assert!(response.results[0].success);
        assert!(!response.results[1].success);
        assert_eq!(response.results[1].error, Some("第二个失败".to_string()));
    }
}