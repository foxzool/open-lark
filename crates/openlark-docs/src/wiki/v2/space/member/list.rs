/// 获取知识空间成员列表
///
/// 获取指定知识空间的成员列表。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 获取知识空间成员列表响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListSpaceMembersResponse {
    /// 成员列表
    pub items: Vec<MemberItem>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 成员信息
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MemberItem {
    /// 成员类型
    pub member_type: String,
    /// 成员ID
    pub member_id: String,
    /// 成员名称
    pub member_name: String,
    /// 成员角色
    pub role: String,
}

impl ApiResponseTrait for ListSpaceMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间成员列表
///
/// 获取指定知识空间的成员列表。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
///
/// # 返回
/// 返回成员列表
pub async fn list_space_members(
    config: &Config,
    space_id: &str,
) -> SDKResult<ListSpaceMembersResponse> {
    list_space_members_with_options(config, space_id, RequestOption::default()).await
}

/// 获取知识空间成员列表（支持自定义选项）
pub async fn list_space_members_with_options(
    config: &Config,
    space_id: &str,
    option: RequestOption,
) -> SDKResult<ListSpaceMembersResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceMemberList(space_id.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<ListSpaceMembersResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
