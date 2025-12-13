use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取浮动图片
///
/// 根据 float_image_id 获取对应浮动图片的信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/get
use serde::{Deserialize, Serialize};

use super::create::*;
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 获取浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFloatImageResponse {
    /// 浮动图片信息
    pub data: Option<FloatImageData>,
}

impl ApiResponseTrait for GetFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取浮动图片
///
/// 根据 float_image_id 获取对应浮动图片的信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/get
pub async fn get_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
) -> SDKResult<GetFloatImageResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<GetFloatImageResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取浮动图片")
}
