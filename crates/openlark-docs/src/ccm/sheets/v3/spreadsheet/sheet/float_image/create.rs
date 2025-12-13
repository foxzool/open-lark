/// 创建浮动图片
///
/// 根据传入的参数创建一张浮动图片。Float_image_token（上传图片至表格后得到）和range（只支持一个单元格）必填。
/// Float_image_id 可选，不填的话会默认生成，长度为10，由 0-9、a-z、A-Z 组合生成。
/// 表格内不重复的图片（浮动图片+单元格图片）总数不超过4000。
/// width 和 height 为图片展示的宽高，可选，不填的话会使用图片的真实宽高。
/// offset_x 和 offset_y 为图片左上角距离所在单元格左上角的偏移，可选，默认为 0。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde_json::json;

// 导入序列化支持
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};
use serde::{Deserialize, Serialize};

/// 创建浮动图片请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageRequest {
    /// 浮动图片ID，可选，不填会自动生成
    pub float_image_id: Option<String>,
    /// 图片token，必填
    pub float_image_token: String,
    /// 图片位置范围，只支持一个单元格
    pub range: String,
    /// 图片展示宽度，可选
    pub width: Option<i32>,
    /// 图片展示高度，可选
    pub height: Option<i32>,
    /// 水平偏移量，可选，默认为0
    pub offset_x: Option<i32>,
    /// 垂直偏移量，可选，默认为0
    pub offset_y: Option<i32>,
}

/// 创建浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageResponse {
    /// 浮动图片信息
    pub data: Option<FloatImageData>,
}

/// 浮动图片数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatImageData {
    /// 浮动图片ID
    pub float_image_id: String,
    /// 图片token
    pub float_image_token: String,
    /// 图片位置范围
    pub range: String,
    /// 图片展示宽度
    pub width: i32,
    /// 图片展示高度
    pub height: i32,
    /// 水平偏移量
    pub offset_x: i32,
    /// 垂直偏移量
    pub offset_y: i32,
}

impl ApiResponseTrait for CreateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建浮动图片
///
/// 根据传入的参数创建一张浮动图片。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/create
pub async fn create_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFloatImageRequest,
) -> SDKResult<CreateFloatImageResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint =
        CcmSheetApiOld::CreateFloatImage(spreadsheet_token.to_string(), sheet_id.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<CreateFloatImageResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(json!(params));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建浮动图片")
}
