/// 获取旧版文档富文本内容
///
/// 获取结构化的文档内容。
/// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document
/// 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 获取旧版文档富文本内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentContentParams {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
}

/// 获取旧版文档富文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentContentResponse {
    /// 文档富文本内容
    pub data: Option<DocumentContent>,
}

/// 文档富文本内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContent {
    /// 文档块列表
    pub blocks: Vec<DocumentBlock>,
    /// 文档标题
    pub title: String,
}

/// 文档块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentBlock {
    /// 块ID
    #[serde(rename = "block_id")]
    pub block_id: i64,
    /// 块类型
    #[serde(rename = "block_type")]
    pub block_type: i32,
    /// 块内容
    pub content: Option<serde_json::Value>,
    /// 子块列表
    pub children: Option<Vec<DocumentBlock>>,
}

impl ApiResponseTrait for GetDocumentContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档富文本内容请求
pub struct GetDocumentContentRequest {
    config: Config,
}

impl GetDocumentContentRequest {
    /// 创建获取旧版文档富文本内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document
    pub async fn execute(
        self,
        params: GetDocumentContentParams,
    ) -> SDKResult<GetDocumentContentResponse> {
        // 验证必填字段
        validate_required!(params.doc_token, "文档token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDocApiOld::Content(params.doc_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetDocumentContentResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
