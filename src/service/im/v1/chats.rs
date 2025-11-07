#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

//! 群聊服务
//!
//! 提供飞书开放平台群聊管理的 v1 版本 API 实现，包括：
//! - 创建群聊
//! - 获取群信息
//! - 更新群信息
//! - 解散群聊
//! - 群成员管理
//! - 群权限管理

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// 群聊服务
#[derive(Debug, Clone)]
pub struct ChatsService {
    config: Config,
}

impl ChatsService {
    /// 创建新的群聊服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建群聊
    ///
    /// 创建一个新的群聊，可以指定群名称、描述、成员等信息
    ///
    /// # 参数
    /// * `req` - 创建群聊请求
    ///
    /// # 返回值
    /// 返回创建的群聊信息
    ///
    /// # 示例
    /// ```rust
    /// let request = CreateChatRequest::new()
    ///     .name("项目讨论组")
    ///     .description("用于项目相关的讨论")
    ///     .user_id_list(vec!["user_1".to_string(), "user_2".to_string()]);
    ///
    /// let response = chats_service.create(&request).await?;
    /// println!("群聊创建成功，chat_id: {}", response.chat_id);
    /// ```
    pub async fn create(&self, req: &CreateChatRequest) -> SDKResult<CreateChatResponse> {
        // 验证请求参数
        req.validate()?;

        debug!("开始创建群聊: {}", req.name);

        let mut query_params: HashMap<&str, String> = HashMap::new();

        // 设置用户ID类型
        let user_id_type = req.user_id_type.as_deref().unwrap_or("open_id");
        query_params.insert("user_id_type", user_id_type.to_string());

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::IM_CHAT_CREATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateChatResponse>::request(api_req, &self.config, None).await?;

        let response = resp.data.unwrap_or_default();
        info!(
            "群聊创建成功: chat_id={}, name={}",
            response.chat_id,
            response.name.as_deref().unwrap_or_default()
        );

        Ok(response)
    }

    /// 解散群聊
    ///
    /// 解散指定的群聊，仅群主或管理员可以执行此操作
    ///
    /// # 参数
    /// * `req` - 解散群聊请求
    ///
    /// # 返回值
    /// 返回解散操作的结果
    ///
    /// # 示例
    /// ```rust
    /// let request = DeleteChatRequest::new("chat_123456")
    ///     .user_id_type("open_id");
    ///
    /// let response = chats_service.delete(&request).await?;
    /// if response.success {
    ///     println!("群聊解散成功");
    /// }
    /// ```
    ///
    /// # 权限要求
    /// - 仅群主可以解散群聊
    /// - 需要具有相应的应用权限
    ///
    /// # 错误处理
    /// - HTTP 403: 权限不足
    /// - HTTP 404: 群聊不存在
    /// - HTTP 400: 参数错误
    pub async fn delete(&self, req: &DeleteChatRequest) -> SDKResult<DeleteChatResponse> {
        // 验证请求参数
        req.validate()?;

        debug!("开始解散群聊: {}", req.chat_id);

        let mut query_params: HashMap<&str, String> = HashMap::new();

        // 设置用户ID类型
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API路径，替换chat_id占位符
        let api_path = crate::core::endpoints_original::Endpoints::IM_CHAT_DELETE
            .replace("{chat_id}", &req.chat_id);

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<DeleteChatResponse>::request(api_req, &self.config, None).await?;

        let response = resp.data.unwrap_or_default();

        if response.success {
            info!("群聊解散成功: chat_id={}", req.chat_id);
        } else {
            warn!(
                "群聊解散失败: chat_id={}, error={:?}",
                req.chat_id, response.error_message
            );
        }

        Ok(response)
    }

    /// 获取群公告基本信息
    ///
    /// 获取指定群聊的公告基本信息，包括标题、内容、创建时间等
    ///
    /// # 参数
    /// * `req` - 获取群公告请求
    ///
    /// # 返回值
    /// 返回群公告基本信息
    ///
    /// # 错误处理
    /// - HTTP 403: 权限不足
    /// - HTTP 404: 群聊不存在或公告不存在
    /// - HTTP 400: 参数错误
    pub async fn get_announcement(
        &self,
        req: &GetGroupAnnouncementRequest,
    ) -> SDKResult<GetGroupAnnouncementResponse> {
        // 验证请求参数
        req.validate()?;

        debug!("开始获取群公告: chat_id={}", req.chat_id);

        let mut query_params: HashMap<&str, String> = HashMap::new();

        // 设置用户ID类型
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API路径，替换chat_id占位符
        let api_path = crate::core::endpoints_original::Endpoints::DOCX_V1_CHAT_ANNOUNCEMENT
            .replace("{}", &req.chat_id);

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<GetGroupAnnouncementResponse>::request(api_req, &self.config, None).await?;

        let response = resp.data.unwrap_or_default();

        if response.success {
            info!("群公告获取成功: chat_id={}", req.chat_id);
            if let Some(announcement) = &response.announcement {
                debug!(
                    "公告详情: title={:?}, status={:?}",
                    announcement.title, announcement.status
                );
            }
        } else {
            warn!(
                "群公告获取失败: chat_id={}, error={:?}",
                req.chat_id, response.error_message
            );
        }

        Ok(response)
    }

    /// 获取群公告块的内容
    ///
    /// 获取指定群公告块的详细内容信息
    ///
    /// # 参数
    /// * `req` - 获取群公告块内容请求
    ///
    /// # 返回值
    /// 返回群公告块内容的详细信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::im::v1::chats::{GetAnnouncementBlockContentRequest};
    /// let request = GetAnnouncementBlockContentRequest::new("chat_id", "block_id")
    ///     .user_id_type("open_id");
    /// let response = service.get_announcement_block_content(&request).await?;
    /// ```
    pub async fn get_announcement_block_content(
        &self,
        req: &GetAnnouncementBlockContentRequest,
    ) -> SDKResult<GetAnnouncementBlockContentResponse> {
        req.validate()?;
        debug!(
            "开始获取群公告块内容: chat_id={}, block_id={}",
            req.chat_id, req.block_id
        );

        // 构建查询参数
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API路径，替换chat_id和block_id占位符
        let api_path = crate::core::endpoints_original::Endpoints::DOCX_V1_CHAT_ANNOUNCEMENT_BLOCK
            .replace("{}", &req.chat_id)
            .replace("{}", &req.block_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<GetAnnouncementBlockContentResponse>::request(api_req, &self.config, None)
                .await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            info!(
                "群公告块内容获取成功: chat_id={}, block_id={}",
                req.chat_id, req.block_id
            );
        } else {
            warn!(
                "群公告块内容获取失败: chat_id={}, block_id={}, error={:?}",
                req.chat_id, req.block_id, response.error_message
            );
        }

        Ok(response)
    }
}

// ==================== 数据模型 ====================

/// 创建群聊请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatRequest {
    /// 群名称
    ///
    /// 群聊的显示名称，长度限制：1-100个字符
    pub name: String,
    /// 群描述
    ///
    /// 群聊的描述信息，长度限制：0-500个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 群头像
    ///
    /// 群聊头像的image_key，通过上传图片接口获得
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 用户ID列表
    ///
    /// 要加入群聊的用户ID列表，最多支持50个用户
    pub user_id_list: Vec<String>,
    /// 用户ID类型
    ///
    /// 可选值：open_id、user_id、union_id
    /// 默认值：open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 群类型
    ///
    /// private: 私有群聊
    /// public: 公开群聊
    /// 默认值：private
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// 群加入权限
    ///
    /// all: 所有人可加入
    /// need_approval: 需要管理员同意
    /// forbidden: 禁止加入
    /// 默认值：need_approval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_permission: Option<String>,
    /// 群共享权限
    ///
    /// all: 所有人可分享
    /// admin: 仅管理员可分享
    /// forbidden: 禁止分享
    /// 默认值：admin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_permission: Option<String>,
    /// 群管理模式
    ///
    /// all: 所有人可管理
    /// admin: 仅管理员可管理
    /// owner: 仅群主可管理
    /// 默认值：admin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_mode: Option<String>,
    /// 去重标识
    ///
    /// 由开发者生成的唯一字符串序列，用于创建群请求去重
    /// 持有相同uuid的请求1小时内至多成功创建一个群
    /// 最大长度：50字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl CreateChatRequest {
    /// 创建新的创建群聊请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置群名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置群描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置群头像
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// 设置用户ID列表
    pub fn user_id_list(mut self, user_id_list: Vec<String>) -> Self {
        self.user_id_list = user_id_list;
        self
    }

    /// 添加单个用户ID
    pub fn add_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id_list.push(user_id.into());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置群类型
    pub fn chat_type(mut self, chat_type: impl Into<String>) -> Self {
        self.chat_type = Some(chat_type.into());
        self
    }

    /// 设置群加入权限
    pub fn join_permission(mut self, join_permission: impl Into<String>) -> Self {
        self.join_permission = Some(join_permission.into());
        self
    }

    /// 设置群共享权限
    pub fn share_permission(mut self, share_permission: impl Into<String>) -> Self {
        self.share_permission = Some(share_permission.into());
        self
    }

    /// 设置群管理模式
    pub fn management_mode(mut self, management_mode: impl Into<String>) -> Self {
        self.management_mode = Some(management_mode.into());
        self
    }

    /// 设置去重标识
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证群名称
        if self.name.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "群名称不能为空".to_string(),
            ));
        }

        if self.name.len() > 100 {
            return Err(LarkAPIError::IllegalParamError(
                "群名称长度不能超过100个字符".to_string(),
            ));
        }

        // 验证群描述
        if let Some(description) = &self.description {
            if description.len() > 500 {
                return Err(LarkAPIError::IllegalParamError(
                    "群描述长度不能超过500个字符".to_string(),
                ));
            }
        }

        // 验证用户ID列表
        if self.user_id_list.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "用户ID列表不能为空".to_string(),
            ));
        }

        if self.user_id_list.len() > 50 {
            return Err(LarkAPIError::IllegalParamError(
                "用户ID数量不能超过50个".to_string(),
            ));
        }

        // 验证用户ID类型
        if let Some(user_id_type) = &self.user_id_type {
            let valid_types = ["open_id", "user_id", "union_id"];
            if !valid_types.contains(&user_id_type.as_str()) {
                return Err(LarkAPIError::IllegalParamError(format!(
                    "无效的用户ID类型: {}，支持的类型: {:?}",
                    user_id_type, valid_types
                )));
            }
        }

        // 验证群类型
        if let Some(chat_type) = &self.chat_type {
            let valid_types = ["private", "public"];
            if !valid_types.contains(&chat_type.as_str()) {
                return Err(LarkAPIError::IllegalParamError(format!(
                    "无效的群类型: {}，支持的类型: {:?}",
                    chat_type, valid_types
                )));
            }
        }

        // 验证群加入权限
        if let Some(join_permission) = &self.join_permission {
            let valid_permissions = ["all", "need_approval", "forbidden"];
            if !valid_permissions.contains(&join_permission.as_str()) {
                return Err(LarkAPIError::IllegalParamError(format!(
                    "无效的加入权限: {}，支持的权限: {:?}",
                    join_permission, valid_permissions
                )));
            }
        }

        // 验证群共享权限
        if let Some(share_permission) = &self.share_permission {
            let valid_permissions = ["all", "admin", "forbidden"];
            if !valid_permissions.contains(&share_permission.as_str()) {
                return Err(LarkAPIError::IllegalParamError(format!(
                    "无效的共享权限: {}，支持的权限: {:?}",
                    share_permission, valid_permissions
                )));
            }
        }

        // 验证群管理模式
        if let Some(management_mode) = &self.management_mode {
            let valid_modes = ["all", "admin", "owner"];
            if !valid_modes.contains(&management_mode.as_str()) {
                return Err(LarkAPIError::IllegalParamError(format!(
                    "无效的管理模式: {}，支持的模式: {:?}",
                    management_mode, valid_modes
                )));
            }
        }

        // 验证UUID长度
        if let Some(uuid) = &self.uuid {
            if uuid.len() > 50 {
                return Err(LarkAPIError::IllegalParamError(
                    "UUID长度不能超过50个字符".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl Default for CreateChatRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            avatar: None,
            user_id_list: Vec::new(),
            user_id_type: Some("open_id".to_string()),
            chat_type: Some("private".to_string()),
            join_permission: Some("need_approval".to_string()),
            share_permission: Some("admin".to_string()),
            management_mode: Some("admin".to_string()),
            uuid: None,
        }
    }
}

/// 创建群聊响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateChatResponse {
    /// 群聊ID
    pub chat_id: String,
    /// 群名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 群描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 群头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 群类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 群成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// 无效的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_id_list: Option<Vec<String>>,
    /// 有效用户数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_user_id_count: Option<i32>,
    /// 无效用户数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_id_count: Option<i32>,
}

impl ApiResponseTrait for CreateChatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 解散群聊请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChatRequest {
    /// 群聊ID
    ///
    /// 要解散的群聊的唯一标识符
    pub chat_id: String,
    /// 用户ID类型
    ///
    /// 可选值：open_id、user_id、union_id
    /// 默认值：open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl DeleteChatRequest {
    /// 创建新的解散群聊请求
    pub fn new(chat_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id_type: Some("open_id".to_string()),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证群聊ID
        if self.chat_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "群聊ID不能为空".to_string(),
            ));
        }

        // 验证用户ID类型
        if let Some(user_id_type) = &self.user_id_type {
            let valid_types = ["open_id", "user_id", "union_id"];
            if !valid_types.contains(&user_id_type.as_str()) {
                return Err(LarkAPIError::IllegalParamError(format!(
                    "无效的用户ID类型: {}，支持的类型: {:?}",
                    user_id_type, valid_types
                )));
            }
        }

        Ok(())
    }
}

/// 解散群聊响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteChatResponse {
    /// 操作是否成功
    pub success: bool,
    /// 错误码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 操作时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_time: Option<String>,
}

impl ApiResponseTrait for DeleteChatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 群公告数据模型 ====================

/// 获取群公告基本信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupAnnouncementRequest {
    /// 群聊ID
    pub chat_id: String,
    /// 用户ID类型，可选值包括 open_id、user_id、union_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetGroupAnnouncementRequest {
    /// 创建新的获取群公告请求
    pub fn new(chat_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        if self.chat_id.is_empty() {
            return Err(LarkAPIError::illegal_param("群聊ID不能为空"));
        }

        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "user_id" | "union_id" => {}
                _ => {
                    return Err(LarkAPIError::illegal_param(
                        "无效的用户ID类型，必须是 open_id、user_id 或 union_id",
                    ));
                }
            }
        }

        Ok(())
    }
}

/// 群公告基本信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupAnnouncementInfo {
    /// 公告标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 公告内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 公告创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 公告更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 公告创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<AnnouncementCreator>,
    /// 公告状态（active/inactive）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 公告创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnnouncementCreator {
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 创建者名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 创建者头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// 获取群公告响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupAnnouncementResponse {
    /// 群公告信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement: Option<GroupAnnouncementInfo>,
    /// 操作是否成功
    pub success: bool,
    /// 错误码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ApiResponseTrait for GetGroupAnnouncementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 群公告块内容数据模型 ====================

/// 获取群公告块内容请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnnouncementBlockContentRequest {
    /// 群聊ID
    pub chat_id: String,
    /// 公告块ID
    pub block_id: String,
    /// 用户ID类型，可选值包括 open_id、user_id、union_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetAnnouncementBlockContentRequest {
    /// 创建新的获取群公告块内容请求
    pub fn new(chat_id: impl Into<String>, block_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            block_id: block_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        if self.chat_id.is_empty() {
            return Err(LarkAPIError::illegal_param("群聊ID不能为空"));
        }

        if self.block_id.is_empty() {
            return Err(LarkAPIError::illegal_param("公告块ID不能为空"));
        }

        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "user_id" | "union_id" => {}
                _ => {
                    return Err(LarkAPIError::illegal_param(
                        "无效的用户ID类型，必须是 open_id、user_id 或 union_id",
                    ));
                }
            }
        }

        Ok(())
    }
}

/// 获取群公告块内容响应数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAnnouncementBlockContentResponseData {
    /// 公告块内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 公告块类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    /// 公告块标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 获取群公告块内容响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAnnouncementBlockContentResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetAnnouncementBlockContentResponseData>,
    /// 操作是否成功
    pub success: bool,
    /// 错误码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ApiResponseTrait for GetAnnouncementBlockContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建群聊构建器
#[derive(Debug, Clone)]
pub struct CreateChatBuilder {
    request: CreateChatRequest,
}

impl Default for CreateChatBuilder {
    fn default() -> Self {
        Self {
            request: CreateChatRequest::default(),
        }
    }
}

impl CreateChatBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置群名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    /// 设置群描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置群头像
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.request.avatar = Some(avatar.into());
        self
    }

    /// 设置用户ID列表
    pub fn user_id_list(mut self, user_id_list: Vec<String>) -> Self {
        self.request.user_id_list = user_id_list;
        self
    }

    /// 添加单个用户ID
    pub fn add_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.request.user_id_list.push(user_id.into());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置群类型
    pub fn chat_type(mut self, chat_type: impl Into<String>) -> Self {
        self.request.chat_type = Some(chat_type.into());
        self
    }

    /// 设置群加入权限
    pub fn join_permission(mut self, join_permission: impl Into<String>) -> Self {
        self.request.join_permission = Some(join_permission.into());
        self
    }

    /// 设置群共享权限
    pub fn share_permission(mut self, share_permission: impl Into<String>) -> Self {
        self.request.share_permission = Some(share_permission.into());
        self
    }

    /// 设置群管理模式
    pub fn management_mode(mut self, management_mode: impl Into<String>) -> Self {
        self.request.management_mode = Some(management_mode.into());
        self
    }

    /// 设置去重标识
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.request.uuid = Some(uuid.into());
        self
    }

    /// 执行创建操作
    pub async fn execute(self, service: &ChatsService) -> SDKResult<CreateChatResponse> {
        service.create(&self.request).await
    }
}

impl ChatsService {
    /// 创建群聊构建器
    pub fn create_chat_builder(&self) -> CreateChatBuilder {
        CreateChatBuilder::new()
    }

    /// 解散群聊构建器
    pub fn delete_chat_builder(&self, chat_id: impl Into<String>) -> DeleteChatBuilder {
        DeleteChatBuilder::new(chat_id)
    }

    /// 获取群公告构建器
    pub fn get_announcement_builder(
        &self,
        chat_id: impl Into<String>,
    ) -> GetGroupAnnouncementBuilder {
        GetGroupAnnouncementBuilder::new(chat_id)
    }

    /// 获取群公告块内容构建器
    pub fn get_announcement_block_content_builder(
        &self,
        chat_id: impl Into<String>,
        block_id: impl Into<String>,
    ) -> GetAnnouncementBlockContentBuilder {
        GetAnnouncementBlockContentBuilder::new(chat_id, block_id)
    }
}

/// 解散群聊构建器
#[derive(Debug, Clone)]
pub struct DeleteChatBuilder {
    request: DeleteChatRequest,
}

impl DeleteChatBuilder {
    /// 创建新的构建器
    pub fn new(chat_id: impl Into<String>) -> Self {
        Self {
            request: DeleteChatRequest::new(chat_id),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行解散操作
    pub async fn execute(self, service: &ChatsService) -> SDKResult<DeleteChatResponse> {
        service.delete(&self.request).await
    }
}

/// 获取群公告构建器
#[derive(Debug, Clone)]
pub struct GetGroupAnnouncementBuilder {
    request: GetGroupAnnouncementRequest,
}

impl GetGroupAnnouncementBuilder {
    /// 创建新的构建器
    pub fn new(chat_id: impl Into<String>) -> Self {
        Self {
            request: GetGroupAnnouncementRequest::new(chat_id),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行获取群公告操作
    pub async fn execute(self, service: &ChatsService) -> SDKResult<GetGroupAnnouncementResponse> {
        service.get_announcement(&self.request).await
    }
}

/// 获取群公告块内容构建器
#[derive(Debug, Clone)]
pub struct GetAnnouncementBlockContentBuilder {
    request: GetAnnouncementBlockContentRequest,
}

impl GetAnnouncementBlockContentBuilder {
    /// 创建新的构建器
    pub fn new(chat_id: impl Into<String>, block_id: impl Into<String>) -> Self {
        Self {
            request: GetAnnouncementBlockContentRequest::new(chat_id, block_id),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行获取群公告块内容操作
    pub async fn execute(
        self,
        service: &ChatsService,
    ) -> SDKResult<GetAnnouncementBlockContentResponse> {
        service.get_announcement_block_content(&self.request).await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_chat_request_creation() {
        let request = CreateChatRequest::new();
        assert_eq!(request.name, "");
        assert_eq!(request.description, None);
        assert_eq!(request.avatar, None);
        assert_eq!(request.user_id_list, Vec::<String>::new());
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.chat_type, Some("private".to_string()));
    }

    #[test]
    fn test_create_chat_request_builder() {
        let user_ids = vec!["user_1".to_string(), "user_2".to_string()];
        let request = CreateChatRequest::new()
            .name("测试群聊")
            .description("这是一个测试群聊")
            .avatar("img_v2_test_avatar")
            .user_id_list(user_ids.clone())
            .user_id_type("user_id")
            .chat_type("public")
            .join_permission("all")
            .share_permission("all")
            .management_mode("owner")
            .uuid("test_uuid_123");

        assert_eq!(request.name, "测试群聊");
        assert_eq!(request.description, Some("这是一个测试群聊".to_string()));
        assert_eq!(request.avatar, Some("img_v2_test_avatar".to_string()));
        assert_eq!(request.user_id_list, user_ids);
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.chat_type, Some("public".to_string()));
        assert_eq!(request.join_permission, Some("all".to_string()));
        assert_eq!(request.share_permission, Some("all".to_string()));
        assert_eq!(request.management_mode, Some("owner".to_string()));
        assert_eq!(request.uuid, Some("test_uuid_123".to_string()));
    }

    #[test]
    fn test_create_chat_request_validation_success() {
        let request = CreateChatRequest::new()
            .name("有效群名")
            .user_id_list(vec!["user_1".to_string()]);

        let result = request.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_chat_request_validation_empty_name() {
        let request = CreateChatRequest::new().user_id_list(vec!["user_1".to_string()]);

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("群名称不能为空"));
    }

    #[test]
    fn test_create_chat_request_validation_name_too_long() {
        let long_name = "a".repeat(101);
        let request = CreateChatRequest::new()
            .name(long_name)
            .user_id_list(vec!["user_1".to_string()]);

        let result = request.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("群名称长度不能超过100个字符"));
    }

    #[test]
    fn test_create_chat_request_validation_empty_user_list() {
        let request = CreateChatRequest::new().name("测试群");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("用户ID列表不能为空"));
    }

    #[test]
    fn test_create_chat_request_validation_too_many_users() {
        let user_list: Vec<String> = (0..51).map(|i| format!("user_{}", i)).collect();
        let request = CreateChatRequest::new()
            .name("测试群")
            .user_id_list(user_list);

        let result = request.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("用户ID数量不能超过50个"));
    }

    #[test]
    fn test_create_chat_request_validation_invalid_user_id_type() {
        let request = CreateChatRequest::new()
            .name("测试群")
            .user_id_list(vec!["user_1".to_string()])
            .user_id_type("invalid_type");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("无效的用户ID类型"));
    }

    #[test]
    fn test_create_chat_request_validation_invalid_chat_type() {
        let request = CreateChatRequest::new()
            .name("测试群")
            .user_id_list(vec!["user_1".to_string()])
            .chat_type("invalid_type");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("无效的群类型"));
    }

    #[test]
    fn test_create_chat_request_validation_uuid_too_long() {
        let long_uuid = "a".repeat(51);
        let request = CreateChatRequest::new()
            .name("测试群")
            .user_id_list(vec!["user_1".to_string()])
            .uuid(long_uuid);

        let result = request.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("UUID长度不能超过50个字符"));
    }

    #[test]
    fn test_create_chat_request_default_values() {
        let request = CreateChatRequest::default();
        assert_eq!(request.name, "");
        assert_eq!(request.description, None);
        assert_eq!(request.avatar, None);
        assert_eq!(request.user_id_list, Vec::<String>::new());
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.chat_type, Some("private".to_string()));
        assert_eq!(request.join_permission, Some("need_approval".to_string()));
        assert_eq!(request.share_permission, Some("admin".to_string()));
        assert_eq!(request.management_mode, Some("admin".to_string()));
        assert_eq!(request.uuid, None);
    }

    #[test]
    fn test_create_chat_response_creation() {
        let response = CreateChatResponse::default();
        assert_eq!(response.chat_id, "");
        assert_eq!(response.name, None);
        assert_eq!(response.description, None);
        assert_eq!(response.avatar, None);
        assert_eq!(response.chat_type, None);
        assert_eq!(response.create_time, None);
        assert_eq!(response.creator_id, None);
        assert_eq!(response.member_count, None);
        assert_eq!(response.invalid_user_id_list, None);
        assert_eq!(response.valid_user_id_count, None);
        assert_eq!(response.invalid_user_id_count, None);
    }

    #[test]
    fn test_create_chat_response_with_data() {
        let invalid_ids = vec!["invalid_user_1".to_string(), "invalid_user_2".to_string()];
        let response = CreateChatResponse {
            chat_id: "chat_123456".to_string(),
            name: Some("测试群聊".to_string()),
            description: Some("测试群描述".to_string()),
            avatar: Some("img_v2_avatar".to_string()),
            chat_type: Some("private".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            creator_id: Some("creator_123".to_string()),
            member_count: Some(98),
            invalid_user_id_list: Some(invalid_ids.clone()),
            valid_user_id_count: Some(98),
            invalid_user_id_count: Some(2),
        };

        assert_eq!(response.chat_id, "chat_123456");
        assert_eq!(response.name, Some("测试群聊".to_string()));
        assert_eq!(response.description, Some("测试群描述".to_string()));
        assert_eq!(response.avatar, Some("img_v2_avatar".to_string()));
        assert_eq!(response.chat_type, Some("private".to_string()));
        assert_eq!(
            response.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(response.creator_id, Some("creator_123".to_string()));
        assert_eq!(response.member_count, Some(98));
        assert_eq!(response.invalid_user_id_list, Some(invalid_ids));
        assert_eq!(response.valid_user_id_count, Some(98));
        assert_eq!(response.invalid_user_id_count, Some(2));
    }

    #[test]
    fn test_create_chat_response_serialization() {
        let response = CreateChatResponse {
            chat_id: "chat_test_123".to_string(),
            name: Some("序列化测试群".to_string()),
            description: Some("用于序列化测试的群聊".to_string()),
            avatar: Some("img_v2_serialize".to_string()),
            chat_type: Some("private".to_string()),
            create_time: Some("2023-12-31T23:59:59Z".to_string()),
            creator_id: Some("creator_test".to_string()),
            member_count: Some(10),
            invalid_user_id_list: Some(vec!["bad_user".to_string()]),
            valid_user_id_count: Some(9),
            invalid_user_id_count: Some(1),
        };

        // Test serialization
        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("chat_test_123"));
        assert!(serialized.contains("序列化测试群"));
        assert!(serialized.contains("用于序列化测试的群聊"));
        assert!(serialized.contains("img_v2_serialize"));
        assert!(serialized.contains("private"));
        assert!(serialized.contains("2023-12-31T23:59:59Z"));
        assert!(serialized.contains("creator_test"));
        assert!(serialized.contains("10"));
        assert!(serialized.contains("bad_user"));
        assert!(serialized.contains("9"));
        assert!(serialized.contains("1"));

        // Test deserialization
        let deserialized: CreateChatResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.chat_id, "chat_test_123");
        assert_eq!(deserialized.name, Some("序列化测试群".to_string()));
        assert_eq!(
            deserialized.description,
            Some("用于序列化测试的群聊".to_string())
        );
        assert_eq!(deserialized.avatar, Some("img_v2_serialize".to_string()));
        assert_eq!(deserialized.chat_type, Some("private".to_string()));
        assert_eq!(
            deserialized.create_time,
            Some("2023-12-31T23:59:59Z".to_string())
        );
        assert_eq!(deserialized.creator_id, Some("creator_test".to_string()));
        assert_eq!(deserialized.member_count, Some(10));
        assert_eq!(deserialized.valid_user_id_count, Some(9));
        assert_eq!(deserialized.invalid_user_id_count, Some(1));
    }

    #[test]
    fn test_create_chat_request_serialization() {
        let request = CreateChatRequest {
            name: "序列化测试群".to_string(),
            description: Some("用于序列化测试".to_string()),
            avatar: Some("img_v2_serialize".to_string()),
            user_id_list: vec!["user_1".to_string(), "user_2".to_string()],
            user_id_type: Some("open_id".to_string()),
            chat_type: Some("private".to_string()),
            join_permission: Some("all".to_string()),
            share_permission: Some("admin".to_string()),
            management_mode: Some("owner".to_string()),
            uuid: Some("serialize_uuid_123".to_string()),
        };

        // Test serialization
        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("序列化测试群"));
        assert!(serialized.contains("用于序列化测试"));
        assert!(serialized.contains("img_v2_serialize"));
        assert!(serialized.contains("user_1"));
        assert!(serialized.contains("user_2"));
        assert!(serialized.contains("open_id"));
        assert!(serialized.contains("private"));
        assert!(serialized.contains("all"));
        assert!(serialized.contains("admin"));
        assert!(serialized.contains("owner"));
        assert!(serialized.contains("serialize_uuid_123"));

        // Test deserialization
        let deserialized: CreateChatRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.name, "序列化测试群");
        assert_eq!(deserialized.description, Some("用于序列化测试".to_string()));
        assert_eq!(deserialized.avatar, Some("img_v2_serialize".to_string()));
        assert_eq!(
            deserialized.user_id_list,
            vec!["user_1".to_string(), "user_2".to_string()]
        );
        assert_eq!(deserialized.user_id_type, Some("open_id".to_string()));
        assert_eq!(deserialized.chat_type, Some("private".to_string()));
        assert_eq!(deserialized.join_permission, Some("all".to_string()));
        assert_eq!(deserialized.share_permission, Some("admin".to_string()));
        assert_eq!(deserialized.management_mode, Some("owner".to_string()));
        assert_eq!(deserialized.uuid, Some("serialize_uuid_123".to_string()));
    }

    #[test]
    fn test_create_chat_api_response_trait() {
        assert_eq!(CreateChatResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_chat_endpoint_constant() {
        // Test that the endpoint constant is properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_CHAT_CREATE,
            "/open-apis/im/v1/chats"
        );
    }

    #[test]
    fn test_create_chat_builder() {
        let builder = CreateChatBuilder::new()
            .name("构建器测试群")
            .description("使用构建器创建的群聊")
            .add_user_id("user_1")
            .add_user_id("user_2")
            .add_user_id("user_3")
            .user_id_type("user_id")
            .chat_type("public")
            .join_permission("need_approval")
            .share_permission("admin")
            .management_mode("all")
            .uuid("builder_uuid_456");

        assert_eq!(builder.request.name, "构建器测试群");
        assert_eq!(
            builder.request.description,
            Some("使用构建器创建的群聊".to_string())
        );
        assert_eq!(
            builder.request.user_id_list,
            vec!["user_1", "user_2", "user_3"]
        );
        assert_eq!(builder.request.user_id_type, Some("user_id".to_string()));
        assert_eq!(builder.request.chat_type, Some("public".to_string()));
        assert_eq!(
            builder.request.join_permission,
            Some("need_approval".to_string())
        );
        assert_eq!(builder.request.share_permission, Some("admin".to_string()));
        assert_eq!(builder.request.management_mode, Some("all".to_string()));
        assert_eq!(builder.request.uuid, Some("builder_uuid_456".to_string()));
    }

    #[test]
    fn test_create_chat_builder_default() {
        let builder = CreateChatBuilder::default();
        assert_eq!(builder.request.name, "");
        assert_eq!(builder.request.description, None);
        assert_eq!(builder.request.avatar, None);
        assert_eq!(builder.request.user_id_list, Vec::<String>::new());
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
        assert_eq!(builder.request.chat_type, Some("private".to_string()));
        assert_eq!(
            builder.request.join_permission,
            Some("need_approval".to_string())
        );
        assert_eq!(builder.request.share_permission, Some("admin".to_string()));
        assert_eq!(builder.request.management_mode, Some("admin".to_string()));
        assert_eq!(builder.request.uuid, None);
    }

    #[test]
    fn test_create_chat_service_builder() {
        let config = Config::default();
        let service = ChatsService::new(config);

        let builder = service.create_chat_builder();
        assert_eq!(builder.request.name, "");
        assert_eq!(builder.request.description, None);
        assert_eq!(builder.request.avatar, None);
        assert_eq!(builder.request.user_id_list, Vec::<String>::new());
    }

    #[test]
    fn test_create_chat_different_chat_types() {
        // Test different chat types
        let chat_types = vec![("private", "私有群聊"), ("public", "公开群聊")];

        for (chat_type, description) in chat_types {
            let request = CreateChatRequest::new()
                .name(format!("{}-测试群", chat_type))
                .description(description)
                .chat_type(chat_type)
                .add_user_id("test_user");

            assert_eq!(request.name, format!("{}-测试群", chat_type));
            assert_eq!(request.description, Some(description.to_string()));
            assert_eq!(request.chat_type, Some(chat_type.to_string()));
            assert_eq!(request.user_id_list, vec!["test_user"]);
        }
    }

    #[test]
    fn test_create_chat_large_user_list() {
        // Test with maximum allowed user ID list (50 users)
        let user_id_list: Vec<String> = (0..50).map(|i| format!("user_{}", i)).collect();
        let request = CreateChatRequest::new()
            .name("大规模用户群测试")
            .user_id_list(user_id_list.clone())
            .description("包含50个用户的大规模群聊测试");

        assert_eq!(request.user_id_list.len(), 50);
        assert_eq!(request.user_id_list, user_id_list);

        // Validation should succeed for exactly 50 users
        let result = request.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_chat_comprehensive_response() {
        // Test comprehensive response with all fields
        let invalid_ids = vec![
            "invalid_user_1".to_string(),
            "invalid_user_2".to_string(),
            "invalid_user_3".to_string(),
            "invalid_user_4".to_string(),
            "invalid_user_5".to_string(),
        ];
        let comprehensive_response = CreateChatResponse {
            chat_id: "comprehensive_chat_001".to_string(),
            name: Some("全面测试群聊".to_string()),
            description: Some("包含所有字段的全面测试群聊".to_string()),
            avatar: Some("comprehensive_avatar".to_string()),
            chat_type: Some("private".to_string()),
            create_time: Some("2023-06-15T10:30:00Z".to_string()),
            creator_id: Some("comprehensive_creator".to_string()),
            member_count: Some(95),
            invalid_user_id_list: Some(invalid_ids.clone()),
            valid_user_id_count: Some(95),
            invalid_user_id_count: Some(5),
        };

        assert_eq!(comprehensive_response.chat_id, "comprehensive_chat_001");
        assert_eq!(
            comprehensive_response.name,
            Some("全面测试群聊".to_string())
        );
        assert_eq!(
            comprehensive_response.description,
            Some("包含所有字段的全面测试群聊".to_string())
        );
        assert_eq!(
            comprehensive_response.avatar,
            Some("comprehensive_avatar".to_string())
        );
        assert_eq!(
            comprehensive_response.chat_type,
            Some("private".to_string())
        );
        assert_eq!(
            comprehensive_response.create_time,
            Some("2023-06-15T10:30:00Z".to_string())
        );
        assert_eq!(
            comprehensive_response.creator_id,
            Some("comprehensive_creator".to_string())
        );
        assert_eq!(comprehensive_response.member_count, Some(95));
        assert_eq!(
            comprehensive_response.invalid_user_id_list,
            Some(invalid_ids)
        );
        assert_eq!(comprehensive_response.valid_user_id_count, Some(95));
        assert_eq!(comprehensive_response.invalid_user_id_count, Some(5));
    }

    #[test]
    fn test_create_chat_request_add_multiple_users() {
        let builder = CreateChatBuilder::new()
            .name("多用户测试群")
            .add_user_id("user_1")
            .add_user_id("user_2")
            .add_user_id("user_3")
            .add_user_id("user_4")
            .add_user_id("user_5");

        assert_eq!(builder.request.user_id_list.len(), 5);
        assert_eq!(
            builder.request.user_id_list,
            vec!["user_1", "user_2", "user_3", "user_4", "user_5"]
        );
    }

    // ==================== 解散群功能测试 ====================

    #[test]
    fn test_delete_chat_request_creation() {
        let request = DeleteChatRequest::new("chat_123456");
        assert_eq!(request.chat_id, "chat_123456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_delete_chat_request_builder() {
        let request = DeleteChatRequest::new("chat_test_789").user_id_type("user_id");

        assert_eq!(request.chat_id, "chat_test_789");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_delete_chat_request_validation_success() {
        let request = DeleteChatRequest::new("valid_chat_id");
        let result = request.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_chat_request_validation_empty_chat_id() {
        let request = DeleteChatRequest::new("");
        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("群聊ID不能为空"));
    }

    #[test]
    fn test_delete_chat_request_validation_invalid_user_id_type() {
        let request = DeleteChatRequest::new("chat_123").user_id_type("invalid_type");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("无效的用户ID类型"));
    }

    #[test]
    fn test_delete_chat_response_creation() {
        let response = DeleteChatResponse::default();
        assert_eq!(response.success, false);
        assert_eq!(response.error_code, None);
        assert_eq!(response.error_message, None);
        assert_eq!(response.operation_time, None);
    }

    #[test]
    fn test_delete_chat_response_with_data() {
        let response = DeleteChatResponse {
            success: true,
            error_code: None,
            error_message: None,
            operation_time: Some("2023-01-01T12:00:00Z".to_string()),
        };

        assert_eq!(response.success, true);
        assert_eq!(response.error_code, None);
        assert_eq!(response.error_message, None);
        assert_eq!(
            response.operation_time,
            Some("2023-01-01T12:00:00Z".to_string())
        );
    }

    #[test]
    fn test_delete_chat_response_failure() {
        let response = DeleteChatResponse {
            success: false,
            error_code: Some(403),
            error_message: Some("权限不足".to_string()),
            operation_time: Some("2023-01-01T12:00:00Z".to_string()),
        };

        assert_eq!(response.success, false);
        assert_eq!(response.error_code, Some(403));
        assert_eq!(response.error_message, Some("权限不足".to_string()));
        assert_eq!(
            response.operation_time,
            Some("2023-01-01T12:00:00Z".to_string())
        );
    }

    #[test]
    fn test_delete_chat_request_serialization() {
        let request = DeleteChatRequest {
            chat_id: "chat_serialize_123".to_string(),
            user_id_type: Some("union_id".to_string()),
        };

        // 测试序列化
        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("chat_serialize_123"));
        assert!(serialized.contains("union_id"));

        // 测试反序列化
        let deserialized: DeleteChatRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.chat_id, "chat_serialize_123");
        assert_eq!(deserialized.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_delete_chat_response_serialization() {
        let response = DeleteChatResponse {
            success: true,
            error_code: None,
            error_message: Some("操作成功".to_string()),
            operation_time: Some("2023-12-31T23:59:59Z".to_string()),
        };

        // 测试序列化
        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("true"));
        assert!(serialized.contains("操作成功"));
        assert!(serialized.contains("2023-12-31T23:59:59Z"));

        // 测试反序列化
        let deserialized: DeleteChatResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.success, true);
        assert_eq!(deserialized.error_message, Some("操作成功".to_string()));
        assert_eq!(
            deserialized.operation_time,
            Some("2023-12-31T23:59:59Z".to_string())
        );
    }

    #[test]
    fn test_delete_chat_api_response_trait() {
        assert_eq!(DeleteChatResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_chat_endpoint_constant() {
        // 测试解散群端点常量是否正确定义
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_CHAT_DELETE,
            "/open-apis/im/v1/chats/{chat_id}"
        );
    }

    #[test]
    fn test_delete_chat_builder() {
        let builder = DeleteChatBuilder::new("chat_builder_test").user_id_type("open_id");

        assert_eq!(builder.request.chat_id, "chat_builder_test");
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_delete_chat_service_builder() {
        let config = Config::default();
        let service = ChatsService::new(config);

        let builder = service.delete_chat_builder("chat_service_test");
        assert_eq!(builder.request.chat_id, "chat_service_test");
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_delete_chat_different_user_id_types() {
        let user_id_types = vec![
            ("open_id", "open_id"),
            ("user_id", "user_id"),
            ("union_id", "union_id"),
        ];

        for (user_id_type, expected) in user_id_types {
            let request = DeleteChatRequest::new("chat_test").user_id_type(user_id_type);

            assert_eq!(request.chat_id, "chat_test");
            assert_eq!(request.user_id_type, Some(expected.to_string()));

            // 验证应该成功
            let result = request.validate();
            assert!(
                result.is_ok(),
                "Valid user_id_type {} should pass validation",
                user_id_type
            );
        }
    }

    #[test]
    fn test_delete_chat_comprehensive_response() {
        let comprehensive_response = DeleteChatResponse {
            success: false,
            error_code: Some(404),
            error_message: Some("群聊不存在或已被删除".to_string()),
            operation_time: Some("2023-06-15T14:30:00Z".to_string()),
        };

        assert_eq!(comprehensive_response.success, false);
        assert_eq!(comprehensive_response.error_code, Some(404));
        assert_eq!(
            comprehensive_response.error_message,
            Some("群聊不存在或已被删除".to_string())
        );
        assert_eq!(
            comprehensive_response.operation_time,
            Some("2023-06-15T14:30:00Z".to_string())
        );
    }

    #[test]
    fn test_delete_chat_request_edge_cases() {
        // 测试包含特殊字符的群ID
        let special_chars_request = DeleteChatRequest::new("chat_特殊字符_123");
        assert_eq!(special_chars_request.chat_id, "chat_特殊字符_123");

        // 测试很长的群ID
        let long_chat_id = "chat_".to_string() + &"a".repeat(100);
        let long_id_request = DeleteChatRequest::new(&long_chat_id);
        assert_eq!(long_id_request.chat_id, long_chat_id);

        // 验证应该成功（群ID长度在API规范允许范围内）
        let result = long_id_request.validate();
        assert!(result.is_ok());
    }

    // ==================== 群公告相关测试 ====================

    #[test]
    fn test_get_group_announcement_request_creation() {
        let request = GetGroupAnnouncementRequest::new("test_chat_id");
        assert_eq!(request.chat_id, "test_chat_id");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_get_group_announcement_request_with_user_id_type() {
        let request = GetGroupAnnouncementRequest::new("test_chat_id").user_id_type("open_id");
        assert_eq!(request.chat_id, "test_chat_id");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_group_announcement_request_validation_empty_chat_id() {
        let request = GetGroupAnnouncementRequest::new("");
        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("群聊ID不能为空"));
    }

    #[test]
    fn test_get_group_announcement_request_validation_invalid_user_id_type() {
        let request = GetGroupAnnouncementRequest::new("chat_123").user_id_type("invalid_type");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("无效的用户ID类型"));
    }

    #[test]
    fn test_get_group_announcement_request_validation_valid_user_id_types() {
        let chat_id = "test_chat_123";
        let valid_types = vec!["open_id", "user_id", "union_id"];

        for user_id_type in valid_types {
            let request = GetGroupAnnouncementRequest::new(chat_id).user_id_type(user_id_type);

            // 验证应该成功
            let result = request.validate();
            assert!(
                result.is_ok(),
                "Valid user_id_type {} should pass validation",
                user_id_type
            );
        }
    }

    #[test]
    fn test_group_announcement_info_creation() {
        let announcement = GroupAnnouncementInfo {
            title: Some("测试公告".to_string()),
            content: Some("这是一个测试公告内容".to_string()),
            status: Some("active".to_string()),
            ..Default::default()
        };

        assert_eq!(announcement.title, Some("测试公告".to_string()));
        assert_eq!(
            announcement.content,
            Some("这是一个测试公告内容".to_string())
        );
        assert_eq!(announcement.status, Some("active".to_string()));
    }

    #[test]
    fn test_announcement_creator_creation() {
        let creator = AnnouncementCreator {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
        };

        assert_eq!(creator.user_id, Some("user_123".to_string()));
        assert_eq!(creator.name, Some("张三".to_string()));
        assert_eq!(
            creator.avatar,
            Some("https://example.com/avatar.jpg".to_string())
        );
    }

    #[test]
    fn test_get_group_announcement_response_creation() {
        let creator = AnnouncementCreator {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
            ..Default::default()
        };

        let announcement = GroupAnnouncementInfo {
            title: Some("测试公告".to_string()),
            content: Some("这是一个测试公告内容".to_string()),
            creator: Some(creator),
            ..Default::default()
        };

        let response = GetGroupAnnouncementResponse {
            announcement: Some(announcement),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.announcement.is_some());
        assert_eq!(
            response.announcement.as_ref().unwrap().title,
            Some("测试公告".to_string())
        );
        assert_eq!(
            response
                .announcement
                .as_ref()
                .unwrap()
                .creator
                .as_ref()
                .unwrap()
                .name,
            Some("张三".to_string())
        );
    }

    #[test]
    fn test_get_group_announcement_builder() {
        let builder = GetGroupAnnouncementBuilder::new("chat_builder_test").user_id_type("open_id");

        assert_eq!(builder.request.chat_id, "chat_builder_test");
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_group_announcement_endpoint_constant() {
        // 测试群公告端点常量是否正确定义
        assert_eq!(
            crate::core::endpoints_original::Endpoints::DOCX_V1_CHAT_ANNOUNCEMENT,
            "/open-apis/docx/v1/chats/{}/announcement"
        );
    }

    #[test]
    fn test_get_group_announcement_response_default() {
        let response = GetGroupAnnouncementResponse::default();
        assert!(!response.success);
        assert!(response.announcement.is_none());
        assert!(response.error_code.is_none());
        assert!(response.error_message.is_none());
    }

    #[test]
    fn test_get_group_announcement_response_error() {
        let error_response = GetGroupAnnouncementResponse {
            success: false,
            error_code: Some(404),
            error_message: Some("群公告不存在".to_string()),
            ..Default::default()
        };

        assert!(!error_response.success);
        assert_eq!(error_response.error_code, Some(404));
        assert_eq!(
            error_response.error_message,
            Some("群公告不存在".to_string())
        );
    }

    #[test]
    fn test_announcement_creator_default() {
        let creator = AnnouncementCreator::default();
        assert!(creator.user_id.is_none());
        assert!(creator.name.is_none());
        assert!(creator.avatar.is_none());
    }

    #[test]
    fn test_group_announcement_info_complete() {
        let timestamp = "2023-12-01T10:00:00Z".to_string();
        let creator = AnnouncementCreator {
            user_id: Some("creator_456".to_string()),
            name: Some("李四".to_string()),
            avatar: Some("https://example.com/avatar2.jpg".to_string()),
        };

        let announcement = GroupAnnouncementInfo {
            title: Some("完整测试公告".to_string()),
            content: Some("这是一个包含所有字段的完整公告".to_string()),
            create_time: Some(timestamp.clone()),
            update_time: Some(timestamp.clone()),
            creator: Some(creator),
            status: Some("active".to_string()),
        };

        assert_eq!(announcement.title, Some("完整测试公告".to_string()));
        assert_eq!(announcement.create_time, Some(timestamp.clone()));
        assert_eq!(announcement.update_time, Some(timestamp));
        assert!(announcement.creator.is_some());
        assert_eq!(
            announcement.creator.as_ref().unwrap().name,
            Some("李四".to_string())
        );
        assert_eq!(announcement.status, Some("active".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            GetGroupAnnouncementResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_get_group_announcement_comprehensive_scenario() {
        // 测试完整的业务场景
        let mut announcement = GroupAnnouncementInfo::default();
        announcement.title = Some("重要通知".to_string());
        announcement.content = Some("明天下午3点开会".to_string());
        announcement.status = Some("active".to_string());

        let creator = AnnouncementCreator {
            user_id: Some("admin_001".to_string()),
            name: Some("管理员".to_string()),
            ..Default::default()
        };
        announcement.creator = Some(creator);

        let response = GetGroupAnnouncementResponse {
            announcement: Some(announcement),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        let announcement_ref = response.announcement.as_ref().unwrap();
        assert_eq!(announcement_ref.title, Some("重要通知".to_string()));
        assert_eq!(
            announcement_ref.content,
            Some("明天下午3点开会".to_string())
        );
        assert_eq!(announcement_ref.status, Some("active".to_string()));
        assert_eq!(
            announcement_ref.creator.as_ref().unwrap().name,
            Some("管理员".to_string())
        );
    }

    // ==================== API #149 获取群公告块内容测试 ====================

    #[test]
    fn test_get_announcement_block_content_request_creation() {
        let request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
        assert_eq!(request.chat_id, "chat_123");
        assert_eq!(request.block_id, "block_456");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_get_announcement_block_content_request_with_user_id_type() {
        let request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456")
            .user_id_type("open_id");
        assert_eq!(request.chat_id, "chat_123");
        assert_eq!(request.block_id, "block_456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_announcement_block_content_request_validation() {
        // 测试正常情况
        let valid_request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
        assert!(valid_request.validate().is_ok());

        // 测试空chat_id
        let empty_chat_request = GetAnnouncementBlockContentRequest::new("", "block_456");
        assert!(empty_chat_request.validate().is_err());

        // 测试空block_id
        let empty_block_request = GetAnnouncementBlockContentRequest::new("chat_123", "");
        assert!(empty_block_request.validate().is_err());
    }

    #[test]
    fn test_get_announcement_block_content_response_creation() {
        let mut block_content = AnnouncementBlockContent::default();
        block_content.block_id = Some("block_456".to_string());
        block_content.block_type = Some("text".to_string());
        block_content.content = Some(serde_json::json!({
            "text": "这是公告块内容"
        }));

        let response = GetAnnouncementBlockContentResponse {
            block: Some(block_content),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        let block_ref = response.block.as_ref().unwrap();
        assert_eq!(block_ref.block_id, Some("block_456".to_string()));
        assert_eq!(block_ref.block_type, Some("text".to_string()));
        assert!(block_ref.content.is_some());
    }

    #[test]
    fn test_get_announcement_block_content_builder() {
        let builder = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456")
            .user_id_type("union_id");

        assert_eq!(builder.request.chat_id, "chat_123");
        assert_eq!(builder.request.block_id, "block_456");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_get_announcement_block_content_builder_validation() {
        // 测试有效构建器
        let valid_builder = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456");
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = GetAnnouncementBlockContentBuilder::new("", "block_456");
        assert!(invalid_builder.request.validate().is_err());
    }

    #[test]
    fn test_announcement_block_content_comprehensive() {
        // 测试完整的公告块内容结构
        let mut comprehensive_block = AnnouncementBlockContent::default();
        comprehensive_block.block_id = Some("comprehensive_block_001".to_string());
        comprehensive_block.parent_id = Some("parent_block_002".to_string());
        comprehensive_block.block_type = Some("rich_text".to_string());
        comprehensive_block.content = Some(serde_json::json!({
            "elements": [
                {
                    "type": "text",
                    "content": "重要通知："
                },
                {
                    "type": "text",
                    "content": "请大家准时参加明天的会议"
                }
            ]
        }));
        comprehensive_block.children = Some(vec![
            "child_block_001".to_string(),
            "child_block_002".to_string(),
        ]);

        let response = GetAnnouncementBlockContentResponse {
            block: Some(comprehensive_block),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        let block_ref = response.block.as_ref().unwrap();
        assert_eq!(
            block_ref.block_id,
            Some("comprehensive_block_001".to_string())
        );
        assert_eq!(block_ref.parent_id, Some("parent_block_002".to_string()));
        assert_eq!(block_ref.block_type, Some("rich_text".to_string()));
        assert!(block_ref.content.is_some());
        assert_eq!(block_ref.children.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_get_announcement_block_content_error_response() {
        let error_response = GetAnnouncementBlockContentResponse {
            block: None,
            success: false,
            error_message: Some("公告块不存在".to_string()),
            error_code: Some("BLOCK_NOT_FOUND".to_string()),
            ..Default::default()
        };

        assert!(!error_response.success);
        assert_eq!(
            error_response.error_message,
            Some("公告块不存在".to_string())
        );
        assert_eq!(
            error_response.error_code,
            Some("BLOCK_NOT_FOUND".to_string())
        );
        assert!(error_response.block.is_none());
    }

    #[test]
    fn test_get_announcement_block_content_all_user_id_types() {
        let user_id_types = ["open_id", "user_id", "union_id"];

        for user_id_type in user_id_types.iter() {
            let request = GetAnnouncementBlockContentRequest::new("chat_test", "block_test")
                .user_id_type(*user_id_type);
            assert!(request.validate().is_ok());
            assert_eq!(request.user_id_type, Some(user_id_type.to_string()));
        }
    }

    #[test]
    fn test_get_announcement_block_content_service_method() {
        // 测试服务方法存在性（实际HTTP调用需要模拟）
        let config = Config::default();
        let service = ChatsService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.get_announcement_block_content_builder("chat_123", "block_456");
        assert_eq!(builder.request.chat_id, "chat_123");
        assert_eq!(builder.request.block_id, "block_456");
    }

    #[test]
    fn test_announcement_block_content_json_serialization() {
        let block_content = AnnouncementBlockContent {
            block_id: Some("block_json_test".to_string()),
            block_type: Some("text".to_string()),
            content: Some(serde_json::json!({
                "format": "plain_text",
                "value": "JSON测试内容"
            })),
            ..Default::default()
        };

        // 测试序列化
        let serialized = serde_json::to_string(&block_content);
        assert!(serialized.is_ok());

        // 测试反序列化
        let deserialized: Result<AnnouncementBlockContent, _> =
            serde_json::from_str(&serialized.unwrap());
        assert!(deserialized.is_ok());
        let block = deserialized.unwrap();
        assert_eq!(block.block_id, Some("block_json_test".to_string()));
        assert_eq!(block.block_type, Some("text".to_string()));
    }

    #[test]
    fn test_get_announcement_block_content_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            crate::core::endpoints_original::Endpoints::DOCX_V1_CHAT_ANNOUNCEMENT_BLOCK,
            "/open-apis/docx/v1/chats/{}/announcement/blocks/{}"
        );

        // 验证路径替换逻辑
        let template = crate::core::endpoints_original::Endpoints::DOCX_V1_CHAT_ANNOUNCEMENT_BLOCK;
        let final_path = template
            .replace("{}", "chat_123")
            .replace("{}", "block_456");
        assert_eq!(
            final_path,
            "/open-apis/docx/v1/chats/chat_123/announcement/blocks/block_456"
        );
    }

    #[test]
    fn test_get_announcement_block_content_edge_cases() {
        // 测试极长chat_id和block_id
        let long_chat_id = "c".repeat(100);
        let long_block_id = "b".repeat(100);
        let long_request = GetAnnouncementBlockContentRequest::new(&long_chat_id, &long_block_id);
        assert!(long_request.validate().is_ok());

        // 测试特殊字符
        let special_request =
            GetAnnouncementBlockContentRequest::new("chat_特殊字符", "block_@#$%");
        assert!(special_request.validate().is_ok());

        // 测试Unicode字符
        let unicode_request = GetAnnouncementBlockContentRequest::new("群聊🎉", "公告块📝");
        assert!(unicode_request.validate().is_ok());
    }

    #[test]
    fn test_get_announcement_block_content_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = GetAnnouncementBlockContentBuilder::new("test_chat", "test_block")
            .user_id_type("open_id");

        // 验证构建器状态
        assert_eq!(builder.request.chat_id, "test_chat");
        assert_eq!(builder.request.block_id, "test_block");
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试链式调用
        let chained_builder = builder
            .user_id_type("user_id") // 重新设置user_id_type
            .request;
        assert_eq!(chained_builder.user_id_type, Some("user_id".to_string()));
    }
}
