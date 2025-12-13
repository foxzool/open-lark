/// 获取云文档权限设置V2
///
/// 此接口用于获取指定文档的详细权限设置信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission/v2/public
use serde::{Deserialize, Serialize};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDrivePermissionApi, api_utils::*};
use super::requests::PublicRequest;
use super::responses::PublicData;

impl ApiResponseTrait for PublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 公开权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicResponse {
    /// 权限设置数据
    pub data: Option<PublicData>,
}

/// 获取公开权限
///
/// 获取指定文档的公开权限设置信息。
pub async fn public(
    request: PublicRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<PublicResponse> {
    // 验证必填字段
    validate_required_field("文档Token", Some(&request.obj_token), "文档Token不能为空")?;

    // 转换为参数
    let params = PublicParams {
        obj_token: request.obj_token,
        permission_type: request.permission_type,
    };

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDrivePermissionApi::Public;

    // 创建API请求
    let mut query_params = vec![("obj_token", params.obj_token.clone())];
    if let Some(ref permission_type) = params.permission_type {
        query_params.push(("permission_type", permission_type.clone()));
    }

    let api_request: ApiRequest<PublicResponse> =
        ApiRequest::get(&api_endpoint.to_url()).query_params(query_params);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;
    extract_response_data(response, "获取公开权限")
}

/// 公开权限参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicParams {
    /// 文件/文件夹token
    pub obj_token: String,
    /// 权限类型
    pub permission_type: Option<String>,
}
