use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取聊天公告
///
/// 此接口用于获取指定群聊的公告基本信息，包括公告内容、创建时间等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docx-v1/chat/announcement/get
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 获取聊天公告请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementRequest {
    /// 群聊ID
    pub chat_id: String,
}

impl GetChatAnnouncementRequest {
    /// 创建获取聊天公告请求
    ///
    /// # 参数
    /// * `chat_id` - 群聊ID
    pub fn new(chat_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// 设置群聊ID
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }
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

/// 获取聊天公告响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementResponse {
    /// 公告信息
    pub data: Option<AnnouncementData>,
}

impl ApiResponseTrait for GetChatAnnouncementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取聊天公告
///
/// 获取指定群聊的公告基本信息，包括公告内容、创建时间等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docx-v1/chat/announcement/get
pub async fn get_chat_announcement(
    request: GetChatAnnouncementRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetChatAnnouncementResponse>> {
    // 使用DocxApiV1枚举生成API端点
    let api_endpoint = DocxApiV1::ChatAnnouncementGet(request.chat_id.clone());

    // 创建API请求
    let mut api_request: ApiRequest<GetChatAnnouncementResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chat_announcement_request_builder() {
        let request = GetChatAnnouncementRequest::new("chat_id");

        assert_eq!(request.chat_id, "chat_id");
    }

    #[test]
    fn test_get_chat_announcement_request_with_id() {
        let request = GetChatAnnouncementRequest::new("initial_id").chat_id("new_chat_id");

        assert_eq!(request.chat_id, "new_chat_id");
    }

    #[test]
    fn test_announcement_data_structure() {
        let user_info = UserInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
        };

        let announcement_data = AnnouncementData {
            content: "公告内容".to_string(),
            create_time: 1640995200,
            update_time: 1640995300,
            creator: Some(user_info.clone()),
            updater: Some(user_info),
        };

        assert_eq!(announcement_data.content, "公告内容");
        assert_eq!(announcement_data.creator.as_ref().unwrap().name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            GetChatAnnouncementResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
