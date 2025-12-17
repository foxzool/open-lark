/// 删除Sheet
///
/// 根据 spreadsheetToken 删除对应的 sheet 文档。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-sheet
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 删除Sheet请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSpreadsheetParams {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
}

/// 删除Sheet响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSpreadsheetResponse {
    /// 删除结果
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除Sheet请求
pub struct DeleteSpreadsheetRequest {
    config: Config,
}

impl DeleteSpreadsheetRequest {
    /// 创建删除Sheet请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-sheet
    pub async fn execute(
        self,
        params: DeleteSpreadsheetParams,
    ) -> SDKResult<DeleteSpreadsheetResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint =
            CcmDriveExplorerApiOld::FileSpreadsheets(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteSpreadsheetResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
