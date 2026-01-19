/// 更新知识空间设置
///
/// 更新指定知识空间的设置信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-setting/update
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 更新知识空间设置请求
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSpaceSettingRequest {
    /// 空间名称（最大100字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 空间描述（最大500字符）
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

/// 更新知识空间设置响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSpaceSettingResponse {
    /// 空间ID
    pub space_id: String,
    /// 空间名称
    pub name: String,
    /// 空间描述
    pub description: Option<String>,
    /// 空间图标
    pub icon: Option<SpaceIcon>,
}

impl ApiResponseTrait for UpdateSpaceSettingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新知识空间设置
///
/// 更新指定知识空间的设置信息。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
/// * `request` - 更新请求
///
/// # 返回
/// 返回更新后的空间设置信息
pub async fn update_space_setting(
    config: &Config,
    space_id: &str,
    request: UpdateSpaceSettingRequest,
) -> SDKResult<UpdateSpaceSettingResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceSettingUpdate(space_id.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<UpdateSpaceSettingResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serde_json::to_value(&request)?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
