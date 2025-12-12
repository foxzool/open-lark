//! 搜索云文档
//!
//! 根据搜索条件进行文档搜索。
//! API文档: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
//! 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocsApiOld;

/// 搜索云文档请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectParams {
    /// 搜索关键词，长度限制：1-100字符
    pub query: String,
    /// 搜索类型，可选值：doc、sheet、bitable、mindnote、file
    #[serde(rename = "doc_type")]
    pub doc_type: Option<String>,
    /// 搜索范围，可选值：mine、shared、public
    pub search_scope: Option<String>,
    /// 页面大小，最大值：100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    /// 分页token
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
}

/// 搜索云文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectResponse {
    /// 搜索结果
    pub data: Option<SearchResult>,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// 文档列表
    pub items: Vec<DocumentItem>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 分页token
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
}

/// 文档项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentItem {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 创建者信息
    pub creator: Option<UserInfo>,
    /// 预览URL
    #[serde(rename = "preview_url")]
    pub preview_url: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

impl ApiResponseTrait for SearchObjectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索云文档请求
pub struct SearchObjectRequest {
    config: Config,
}

impl SearchObjectRequest {
    /// 创建搜索云文档请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
    pub async fn execute(
        self,
        params: SearchObjectParams,
    ) -> SDKResult<SearchObjectResponse> {
        // 验证必填字段
        validate_required!(params.query, "搜索关键词不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDocsApiOld::SearchObject;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<SearchObjectResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serde_json::to_value(params).map_err(|e| {
                    openlark_core::error::validation_error(
                        "参数序列化失败",
                        &format!("无法序列化请求参数: {}", e)
                    )
                })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}