/// 获取旧版文档元信息
///
/// 根据 docToken 获取元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::*;
use super::requests::DocumentMetaRequest;
use super::responses::DocumentMetaData;
use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};

impl ApiResponseTrait for DocumentMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档元信息
///
/// 根据 docToken 获取元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta
pub async fn get_document_meta(
    request: DocumentMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<super::responses::DocumentMetaData> {
    // 验证必填字段
    validate_required_field("文档Token", Some(&request.document_token), "文档Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let document_token = request.document_token.clone();
    let api_endpoint = CcmDocApiOld::Meta(document_token);

    // 创建API请求
    let api_request: ApiRequest<DocumentMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;

    // 提取响应包装器
    let resp: DocumentMetaResponse = extract_response_data(response, "获取文档元信息")?;

    // 提取内部数据并转换为responses::DocumentMetaData格式
    let meta = resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("document_meta", "文档元信息数据为空")
    })?;

    // 转换为responses::DocumentMetaData格式（包含folder_token字段）
    Ok(super::responses::DocumentMetaData {
        doc_token: meta.doc_token,
        title: meta.title,
        doc_type: meta.doc_type,
        create_time: meta.create_time,
        update_time: meta.update_time,
        creator: meta.creator.map(|c| super::responses::User {
            open_id: c.open_id,
            name: c.name,
        }),
        updater: meta.updater.map(|u| super::responses::User {
            open_id: u.open_id,
            name: u.name,
        }),
        folder_token: None,  // DocumentMeta中没有folder_token字段
    })
}
