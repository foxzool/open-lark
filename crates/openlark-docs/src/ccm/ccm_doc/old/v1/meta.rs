//! 获取旧版文档元信息
//!
//! 根据 docToken 获取元数据。
//! API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta
//! 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 获取旧版文档元信息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentMetaParams {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
}

/// 获取旧版文档元信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentMetaResponse {
    /// 文档元信息
    pub data: Option<DocumentMeta>,
}

/// 文档元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMeta {
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
    /// 更新者信息
    pub updater: Option<UserInfo>,
    /// 父节点token
    #[serde(rename = "parent_node_token")]
    pub parent_node_token: Option<String>,
    /// 父节点类型
    #[serde(rename = "parent_node_type")]
    pub parent_node_type: Option<String>,
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

impl ApiResponseTrait for GetDocumentMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档元信息请求
pub struct GetDocumentMetaRequest {
    config: Config,
}

impl GetDocumentMetaRequest {
    /// 创建获取旧版文档元信息请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta
    pub async fn execute(
        self,
        params: GetDocumentMetaParams,
    ) -> SDKResult<GetDocumentMetaResponse> {
        // 验证必填字段
        validate_required!(params.doc_token, "文档token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDocApiOld::Meta(params.doc_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetDocumentMetaResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}