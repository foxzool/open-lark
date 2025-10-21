use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};
use crate::impl_full_service;

pub struct ChatsService {
    pub config: Config,
}

impl ChatsService {
    /// 获取用户或机器人所在的群列表
    ///
    /// 获取当前用户或机器人参与的所有群聊列表，包括群的基本信息、群主、
    /// 群状态等。支持分页查询和多种排序方式。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create
    pub async fn list(
        &self,
        list_chat_request: ListChatRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ListChatRespData> {
        let mut api_req = list_chat_request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = crate::core::endpoints::im::IM_CHAT_CREATE.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<ListChatRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    pub fn list_iter(
        &self,
        list_chat_request: ListChatRequest,
        option: Option<RequestOption>,
    ) -> ListChatIterator<'_> {
        ListChatIterator {
            service: self,
            request: list_chat_request,
            option,
            has_more: true,
        }
    }
}

// 接入统一 Service 抽象（IM v1 - ChatsService）
impl_full_service!(ChatsService, "im.chats", "v1");

pub struct ListChatIterator<'a> {
    service: &'a ChatsService,
    request: ListChatRequest,
    option: Option<RequestOption>,
    has_more: bool,
}
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im


impl ListChatIterator<'_> {
    pub async fn next(&mut self) -> Option<Vec<ListChat>> {
        if !self.has_more {
            return None;
        }
        match self
            .service
            .list(self.request.clone(), self.option.clone())
            .await
        {
            Ok(data) => {
                self.has_more = data.has_more;
                if data.has_more {
                    self.request
                        .api_req
                        .query_params
                        .insert("page_token", data.page_token.to_string());
                    Some(data.items)
                } else if data.items.is_empty() {
                    None
                } else {
                    Some(data.items)
                }
            }
            Err(_) => None,
        }
    }
}

#[derive(Default, Clone)]
pub struct ListChatRequest {
    api_req: ApiRequest,
}

impl ListChatRequest {
    pub fn builder() -> ListChatRequestBuilder {
        ListChatRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListChatRequestBuilder {
    request: ListChatRequest,
}

impl ListChatRequestBuilder {
    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("user_id_type", user_id_type.to_string());
        self
    }

    /// 群组排序方式
    pub fn sort_type(mut self, sort_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("sort_type", sort_type.to_string());
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的
    /// page_token，下次遍历可采用该page_token 获取查询结果
    ///
    /// 示例值：dmJCRHhpd3JRbGV1VEVNRFFyTitRWDY5ZFkybmYrMEUwMUFYT0VMMWdENEtuYUhsNUxGMDIwemtvdE5ORjBNQQ==
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_token", page_token.to_string());
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_size", page_size.to_string());
        self
    }

    pub fn build(self) -> ListChatRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到ListChatRequestBuilder
crate::impl_executable_builder_owned!(
    ListChatRequestBuilder,
    ChatsService,
    ListChatRequest,
    ListChatRespData,
    list
);

#[derive(Debug, Serialize, Deserialize)]
pub struct ListChatRespData {
    /// chat 列表
    pub items: Vec<ListChat>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: String,
    /// 是否还有更多项
    pub has_more: bool,
}

impl ApiResponseTrait for ListChatRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// chat 列表
#[derive(Debug, Serialize, Deserialize)]
pub struct ListChat {
    /// 群组 ID
    pub chat_id: String,
    /// 群头像 URL
    pub avatar: String,
    /// 群名称
    pub name: String,
    /// 群描述
    pub description: String,
    /// 群主 ID
    pub owner_id: String,
    /// 群主 ID 类型
    pub owner_id_type: String,
    /// 是否是外部群
    pub external: bool,
    /// 租户Key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，
    /// 也可以用作租户在应用中的唯一标识
    pub tenant_key: String,
    /// 群状态
    pub chat_status: String,
}

/// 创建群的请求参数
#[derive(Debug, Default)]
pub struct CreateChatRequest {
    api_req: ApiRequest,
}

impl CreateChatRequest {
    pub fn builder() -> CreateChatRequestBuilder {
        CreateChatRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateChatRequestBuilder {
    request: CreateChatRequest,
}

impl CreateChatRequestBuilder {
    /// 群名称，最大长度 64 个字符
    pub fn name(mut self, name: impl ToString) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "name".to_string(),
            serde_json::Value::String(name.to_string()),
        );
        self.update_body(body_map);
        self
    }

    /// 群头像 URL，最大长度 1024 个字符
    pub fn avatar(mut self, avatar: impl ToString) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "avatar".to_string(),
            serde_json::Value::String(avatar.to_string()),
        );
        self.update_body(body_map);
        self
    }

    /// 群描述，最大长度 1024 个字符
    pub fn description(mut self, description: impl ToString) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "description".to_string(),
            serde_json::Value::String(description.to_string()),
        );
        self.update_body(body_map);
        self
    }

    /// 是否为外部群，默认为 false
    pub fn external(mut self, external: bool) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert("external".to_string(), serde_json::Value::Bool(external));
        self.update_body(body_map);
        self
    }

    /// 是否开启群消息免打扰，默认为 false
    pub fn message_alert(mut self, message_alert: bool) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "message_alert".to_string(),
            serde_json::Value::Bool(message_alert),
        );
        self.update_body(body_map);
        self
    }

    /// 群成员的 open_id 列表，最大支持 100 个成员
    pub fn user_id_list(mut self, user_id_list: Vec<impl ToString>) -> Self {
        let ids: Vec<serde_json::Value> = user_id_list
            .into_iter()
            .map(|id| serde_json::Value::String(id.to_string()))
            .collect();
        let mut body_map = self.get_body_map();
        body_map.insert("user_id_list".to_string(), serde_json::Value::Array(ids));
        self.update_body(body_map);
        self
    }

    /// 群成员的 union_id 列表，最大支持 100 个成员
    pub fn union_id_list(mut self, union_id_list: Vec<impl ToString>) -> Self {
        let ids: Vec<serde_json::Value> = union_id_list
            .into_iter()
            .map(|id| serde_json::Value::String(id.to_string()))
            .collect();
        let mut body_map = self.get_body_map();
        body_map.insert("union_id_list".to_string(), serde_json::Value::Array(ids));
        self.update_body(body_map);
        self
    }

    /// 群成员的 user_id 列表，最大支持 100 个成员
    pub fn user_id_type_user_list(mut self, user_id_type_user_list: Vec<impl ToString>) -> Self {
        let ids: Vec<serde_json::Value> = user_id_type_user_list
            .into_iter()
            .map(|id| serde_json::Value::String(id.to_string()))
            .collect();
        let mut body_map = self.get_body_map();
        body_map.insert(
            "user_id_type_user_list".to_string(),
            serde_json::Value::Array(ids),
        );
        self.update_body(body_map);
        self
    }

    /// 群公告，最大长度 1024 个字符
    pub fn announcement(mut self, announcement: impl ToString) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "announcement".to_string(),
            serde_json::Value::String(announcement.to_string()),
        );
        self.update_body(body_map);
        self
    }

    /// 是否启用群管理，默认为 false
    pub fn public_field(mut self, public_field: bool) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "public_field".to_string(),
            serde_json::Value::Bool(public_field),
        );
        self.update_body(body_map);
        self
    }

    /// 群加入权限，默认为 1
    /// 1：任何人可加入
    /// 2：需要管理员审批
    /// 3：禁止加入
    pub fn join_permission(mut self, join_permission: i32) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "join_permission".to_string(),
            serde_json::Value::Number(join_permission.into()),
        );
        self.update_body(body_map);
        self
    }

    /// 是否开启群共享聊天记录，默认为 false
    pub fn share_allowed(mut self, share_allowed: bool) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "share_allowed".to_string(),
            serde_json::Value::Bool(share_allowed),
        );
        self.update_body(body_map);
        self
    }

    /// 群聊日历可见性，默认为 0
    /// 0：所有人可见
    /// 1：仅群成员可见
    pub fn calendar_visible(mut self, calendar_visible: i32) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "calendar_visible".to_string(),
            serde_json::Value::Number(calendar_visible.into()),
        );
        self.update_body(body_map);
        self
    }

    /// 群日程可见性，默认为 0
    /// 0：所有人可见
    /// 1：仅群成员可见
    pub fn schedule_visible(mut self, schedule_visible: i32) -> Self {
        let mut body_map = self.get_body_map();
        body_map.insert(
            "schedule_visible".to_string(),
            serde_json::Value::Number(schedule_visible.into()),
        );
        self.update_body(body_map);
        self
    }

    pub fn build(self) -> CreateChatRequest {
        self.request
    }

    // Helper methods
    fn get_body_map(&self) -> serde_json::Map<String, serde_json::Value> {
        if self.request.api_req.body.is_empty() {
            serde_json::Map::new()
        } else {
            serde_json::from_slice(&self.request.api_req.body).unwrap_or_default()
        }
    }

    fn update_body(&mut self, body_map: serde_json::Map<String, serde_json::Value>) {
        self.request.api_req.body = serde_json::to_vec(&body_map).unwrap_or_default();
    }
}

// 应用ExecutableBuilder trait到CreateChatRequestBuilder
crate::impl_executable_builder_owned!(
    CreateChatRequestBuilder,
    ChatsService,
    CreateChatRequest,
    CreateChatRespData,
    create
);

impl ChatsService {
    /// 创建群
    ///
    /// 创建群并设置群头像、群名、群描述等。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create
    pub async fn create(
        &self,
        create_chat_request: CreateChatRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateChatRespData> {
        let mut api_req = create_chat_request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = crate::core::endpoints::im::IM_CHAT_CREATE.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<CreateChatRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatRespData {
    /// 群 ID
    pub chat_id: String,
    /// 群名称
    pub name: String,
    /// 群头像 URL
    pub avatar: Option<String>,
    /// 群描述
    pub description: Option<String>,
    /// 群创建时间
    pub create_time: String,
}

impl ApiResponseTrait for CreateChatRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 群加入权限常量
pub mod join_permissions {
    /// 任何人可加入
    pub const ANYONE: i32 = 1;
    /// 需要管理员审批
    pub const APPROVAL_REQUIRED: i32 = 2;
    /// 禁止加入
    pub const FORBIDDEN: i32 = 3;
}

/// 可见性常量
pub mod visibility {
    /// 所有人可见
    pub const ALL: i32 = 0;
    /// 仅群成员可见
    pub const MEMBERS_ONLY: i32 = 1;
}
