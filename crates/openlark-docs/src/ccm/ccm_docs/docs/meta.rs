use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取文档元数据
///
/// 此接口用于获取指定文档的元数据信息，包括文档属性、权限、访问统计等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/meta
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDocsApiOld, api_utils::*};

use serde_json::json;

/// 获取元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaRequest {
    /// 文档token
    pub document_token: String,
}

impl GetMetaRequest {
    /// 创建获取元数据请求
    ///
    /// # 参数
    /// * `document_token` - 文档token
    pub fn new(document_token: impl Into<String>) -> Self {
        Self {
            document_token: document_token.into(),
        }
    }

    /// 设置文档token
    pub fn document_token(mut self, document_token: impl Into<String>) -> Self {
        self.document_token = document_token.into();
        self
    }
}

/// 文档元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    /// 文档ID
    pub document_id: String,
    /// 文档token
    pub document_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub obj_type: String,
    /// 文档URL
    pub url: String,
    /// 文档大小
    pub size: Option<i64>,
    /// 文档所有者
    pub owner: Option<UserInfo>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 文档权限
    pub permissions: Option<serde_json::Value>,
    /// 访问统计
    pub access_stats: Option<serde_json::Value>,
    /// 自定义属性
    pub extra: Option<serde_json::Value>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 获取元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaResponse {
    /// 文档元数据
    pub data: Option<MetaData>,
}

impl ApiResponseTrait for GetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档元数据
///
/// 获取指定文档的元数据信息，包括文档属性、权限、访问统计等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/meta
pub async fn get_meta(
    request: GetMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetMetaResponse>> {
    // 使用CcmDocsApiOld枚举生成API端点
    let api_endpoint = CcmDocsApiOld::GetMeta(request.document_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<GetMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

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
    fn test_get_meta_request_builder() {
        let request = GetMetaRequest::new("document_token");

        assert_eq!(request.document_token, "document_token");
    }

    #[test]
    fn test_get_meta_request_with_token() {
        let request = GetMetaRequest::new("initial_token").document_token("new_token");

        assert_eq!(request.document_token, "new_token");
    }

    #[test]
    fn test_meta_data_structure() {
        let user_info = UserInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
            email: Some("email@example.com".to_string()),
        };

        let meta_data = MetaData {
            document_id: "doc_id".to_string(),
            document_token: "doc_token".to_string(),
            title: "文档标题".to_string(),
            obj_type: "doc".to_string(),
            url: "https://example.com".to_string(),
            size: Some(1024),
            owner: Some(user_info.clone()),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            update_time: "2023-01-01T00:00:00Z".to_string(),
            permissions: Some(json!({"read": true})),
            access_stats: Some(json!({"views": 100})),
            extra: Some(json!({"custom": "value"})),
        };

        assert_eq!(meta_data.document_id, "doc_id");
        assert_eq!(meta_data.owner.as_ref().unwrap().name, "用户名");
        assert_eq!(meta_data.size, Some(1024));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetMetaResponse::data_format(), ResponseFormat::Data);
    }
}
