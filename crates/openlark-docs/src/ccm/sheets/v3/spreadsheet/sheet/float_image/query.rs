use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 查询浮动图片
///
/// 返回子表内所有的浮动图片信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/query
use serde::{Deserialize, Serialize};

use super::create::*;
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 查询浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFloatImagesResponse {
    /// 浮动图片列表
    pub data: Option<QueryFloatImagesData>,
}

/// 查询浮动图片数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFloatImagesData {
    /// 浮动图片项目
    pub items: Vec<FloatImageData>,
}

impl ApiResponseTrait for QueryFloatImagesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询浮动图片
///
/// 返回子表内所有的浮动图片信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/query
pub async fn query_float_images(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<QueryFloatImagesResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint =
        CcmSheetApiOld::QueryFloatImages(spreadsheet_token.to_string(), sheet_id.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<QueryFloatImagesResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查询浮动图片")
}
