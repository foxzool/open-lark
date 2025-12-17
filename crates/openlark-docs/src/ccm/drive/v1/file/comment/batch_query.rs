/// 批量获取评论
///
/// 该接口用于根据评论 ID 列表批量获取评论。
/// docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/batch_query
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量获取评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryCommentRequest {
    /// 文件token
    pub file_token: String,
    /// 评论ID列表
    pub comment_ids: Vec<String>,
}

impl BatchQueryCommentRequest {
    /// 创建批量获取评论请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `comment_ids` - 评论ID列表
    pub fn new(file_token: impl Into<String>, comment_ids: Vec<String>) -> Self {
        Self {
            file_token: file_token.into(),
            comment_ids,
        }
    }
}

/// 批量获取评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryCommentResponse {
    /// 评论列表
    pub comments: Vec<BatchCommentInfo>,
    /// 批量操作结果
    pub batch_result: BatchOperationResult,
}

/// 评论信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCommentInfo {
    /// 评论ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
    /// 评论类型
    pub comment_type: String,
    /// 创建者信息
    pub creator: BatchCreatorInfo,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 父评论ID（回复评论时）
    pub parent_comment_id: Option<String>,
    /// 回复评论数量
    pub reply_count: i32,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户类型
    pub user_type: String,
}

/// 批量操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// 成功数量
    pub success_count: i32,
    /// 失败数量
    pub failed_count: i32,
    /// 总数量
    pub total_count: i32,
}

impl ApiResponseTrait for BatchQueryCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取评论
///
/// 该接口用于根据评论 ID 列表批量获取评论。
/// docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/batch_query
pub async fn batch_query_comment(
    request: BatchQueryCommentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<BatchQueryCommentResponse>> {
    // 构建请求体
    let body = serde_json::json!({
        "comment_ids": request.comment_ids
    });

    // 创建API请求
    let mut api_request: ApiRequest<BatchQueryCommentResponse> = ApiRequest::post(&format!(
        "/open-apis/drive/v1/files/{}/comments/batch_query",
        request.file_token
    ))
    .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_query_comment_request_builder() {
        let comment_ids = vec!["comment_1".to_string(), "comment_2".to_string()];
        let request = BatchQueryCommentRequest::new("file_token", comment_ids);

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_ids.len(), 2);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchQueryCommentResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
