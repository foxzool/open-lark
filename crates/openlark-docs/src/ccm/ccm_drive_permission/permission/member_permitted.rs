/// 判断协作者是否有某权限
///
/// 此接口用于检查指定协作者是否对特定文档具有某种权限。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission/member_permitted
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde_json::json;

use super::models::*;
use super::requests::MemberPermittedRequest;
use super::responses::MemberPermittedData;
use crate::common::{api_endpoints::CcmDrivePermissionApiOld, api_utils::*};

impl ApiResponseTrait for MemberPermittedResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 判断协作者是否有某权限
///
/// 此接口用于检查指定协作者是否对特定文档具有某种权限。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission/member_permitted
pub async fn member_permitted(
    request: MemberPermittedRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<MemberPermittedResponse> {
    // 验证必填字段
    validate_required_field("文档Token", Some(&request.obj_token), "文档Token不能为空")?;
    validate_required_field("成员ID", Some(&request.member_id), "成员ID不能为空")?;
    validate_required_field("权限类型", Some(&request.perm_type), "权限类型不能为空")?;

    // 构建请求体
    let request_body = json!({
        "obj_token": request.obj_token,
        "member_id": request.member_id,
        "perm_type": request.perm_type
    });

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDrivePermissionApiOld::MemberPermitted;

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<MemberPermittedResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request_body, "判断协作者权限")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;
    extract_response_data(response, "判断协作者权限")
}
