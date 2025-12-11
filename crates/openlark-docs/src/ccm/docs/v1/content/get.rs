//! 获取云文档内容
//!
//! 获取指定云文档的详细内容。
//! API文档: https://open.feishu.cn/document/docs/docs-v1/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

// TODO: 定义DocsContent结构体或使用通用类型
use crate::common::api_endpoints::DocsApiV1;

/// 获取云文档内容请求
pub struct GetDocsContentRequest {
    config: Config,
}

/// 获取云文档内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocsContentParams {
    /// 文档Token
    pub document_token: String,
}

/// 获取云文档内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocsContentResponse {
    /// 文档内容信息
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetDocsContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetDocsContentRequest {
    /// 创建获取云文档内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/docs/docs-v1/get
    /// 对应CSV记录: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-v1/content/get
    pub async fn execute(self, params: GetDocsContentParams) -> SDKResult<GetDocsContentResponse> {
        // 验证必填字段
        validate_required!(params.document_token, "文档Token不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = DocsApiV1::ContentGet;

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetDocsContentResponse> =
            ApiRequest::get(&api_endpoint.to_url()).query("document_token", &params.document_token);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
