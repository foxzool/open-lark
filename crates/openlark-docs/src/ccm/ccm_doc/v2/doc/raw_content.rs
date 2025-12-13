/// 获取旧版文档纯文本内容
///
/// 获取文档的纯文本内容，不包含富文本格式信息。
/// docPath: https://open.feishu.cn/document/ukTMukTMukTM/ukzNzUjL5czM14SO3MTN
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::super::models::*;
use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};

impl ApiResponseTrait for RawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档纯文本内容
///
/// 获取文档的纯文本内容，不包含富文本格式信息。
/// docPath: https://open.feishu.cn/document/ukTMukTMukTM/ukzNzUjL5czM14SO3MTN
pub async fn get_raw_content(config: &Config, doc_token: &str) -> SDKResult<RawContentResponse> {
    // 验证必填字段
    validate_required_field("文档Token", Some(doc_token), "文档Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDocApiOld::RawContent(doc_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<RawContentResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取文档纯文本内容")
}
