//! 更新知识空间设置
//!
//! 根据 space_id 更新知识空间公共设置。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-setting/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::WikiApiV2, api_utils::*};
use crate::wiki::v2::models::WikiSpaceSetting;

/// 更新知识空间设置请求
pub struct UpdateWikiSpaceSettingRequest {
    space_id: String,
    config: Config,
}

/// 更新知识空间设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWikiSpaceSettingResponse {
    /// 知识空间设置信息
    pub setting: Option<WikiSpaceSetting>,
}

impl ApiResponseTrait for UpdateWikiSpaceSettingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateWikiSpaceSettingRequest {
    /// 创建更新知识空间设置请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(
        self,
        setting: WikiSpaceSetting,
    ) -> SDKResult<UpdateWikiSpaceSettingResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceSettingUpdate(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UpdateWikiSpaceSettingResponse> =
            ApiRequest::put(&api_endpoint.to_url())
                .body(serialize_params(&setting, "更新知识空间设置")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "更新知识空间设置")
    }
}
