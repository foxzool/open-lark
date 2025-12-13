/// 编辑旧版文档内容
///
/// 批量编辑更新文档内容，包括更新标题、范围删除、插入内容。
/// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document
/// 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 编辑旧版文档内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateDocumentParams {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 批量更新操作列表
    pub requests: Vec<UpdateRequest>,
}

/// 更新操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRequest {
    /// 操作类型
    #[serde(rename = "operation_type")]
    pub operation_type: i32,
    /// 操作参数
    pub data: Option<serde_json::Value>,
}

/// 编辑旧版文档内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateDocumentResponse {
    /// 更新结果
    pub data: Option<BatchUpdateResult>,
}

/// 批量更新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateResult {
    /// 更新操作结果列表
    pub results: Vec<UpdateResult>,
}

/// 更新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    /// 块ID
    #[serde(rename = "block_id")]
    pub block_id: Option<i64>,
    /// 操作状态
    pub status: i32,
    /// 错误信息
    pub message: Option<String>,
}

impl ApiResponseTrait for BatchUpdateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 编辑旧版文档内容请求
pub struct BatchUpdateDocumentRequest {
    config: Config,
}

impl BatchUpdateDocumentRequest {
    /// 创建编辑旧版文档内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document
    pub async fn execute(
        self,
        params: BatchUpdateDocumentParams,
    ) -> SDKResult<BatchUpdateDocumentResponse> {
        // 验证必填字段
        validate_required!(params.doc_token, "文档token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDocApiOld::BatchUpdate(params.doc_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<BatchUpdateDocumentResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_value(params).map_err(|e| {
            openlark_core::error::validation_error(
                "参数序列化失败",
                &format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
