//! 获取群公告基本信息
//!
//! 获取指定群聊的公告基本信息。
//! API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DocxApiV1;

/// 获取群公告基本信息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementParams {
    /// 群聊ID
    pub chat_id: String,
}

/// 获取群公告基本信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementResponse {
    /// 公告信息
    pub data: Option<AnnouncementData>,
}

/// 公告数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementData {
    /// 公告内容
    pub content: String,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
    /// 创建者信息
    pub creator: Option<UserInfo>,
    /// 更新者信息
    pub updater: Option<UserInfo>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

impl ApiResponseTrait for GetChatAnnouncementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告基本信息请求
pub struct GetChatAnnouncementRequest {
    config: Config,
}

impl GetChatAnnouncementRequest {
    /// 创建获取群公告基本信息请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement/get
    /// 对应CSV记录: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement/get
    pub async fn execute(
        self,
        params: GetChatAnnouncementParams,
    ) -> SDKResult<GetChatAnnouncementResponse> {
        // 验证必填字段
        validate_required!(params.chat_id, "群聊ID不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::ChatAnnouncementGet(params.chat_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetChatAnnouncementResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
