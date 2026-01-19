/// 添加知识空间成员
///
/// 向指定知识空间添加成员。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 添加知识空间成员请求
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateSpaceMemberRequest {
    /// 成员类型（user/group）
    pub member_type: String,
    /// 成员ID
    pub member_id: String,
    /// 成员角色
    pub role: String,
}

/// 添加知识空间成员响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateSpaceMemberResponse {
    /// 成员类型
    pub member_type: String,
    /// 成员ID
    pub member_id: String,
    /// 成员名称
    pub member_name: String,
    /// 成员角色
    pub role: String,
}

impl ApiResponseTrait for CreateSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加知识空间成员
///
/// 向指定知识空间添加成员。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
/// * `request` - 添加成员请求
///
/// # 返回
/// 返回添加的成员信息
pub async fn create_space_member(
    config: &Config,
    space_id: &str,
    request: CreateSpaceMemberRequest,
) -> SDKResult<CreateSpaceMemberResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceMemberCreate(space_id.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<CreateSpaceMemberResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(&request)?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
