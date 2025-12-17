/// 获取词典分类
///
/// 获取飞书词典当前分类。
/// docPath: https://open.feishu.cn/document/lingo-v1/classification/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct ListClassificationResponse {
    pub data: Option<ClassificationData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ClassificationData {
    pub classifications: Vec<ClassificationItem>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ClassificationItem {
    pub classification_id: String,
    pub name: String,
    pub level: i32,
    pub parent_id: Option<String>,
    pub children: Option<Vec<ClassificationItem>>,
}

impl ApiResponseTrait for ListClassificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词典分类
///
/// 获取飞书词典当前分类。
/// docPath: https://open.feishu.cn/document/lingo-v1/classification/list
pub async fn list_classification(
    config: &Config,
) -> SDKResult<Vec<ClassificationItem>> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::ClassificationList;

    // 创建API请求
    let api_request: ApiRequest<ListClassificationResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: ListClassificationResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.map(|data| data.classifications).ok_or_else(|| {
        openlark_core::error::validation_error("classification_data", "Classification data is missing")
    })
}