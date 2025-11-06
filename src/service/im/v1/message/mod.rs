//! IM v1 消息服务
//!
//! 提供飞书开放平台即时消息服务的v1版本API实现，包括：
//! - 发送文本、富文本、卡片、文件等各种类型消息
//! - 编辑已发送的消息
//! - 撤回消息
//! - 获取消息详情
//! - 消息回复和表情回复
//! - Pin消息管理

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};

/// 消息内容类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum MessageContent {
    /// 文本消息
    #[serde(rename = "text")]
    Text { text: String },
    /// 富文本消息
    #[serde(rename = "post")]
    RichText {
        #[serde(rename = "zh_cn")]
        zh_cn: RichTextContent
    },
    /// 卡片消息
    #[serde(rename = "interactive")]
    Interactive {
        config: Option<CardConfig>,
        elements: Vec<CardElement>
    },
    /// 图片消息
    #[serde(rename = "image")]
    Image {
        image_key: String
    },
    /// 文件消息
    #[serde(rename = "file")]
    File {
        file_key: String
    },
    /// 音频消息
    #[serde(rename = "audio")]
    Audio {
        file_key: String,
        duration: Option<i32>,
    },
    /// 视频消息
    #[serde(rename = "video")]
    Video {
        file_key: String,
        title: Option<String>,
        duration: Option<i32>,
    },
    /// 媒体消息（通用）
    #[serde(rename = "media")]
    Media {
        file_key: String,
        image_key: Option<String>,
        file_name: Option<String>,
        duration: Option<i32>,
    },
    /// 分享消息
    #[serde(rename = "share_chat")]
    ShareChat {
        chat_id: String,
        description: Option<String>,
    },
    /// 位置消息
    #[serde(rename = "location")]
    Location {
        name: String,
        longitude: f64,
        latitude: f64,
        address: Option<String>,
    },
    /// 任务卡片
    #[serde(rename = "todo")]
    Todo {
        todo_id: String,
        status: Option<String>,
    },
    /// 日程邀请
    #[serde(rename = "calendar")]
    Calendar {
        summary: String,
        start_time: String,
        end_time: String,
        location: Option<String>,
    },
    /// 投票
    #[serde(rename = "vote")]
    Vote {
        title: String,
        options: Vec<String>,
    },
}

/// 富文本内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextContent {
    pub title: Option<String>,
    pub content: Vec<RichTextElement>,
}

/// 富文本元素
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum RichTextElement {
    #[serde(rename = "text")]
    Text {
        text: String,
        style: Option<TextStyle>,
    },
    #[serde(rename = "a")]
    Link {
        text: String,
        href: String,
        style: Option<TextStyle>,
    },
    #[serde(rename = "at")]
    At {
        user_id: Option<String>,
        name: String,
    },
}

/// 文本样式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
}

/// 卡片配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wide_screen_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_forward: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_multi: Option<bool>,
}

/// 卡片元素
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum CardElement {
    #[serde(rename = "div")]
    Div {
        text: RichTextContent,
    },
    #[serde(rename = "img")]
    Image {
        img_key: String,
        alt: Option<RichTextContent>,
        title: Option<RichTextContent>,
        mode: Option<String>,
    },
    #[serde(rename = "button")]
    Button {
        text: RichTextContent,
        url: Option<String>,
        action: Option<Value>,
        type_: Option<String>,
    },
}

/// 消息服务
#[derive(Debug, Clone)]
pub struct MessageService {
    config: Config,
}

impl MessageService {
    /// 创建新的消息服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 编辑消息
    ///
    /// 编辑已发送的消息内容
    ///
    /// # 参数
    /// * `message_id` - 要编辑的消息ID
    /// * `req` - 编辑消息请求
    ///
    /// # 返回值
    /// 返回编辑后的消息信息
    pub async fn update(&self, message_id: &str, req: &UpdateMessageRequest) -> SDKResult<UpdateMessageResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::IM_V1_UPDATE_MESSAGE
            .replace("{message_id}", message_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&req)?,
            ..Default::default()
        };

        let resp = Transport::<UpdateMessageResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取消息详情
    ///
    /// 根据消息ID获取消息的详细信息
    ///
    /// # 参数
    /// * `message_id` - 消息ID
    /// * `req` - 获取消息请求
    ///
    /// # 返回值
    /// 返回消息的详细信息
    pub async fn get(&self, message_id: &str, req: &GetMessageRequest) -> SDKResult<GetMessageResponse> {
        let mut api_path = crate::core::endpoints_original::Endpoints::IM_V1_GET_MESSAGE
            .replace("{message_id}", message_id);

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(thread_id_type) = &req.thread_id_type {
            query_params.push(format!("thread_id_type={}", thread_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetMessageResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 撤回消息
    ///
    /// 撤回已发送的消息
    ///
    /// # 参数
    /// * `message_id` - 要撤回的消息ID
    ///
    /// # 返回值
    /// 返回撤回操作的结果
    pub async fn delete(&self, message_id: &str) -> SDKResult<DeleteMessageResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::IM_V1_DELETE_MESSAGE
            .replace("{message_id}", message_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<DeleteMessageResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 部分更新消息
    ///
    /// 对已发送的消息进行部分更新（主要用于更新消息卡片）
    ///
    /// # 参数
    /// * `message_id` - 要更新的消息ID
    /// * `req` - 部分更新消息请求
    ///
    /// # 返回值
    /// 返回更新后的消息信息
    pub async fn patch(&self, message_id: &str, req: &PatchMessageRequest) -> SDKResult<PatchMessageResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::IM_V1_MESSAGE_PATCH
            .replace("{message_id}", message_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&req)?,
            ..Default::default()
        };

        let resp = Transport::<PatchMessageResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 转发消息
    ///
    /// 将指定消息转发给用户、群聊或话题
    ///
    /// # 参数
    /// * `message_id` - 要转发的消息ID
    /// * `req` - 转发消息请求
    ///
    /// # 返回值
    /// 返回转发后的消息信息
    pub async fn forward(&self, message_id: &str, req: &ForwardMessageRequest) -> SDKResult<ForwardMessageResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::IM_V1_FORWARD_MESSAGE
            .replace("{message_id}", message_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&req)?,
            ..Default::default()
        };

        let resp = Transport::<ForwardMessageResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 上传图片
    ///
    /// 上传图片到飞书服务器，获取图片的image_key用于发送消息
    ///
    /// # 参数
    /// * `req` - 图片上传请求，包含文件数据和元信息
    ///
    /// # 返回值
    /// 返回上传成功的图片信息，包括image_key等
    ///
    /// # 示例
    ///
    /// ```rust
    /// let image_data = std::fs::read("test.jpg")?;
    /// let request = ImageUploadRequest {
    ///     file_name: "test.jpg".to_string(),
    ///     image_type: "message".to_string(),
    ///     file_data: image_data,
    /// };
    ///
    /// let response = client.im.v1.message.upload_image(&request).await?;
    /// println!("图片上传成功，image_key: {}", response.image_key);
    /// ```
    pub async fn upload_image(&self, req: &ImageUploadRequest) -> SDKResult<ImageUploadResponse> {
        debug!("开始上传图片: {}", req.file_name);

        let mut api_req = ApiRequest::default();
        api_req.http_method = reqwest::Method::POST;
        api_req.api_path = crate::core::endpoints_original::Endpoints::IM_V1_IMAGES_UPLOAD.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.file = req.file_data.clone();

        // 构建表单数据
        let mut form_data = serde_json::Map::new();
        form_data.insert("file_name".to_string(), serde_json::Value::String(req.file_name.clone()));
        form_data.insert("image_type".to_string(), serde_json::Value::String(req.image_type.clone()));

        let body = serde_json::to_vec(&form_data).unwrap();
        api_req.body = body;

        let option = Some(crate::core::req_option::RequestOption::builder().file_upload(true).build());
        let resp = Transport::<ImageUploadResponse>::request(api_req, &self.config, option).await?;

        match resp.data {
            Some(data) => {
                info!("图片上传成功: {} -> {}", req.file_name, data.image_key);
                Ok(data)
            }
            None => Err(crate::core::error::SDKError::APIError("上传响应数据为空".to_string())),
        }
    }
}

// ==================== 数据模型 ====================

/// 编辑消息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageRequest {
    /// 接收者ID
    pub receive_id: String,
    /// 接收者类型
    pub receive_id_type: String,
    /// 消息内容
    pub content: MessageContent,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    /// 引用的消息ID（回复消息时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    /// 消息uuid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl Default for UpdateMessageRequest {
    fn default() -> Self {
        Self {
            receive_id: String::new(),
            receive_id_type: "user_id".to_string(),
            content: MessageContent::Text { text: String::new() },
            msg_type: None,
            quote: None,
            uuid: None,
        }
    }
}

/// 编辑消息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateMessageResponse {
    /// 消息ID
    pub message_id: Option<String>,
    /// 消息创建时间
    pub create_time: Option<String>,
    /// 消息更新时间
    pub update_time: Option<String>,
}

impl ApiResponseTrait for UpdateMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取消息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMessageRequest {
    /// thread_id类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id_type: Option<String>,
}

/// 获取消息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetMessageResponse {
    /// 消息信息
    pub message: Option<Message>,
}

impl ApiResponseTrait for GetMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除消息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteMessageResponse {
    /// 是否成功
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 部分更新消息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMessageRequest {
    /// 消息内容
    pub content: MessageContent,
}

impl Default for PatchMessageRequest {
    fn default() -> Self {
        Self {
            content: MessageContent::Text { text: String::new() },
        }
    }
}

/// 部分更新消息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchMessageResponse {
    /// 消息ID
    pub message_id: Option<String>,
    /// 消息更新时间
    pub update_time: Option<String>,
}

impl ApiResponseTrait for PatchMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 转发消息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardMessageRequest {
    /// 接收者ID
    pub receive_id: String,
    /// 接收者类型
    pub receive_id_type: String,
    /// 转发消息的UUID，用于避免重复转发
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl Default for ForwardMessageRequest {
    fn default() -> Self {
        Self {
            receive_id: String::new(),
            receive_id_type: "user_id".to_string(),
            uuid: None,
        }
    }
}

/// 转发消息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ForwardMessageResponse {
    /// 消息ID
    pub message_id: Option<String>,
    /// 消息创建时间
    pub create_time: Option<String>,
    /// 消息更新时间
    pub update_time: Option<String>,
}

impl ApiResponseTrait for ForwardMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 图片上传数据模型 ====================

/// 图片上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUploadRequest {
    /// 文件名
    ///
    /// 示例值："avatar.jpg"
    pub file_name: String,
    /// 图片类型
    ///
    /// 可选值：
    /// - "message": 消息图片
    /// - "avatar": 用户头像
    /// - "chat_avatar": 群头像
    pub image_type: String,
    /// 图片二进制内容
    #[serde(skip)]
    pub file_data: Vec<u8>,
}

impl Default for ImageUploadRequest {
    fn default() -> Self {
        Self {
            file_name: String::new(),
            image_type: "message".to_string(),
            file_data: Vec::new(),
        }
    }
}

impl ImageUploadRequest {
    /// 创建新的图片上传请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件名
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = file_name.into();
        self
    }

    /// 设置图片类型
    pub fn image_type(mut self, image_type: impl Into<String>) -> Self {
        self.image_type = image_type.into();
        self
    }

    /// 设置文件数据
    pub fn file_data(mut self, file_data: Vec<u8>) -> Self {
        self.file_data = file_data;
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证文件名
        if self.file_name.is_empty() {
            return Err(crate::core::error::SDKError::InvalidParameter("文件名不能为空".to_string()));
        }

        // 验证图片类型
        let valid_types = ["message", "avatar", "chat_avatar"];
        if !valid_types.contains(&self.image_type.as_str()) {
            return Err(crate::core::error::SDKError::InvalidParameter(
                format!("无效的图片类型: {}，支持的类型: {:?}", self.image_type, valid_types)
            ));
        }

        // 验证文件数据
        if self.file_data.is_empty() {
            return Err(crate::core::error::SDKError::InvalidParameter("文件数据不能为空".to_string()));
        }

        // 验证文件大小 (IM图片最大10MB)
        if self.file_data.len() > 10 * 1024 * 1024 {
            return Err(crate::core::error::SDKError::InvalidParameter(
                "图片文件大小不能超过10MB".to_string()
            ));
        }

        // 验证文件扩展名
        let valid_extensions = [".jpg", ".jpeg", ".png", ".gif", ".webp", ".bmp"];
        let has_valid_extension = valid_extensions.iter().any(|ext| {
            self.file_name.to_lowercase().ends_with(ext)
        });

        if !has_valid_extension {
            return Err(crate::core::error::SDKError::InvalidParameter(
                format!("不支持的图片格式，支持的格式: {:?}", valid_extensions)
            ));
        }

        Ok(())
    }
}

/// 图片上传响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageUploadResponse {
    /// 图片唯一标识
    pub image_key: String,
    /// 图片宽度（像素）
    pub width: Option<i32>,
    /// 图片高度（像素）
    pub height: Option<i32>,
    /// 创建时间
    pub create_time: Option<String>,
}

impl ApiResponseTrait for ImageUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 图片上传构建器模式 ====================

/// 图片上传构建器
#[derive(Debug, Clone)]
pub struct ImageUploadBuilder {
    request: ImageUploadRequest,
}

impl Default for ImageUploadBuilder {
    fn default() -> Self {
        Self {
            request: ImageUploadRequest::default(),
        }
    }
}

impl ImageUploadBuilder {
    /// 创建新的图片上传构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件名
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.request.file_name = file_name.into();
        self
    }

    /// 设置图片类型
    pub fn image_type(mut self, image_type: impl Into<String>) -> Self {
        self.request.image_type = image_type.into();
        self
    }

    /// 设置文件数据
    pub fn file_data(mut self, file_data: Vec<u8>) -> Self {
        self.request.file_data = file_data;
        self
    }

    /// 从文件路径加载数据
    pub fn from_file_path<P: AsRef<std::path::Path>>(mut self, path: P) -> std::io::Result<Self> {
        let file_data = std::fs::read(path)?;
        self.request.file_data = file_data;
        Ok(self)
    }

    /// 执行上传
    pub async fn execute(self, service: &MessageService) -> SDKResult<ImageUploadResponse> {
        // 验证请求参数
        self.request.validate()?;

        // 执行上传
        service.upload_image(&self.request).await
    }
}

/// 消息信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    /// 消息ID
    pub message_id: Option<String>,
    /// 消息类型
    pub msg_type: Option<String>,
    /// 消息内容
    pub content: Option<Value>,
    /// 创建者
    pub creator: Option<MessageCreator>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 消息是否被删除
    pub deleted: Option<bool>,
    /// 更新时间
    pub updated: Option<bool>,
    /// 父消息ID（回复消息时使用）
    pub parent: Option<String>,
    /// 消息所在会话ID
    pub chat_id: Option<String>,
    /// 消息所在群信息
    pub chat: Option<MessageChat>,
    /// 消息所在话题信息
    pub thread: Option<MessageThread>,
    /// 消息状态
    pub message_status: Option<String>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            message_id: None,
            msg_type: None,
            content: None,
            creator: None,
            create_time: None,
            update_time: None,
            deleted: None,
            updated: None,
            parent: None,
            chat_id: None,
            chat: None,
            thread: None,
            message_status: None,
        }
    }
}

/// 消息创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageCreator {
    /// 用户ID
    pub id: Option<String>,
    /// 用户ID类型
    pub id_type: Option<String>,
    /// 用户类型
    pub type_: Option<String>,
    /// 用户信息
    pub info: Option<UserInfo>,
}

impl Default for MessageCreator {
    fn default() -> Self {
        Self {
            id: None,
            id_type: None,
            type_: None,
            info: None,
        }
    }
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<String>,
    /// 用户名称
    pub name: Option<String>,
    /// 用户头像
    pub avatar: Option<String>,
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            user_id: None,
            user_id_type: None,
            name: None,
            avatar: None,
        }
    }
}

/// 消息所在群信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageChat {
    /// 群ID
    pub chat_id: Option<String>,
    /// 群信息
    pub info: Option<GroupInfo>,
}

impl Default for MessageChat {
    fn default() -> Self {
        Self {
            chat_id: None,
            info: None,
        }
    }
}

/// 群信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupInfo {
    /// 群ID
    pub id: Option<String>,
    /// 群ID类型
    pub id_type: Option<String>,
    /// 群名称
    pub name: Option<String>,
    /// 群头像
    pub avatar: Option<String>,
}

impl Default for GroupInfo {
    fn default() -> Self {
        Self {
            id: None,
            id_type: None,
            name: None,
            avatar: None,
        }
    }
}

/// 消息所在话题信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageThread {
    /// 话题ID
    pub thread_id: Option<String>,
    /// 话题信息
    pub info: Option<ThreadInfo>,
}

impl Default for MessageThread {
    fn default() -> Self {
        Self {
            thread_id: None,
            info: None,
        }
    }
}

/// 话题信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreadInfo {
    /// 话题ID
    pub id: Option<String>,
    /// 话题ID类型
    pub id_type: Option<String>,
    /// 话题名称
    pub name: Option<String>,
}

impl Default for ThreadInfo {
    fn default() -> Self {
        Self {
            id: None,
            id_type: None,
            name: None,
        }
    }
}

// ==================== 构建器模式 ====================

/// 编辑消息构建器
#[derive(Debug, Clone)]
pub struct UpdateMessageBuilder {
    request: UpdateMessageRequest,
}

impl Default for UpdateMessageBuilder {
    fn default() -> Self {
        Self {
            request: UpdateMessageRequest::default(),
        }
    }
}

impl UpdateMessageBuilder {
    /// 创建新的编辑消息构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置接收者ID
    pub fn receive_id(mut self, receive_id: impl Into<String>) -> Self {
        self.request.receive_id = receive_id.into();
        self
    }

    /// 设置接收者类型
    pub fn receive_id_type(mut self, receive_id_type: impl Into<String>) -> Self {
        self.request.receive_id_type = receive_id_type.into();
        self
    }

    /// 设置消息内容
    pub fn content(mut self, content: MessageContent) -> Self {
        self.request.content = content;
        self
    }

    /// 设置消息类型
    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.request.msg_type = Some(msg_type.into());
        self
    }

    /// 设置引用消息ID
    pub fn quote(mut self, quote: impl Into<String>) -> Self {
        self.request.quote = Some(quote.into());
        self
    }

    /// 设置消息UUID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.request.uuid = Some(uuid.into());
        self
    }

    /// 执行编辑消息
    pub async fn execute(self, service: &MessageService, message_id: &str) -> SDKResult<UpdateMessageResponse> {
        service.update(message_id, &self.request).await
    }
}

/// 获取消息构建器
#[derive(Debug, Clone)]
pub struct GetMessageBuilder {
    request: GetMessageRequest,
}

impl Default for GetMessageBuilder {
    fn default() -> Self {
        Self {
            request: GetMessageRequest::default(),
        }
    }
}

impl GetMessageBuilder {
    /// 创建新的获取消息构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置thread_id类型
    pub fn thread_id_type(mut self, thread_id_type: impl Into<String>) -> Self {
        self.request.thread_id_type = Some(thread_id_type.into());
        self
    }

    /// 执行获取消息
    pub async fn execute(self, service: &MessageService, message_id: &str) -> SDKResult<GetMessageResponse> {
        service.get(message_id, &self.request).await
    }
}

/// 部分更新消息构建器
#[derive(Debug, Clone)]
pub struct PatchMessageBuilder {
    request: PatchMessageRequest,
}

impl Default for PatchMessageBuilder {
    fn default() -> Self {
        Self {
            request: PatchMessageRequest::default(),
        }
    }
}

impl PatchMessageBuilder {
    /// 创建新的部分更新消息构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置消息内容
    pub fn content(mut self, content: MessageContent) -> Self {
        self.request.content = content;
        self
    }

    /// 执行部分更新消息
    pub async fn execute(self, service: &MessageService, message_id: &str) -> SDKResult<PatchMessageResponse> {
        service.patch(message_id, &self.request).await
    }
}

/// 转发消息构建器
#[derive(Debug, Clone)]
pub struct ForwardMessageBuilder {
    request: ForwardMessageRequest,
}

impl Default for ForwardMessageBuilder {
    fn default() -> Self {
        Self {
            request: ForwardMessageRequest::default(),
        }
    }
}

impl ForwardMessageBuilder {
    /// 创建新的转发消息构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置接收者ID
    pub fn receive_id(mut self, receive_id: impl Into<String>) -> Self {
        self.request.receive_id = receive_id.into();
        self
    }

    /// 设置接收者类型
    pub fn receive_id_type(mut self, receive_id_type: impl Into<String>) -> Self {
        self.request.receive_id_type = receive_id_type.into();
        self
    }

    /// 设置转发消息的UUID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.request.uuid = Some(uuid.into());
        self
    }

    /// 执行转发消息
    pub async fn execute(self, service: &MessageService, message_id: &str) -> SDKResult<ForwardMessageResponse> {
        service.forward(message_id, &self.request).await
    }
}

impl MessageService {
    /// 创建编辑消息构建器
    pub fn update_message_builder(&self) -> UpdateMessageBuilder {
        UpdateMessageBuilder::new()
    }

    /// 创建获取消息构建器
    pub fn get_message_builder(&self) -> GetMessageBuilder {
        GetMessageBuilder::new()
    }

    /// 创建部分更新消息构建器
    pub fn patch_message_builder(&self) -> PatchMessageBuilder {
        PatchMessageBuilder::new()
    }

    /// 创建转发消息构建器
    pub fn forward_message_builder(&self) -> ForwardMessageBuilder {
        ForwardMessageBuilder::new()
    }

    /// 创建图片上传构建器
    pub fn upload_image_builder(&self) -> ImageUploadBuilder {
        ImageUploadBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_service_creation() {
        let config = Config::default();
        let service = MessageService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_text_message_content() {
        let content = MessageContent::Text {
            text: "Hello World".to_string()
        };

        if let MessageContent::Text { text } = content {
            assert_eq!(text, "Hello World");
        } else {
            panic!("Expected Text content");
        }
    }

    #[test]
    fn test_rich_text_content() {
        let content = MessageContent::RichText {
            zh_cn: RichTextContent {
                title: Some("标题".to_string()),
                content: vec![
                    RichTextElement::Text {
                        text: "Hello".to_string(),
                        style: Some(TextStyle {
                            bold: Some(true),
                            text_color: Some("red".to_string()),
                            ..Default::default()
                        }),
                    },
                ],
            },
        };

        if let MessageContent::RichText { zh_cn } = content {
            assert_eq!(zh_cn.title, Some("标题".to_string()));
            assert_eq!(zh_cn.content.len(), 1);
        } else {
            panic!("Expected RichText content");
        }
    }

    #[test]
    fn test_interactive_card_content() {
        let content = MessageContent::Interactive {
            config: Some(CardConfig {
                wide_screen_mode: Some(true),
                ..Default::default()
            }),
            elements: vec![
                CardElement::Div {
                    text: RichTextContent {
                        title: None,
                        content: vec![
                            RichTextElement::Text {
                                text: "Card content".to_string(),
                                style: None,
                            },
                        ],
                    },
                },
            ],
        };

        if let MessageContent::Interactive { config, elements } = content {
            assert!(config.is_some());
            assert_eq!(elements.len(), 1);
        } else {
            panic!("Expected Interactive content");
        }
    }

    #[test]
    fn test_update_message_request_default() {
        let request = UpdateMessageRequest::default();
        assert_eq!(request.receive_id_type, "user_id");
        assert_eq!(request.content, MessageContent::Text { text: String::new() });
        assert!(request.msg_type.is_none());
        assert!(request.quote.is_none());
        assert!(request.uuid.is_none());
    }

    #[test]
    fn test_update_message_request_with_data() {
        let request = UpdateMessageRequest {
            receive_id: "user_123".to_string(),
            receive_id_type: "open_id".to_string(),
            content: MessageContent::Text { text: "Updated message".to_string() },
            msg_type: Some("text".to_string()),
            quote: Some("parent_msg_id".to_string()),
            uuid: Some("unique_id".to_string()),
        };

        assert_eq!(request.receive_id, "user_123");
        assert_eq!(request.receive_id_type, "open_id");
        assert_eq!(request.msg_type, Some("text".to_string()));
        assert_eq!(request.quote, Some("parent_msg_id".to_string()));
        assert_eq!(request.uuid, Some("unique_id".to_string()));
    }

    #[test]
    fn test_get_message_request_default() {
        let request = GetMessageRequest::default();
        assert!(request.thread_id_type.is_none());
    }

    #[test]
    fn test_get_message_request_with_thread_type() {
        let request = GetMessageRequest {
            thread_id_type: Some("open_thread_id".to_string()),
        };

        assert_eq!(request.thread_id_type, Some("open_thread_id".to_string()));
    }

    #[test]
    fn test_patch_message_request_default() {
        let request = PatchMessageRequest::default();
        assert_eq!(request.content, MessageContent::Text { text: String::new() });
    }

    #[test]
    fn test_patch_message_request_with_content() {
        let content = MessageContent::Text { text: "Patched content".to_string() };
        let request = PatchMessageRequest { content };

        if let MessageContent::Text { text } = request.content {
            assert_eq!(text, "Patched content");
        } else {
            panic!("Expected Text content");
        }
    }

    #[test]
    fn test_message_default_creation() {
        let message = Message::default();
        assert!(message.message_id.is_none());
        assert!(message.msg_type.is_none());
        assert!(message.content.is_none());
        assert!(message.creator.is_none());
        assert!(message.create_time.is_none());
        assert!(message.update_time.is_none());
        assert!(message.deleted.is_none());
        assert!(message.updated.is_none());
        assert!(message.parent.is_none());
        assert!(message.chat_id.is_none());
        assert!(message.chat.is_none());
        assert!(message.thread.is_none());
        assert!(message.message_status.is_none());
    }

    #[test]
    fn test_message_with_data() {
        let message = Message {
            message_id: Some("msg_123".to_string()),
            msg_type: Some("text".to_string()),
            content: Some(serde_json::json!({"text": "Hello World"})),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-01T00:01:00Z".to_string()),
            ..Default::default()
        };

        assert_eq!(message.message_id, Some("msg_123".to_string()));
        assert_eq!(message.msg_type, Some("text".to_string()));
        assert!(message.content.is_some());
        assert_eq!(message.create_time, Some("2023-01-01T00:00:00Z".to_string()));
        assert_eq!(message.update_time, Some("2023-01-01T00:01:00Z".to_string()));
    }

    #[test]
    fn test_update_message_builder() {
        let builder = UpdateMessageBuilder::new()
            .receive_id("user_123")
            .receive_id_type("open_id")
            .content(MessageContent::Text { text: "Updated message".to_string() })
            .msg_type("text")
            .quote("parent_msg_id")
            .uuid("unique_id");

        assert_eq!(builder.request.receive_id, "user_123");
        assert_eq!(builder.request.receive_id_type, "open_id");
        assert_eq!(builder.request.msg_type, Some("text".to_string()));
        assert_eq!(builder.request.quote, Some("parent_msg_id".to_string()));
        assert_eq!(builder.request.uuid, Some("unique_id".to_string()));
    }

    #[test]
    fn test_get_message_builder() {
        let builder = GetMessageBuilder::new()
            .thread_id_type("open_thread_id");

        assert_eq!(builder.request.thread_id_type, Some("open_thread_id".to_string()));
    }

    #[test]
    fn test_patch_message_builder() {
        let content = MessageContent::Text { text: "Patched content".to_string() };
        let builder = PatchMessageBuilder::new().content(content);

        if let MessageContent::Text { text } = builder.request.content {
            assert_eq!(text, "Patched content");
        } else {
            panic!("Expected Text content");
        }
    }

    #[test]
    fn test_text_style_serialization() {
        let style = TextStyle {
            bold: Some(true),
            italic: Some(true),
            text_color: Some("red".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&style).unwrap();
        let deserialized: TextStyle = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.bold, Some(true));
        assert_eq!(deserialized.italic, Some(true));
        assert_eq!(deserialized.text_color, Some("red".to_string()));
    }

    #[test]
    fn test_rich_text_element_serialization() {
        let element = RichTextElement::Link {
            text: "Click here".to_string(),
            href: "https://example.com".to_string(),
            style: Some(TextStyle {
                underline: Some(true),
                ..Default::default()
            }),
        };

        let serialized = serde_json::to_string(&element).unwrap();
        let deserialized: RichTextElement = serde_json::from_str(&serialized).unwrap();

        if let RichTextElement::Link { text, href, style } = deserialized {
            assert_eq!(text, "Click here");
            assert_eq!(href, "https://example.com");
            assert!(style.is_some());
        } else {
            panic!("Expected Link element");
        }
    }

    #[test]
    fn test_api_response_trait_implementations() {
        assert_eq!(UpdateMessageResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetMessageResponse::data_format(), ResponseFormat::Data);
        assert_eq!(DeleteMessageResponse::data_format(), ResponseFormat::Data);
        assert_eq!(PatchMessageResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = UpdateMessageRequest {
            receive_id: "user_123".to_string(),
            receive_id_type: "open_id".to_string(),
            content: MessageContent::Text { text: "Test message".to_string() },
            msg_type: Some("text".to_string()),
            quote: Some("parent_id".to_string()),
            uuid: Some("uuid_123".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateMessageRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.receive_id, deserialized.receive_id);
        assert_eq!(request.receive_id_type, deserialized.receive_id_type);
        assert_eq!(request.msg_type, deserialized.msg_type);
        assert_eq!(request.quote, deserialized.quote);
        assert_eq!(request.uuid, deserialized.uuid);
    }

    #[test]
    fn test_endpoint_constants() {
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_UPDATE_MESSAGE,
            "/open-apis/im/v1/messages/{message_id}"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_GET_MESSAGE,
            "/open-apis/im/v1/messages/{message_id}"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_DELETE_MESSAGE,
            "/open-apis/im/v1/messages/{message_id}"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_MESSAGE_PATCH,
            "/open-apis/im/v1/messages/{message_id}"
        );
    }

    #[test]
    fn test_complex_message_content() {
        let content = MessageContent::Interactive {
            config: Some(CardConfig {
                wide_screen_mode: Some(true),
                enable_forward: Some(false),
                update_multi: Some(true),
            }),
            elements: vec![
                CardElement::Div {
                    text: RichTextContent {
                        title: Some("通知".to_string()),
                        content: vec![
                            RichTextElement::Text {
                                text: "您有一个新任务".to_string(),
                                style: Some(TextStyle {
                                    bold: Some(true),
                                    text_color: Some("blue".to_string()),
                                    ..Default::default()
                                }),
                            },
                        ],
                    },
                },
                CardElement::Button {
                    text: RichTextContent {
                        title: None,
                        content: vec![
                            RichTextElement::Text {
                                text: "查看详情".to_string(),
                                style: None,
                            },
                        ],
                    },
                    url: Some("https://example.com".to_string()),
                    action: None,
                    type_: Some("primary".to_string()),
                },
            ],
        };

        if let MessageContent::Interactive { config, elements } = content {
            assert!(config.is_some());
            assert_eq!(elements.len(), 2);
        } else {
            panic!("Expected Interactive content");
        }
    }

    #[test]
    fn test_forward_message_request_default() {
        let request = ForwardMessageRequest::default();
        assert_eq!(request.receive_id_type, "user_id");
        assert_eq!(request.receive_id, String::new());
        assert!(request.uuid.is_none());
    }

    #[test]
    fn test_forward_message_request_with_data() {
        let request = ForwardMessageRequest {
            receive_id: "user_123".to_string(),
            receive_id_type: "open_id".to_string(),
            uuid: Some("unique_forward_id".to_string()),
        };

        assert_eq!(request.receive_id, "user_123");
        assert_eq!(request.receive_id_type, "open_id");
        assert_eq!(request.uuid, Some("unique_forward_id".to_string()));
    }

    #[test]
    fn test_forward_message_response_default() {
        let response = ForwardMessageResponse::default();
        assert!(response.message_id.is_none());
        assert!(response.create_time.is_none());
        assert!(response.update_time.is_none());
    }

    #[test]
    fn test_forward_message_response_with_data() {
        let response = ForwardMessageResponse {
            message_id: Some("forwarded_msg_456".to_string()),
            create_time: Some("2023-01-01T12:00:00Z".to_string()),
            update_time: Some("2023-01-01T12:00:01Z".to_string()),
        };

        assert_eq!(response.message_id, Some("forwarded_msg_456".to_string()));
        assert_eq!(response.create_time, Some("2023-01-01T12:00:00Z".to_string()));
        assert_eq!(response.update_time, Some("2023-01-01T12:00:01Z".to_string()));
    }

    #[test]
    fn test_forward_message_builder() {
        let builder = ForwardMessageBuilder::new()
            .receive_id("user_789")
            .receive_id_type("open_id")
            .uuid("forward_uuid_123");

        assert_eq!(builder.request.receive_id, "user_789");
        assert_eq!(builder.request.receive_id_type, "open_id");
        assert_eq!(builder.request.uuid, Some("forward_uuid_123".to_string()));
    }

    #[test]
    fn test_forward_message_builder_default() {
        let builder = ForwardMessageBuilder::default();
        assert_eq!(builder.request.receive_id_type, "user_id");
        assert_eq!(builder.request.receive_id, String::new());
        assert!(builder.request.uuid.is_none());
    }

    #[test]
    fn test_forward_message_builder_chain_calls() {
        let builder = ForwardMessageBuilder::new()
            .receive_id("chat_456")
            .receive_id_type("chat_id")
            .uuid("chain_test_uuid");

        assert_eq!(builder.request.receive_id, "chat_456");
        assert_eq!(builder.request.receive_id_type, "chat_id");
        assert_eq!(builder.request.uuid, Some("chain_test_uuid".to_string()));
    }

    #[test]
    fn test_forward_message_serialization() {
        let request = ForwardMessageRequest {
            receive_id: "group_123".to_string(),
            receive_id_type: "chat_id".to_string(),
            uuid: Some("test_uuid_456".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ForwardMessageRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.receive_id, deserialized.receive_id);
        assert_eq!(request.receive_id_type, deserialized.receive_id_type);
        assert_eq!(request.uuid, deserialized.uuid);
    }

    #[test]
    fn test_forward_message_response_serialization() {
        let response = ForwardMessageResponse {
            message_id: Some("msg_789".to_string()),
            create_time: Some("2023-12-31T23:59:59Z".to_string()),
            update_time: Some("2024-01-01T00:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ForwardMessageResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.message_id, deserialized.message_id);
        assert_eq!(response.create_time, deserialized.create_time);
        assert_eq!(response.update_time, deserialized.update_time);
    }

    #[test]
    fn test_forward_message_different_receive_id_types() {
        // Test user_id type
        let user_request = ForwardMessageBuilder::new()
            .receive_id("user_123")
            .receive_id_type("user_id");

        assert_eq!(user_request.request.receive_id_type, "user_id");

        // Test open_id type
        let open_request = ForwardMessageBuilder::new()
            .receive_id("ou_123456789")
            .receive_id_type("open_id");

        assert_eq!(open_request.request.receive_id_type, "open_id");

        // Test chat_id type
        let chat_request = ForwardMessageBuilder::new()
            .receive_id("oc_123456789")
            .receive_id_type("chat_id");

        assert_eq!(chat_request.request.receive_id_type, "chat_id");
    }

    #[test]
    fn test_forward_message_optional_uuid() {
        // Test without UUID
        let request_no_uuid = ForwardMessageBuilder::new()
            .receive_id("user_123")
            .receive_id_type("user_id");

        assert!(request_no_uuid.request.uuid.is_none());

        // Test with UUID
        let request_with_uuid = ForwardMessageBuilder::new()
            .receive_id("user_123")
            .receive_id_type("user_id")
            .uuid("unique_forward_uuid");

        assert_eq!(request_with_uuid.request.uuid, Some("unique_forward_uuid".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(ForwardMessageResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_endpoint_constants() {
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_FORWARD_MESSAGE,
            "/open-apis/im/v1/messages/{message_id}/forward"
        );
    }

    #[test]
    fn test_forward_message_request_json_structure() {
        let request = ForwardMessageRequest {
            receive_id: "test_user".to_string(),
            receive_id_type: "user_id".to_string(),
            uuid: Some("test-uuid-123".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json.get("receive_id").unwrap().as_str().unwrap(), "test_user");
        assert_eq!(json.get("receive_id_type").unwrap().as_str().unwrap(), "user_id");
        assert_eq!(json.get("uuid").unwrap().as_str().unwrap(), "test-uuid-123");
    }

    #[test]
    fn test_forward_message_response_json_structure() {
        let response = ForwardMessageResponse {
            message_id: Some("msg_forwarded_123".to_string()),
            create_time: Some("2023-06-15T10:30:00Z".to_string()),
            update_time: Some("2023-06-15T10:30:01Z".to_string()),
        };

        let json = serde_json::to_value(&response).unwrap();

        assert_eq!(json.get("message_id").unwrap().as_str().unwrap(), "msg_forwarded_123");
        assert_eq!(json.get("create_time").unwrap().as_str().unwrap(), "2023-06-15T10:30:00Z");
        assert_eq!(json.get("update_time").unwrap().as_str().unwrap(), "2023-06-15T10:30:01Z");
    }

    #[test]
    fn test_forward_message_builder_with_realistic_data() {
        let builder = ForwardMessageBuilder::new()
            .receive_id("oc_a0553eda8614c201fa69617a4f9c2a8b") // 真实的群聊ID格式
            .receive_id_type("chat_id")
            .uuid("forward_20240615_143022");

        assert_eq!(builder.request.receive_id, "oc_a0553eda8614c201fa69617a4f9c2a8b");
        assert_eq!(builder.request.receive_id_type, "chat_id");
        assert_eq!(builder.request.uuid, Some("forward_20240615_143022".to_string()));
    }

    #[test]
    fn test_forward_message_edge_cases() {
        // Test empty receive_id
        let empty_request = ForwardMessageRequest {
            receive_id: String::new(),
            receive_id_type: "user_id".to_string(),
            uuid: None,
        };

        assert_eq!(empty_request.receive_id, "");

        // Test very long receive_id
        let long_request = ForwardMessageRequest {
            receive_id: "a".repeat(1000),
            receive_id_type: "user_id".to_string(),
            uuid: Some("uuid".repeat(100)),
        };

        assert_eq!(long_request.receive_id.len(), 1000);
        assert!(long_request.uuid.unwrap().len(), 103);
    }

    // ==================== 图片上传测试 ====================

    #[test]
    fn test_image_upload_request_default() {
        let request = ImageUploadRequest::default();
        assert_eq!(request.file_name, "");
        assert_eq!(request.image_type, "message");
        assert_eq!(request.file_data.len(), 0);
    }

    #[test]
    fn test_image_upload_request_builder() {
        let image_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
        let request = ImageUploadRequest::new()
            .file_name("test.jpg")
            .image_type("avatar")
            .file_data(image_data.clone());

        assert_eq!(request.file_name, "test.jpg");
        assert_eq!(request.image_type, "avatar");
        assert_eq!(request.file_data, image_data);
    }

    #[test]
    fn test_image_upload_request_validation_success() {
        let valid_requests = vec![
            ImageUploadRequest {
                file_name: "avatar.jpg".to_string(),
                image_type: "message".to_string(),
                file_data: vec![0u8; 1024],
            },
            ImageUploadRequest {
                file_name: "profile.png".to_string(),
                image_type: "avatar".to_string(),
                file_data: vec![0u8; 512],
            },
            ImageUploadRequest {
                file_name: "chat_avatar.gif".to_string(),
                image_type: "chat_avatar".to_string(),
                file_data: vec![0u8; 2048],
            },
        ];

        for request in valid_requests {
            assert!(request.validate().is_ok(), "Validation should succeed for {:?}", request.file_name);
        }
    }

    #[test]
    fn test_image_upload_request_validation_empty_filename() {
        let request = ImageUploadRequest {
            file_name: "".to_string(),
            image_type: "message".to_string(),
            file_data: vec![0u8; 1024],
        };

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("文件名不能为空"));
    }

    #[test]
    fn test_image_upload_request_validation_invalid_type() {
        let request = ImageUploadRequest {
            file_name: "test.jpg".to_string(),
            image_type: "invalid_type".to_string(),
            file_data: vec![0u8; 1024],
        };

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("无效的图片类型"));
    }

    #[test]
    fn test_image_upload_request_validation_empty_data() {
        let request = ImageUploadRequest {
            file_name: "test.jpg".to_string(),
            image_type: "message".to_string(),
            file_data: vec![],
        };

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("文件数据不能为空"));
    }

    #[test]
    fn test_image_upload_request_validation_too_large() {
        let request = ImageUploadRequest {
            file_name: "large.jpg".to_string(),
            image_type: "message".to_string(),
            file_data: vec![0u8; 11 * 1024 * 1024], // 11MB
        };

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("图片文件大小不能超过10MB"));
    }

    #[test]
    fn test_image_upload_request_validation_invalid_extension() {
        let request = ImageUploadRequest {
            file_name: "test.txt".to_string(),
            image_type: "message".to_string(),
            file_data: vec![0u8; 1024],
        };

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("不支持的图片格式"));
    }

    #[test]
    fn test_image_upload_request_validation_valid_extensions() {
        let valid_extensions = ["jpg", "jpeg", "png", "gif", "webp", "bmp"];

        for ext in valid_extensions {
            let request = ImageUploadRequest {
                file_name: format!("test.{}", ext),
                image_type: "message".to_string(),
                file_data: vec![0u8; 1024],
            };

            assert!(request.validate().is_ok(), "Extension {} should be valid", ext);
        }
    }

    #[test]
    fn test_image_upload_response_default() {
        let response = ImageUploadResponse::default();
        assert_eq!(response.image_key, "");
        assert_eq!(response.width, None);
        assert_eq!(response.height, None);
        assert_eq!(response.create_time, None);
    }

    #[test]
    fn test_image_upload_response_with_data() {
        let response = ImageUploadResponse {
            image_key: "img_v2_123456".to_string(),
            width: Some(1920),
            height: Some(1080),
            create_time: Some("2024-06-15T14:30:22Z".to_string()),
        };

        assert_eq!(response.image_key, "img_v2_123456");
        assert_eq!(response.width, Some(1920));
        assert_eq!(response.height, Some(1080));
        assert_eq!(response.create_time, Some("2024-06-15T14:30:22Z"));
    }

    #[test]
    fn test_image_upload_builder_default() {
        let builder = ImageUploadBuilder::default();
        assert_eq!(builder.request.file_name, "");
        assert_eq!(builder.request.image_type, "message");
        assert_eq!(builder.request.file_data.len(), 0);
    }

    #[test]
    fn test_image_upload_builder_fluent_api() {
        let image_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let builder = ImageUploadBuilder::new()
            .file_name("profile.jpg")
            .image_type("avatar")
            .file_data(image_data);

        assert_eq!(builder.request.file_name, "profile.jpg");
        assert_eq!(builder.request.image_type, "avatar");
        assert_eq!(builder.request.file_data, image_data);
    }

    #[test]
    fn test_image_upload_builder_with_string_types() {
        let builder = ImageUploadBuilder::new()
            .file_name("test.png")
            .image_type("message");

        assert_eq!(builder.request.file_name, "test.png");
        assert_eq!(builder.request.image_type, "message");
    }

    #[test]
    fn test_image_upload_request_serialization() {
        let request = ImageUploadRequest {
            file_name: "test.jpg".to_string(),
            image_type: "message".to_string(),
            file_data: vec![1, 2, 3, 4],
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ImageUploadRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.file_name, "test.jpg");
        assert_eq!(deserialized.image_type, "message");
        // file_data should be skipped in serialization
        assert_eq!(deserialized.file_data.len(), 0);
    }

    #[test]
    fn test_image_upload_response_serialization() {
        let response = ImageUploadResponse {
            image_key: "img_test_123".to_string(),
            width: Some(800),
            height: Some(600),
            create_time: Some("2024-06-15T10:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ImageUploadResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.image_key, "img_test_123");
        assert_eq!(deserialized.width, Some(800));
        assert_eq!(deserialized.height, Some(600));
        assert_eq!(deserialized.create_time, Some("2024-06-15T10:00:00Z"));
    }

    #[test]
    fn test_image_upload_api_response_trait() {
        assert_eq!(ImageUploadResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_image_upload_edge_cases() {
        // Test case sensitivity in extensions
        let uppercase_request = ImageUploadRequest {
            file_name: "test.JPG".to_string(), // uppercase extension
            image_type: "message".to_string(),
            file_data: vec![0u8; 1024],
        };

        assert!(uppercase_request.validate().is_ok(), "Uppercase extension should be valid");

        // Test mixed case extensions
        let mixed_case_request = ImageUploadRequest {
            file_name: "test.PnG".to_string(), // mixed case extension
            image_type: "message".to_string(),
            file_data: vec![0u8; 1024],
        };

        assert!(mixed_case_request.validate().is_ok(), "Mixed case extension should be valid");
    }

    #[test]
    fn test_image_upload_builder_edge_cases() {
        // Test very long filename
        let long_filename = "a".repeat(200) + ".jpg";
        let builder = ImageUploadBuilder::new()
            .file_name(&long_filename)
            .image_type("message");

        assert_eq!(builder.request.file_name.len(), long_filename.len());

        // Test empty image type (should use default)
        let builder = ImageUploadBuilder::new()
            .file_name("test.jpg")
            .image_type("");

        assert_eq!(builder.request.image_type, "");
    }
}