use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::lingo::models::{Draft, OuterInfo, RelatedMeta},
};

/// 草稿管理服务
pub struct DraftService {
    pub config: Config,
}

impl DraftService {
    /// 创建草稿管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建草稿
    ///
    /// 创建词条草稿，可以是新词条的草稿或是对现有词条的修改草稿。
    ///
    /// # Arguments
    ///
    /// * `request` - 草稿创建请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回创建的草稿信息
    pub async fn create_draft(
        &self,
        request: DraftCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DraftCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/lingo/v1/drafts".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新草稿
    ///
    /// 更新指定草稿的内容。
    ///
    /// # Arguments
    ///
    /// * `draft_id` - 草稿ID
    /// * `request` - 草稿更新请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回更新后的草稿信息
    pub async fn update_draft(
        &self,
        draft_id: &str,
        request: DraftUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DraftUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: format!("/open-apis/lingo/v1/drafts/{draft_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 草稿创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftCreateRequest {
    /// 词条ID（如果是更新现有词条的草稿）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// 主名称
    pub main_keys: Vec<String>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 词条释义富文本
    pub description: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 外链（用于跳转到释义页面）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 相关词条ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
}

/// 草稿创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DraftCreateResponse {
    /// 创建的草稿信息
    pub draft: Draft,
}

impl ApiResponseTrait for DraftCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 草稿更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftUpdateRequest {
    /// 主名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_keys: Option<Vec<String>>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 词条释义富文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 外链（用于跳转到释义页面）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 相关词条ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
}

/// 草稿更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DraftUpdateResponse {
    /// 更新后的草稿信息
    pub draft: Draft,
}

impl ApiResponseTrait for DraftUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
