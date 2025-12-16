/// 删除Doc
///
/// 根据 docToken 删除对应的 Docs 文档。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-a-doc

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 删除Doc请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocParams {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
}

/// 删除Doc响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocResponse {
    /// 删除结果
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteDocResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除Doc请求
pub struct DeleteDocRequest {
    config: Config,
}

impl DeleteDocRequest {
    /// 创建删除Doc请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-a-doc
    pub async fn execute(
        self,
        params: DeleteDocParams,
    ) -> SDKResult<DeleteDocResponse> {
        // 验证必填字段
        validate_required!(params.doc_token, "文档token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDriveExplorerApiOld::FileDocs(params.doc_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteDocResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}