/// 转移拥有者
///
/// 此接口用于将指定文档的所有权转移给新的拥有者。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission/member_transfer
use serde::{Deserialize, Serialize};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDrivePermissionApi, api_utils::*};
use super::requests::MemberTransferRequest;
use super::responses::MemberTransferData;

impl ApiResponseTrait for MemberTransferResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 成员转移响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberTransferResponse {
    /// 转移结果
    pub data: Option<MemberTransferData>,
}

/// 转移拥有者
///
/// 将指定文档的所有权转移给新的拥有者。
pub async fn member_transfer(
    request: MemberTransferRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<MemberTransferResponse> {
    // 验证必填字段
    validate_required_field("文档Token", Some(&request.obj_token), "文档Token不能为空")?;
    validate_required_field("目标成员ID", Some(&request.target_id), "目标成员ID不能为空")?;

    // 转换为参数
    let params = MemberTransferParams {
        obj_token: request.obj_token,
        target_id: request.target_id,
        target_type: request.target_type,
    };

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDrivePermissionApi::MemberTransfer;

    // 创建API请求
    let api_request: ApiRequest<MemberTransferResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "转移拥有者")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;
    extract_response_data(response, "转移拥有者")
}

/// 成员转移参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberTransferParams {
    /// 文件/文件夹token
    pub obj_token: String,
    /// 目标成员ID
    pub target_id: String,
    /// 目标类型
    pub target_type: Option<String>,
}
