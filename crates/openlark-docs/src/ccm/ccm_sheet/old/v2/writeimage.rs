/// 写入图片
///
/// 根据 spreadsheetToken 向指定单元格插入图片。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/insert-image
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 写入图片请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteImageParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 图片信息
    pub image: ImageInfo,
}

/// 图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 起始行索引（从0开始）
    #[serde(rename = "startRowIndex")]
    pub start_row_index: i32,
    /// 起始列索引（从0开始）
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: i32,
    /// 结束行索引（不包含）
    #[serde(rename = "endRowIndex")]
    pub end_row_index: i32,
    /// 结束列索引（不包含）
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: i32,
    /// 图片URL
    #[serde(rename = "image_url")]
    pub image_url: String,
    /// 图片宽度（像素）
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 图片高度（像素）
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

/// 写入图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteImageResponse {
    /// 写入结果
    pub data: Option<WriteImageResult>,
}

/// 写入图片结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteImageResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 是否成功
    pub success: bool,
    /// 图片ID
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

impl ApiResponseTrait for WriteImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 写入图片请求
pub struct WriteImageRequest {
    config: Config,
}

impl WriteImageRequest {
    /// 创建写入图片请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/insert-image
    pub async fn execute(self, params: WriteImageParams) -> SDKResult<WriteImageResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ValuesImage(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<WriteImageResponse> = ApiRequest::post(
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
