/// 删除知识空间成员
///
/// 从指定知识空间删除成员。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 删除知识空间成员响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteSpaceMemberResponse {
    /// 是否成功
    pub success: bool,
}

impl ApiResponseTrait for DeleteSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除知识空间成员
///
/// 从指定知识空间删除成员。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
/// * `member_id` - 成员ID
///
/// # 返回
/// 返回删除结果
pub async fn delete_space_member(
    config: &Config,
    space_id: &str,
    member_id: &str,
) -> SDKResult<DeleteSpaceMemberResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceMemberDelete(space_id.to_string(), member_id.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<DeleteSpaceMemberResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
