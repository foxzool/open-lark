/// 更新浮动图片
///
/// 更新已有的浮动图片位置和宽高，包括 range、width、height、offset_x 和 offset_y，不包括 float_image_id 和 float_image_token。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/patch
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde_json::json;

use super::create::*;
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

// 导入序列化支持
use serde::{Deserialize, Serialize};

/// 更新浮动图片请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageRequest {
    /// 图片位置范围，只支持一个单元格
    pub range: Option<String>,
    /// 图片展示宽度，可选
    pub width: Option<i32>,
    /// 图片展示高度，可选
    pub height: Option<i32>,
    /// 水平偏移量，可选
    pub offset_x: Option<i32>,
    /// 垂直偏移量，可选
    pub offset_y: Option<i32>,
}

/// 更新浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageResponse {
    /// 浮动图片信息
    pub data: Option<FloatImageData>,
}

impl ApiResponseTrait for UpdateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新浮动图片
///
/// 更新已有的浮动图片位置和宽高。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/patch
pub async fn update_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
    params: UpdateFloatImageRequest,
) -> SDKResult<UpdateFloatImageResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<UpdateFloatImageResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(json!(params));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新浮动图片")
}
