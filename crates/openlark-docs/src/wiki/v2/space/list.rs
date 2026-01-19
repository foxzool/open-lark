/// 获取知识空间列表
///
/// 获取用户可访问的所有知识空间。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 获取知识空间列表响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListSpacesResponse {
    /// 空间列表
    pub items: Vec<SpaceItem>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 空间信息
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SpaceItem {
    /// 空间ID
    pub space_id: String,
    /// 空间名称
    pub name: String,
    /// 空间描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 空间图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<SpaceIcon>,
}

/// 空间图标
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SpaceIcon {
    /// 图标类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_type: Option<String>,
    /// 图标值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_value: Option<String>,
}

impl ApiResponseTrait for ListSpacesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间列表
///
/// 获取用户可访问的所有知识空间。
///
/// # 参数
/// * `config` - SDK 配置
///
/// # 返回
/// 返回知识空间列表
pub async fn list_spaces(config: &Config) -> SDKResult<ListSpacesResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceList;

    // 创建 API 请求
    let api_request: ApiRequest<ListSpacesResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
