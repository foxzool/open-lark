use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 删除浮动图片
///
/// 删除 float_image_id 对应的浮动图片。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/delete
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 删除浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFloatImageResponse {
    /// 删除结果
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除浮动图片
///
/// 删除 float_image_id 对应的浮动图片。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/delete
pub async fn delete_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
) -> SDKResult<DeleteFloatImageResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DeleteFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<DeleteFloatImageResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除浮动图片")
}
