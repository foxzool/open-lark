/// 搜索云文档
///
/// 根据搜索条件进行文档搜索。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDocsApiOld, api_utils::*};
use super::super::models::*;
use super::super::requests::SearchObjectRequest;
use super::super::responses::SearchObjectData;

impl ApiResponseTrait for SearchObjectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索云文档
///
/// 根据搜索条件进行文档搜索。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
pub async fn search_object(
    request: SearchObjectRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<SearchObjectResponse> {
    // 验证必填字段
    validate_required_field("搜索关键字", Some(&request.query), "搜索关键字不能为空")?;

    // 转换为参数
    let params = SearchObjectParams {
        query: request.query,
        doc_type: request.doc_type,
        scope: request.scope,
        page_size: request.page_size,
        page_token: request.page_token,
        sort_field: request.sort_field,
        sort_direction: request.sort_direction,
    };

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDocsApiOld::SearchObject;

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let mut api_request: ApiRequest<SearchObjectResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&params, "搜索云文档")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;
    extract_response_data(response, "搜索云文档")
}