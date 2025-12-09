//! Markdown/HTML 内容转换为文档块
//!
//! 将 Markdown/HTML 格式的内容转换为文档块，以便于将 Markdown/HTML 格式的内容插入到文档中。目前支持转换为的块类型包含文本、一到九级标题、无序列表、有序列表、代码块、引用、待办事项、图片、表格、表格单元格。
//! API文档: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::docx::v1::models::DocumentBlockInfo;
use crate::common::api_endpoints::DocxApiV1;

/// 内容转换为文档块请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct ConvertContentToBlocksParams {
    /// 内容类型：markdown 或 html
    pub content_type: String,
    /// 要转换的内容
    pub content: String,
    /// 文档标题（可选）
    pub title: Option<String>,
}

/// 内容转换为文档块响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ConvertContentToBlocksResponse {
    /// 转换后的块信息
    pub data: Vec<DocumentBlockInfo>,
}

impl ApiResponseTrait for ConvertContentToBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 内容转换为文档块请求
pub struct ConvertContentToBlocksRequest {
    config: Config,
}

impl ConvertContentToBlocksRequest {
    /// 创建新的内容转换为文档块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/convert-content-to-blocks
    pub async fn execute(self, params: ConvertContentToBlocksParams) -> SDKResult<ConvertContentToBlocksResponse> {
        // 验证必填字段
        validate_required!(params.content_type, "内容类型不能为空");
        validate_required!(params.content, "内容不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::DocumentConvert;

        // 创建API请求
        let api_request: ApiRequest<ConvertContentToBlocksResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 设置请求体
        let mut body = serde_json::json!({
            "content_type": params.content_type,
            "content": params.content
        });

        if let Some(title) = params.title {
            body["title"] = serde_json::Value::String(title);
        }

        let request_with_body = api_request.body(body);

        // 发送请求
        let response = Transport::request(request_with_body, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}