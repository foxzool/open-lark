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

use crate::{
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

    // ==================== 文件下载管理 ====================

    /// 下载文件
    ///
    /// 根据 file_key 下载指定文件的二进制数据
    ///
    /// # 参数
    /// * `file_key` - 文件的唯一标识符，通过上传文件接口获得
    ///
    /// # 返回值
    /// 返回文件的二进制数据和相关元信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::im::v1::message::MessageService;
    ///
    /// let service = MessageService::new(config);
    /// let response = service
    ///     .download_file("file_v2_0411a8d4-8e73-4d5b-9c8f-6f7a1b2c3d4e")
    ///     .await?;
    /// println!("文件大小: {} 字节", response.data.len());
    /// println!("文件名: {:?}", response.file_name);
    /// ```
    pub async fn download_file(&self, file_key: &str) -> SDKResult<FileDownloadResponse> {
        debug!("开始下载文件: {}", file_key);

        let api_path = crate::core::endpoints_original::Endpoints::IM_V1_DOWNLOAD_FILE
            .replace("{file_key}", file_key);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<FileDownloadResponse>::request(api_req, &self.config, None).await?;

        match resp.data {
            Some(data) => {
                info!("文件下载成功: {} -> {} 字节", file_key, data.data.len());
                Ok(data)
            }
            None => Err(crate::core::error::SDKError::APIError("文件下载响应数据为空".to_string())),
        }
    }

    // ==================== 图片下载管理 ====================

    /// 下载图片
    ///
    /// 根据 image_key 下载指定图片的二进制数据
    ///
    /// # 参数
    /// * `image_key` - 图片的唯一标识符，通过上传图片接口获得
    ///
    /// # 返回值
    /// 返回图片的二进制数据和相关元信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::im::v1::message::MessageService;
    ///
    /// let service = MessageService::new(config);
    /// let response = service
    ///     .download_image("img_v2_0411a8d4-8e73-4d5b-9c8f-6f7a1b2c3d4e")
    ///     .await?;
    /// println!("图片大小: {} 字节", response.data.len());
    /// println!("文件名: {:?}", response.file_name);
    /// ```
    pub async fn download_image(&self, image_key: &str) -> SDKResult<ImageDownloadResponse> {
        debug!("开始下载图片: {}", image_key);

        let api_path = crate::core::endpoints_original::Endpoints::IM_V1_DOWNLOAD_IMAGE
            .replace("{image_key}", image_key);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<ImageDownloadResponse>::request(api_req, &self.config, None).await?;

        match resp.data {
            Some(data) => {
                info!("图片下载成功: {} -> {} 字节", image_key, data.data.len());
                Ok(data)
            }
            None => Err(crate::core::error::SDKError::APIError("图片下载响应数据为空".to_string())),
        }
    }

    // ==================== Pin 消息管理 ====================

    /// 创建 Pin 消息
    ///
    /// 将指定消息 Pin 到群聊中
    ///
    /// # 参数
    /// * `message_id` - 要 Pin 的消息 ID
    /// * `user_id_type` - 用户 ID 类型，可选值包括 open_id、user_id、union_id
    ///
    /// # 返回值
    /// 返回创建的 Pin 消息信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::im::v1::message::{MessageService, UserIdType};
    ///
    /// let service = MessageService::new(config);
    /// let response = service
    ///     .pin_message("msg_123", Some(UserIdType::OpenId))
    ///     .await?;
    /// println!("Pin 消息创建成功，pin_id: {}", response.pin.pin_id);
    /// ```
    pub async fn pin_message(
        &self,
        message_id: &str,
        user_id_type: Option<UserIdType>,
    ) -> SDKResult<PinMessageResponse> {
        debug!("开始创建 Pin 消息: {}", message_id);

        let mut query_params = Vec::new();
        query_params.push(format!("message_id={}", message_id));

        if let Some(user_id_type) = user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type.as_str()));
        }

        let mut api_path = crate::core::endpoints_original::Endpoints::IM_V1_PINS.to_string();
        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<PinMessageResponse>::request(api_req, &self.config, None).await?;

        match resp.data {
            Some(data) => {
                info!("Pin 消息创建成功: {} -> {}", message_id, data.pin.pin_id);
                Ok(data)
            }
            None => Err(crate::core::error::SDKError::APIError("Pin 创建响应数据为空".to_string())),
        }
    }

    /// 删除 Pin 消息
    ///
    /// 移除指定的 Pin 消息
    ///
    /// # 参数
    /// * `pin_id` - Pin 消息 ID
    /// * `user_id_type` - 用户 ID 类型，可选值包括 open_id、user_id、union_id
    ///
    /// # 返回值
    /// 返回删除操作结果
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::im::v1::message::{MessageService, UserIdType};
    ///
    /// let service = MessageService::new(config);
    /// let response = service
    ///     .unpin_message("pin_456", Some(UserIdType::OpenId))
    ///     .await?;
    /// println!("Pin 消息删除成功");
    /// ```
    pub async fn unpin_message(
        &self,
        pin_id: &str,
        user_id_type: Option<UserIdType>,
    ) -> SDKResult<EmptyResponse> {
        debug!("开始删除 Pin 消息: {}", pin_id);

        let mut query_params = Vec::new();
        if let Some(user_id_type) = user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type.as_str()));
        }

        let mut api_path = crate::core::endpoints_original::Endpoints::IM_V1_DELETE_PIN
            .replace("{pin_id}", pin_id);

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<EmptyResponse>::request(api_req, &self.config, None).await?;

        info!("Pin 消息删除成功: {}", pin_id);
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取群内 Pin 消息列表
    ///
    /// 获取指定群聊中的所有 Pin 消息
    ///
    /// # 参数
    /// * `chat_id` - 群聊 ID
    /// * `user_id_type` - 用户 ID 类型，可选值包括 open_id、user_id、union_id
    /// * `page_size` - 分页大小，默认为 20
    /// * `page_token` - 分页标记，用于获取下一页数据
    ///
    /// # 返回值
    /// 返回群内 Pin 消息列表，支持分页
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::im::v1::message::{MessageService, UserIdType};
    ///
    /// let service = MessageService::new(config);
    /// let response = service
    ///     .list_pins("chat_789", Some(UserIdType::OpenId), Some(20), None)
    ///     .await?;
    /// println!("获取到 {} 条 Pin 消息", response.items.len());
    /// ```
    pub async fn list_pins(
        &self,
        chat_id: &str,
        user_id_type: Option<UserIdType>,
        page_size: Option<i32>,
        page_token: Option<String>,
    ) -> SDKResult<ListPinMessagesResponse> {
        debug!("开始获取群内 Pin 消息列表: {}", chat_id);

        let mut query_params = Vec::new();
        query_params.push(format!("chat_id={}", chat_id));

        if let Some(user_id_type) = user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type.as_str()));
        }

        if let Some(page_size) = page_size {
            query_params.push(format!("page_size={}", page_size));
        }

        if let Some(page_token) = page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        let mut api_path = crate::core::endpoints_original::Endpoints::IM_V1_PINS.to_string();
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

        let resp = Transport::<ListPinMessagesResponse>::request(api_req, &self.config, None).await?;

        match resp.data {
            Some(data) => {
                info!("获取群内 Pin 消息列表成功: {} -> {} 条", chat_id, data.items.len());
                Ok(data)
            }
            None => Err(crate::core::error::SDKError::APIError("Pin 列表响应数据为空".to_string())),
        }
    }

    /// 批量发送消息
    ///
    /// 向多个接收者批量发送消息，支持多种消息类型
    ///
    /// # 参数
    /// * `req` - 批量发送消息请求
    ///
    /// # 返回值
    /// 返回批量发送消息的响应，包含批量消息ID和接收者验证结果
    ///
    /// # 示例
    /// ```rust
    /// let request = BatchSendMessageRequest::new()
    ///     .receive_id_list(vec!["user_1".to_string(), "user_2".to_string()])
    ///     .msg_type("text")
    ///     .content("{\"text\":\"批量消息测试\"}")
    ///     .receive_id_type("open_id")
    ///     .uuid("unique_batch_id_123");
    ///
    /// let response = message_service.batch_send_message(&request).await?;
    /// println!("批量消息发送成功，batch_id: {}", response.batch_message_id);
    /// ```
    pub async fn batch_send_message(&self, req: &BatchSendMessageRequest) -> SDKResult<BatchSendMessageResponse> {
        // 验证请求参数
        req.validate()?;

        debug!("开始批量发送消息: {} 个接收者", req.receive_id_list.len());

        let mut query_params = HashMap::new();

        // 设置用户ID类型
        let receive_id_type = req.receive_id_type.as_deref().unwrap_or("open_id");
        query_params.insert("receive_id_type".to_string(), receive_id_type.to_string());

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::MESSAGE_V4_BATCH_SEND.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<BatchSendMessageResponse>::request(api_req, &self.config, None).await?;

        let response = resp.data.unwrap_or_default();
        info!("批量发送消息完成: batch_id={}, 有效接收者={}, 无效接收者={}",
              response.batch_message_id,
              response.valid_receive_id_count.unwrap_or(0),
              response.invalid_receive_id_count.unwrap_or(0));

        Ok(response)
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

// ==================== 文件下载数据模型 ====================

/// 文件下载请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadRequest {
    /// 文件唯一标识
    ///
    /// 通过上传文件接口获得的 file_key
    pub file_key: String,
}

impl FileDownloadRequest {
    /// 创建新的文件下载请求
    pub fn new(file_key: impl Into<String>) -> Self {
        Self {
            file_key: file_key.into(),
        }
    }
}

/// 文件下载响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileDownloadResponse {
    /// 文件二进制数据
    #[serde(skip)]
    pub data: Vec<u8>,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件MIME类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 文件扩展名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// 文件类型（doc, sheet, image等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

impl ApiResponseTrait for FileDownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 图片下载数据模型 ====================

/// 图片下载请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDownloadRequest {
    /// 图片唯一标识
    ///
    /// 通过上传图片接口获得的 image_key
    pub image_key: String,
}

impl ImageDownloadRequest {
    /// 创建新的图片下载请求
    pub fn new(image_key: impl Into<String>) -> Self {
        Self {
            image_key: image_key.into(),
        }
    }
}

/// 图片下载响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageDownloadResponse {
    /// 图片二进制数据
    #[serde(skip)]
    pub data: Vec<u8>,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件MIME类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 图片宽度（像素）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 图片高度（像素）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

impl ApiResponseTrait for ImageDownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 批量发送消息数据模型 ====================

/// 批量发送消息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSendMessageRequest {
    /// 消息接收者ID列表
    ///
    /// 最多支持1000个接收者
    /// 支持open_id、user_id、union_id
    pub receive_id_list: Vec<String>,
    /// 消息类型
    ///
    /// 支持的消息类型：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等
    pub msg_type: String,
    /// 消息内容
    ///
    /// JSON结构序列化后的字符串，不同msg_type对应不同内容
    /// 注意：JSON字符串需进行转义，如换行符转义后为\\n
    /// 文本消息请求体最大不能超过150KB，卡片及富文本消息请求体最大不能超过30KB
    pub content: String,
    /// 用户ID类型
    ///
    /// 可选值：open_id、user_id、union_id
    /// 默认值：open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_id_type: Option<String>,
    /// 去重标识
    ///
    /// 由开发者生成的唯一字符串序列，用于发送消息请求去重
    /// 持有相同uuid的请求1小时内至多成功发送一条消息
    /// 最大长度：50字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl BatchSendMessageRequest {
    /// 创建新的批量发送消息请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置接收者ID列表
    pub fn receive_id_list(mut self, receive_id_list: Vec<String>) -> Self {
        self.receive_id_list = receive_id_list;
        self
    }

    /// 设置消息类型
    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.msg_type = msg_type.into();
        self
    }

    /// 设置消息内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// 设置用户ID类型
    pub fn receive_id_type(mut self, receive_id_type: impl Into<String>) -> Self {
        self.receive_id_type = Some(receive_id_type.into());
        self
    }

    /// 设置去重标识
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证接收者列表
        if self.receive_id_list.is_empty() {
            return Err(crate::core::error::SDKError::InvalidParameter("接收者列表不能为空".to_string()));
        }

        if self.receive_id_list.len() > 1000 {
            return Err(crate::core::error::SDKError::InvalidParameter("接收者数量不能超过1000个".to_string()));
        }

        // 验证消息类型
        if self.msg_type.is_empty() {
            return Err(crate::core::error::SDKError::InvalidParameter("消息类型不能为空".to_string()));
        }

        // 验证消息内容
        if self.content.is_empty() {
            return Err(crate::core::error::SDKError::InvalidParameter("消息内容不能为空".to_string()));
        }

        // 验证用户ID类型
        if let Some(receive_id_type) = &self.receive_id_type {
            let valid_types = ["open_id", "user_id", "union_id"];
            if !valid_types.contains(&receive_id_type.as_str()) {
                return Err(crate::core::error::SDKError::InvalidParameter(
                    format!("无效的用户ID类型: {}，支持的类型: {:?}", receive_id_type, valid_types)
                ));
            }
        }

        // 验证UUID长度
        if let Some(uuid) = &self.uuid {
            if uuid.len() > 50 {
                return Err(crate::core::error::SDKError::InvalidParameter("UUID长度不能超过50个字符".to_string()));
            }
        }

        Ok(())
    }
}

impl Default for BatchSendMessageRequest {
    fn default() -> Self {
        Self {
            receive_id_list: Vec::new(),
            msg_type: String::new(),
            content: String::new(),
            receive_id_type: Some("open_id".to_string()),
            uuid: None,
        }
    }
}

/// 批量发送消息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchSendMessageResponse {
    /// 批量消息ID
    ///
    /// 用于查询批量消息的状态和进度
    pub batch_message_id: String,
    /// 无效的接收者ID列表
    ///
    /// 包含格式错误或不存在的接收者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_receive_id_list: Option<Vec<String>>,
    /// 有效接收者数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_receive_id_count: Option<i32>,
    /// 无效接收者数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_receive_id_count: Option<i32>,
}

impl ApiResponseTrait for BatchSendMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== Pin 消息数据模型 ====================

/// 用户 ID 类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserIdType {
    #[serde(rename = "open_id")]
    OpenId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "union_id")]
    UnionId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::OpenId => "open_id",
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
        }
    }
}

/// Pin 消息信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinMessageInfo {
    /// Pin 消息 ID
    pub pin_id: String,
    /// 消息 ID
    pub message_id: String,
    /// 群聊 ID
    pub chat_id: String,
    /// Pin 操作者 ID
    pub operator_id: String,
    /// 创建时间
    pub create_time: String,
}

/// 创建 Pin 消息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PinMessageResponse {
    /// Pin 消息信息
    pub pin: PinMessageInfo,
}

impl ApiResponseTrait for PinMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyResponse {}

impl ApiResponseTrait for EmptyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取 Pin 消息列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListPinMessagesResponse {
    /// Pin 消息列表
    pub items: Vec<PinMessageInfo>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListPinMessagesResponse {
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

// ==================== Pin 消息构建器模式 ====================

/// 创建 Pin 消息构建器
#[derive(Debug, Clone)]
pub struct PinMessageBuilder {
    message_id: String,
    user_id_type: Option<UserIdType>,
}

impl PinMessageBuilder {
    /// 创建新的构建器
    pub fn new(message_id: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行创建操作
    pub async fn execute(self, service: &MessageService) -> SDKResult<PinMessageResponse> {
        service.pin_message(&self.message_id, self.user_id_type).await
    }
}

/// 删除 Pin 消息构建器
#[derive(Debug, Clone)]
pub struct UnpinMessageBuilder {
    pin_id: String,
    user_id_type: Option<UserIdType>,
}

impl UnpinMessageBuilder {
    /// 创建新的构建器
    pub fn new(pin_id: impl Into<String>) -> Self {
        Self {
            pin_id: pin_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行删除操作
    pub async fn execute(self, service: &MessageService) -> SDKResult<EmptyResponse> {
        service.unpin_message(&self.pin_id, self.user_id_type).await
    }
}

/// 图片下载构建器
#[derive(Debug, Clone)]
pub struct ImageDownloadBuilder {
    image_key: String,
}

impl ImageDownloadBuilder {
    /// 创建新的构建器
    pub fn new(image_key: impl Into<String>) -> Self {
        Self {
            image_key: image_key.into(),
        }
    }

    /// 执行下载操作
    pub async fn execute(self, service: &MessageService) -> SDKResult<ImageDownloadResponse> {
        service.download_image(&self.image_key).await
    }
}

/// 文件下载构建器
#[derive(Debug, Clone)]
pub struct FileDownloadBuilder {
    file_key: String,
}

impl FileDownloadBuilder {
    /// 创建新的构建器
    pub fn new(file_key: impl Into<String>) -> Self {
        Self {
            file_key: file_key.into(),
        }
    }

    /// 执行下载操作
    pub async fn execute(self, service: &MessageService) -> SDKResult<FileDownloadResponse> {
        service.download_file(&self.file_key).await
    }
}

/// 批量发送消息构建器
#[derive(Debug, Clone)]
pub struct BatchSendMessageBuilder {
    request: BatchSendMessageRequest,
}

impl Default for BatchSendMessageBuilder {
    fn default() -> Self {
        Self {
            request: BatchSendMessageRequest::default(),
        }
    }
}

impl BatchSendMessageBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置接收者ID列表
    pub fn receive_id_list(mut self, receive_id_list: Vec<String>) -> Self {
        self.request.receive_id_list = receive_id_list;
        self
    }

    /// 添加单个接收者ID
    pub fn add_receive_id(mut self, receive_id: impl Into<String>) -> Self {
        self.request.receive_id_list.push(receive_id.into());
        self
    }

    /// 设置消息类型
    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.request.msg_type = msg_type.into();
        self
    }

    /// 设置消息内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.request.content = content.into();
        self
    }

    /// 设置用户ID类型
    pub fn receive_id_type(mut self, receive_id_type: impl Into<String>) -> Self {
        self.request.receive_id_type = Some(receive_id_type.into());
        self
    }

    /// 设置去重标识
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.request.uuid = Some(uuid.into());
        self
    }

    /// 执行批量发送操作
    pub async fn execute(self, service: &MessageService) -> SDKResult<BatchSendMessageResponse> {
        service.batch_send_message(&self.request).await
    }
}

/// 获取 Pin 消息列表构建器
#[derive(Debug, Clone)]
pub struct ListPinMessagesBuilder {
    chat_id: String,
    user_id_type: Option<UserIdType>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListPinMessagesBuilder {
    /// 创建新的构建器
    pub fn new(chat_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行查询操作
    pub async fn execute(self, service: &MessageService) -> SDKResult<ListPinMessagesResponse> {
        service.list_pins(&self.chat_id, self.user_id_type, self.page_size, self.page_token).await
    }
}

impl MessageService {
    /// 创建 Pin 消息构建器
    pub fn pin_message_builder(&self, message_id: impl Into<String>) -> PinMessageBuilder {
        PinMessageBuilder::new(message_id)
    }

    /// 删除 Pin 消息构建器
    pub fn unpin_message_builder(&self, pin_id: impl Into<String>) -> UnpinMessageBuilder {
        UnpinMessageBuilder::new(pin_id)
    }

    /// 获取 Pin 消息列表构建器
    pub fn list_pins_builder(&self, chat_id: impl Into<String>) -> ListPinMessagesBuilder {
        ListPinMessagesBuilder::new(chat_id)
    }

    /// 创建图片下载构建器
    pub fn download_image_builder(&self, image_key: impl Into<String>) -> ImageDownloadBuilder {
        ImageDownloadBuilder::new(image_key)
    }

    /// 创建文件下载构建器
    pub fn download_file_builder(&self, file_key: impl Into<String>) -> FileDownloadBuilder {
        FileDownloadBuilder::new(file_key)
    }

    /// 创建批量发送消息构建器
    pub fn batch_send_message_builder(&self) -> BatchSendMessageBuilder {
        BatchSendMessageBuilder::new()
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

    // ==================== Pin 消息测试 ====================

    #[test]
    fn test_user_id_type() {
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
    }

    #[test]
    fn test_pin_message_info_creation() {
        let pin_info = PinMessageInfo {
            pin_id: "pin_123".to_string(),
            message_id: "msg_456".to_string(),
            chat_id: "chat_789".to_string(),
            operator_id: "user_abc".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(pin_info.pin_id, "pin_123");
        assert_eq!(pin_info.message_id, "msg_456");
        assert_eq!(pin_info.chat_id, "chat_789");
        assert_eq!(pin_info.operator_id, "user_abc");
        assert_eq!(pin_info.create_time, "2023-01-01T00:00:00Z");
    }

    #[test]
    fn test_pin_message_response_default() {
        let response = PinMessageResponse::default();
        assert_eq!(response.pin.pin_id, "");
    }

    #[test]
    fn test_empty_response_default() {
        let response = EmptyResponse::default();
        // Empty response has no fields to test
    }

    #[test]
    fn test_list_pin_messages_response_default() {
        let response = ListPinMessagesResponse::default();
        assert_eq!(response.items.len(), 0);
        assert_eq!(response.has_more, None);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_pin_message_builder() {
        let builder = PinMessageBuilder::new("msg_123")
            .user_id_type(UserIdType::OpenId);

        assert_eq!(builder.message_id, "msg_123");
        assert_eq!(builder.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_unpin_message_builder() {
        let builder = UnpinMessageBuilder::new("pin_456")
            .user_id_type(UserIdType::UserId);

        assert_eq!(builder.pin_id, "pin_456");
        assert_eq!(builder.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_list_pin_messages_builder() {
        let builder = ListPinMessagesBuilder::new("chat_789")
            .user_id_type(UserIdType::UnionId)
            .page_size(20)
            .page_token("token_abc");

        assert_eq!(builder.chat_id, "chat_789");
        assert_eq!(builder.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(builder.page_size, Some(20));
        assert_eq!(builder.page_token, Some("token_abc".to_string()));
    }

    #[test]
    fn test_pin_message_service_builders() {
        let config = Config::default();
        let service = MessageService::new(config);

        let pin_builder = service.pin_message_builder("msg_test");
        assert_eq!(pin_builder.message_id, "msg_test");

        let unpin_builder = service.unpin_message_builder("pin_test");
        assert_eq!(unpin_builder.pin_id, "pin_test");

        let list_builder = service.list_pins_builder("chat_test");
        assert_eq!(list_builder.chat_id, "chat_test");
    }

    #[test]
    fn test_pin_api_response_trait_implementation() {
        assert_eq!(PinMessageResponse::data_format(), ResponseFormat::Data);
        assert_eq!(EmptyResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListPinMessagesResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_pin_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_PINS,
            "/open-apis/im/v1/pins"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_DELETE_PIN,
            "/open-apis/im/v1/pins/{pin_id}"
        );
    }

    #[test]
    fn test_comprehensive_pin_data() {
        // Test comprehensive pin data with all fields
        let comprehensive_pin = PinMessageInfo {
            pin_id: "comprehensive_pin_001".to_string(),
            message_id: "msg_comprehensive_002".to_string(),
            chat_id: "chat_comprehensive_003".to_string(),
            operator_id: "operator_comprehensive_004".to_string(),
            create_time: "2023-12-31T23:59:59Z".to_string(),
        };

        assert_eq!(
            comprehensive_pin.pin_id,
            "comprehensive_pin_001"
        );
        assert_eq!(
            comprehensive_pin.message_id,
            "msg_comprehensive_002"
        );
        assert_eq!(
            comprehensive_pin.chat_id,
            "chat_comprehensive_003"
        );
        assert_eq!(
            comprehensive_pin.operator_id,
            "operator_comprehensive_004"
        );
        assert_eq!(
            comprehensive_pin.create_time,
            "2023-12-31T23:59:59Z"
        );
    }

    #[test]
    fn test_list_pin_messages_response_with_data() {
        let mut list_response = ListPinMessagesResponse::default();
        list_response.items = vec![
            PinMessageInfo {
                pin_id: "pin_list_001".to_string(),
                message_id: "msg_list_001".to_string(),
                chat_id: "chat_list".to_string(),
                operator_id: "operator_list".to_string(),
                create_time: "2023-01-01T00:00:00Z".to_string(),
            },
            PinMessageInfo {
                pin_id: "pin_list_002".to_string(),
                message_id: "msg_list_002".to_string(),
                chat_id: "chat_list".to_string(),
                operator_id: "operator_list".to_string(),
                create_time: "2023-01-02T00:00:00Z".to_string(),
            },
        ];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page_token".to_string());

        assert_eq!(list_response.items.len(), 2);
        assert_eq!(list_response.items[0].pin_id, "pin_list_001");
        assert_eq!(list_response.items[1].pin_id, "pin_list_002");
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(list_response.page_token, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_pin_builders_with_different_user_id_types() {
        let config = Config::default();
        let service = MessageService::new(config);

        // Test with OpenId
        let open_id_builder = service.pin_message_builder("msg_open")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(open_id_builder.user_id_type, Some(UserIdType::OpenId));

        // Test with UserId
        let user_id_builder = service.unpin_message_builder("pin_user")
            .user_id_type(UserIdType::UserId);
        assert_eq!(user_id_builder.user_id_type, Some(UserIdType::UserId));

        // Test with UnionId
        let union_id_builder = service.list_pins_builder("chat_union")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(union_id_builder.user_id_type, Some(UserIdType::UnionId));

        // Test without user_id_type (None)
        let none_builder = service.pin_message_builder("msg_none");
        assert_eq!(none_builder.user_id_type, None);
    }

    #[test]
    fn test_pin_serialization_deserialization() {
        let pin_info = PinMessageInfo {
            pin_id: "pin_serde_001".to_string(),
            message_id: "msg_serde_002".to_string(),
            chat_id: "chat_serde_003".to_string(),
            operator_id: "operator_serde_004".to_string(),
            create_time: "2023-06-15T10:30:00Z".to_string(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&pin_info).unwrap();
        assert!(serialized.contains("pin_serde_001"));
        assert!(serialized.contains("msg_serde_002"));

        // Test deserialization
        let deserialized: PinMessageInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.pin_id, pin_info.pin_id);
        assert_eq!(deserialized.message_id, pin_info.message_id);
        assert_eq!(deserialized.chat_id, pin_info.chat_id);
        assert_eq!(deserialized.operator_id, pin_info.operator_id);
        assert_eq!(deserialized.create_time, pin_info.create_time);
    }

    #[test]
    fn test_user_id_type_serialization() {
        // Test OpenId serialization
        let open_id = UserIdType::OpenId;
        let serialized = serde_json::to_string(&open_id).unwrap();
        assert_eq!(serialized, "\"open_id\"");

        // Test UserId serialization
        let user_id = UserIdType::UserId;
        let serialized = serde_json::to_string(&user_id).unwrap();
        assert_eq!(serialized, "\"user_id\"");

        // Test UnionId serialization
        let union_id = UserIdType::UnionId;
        let serialized = serde_json::to_string(&union_id).unwrap();
        assert_eq!(serialized, "\"union_id\"");

        // Test deserialization
        let deserialized: UserIdType = serde_json::from_str("\"open_id\"").unwrap();
        assert_eq!(deserialized.as_str(), "open_id");
    }

    // ==================== 图片下载测试 ====================

    #[test]
    fn test_image_download_request_creation() {
        let request = ImageDownloadRequest::new("img_v2_test_key_123");
        assert_eq!(request.image_key, "img_v2_test_key_123");
    }

    #[test]
    fn test_image_download_request_with_string() {
        let image_key = "img_v2_0411a8d4-8e73-4d5b-9c8f-6f7a1b2c3d4e".to_string();
        let request = ImageDownloadRequest::new(image_key.clone());
        assert_eq!(request.image_key, image_key);
    }

    #[test]
    fn test_image_download_response_default() {
        let response = ImageDownloadResponse::default();
        assert_eq!(response.data.len(), 0);
        assert_eq!(response.file_name, None);
        assert_eq!(response.file_size, None);
        assert_eq!(response.content_type, None);
        assert_eq!(response.width, None);
        assert_eq!(response.height, None);
    }

    #[test]
    fn test_image_download_response_with_data() {
        let mut response = ImageDownloadResponse::default();
        response.data = vec![1, 2, 3, 4, 5];
        response.file_name = Some("test_image.jpg".to_string());
        response.file_size = Some(1024);
        response.content_type = Some("image/jpeg".to_string());
        response.width = Some(800);
        response.height = Some(600);

        assert_eq!(response.data, vec![1, 2, 3, 4, 5]);
        assert_eq!(response.file_name, Some("test_image.jpg".to_string()));
        assert_eq!(response.file_size, Some(1024));
        assert_eq!(response.content_type, Some("image/jpeg".to_string()));
        assert_eq!(response.width, Some(800));
        assert_eq!(response.height, Some(600));
    }

    #[test]
    fn test_image_download_builder() {
        let builder = ImageDownloadBuilder::new("img_v2_builder_test");
        assert_eq!(builder.image_key, "img_v2_builder_test");
    }

    #[test]
    fn test_image_download_builder_with_string() {
        let image_key = "img_v2_complex_key_abc123".to_string();
        let builder = ImageDownloadBuilder::new(image_key.clone());
        assert_eq!(builder.image_key, image_key);
    }

    #[test]
    fn test_image_download_service_builder() {
        let config = Config::default();
        let service = MessageService::new(config);

        let builder = service.download_image_builder("img_v2_service_test");
        assert_eq!(builder.image_key, "img_v2_service_test");
    }

    #[test]
    fn test_image_download_serialization() {
        let request = ImageDownloadRequest {
            image_key: "img_v2_serialize_test".to_string(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("img_v2_serialize_test"));

        // Test deserialization
        let deserialized: ImageDownloadRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.image_key, "img_v2_serialize_test");
    }

    #[test]
    fn test_image_download_response_serialization() {
        let response = ImageDownloadResponse {
            data: vec![10, 20, 30],
            file_name: Some("serialization_test.png".to_string()),
            file_size: Some(2048),
            content_type: Some("image/png".to_string()),
            width: Some(1024),
            height: Some(768),
        };

        // Note: data field is skipped in serialization, so it won't appear in JSON
        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("serialization_test.png"));
        assert!(serialized.contains("2048"));
        assert!(serialized.contains("image/png"));
        assert!(serialized.contains("1024"));
        assert!(serialized.contains("768"));

        // Test deserialization
        let deserialized: ImageDownloadResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.file_name, Some("serialization_test.png".to_string()));
        assert_eq!(deserialized.file_size, Some(2048));
        assert_eq!(deserialized.content_type, Some("image/png".to_string()));
        assert_eq!(deserialized.width, Some(1024));
        assert_eq!(deserialized.height, Some(768));
        // data field will be empty after deserialization since it's skipped
        assert_eq!(deserialized.data.len(), 0);
    }

    #[test]
    fn test_image_download_api_response_trait() {
        assert_eq!(ImageDownloadResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_image_download_endpoint_constant() {
        // Test that the endpoint constant is properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_DOWNLOAD_IMAGE,
            "/open-apis/im/v1/images/{image_key}"
        );
    }

    #[test]
    fn test_image_download_with_different_keys() {
        // Test various image key formats
        let keys = vec![
            "img_v2_simple",
            "img_v2_0411a8d4-8e73-4d5b-9c8f-6f7a1b2c3d4e",
            "img_v2_1234567890",
            "img_v2_with_underscores_and_numbers_123",
        ];

        for key in keys {
            let request = ImageDownloadRequest::new(key);
            assert_eq!(request.image_key, key);

            let builder = ImageDownloadBuilder::new(key);
            assert_eq!(builder.image_key, key);
        }
    }

    #[test]
    fn test_comprehensive_image_download_response() {
        // Test comprehensive response with all fields
        let comprehensive_response = ImageDownloadResponse {
            data: vec![0xFF, 0xD8, 0xFF, 0xE0], // JPEG header bytes
            file_name: Some("comprehensive_test_image.jpg".to_string()),
            file_size: Some(5242880), // 5MB
            content_type: Some("image/jpeg".to_string()),
            width: Some(1920),
            height: Some(1080),
        };

        assert_eq!(comprehensive_response.data, vec![0xFF, 0xD8, 0xFF, 0xE0]);
        assert_eq!(comprehensive_response.file_name, Some("comprehensive_test_image.jpg".to_string()));
        assert_eq!(comprehensive_response.file_size, Some(5242880));
        assert_eq!(comprehensive_response.content_type, Some("image/jpeg".to_string()));
        assert_eq!(comprehensive_response.width, Some(1920));
        assert_eq!(comprehensive_response.height, Some(1080));
    }

    #[test]
    fn test_image_download_edge_cases() {
        // Test empty image key
        let empty_request = ImageDownloadRequest::new("");
        assert_eq!(empty_request.image_key, "");

        // Test very long image key
        let long_key = "img_v2_".to_string() + &"a".repeat(100);
        let long_request = ImageDownloadRequest::new(&long_key);
        assert_eq!(long_request.image_key, long_key);

        // Test response with only data
        let data_only_response = ImageDownloadResponse {
            data: vec![1, 2, 3],
            file_name: None,
            file_size: None,
            content_type: None,
            width: None,
            height: None,
        };
        assert_eq!(data_only_response.data, vec![1, 2, 3]);
        assert_eq!(data_only_response.file_name, None);
        assert_eq!(data_only_response.file_size, None);
        assert_eq!(data_only_response.content_type, None);
        assert_eq!(data_only_response.width, None);
        assert_eq!(data_only_response.height, None);
    }

    // ==================== 文件下载单元测试 ====================

    #[test]
    fn test_file_download_request_creation() {
        let request = FileDownloadRequest::new("file_v2_0411a8d4-8e73-4d5b-9c8f-6f7a1b2c3d4e");
        assert_eq!(request.file_key, "file_v2_0411a8d4-8e73-4d5b-9c8f-6f7a1b2c3d4e");
    }

    #[test]
    fn test_file_download_request_with_string() {
        let file_key = "file_v2_complex_key_abc123".to_string();
        let request = FileDownloadRequest::new(file_key.clone());
        assert_eq!(request.file_key, file_key);
    }

    #[test]
    fn test_file_download_request_default() {
        let request = FileDownloadRequest::new("default_test");
        assert_eq!(request.file_key, "default_test");
    }

    #[test]
    fn test_file_download_response_creation() {
        let response = FileDownloadResponse::default();
        assert_eq!(response.data.len(), 0);
        assert_eq!(response.file_name, None);
        assert_eq!(response.file_size, None);
        assert_eq!(response.content_type, None);
    }

    #[test]
    fn test_file_download_response_with_data() {
        let mut response = FileDownloadResponse::default();
        response.data = vec![1, 2, 3, 4, 5];
        response.file_name = Some("test_document.pdf".to_string());
        response.file_size = Some(2048);
        response.content_type = Some("application/pdf".to_string());

        assert_eq!(response.data, vec![1, 2, 3, 4, 5]);
        assert_eq!(response.file_name, Some("test_document.pdf".to_string()));
        assert_eq!(response.file_size, Some(2048));
        assert_eq!(response.content_type, Some("application/pdf".to_string()));
    }

    #[test]
    fn test_file_download_builder() {
        let builder = FileDownloadBuilder::new("file_v2_builder_test");
        assert_eq!(builder.file_key, "file_v2_builder_test");
    }

    #[test]
    fn test_file_download_builder_with_string() {
        let file_key = "file_v2_complex_key_abc123".to_string();
        let builder = FileDownloadBuilder::new(file_key.clone());
        assert_eq!(builder.file_key, file_key);
    }

    #[test]
    fn test_file_download_service_builder() {
        let config = Config::default();
        let service = MessageService::new(config);

        let builder = service.download_file_builder("file_v2_service_test");
        assert_eq!(builder.file_key, "file_v2_service_test");
    }

    #[test]
    fn test_file_download_serialization() {
        let request = FileDownloadRequest {
            file_key: "file_v2_serialize_test".to_string(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("file_v2_serialize_test"));

        // Test deserialization
        let deserialized: FileDownloadRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.file_key, "file_v2_serialize_test");
    }

    #[test]
    fn test_file_download_response_serialization() {
        let response = FileDownloadResponse {
            data: vec![10, 20, 30],
            file_name: Some("serialization_test.docx".to_string()),
            file_size: Some(4096),
            content_type: Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string()),
        };

        // Note: data field is skipped in serialization, so it won't appear in JSON
        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("serialization_test.docx"));
        assert!(serialized.contains("4096"));
        assert!(serialized.contains("application/vnd.openxmlformats-officedocument.wordprocessingml.document"));

        // Test deserialization
        let deserialized: FileDownloadResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.file_name, Some("serialization_test.docx".to_string()));
        assert_eq!(deserialized.file_size, Some(4096));
        assert_eq!(deserialized.content_type, Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string()));
        // data field will be empty after deserialization since it's skipped
        assert_eq!(deserialized.data.len(), 0);
    }

    #[test]
    fn test_file_download_api_response_trait() {
        assert_eq!(FileDownloadResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_file_download_endpoint_constant() {
        // Test that the endpoint constant is properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_DOWNLOAD_FILE,
            "/open-apis/im/v1/files/{file_key}/download"
        );
    }

    #[test]
    fn test_file_download_with_different_keys() {
        // Test various file key formats
        let keys = vec![
            "file_v2_simple",
            "file_v2_0411a8d4-8e73-4d5b-9c8f-6f7a1b2c3d4e",
            "file_v2_1234567890",
            "file_v2_with_underscores_and_numbers_123",
        ];

        for key in keys {
            let request = FileDownloadRequest::new(key);
            assert_eq!(request.file_key, key);

            let builder = FileDownloadBuilder::new(key);
            assert_eq!(builder.file_key, key);
        }
    }

    #[test]
    fn test_comprehensive_file_download_response() {
        // Test comprehensive response with all fields
        let comprehensive_response = FileDownloadResponse {
            data: vec![0x25, 0x50, 0x44, 0x46], // PDF header bytes
            file_name: Some("comprehensive_test_document.pdf".to_string()),
            file_size: Some(10485760), // 10MB
            content_type: Some("application/pdf".to_string()),
        };

        assert_eq!(comprehensive_response.data, vec![0x25, 0x50, 0x44, 0x46]);
        assert_eq!(comprehensive_response.file_name, Some("comprehensive_test_document.pdf".to_string()));
        assert_eq!(comprehensive_response.file_size, Some(10485760));
        assert_eq!(comprehensive_response.content_type, Some("application/pdf".to_string()));
    }

    #[test]
    fn test_file_download_edge_cases() {
        // Test empty file key
        let empty_request = FileDownloadRequest::new("");
        assert_eq!(empty_request.file_key, "");

        // Test very long file key
        let long_key = "file_v2_".to_string() + &"a".repeat(100);
        let long_request = FileDownloadRequest::new(&long_key);
        assert_eq!(long_request.file_key, long_key);

        // Test response with only data
        let data_only_response = FileDownloadResponse {
            data: vec![1, 2, 3],
            file_name: None,
            file_size: None,
            content_type: None,
        };
        assert_eq!(data_only_response.data, vec![1, 2, 3]);
        assert_eq!(data_only_response.file_name, None);
        assert_eq!(data_only_response.file_size, None);
        assert_eq!(data_only_response.content_type, None);
    }

    #[test]
    fn test_file_download_different_file_types() {
        // Test different file types and content types
        let test_cases = vec![
            ("document.pdf", "application/pdf", vec![0x25, 0x50, 0x44, 0x46]),
            ("spreadsheet.xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", vec![0x50, 0x4B, 0x03, 0x04]),
            ("presentation.pptx", "application/vnd.openxmlformats-officedocument.presentationml.presentation", vec![0x50, 0x4B, 0x03, 0x04]),
            ("text.txt", "text/plain", vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]), // "Hello" in ASCII
            ("archive.zip", "application/zip", vec![0x50, 0x4B, 0x03, 0x04]),
        ];

        for (filename, content_type, data) in test_cases {
            let response = FileDownloadResponse {
                data: data.clone(),
                file_name: Some(filename.to_string()),
                file_size: Some(data.len() as i64),
                content_type: Some(content_type.to_string()),
            };

            assert_eq!(response.data, data);
            assert_eq!(response.file_name, Some(filename.to_string()));
            assert_eq!(response.file_size, Some(data.len() as i64));
            assert_eq!(response.content_type, Some(content_type.to_string()));
        }
    }

    #[test]
    fn test_file_download_large_file_support() {
        // Test handling of large files (up to say 100MB for demonstration)
        let large_data = vec![0u8; 100 * 1024 * 1024]; // 100MB of zeros
        let large_file_response = FileDownloadResponse {
            data: large_data.clone(),
            file_name: Some("large_file.bin".to_string()),
            file_size: Some(large_data.len() as i64),
            content_type: Some("application/octet-stream".to_string()),
        };

        assert_eq!(large_file_response.data.len(), 100 * 1024 * 1024);
        assert_eq!(large_file_response.file_size, Some(100 * 1024 * 1024));
        assert_eq!(large_file_response.content_type, Some("application/octet-stream".to_string()));
    }

    // ==================== 批量发送消息单元测试 ====================

    #[test]
    fn test_batch_send_message_request_creation() {
        let request = BatchSendMessageRequest::new();
        assert_eq!(request.receive_id_list.len(), 0);
        assert_eq!(request.msg_type, "");
        assert_eq!(request.content, "");
        assert_eq!(request.receive_id_type, Some("open_id".to_string()));
        assert_eq!(request.uuid, None);
    }

    #[test]
    fn test_batch_send_message_request_builder() {
        let receive_id_list = vec!["user_1".to_string(), "user_2".to_string(), "user_3".to_string()];
        let request = BatchSendMessageRequest::new()
            .receive_id_list(receive_id_list.clone())
            .msg_type("text")
            .content("{\"text\":\"批量消息测试\"}")
            .receive_id_type("user_id")
            .uuid("unique_batch_id_123");

        assert_eq!(request.receive_id_list, receive_id_list);
        assert_eq!(request.msg_type, "text");
        assert_eq!(request.content, "{\"text\":\"批量消息测试\"}");
        assert_eq!(request.receive_id_type, Some("user_id".to_string()));
        assert_eq!(request.uuid, Some("unique_batch_id_123".to_string()));
    }

    #[test]
    fn test_batch_send_message_request_validation_success() {
        let receive_id_list = vec!["user_1".to_string(), "user_2".to_string()];
        let request = BatchSendMessageRequest::new()
            .receive_id_list(receive_id_list)
            .msg_type("text")
            .content("{\"text\":\"测试消息\"}")
            .receive_id_type("open_id");

        let result = request.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_batch_send_message_request_validation_empty_receive_id_list() {
        let request = BatchSendMessageRequest::new()
            .msg_type("text")
            .content("{\"text\":\"测试消息\"}");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("接收者列表不能为空"));
    }

    #[test]
    fn test_batch_send_message_request_validation_too_many_receive_ids() {
        let receive_id_list: Vec<String> = (0..1001).map(|i| format!("user_{}", i)).collect();
        let request = BatchSendMessageRequest::new()
            .receive_id_list(receive_id_list)
            .msg_type("text")
            .content("{\"text\":\"测试消息\"}");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("接收者数量不能超过1000个"));
    }

    #[test]
    fn test_batch_send_message_request_validation_empty_msg_type() {
        let request = BatchSendMessageRequest::new()
            .receive_id_list(vec!["user_1".to_string()])
            .content("{\"text\":\"测试消息\"}");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("消息类型不能为空"));
    }

    #[test]
    fn test_batch_send_message_request_validation_empty_content() {
        let request = BatchSendMessageRequest::new()
            .receive_id_list(vec!["user_1".to_string()])
            .msg_type("text");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("消息内容不能为空"));
    }

    #[test]
    fn test_batch_send_message_request_validation_invalid_receive_id_type() {
        let request = BatchSendMessageRequest::new()
            .receive_id_list(vec!["user_1".to_string()])
            .msg_type("text")
            .content("{\"text\":\"测试消息\"}")
            .receive_id_type("invalid_type");

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("无效的用户ID类型"));
    }

    #[test]
    fn test_batch_send_message_request_validation_uuid_too_long() {
        let long_uuid = "a".repeat(51);
        let request = BatchSendMessageRequest::new()
            .receive_id_list(vec!["user_1".to_string()])
            .msg_type("text")
            .content("{\"text\":\"测试消息\"}")
            .uuid(long_uuid);

        let result = request.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("UUID长度不能超过50个字符"));
    }

    #[test]
    fn test_batch_send_message_request_default_values() {
        let request = BatchSendMessageRequest::default();
        assert_eq!(request.receive_id_list, Vec::<String>::new());
        assert_eq!(request.msg_type, "");
        assert_eq!(request.content, "");
        assert_eq!(request.receive_id_type, Some("open_id".to_string()));
        assert_eq!(request.uuid, None);
    }

    #[test]
    fn test_batch_send_message_response_creation() {
        let response = BatchSendMessageResponse::default();
        assert_eq!(response.batch_message_id, "");
        assert_eq!(response.invalid_receive_id_list, None);
        assert_eq!(response.valid_receive_id_count, None);
        assert_eq!(response.invalid_receive_id_count, None);
    }

    #[test]
    fn test_batch_send_message_response_with_data() {
        let invalid_ids = vec!["invalid_user_1".to_string(), "invalid_user_2".to_string()];
        let response = BatchSendMessageResponse {
            batch_message_id: "batch_123456".to_string(),
            invalid_receive_id_list: Some(invalid_ids.clone()),
            valid_receive_id_count: Some(98),
            invalid_receive_id_count: Some(2),
        };

        assert_eq!(response.batch_message_id, "batch_123456");
        assert_eq!(response.invalid_receive_id_list, Some(invalid_ids));
        assert_eq!(response.valid_receive_id_count, Some(98));
        assert_eq!(response.invalid_receive_id_count, Some(2));
    }

    #[test]
    fn test_batch_send_message_response_serialization() {
        let response = BatchSendMessageResponse {
            batch_message_id: "batch_test_123".to_string(),
            invalid_receive_id_list: Some(vec!["bad_user".to_string()]),
            valid_receive_id_count: Some(99),
            invalid_receive_id_count: Some(1),
        };

        // Test serialization
        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("batch_test_123"));
        assert!(serialized.contains("bad_user"));
        assert!(serialized.contains("99"));
        assert!(serialized.contains("1"));

        // Test deserialization
        let deserialized: BatchSendMessageResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.batch_message_id, "batch_test_123");
        assert_eq!(deserialized.valid_receive_id_count, Some(99));
        assert_eq!(deserialized.invalid_receive_id_count, Some(1));
    }

    #[test]
    fn test_batch_send_message_request_serialization() {
        let request = BatchSendMessageRequest {
            receive_id_list: vec!["user_1".to_string(), "user_2".to_string()],
            msg_type: "text".to_string(),
            content: "{\"text\":\"序列化测试\"}".to_string(),
            receive_id_type: Some("open_id".to_string()),
            uuid: Some("test_uuid_123".to_string()),
        };

        // Test serialization
        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("user_1"));
        assert!(serialized.contains("user_2"));
        assert!(serialized.contains("text"));
        assert!(serialized.contains("序列化测试"));
        assert!(serialized.contains("open_id"));
        assert!(serialized.contains("test_uuid_123"));

        // Test deserialization
        let deserialized: BatchSendMessageRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.receive_id_list, vec!["user_1".to_string(), "user_2".to_string()]);
        assert_eq!(deserialized.msg_type, "text");
        assert_eq!(deserialized.content, "{\"text\":\"序列化测试\"}");
        assert_eq!(deserialized.receive_id_type, Some("open_id".to_string()));
        assert_eq!(deserialized.uuid, Some("test_uuid_123".to_string()));
    }

    #[test]
    fn test_batch_send_message_api_response_trait() {
        assert_eq!(BatchSendMessageResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_batch_send_message_endpoint_constant() {
        // Test that the endpoint constant is properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::MESSAGE_V4_BATCH_SEND,
            "/open-apis/message/v4/batch_send/"
        );
    }

    #[test]
    fn test_batch_send_message_builder() {
        let builder = BatchSendMessageBuilder::new()
            .receive_id_list(vec!["user_1".to_string(), "user_2".to_string()])
            .msg_type("text")
            .content("{\"text\":\"构建器测试\"}")
            .receive_id_type("user_id")
            .uuid("builder_test_uuid");

        assert_eq!(builder.request.receive_id_list, vec!["user_1".to_string(), "user_2".to_string()]);
        assert_eq!(builder.request.msg_type, "text");
        assert_eq!(builder.request.content, "{\"text\":\"构建器测试\"}");
        assert_eq!(builder.request.receive_id_type, Some("user_id".to_string()));
        assert_eq!(builder.request.uuid, Some("builder_test_uuid".to_string()));
    }

    #[test]
    fn test_batch_send_message_builder_add_receive_id() {
        let builder = BatchSendMessageBuilder::new()
            .add_receive_id("user_1")
            .add_receive_id("user_2")
            .add_receive_id("user_3")
            .msg_type("post")
            .content("{\"post\":{\"zh_cn\":{\"title\":\"测试标题\"}}}");

        assert_eq!(builder.request.receive_id_list, vec!["user_1", "user_2", "user_3"]);
        assert_eq!(builder.request.msg_type, "post");
        assert_eq!(builder.request.content, "{\"post\":{\"zh_cn\":{\"title\":\"测试标题\"}}}");
    }

    #[test]
    fn test_batch_send_message_builder_default() {
        let builder = BatchSendMessageBuilder::default();
        assert_eq!(builder.request.receive_id_list, Vec::<String>::new());
        assert_eq!(builder.request.msg_type, "");
        assert_eq!(builder.request.content, "");
        assert_eq!(builder.request.receive_id_type, Some("open_id".to_string()));
        assert_eq!(builder.request.uuid, None);
    }

    #[test]
    fn test_batch_send_message_service_builder() {
        let config = Config::default();
        let service = MessageService::new(config);

        let builder = service.batch_send_message_builder();
        assert_eq!(builder.request.receive_id_list, Vec::<String>::new());
        assert_eq!(builder.request.msg_type, "");
        assert_eq!(builder.request.content, "");
    }

    #[test]
    fn test_batch_send_message_different_message_types() {
        // Test different message types
        let message_types = vec![
            ("text", "{\"text\":\"文本消息\"}"),
            ("post", "{\"post\":{\"zh_cn\":{\"title\":\"富文本消息\"}}}"),
            ("image", "{\"image_key\":\"img_v2_key_123\"}"),
            ("file", "{\"file_key\":\"file_v2_key_456\"}"),
            ("interactive", "{\"type\":\"template\",\"data\":{\"template_id\":\"template_123\"}}"),
        ];

        for (msg_type, content) in message_types {
            let request = BatchSendMessageRequest::new()
                .receive_id_list(vec!["test_user".to_string()])
                .msg_type(msg_type)
                .content(content);

            assert_eq!(request.msg_type, msg_type);
            assert_eq!(request.content, content);
        }
    }

    #[test]
    fn test_batch_send_message_large_receive_id_list() {
        // Test with maximum allowed receive ID list (1000 users)
        let receive_id_list: Vec<String> = (0..1000).map(|i| format!("user_{}", i)).collect();
        let request = BatchSendMessageRequest::new()
            .receive_id_list(receive_id_list.clone())
            .msg_type("text")
            .content("{\"text\":\"大规模批量消息测试\"}");

        assert_eq!(request.receive_id_list.len(), 1000);
        assert_eq!(request.receive_id_list, receive_id_list);

        // Validation should succeed for exactly 1000 users
        let result = request.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_batch_send_message_comprehensive_response() {
        // Test comprehensive response with all fields
        let invalid_ids = vec![
            "invalid_user_1".to_string(),
            "invalid_user_2".to_string(),
            "invalid_user_3".to_string(),
        ];
        let comprehensive_response = BatchSendMessageResponse {
            batch_message_id: "comprehensive_batch_001".to_string(),
            invalid_receive_id_list: Some(invalid_ids.clone()),
            valid_receive_id_count: Some(997),
            invalid_receive_id_count: Some(3),
        };

        assert_eq!(comprehensive_response.batch_message_id, "comprehensive_batch_001");
        assert_eq!(comprehensive_response.invalid_receive_id_list, Some(invalid_ids));
        assert_eq!(comprehensive_response.valid_receive_id_count, Some(997));
        assert_eq!(comprehensive_response.invalid_receive_id_count, Some(3));
    }
}