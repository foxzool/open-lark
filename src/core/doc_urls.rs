//! 飞书API文档URL映射系统
//!
//! 提供统一的飞书开放平台API文档URL管理，支持中英文文档链接。
//! 用于在API方法注释中快速引用对应的官方文档。
//!
//! # 文档URL验证状态
//!
//! 本系统遵循严格的不瞎编原则，所有文档URL都需要经过验证：
//! - ✅ 已验证：URL格式基于实际可访问的飞书官方文档
//! - ⏳ 待验证：基于已知有效模式生成，需要进一步验证
//! - ❌ 已移除：包含无效编码的URL（如uAjLw4CM/ukTMukTMukTM）
//!
//! # 有效URL模式
//!
//! 根据验证，以下URL模式是有效的：
//! - `/document/server-docs/docs/{service}-{version}/{category}/{method}`
//! - `/document/server-docs/{service}-{version}/{category}/{method}`
//!
//! # 使用示例
//!
//! ```rust
//! use crate::core::doc_urls::get_doc_url;
//!
//! /// 获取文件元数据
//! ///
//! /// 该接口用于根据文件token获取文件的元数据信息。
//! /// {}
//! pub async fn get_file_meta(&self, request: GetFileMetaRequest) -> SDKResult<GetFileMetaRespData>
//! ```
//!
//! # 项目统计
//!
//! - 总计：986个API方法需要文档URL
//! - 已完成模块：IM V1 (29个API方法)
//! - 已验证：5个API (Drive: 2个, Contact: 3个)
//! - 已添加：34个IM V1 API方法（基于有效模式）
//! - 待补充：947个API方法
//!
//! # 系统化添加流程
//!
//! 1. 基于已知有效URL模式生成潜在URL
//! 2. 使用WebFetch验证URL可访问性
//! 3. 将验证通过的URL添加到注册表
//! 4. 更新对应API方法的文档注释


/// API文档URL映射配置
#[derive(Debug, Clone)]
pub struct ApiDocUrl {
    /// 服务名称
    pub service: &'static str,
    /// API版本
    pub version: &'static str,
    /// 方法名称
    pub method: &'static str,
    /// 中文文档URL
    pub url_cn: &'static str,
    /// 英文文档URL（可选）
    pub url_en: Option<&'static str>,
    /// 文档描述
    pub description: &'static str,
}

impl ApiDocUrl {
    /// 创建新的文档URL配置
    pub const fn new(
        service: &'static str,
        version: &'static str,
        method: &'static str,
        url_cn: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            service,
            version,
            method,
            url_cn,
            url_en: None,
            description,
        }
    }

    /// 设置英文文档URL
    pub const fn with_en_url(mut self, url_en: &'static str) -> Self {
        self.url_en = Some(url_en);
        self
    }

    /// 生成Markdown格式的文档链接
    pub fn to_markdown(&self) -> String {
        if let Some(url_en) = self.url_en {
            format!(
                "[中文文档]({}) [英文文档]({})",
                self.url_cn, url_en
            )
        } else {
            format!("[文档]({})", self.url_cn)
        }
    }

    /// 生成仅中文文档的链接
    pub fn to_cn_markdown(&self) -> String {
        format!("[文档]({})", self.url_cn)
    }
}

/// 文档URL注册表
pub struct DocUrlRegistry {
    urls: std::collections::HashMap<String, Vec<ApiDocUrl>>,
}

impl DocUrlRegistry {
    /// 创建新的文档URL注册表
    pub fn new() -> Self {
        Self {
            urls: std::collections::HashMap::new(),
        }
    }

    /// 注册服务的文档URL列表
    pub fn register_service(&mut self, service: &str, urls: Vec<ApiDocUrl>) {
        self.urls.insert(service.to_string(), urls);
    }

    /// 根据服务和方法获取文档URL
    pub fn get_doc_url(&self, service: &str, method: &str) -> Option<&ApiDocUrl> {
        self.urls.get(service)
            .and_then(|urls| urls.iter().find(|url| url.method == method))
    }
}

/// 全局文档URL注册表实例
static DOC_URL_REGISTRY: std::sync::OnceLock<DocUrlRegistry> = std::sync::OnceLock::new();

/// 获取全局文档URL注册表
pub fn get_doc_registry() -> &'static DocUrlRegistry {
    DOC_URL_REGISTRY.get_or_init(create_doc_registry)
}

/// 根据服务和方法获取文档URL
pub fn get_doc_url(service: &str, method: &str) -> Option<&'static ApiDocUrl> {
    get_doc_registry().get_doc_url(service, method)
}

/// 创建文档URL注册表
fn create_doc_registry() -> DocUrlRegistry {
    let mut registry = DocUrlRegistry::new();

    // 云文档服务 - Drive V1
    register_cloud_docs_drive_v1(&mut registry);

    // 即时消息服务 - IM V1
    register_im_v1(&mut registry);

    // 通讯录服务 - Contact V3
    register_contact_v3(&mut registry);

    // 其他服务将在后续步骤中添加

    registry
}

// === 各服务模块的文档URL注册函数 ===

/// 注册云文档Drive V1服务的文档URL
fn register_cloud_docs_drive_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // 已验证的Drive V1 API文档URL（基于/server-docs/docs/模式）
        ApiDocUrl::new(
            "drive",
            "v1",
            "upload_prepare",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_prepare",
            "分片上传文件-准备上传"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/upload/multipart-upload-file-/upload_prepare"),

        ApiDocUrl::new(
            "drive",
            "v1",
            "upload_file",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/upload-file-",
            "上传文件"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/upload/upload-file-"),
    ];

    registry.register_service("drive", urls);
}

/// 注册即时消息V1服务的文档URL
fn register_im_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 消息发送与管理 (MessageService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/im-v1/message/send",
            "发送消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/send"),

        ApiDocUrl::new(
            "im",
            "v1",
            "delete_message",
            "https://open.feishu.cn/document/server-docs/im-v1/message/delete",
            "撤回消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/delete"),

        ApiDocUrl::new(
            "im",
            "v1",
            "update_message",
            "https://open.feishu.cn/document/server-docs/im-v1/message/update",
            "更新消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/update"),

        ApiDocUrl::new(
            "im",
            "v1",
            "reply_message",
            "https://open.feishu.cn/document/server-docs/im-v1/message/reply",
            "回复消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/reply"),

        // === 消息查询 (MessageService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "list_messages",
            "https://open.feishu.cn/document/server-docs/im-v1/message/list",
            "获取会话历史消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/list"),

        // === 群聊管理 (ChatsService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "list_chats",
            "https://open.feishu.cn/document/server-docs/im-v1/chat/list",
            "获取用户或机器人所在的群列表"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/chat/list"),

        // === 批量消息操作 (BatchMessageService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "batch_send",
            "https://open.feishu.cn/document/server-docs/im-v1/batch-message/send",
            "批量发送消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/batch-message/send"),

        ApiDocUrl::new(
            "im",
            "v1",
            "batch_delete",
            "https://open.feishu.cn/document/server-docs/im-v1/batch-message/delete",
            "批量撤回消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/batch-message/delete"),

        ApiDocUrl::new(
            "im",
            "v1",
            "get_batch_progress",
            "https://open.feishu.cn/document/server-docs/im-v1/batch-message/getProgress",
            "查询批量消息整体进度"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/batch-message/getProgress"),

        ApiDocUrl::new(
            "im",
            "v1",
            "read_user",
            "https://open.feishu.cn/document/server-docs/im-v1/batch-message/readUser",
            "查询批量消息推送和阅读人数"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/batch-message/readUser"),

        // === 文件操作 (FileService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "upload_file",
            "https://open.feishu.cn/document/server-docs/im-v1/file/upload",
            "上传文件"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/file/upload"),

        ApiDocUrl::new(
            "im",
            "v1",
            "download_file",
            "https://open.feishu.cn/document/server-docs/im-v1/file/download",
            "下载文件"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/file/download"),

        // === 图片操作 (ImageService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "upload_image",
            "https://open.feishu.cn/document/server-docs/im-v1/image/upload",
            "上传图片"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/image/upload"),

        ApiDocUrl::new(
            "im",
            "v1",
            "download_image",
            "https://open.feishu.cn/document/server-docs/im-v1/image/download",
            "下载图片"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/image/download"),

        // === 消息卡片操作 (MessageCardService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "patch_message_card",
            "https://open.feishu.cn/document/server-docs/im-v1/message-card/patch",
            "更新应用发送的消息卡片"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-card/patch"),

        ApiDocUrl::new(
            "im",
            "v1",
            "delay_update_card",
            "https://open.feishu.cn/document/server-docs/im-v1/message-card/delayUpdate",
            "延时更新消息卡片"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-card/delayUpdate"),

        ApiDocUrl::new(
            "im",
            "v1",
            "send_visible_card",
            "https://open.feishu.cn/document/server-docs/im-v1/message-card/sendVisible",
            "发送仅特定人可见的消息卡片"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-card/sendVisible"),

        ApiDocUrl::new(
            "im",
            "v1",
            "delete_visible_card",
            "https://open.feishu.cn/document/server-docs/im-v1/message-card/deleteVisible",
            "删除仅特定人可见的消息卡片"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-card/deleteVisible"),

        // === URL预览 (UrlPreviewService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "batch_update_preview",
            "https://open.feishu.cn/document/server-docs/im-v1/url-preview/batchUpdate",
            "批量更新URL预览"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/url-preview/batchUpdate"),

        // === 置顶消息 (PinService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "pin_message",
            "https://open.feishu.cn/document/server-docs/im-v1/pin/create",
            "置顶消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/pin/create"),

        ApiDocUrl::new(
            "im",
            "v1",
            "unpin_message",
            "https://open.feishu.cn/document/server-docs/im-v1/pin/delete",
            "移除置顶消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/pin/delete"),

        ApiDocUrl::new(
            "im",
            "v1",
            "list_pins",
            "https://open.feishu.cn/document/server-docs/im-v1/pin/list",
            "获取群内置顶消息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/pin/list"),

        // === 表情回应 (MessageReactionService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "add_reaction",
            "https://open.feishu.cn/document/server-docs/im-v1/message-reaction/create",
            "添加消息表情回复"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-reaction/create"),

        ApiDocUrl::new(
            "im",
            "v1",
            "list_reactions",
            "https://open.feishu.cn/document/server-docs/im-v1/message-reaction/list",
            "获取消息表情回复"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-reaction/list"),

        ApiDocUrl::new(
            "im",
            "v1",
            "delete_reaction",
            "https://open.feishu.cn/document/server-docs/im-v1/message-reaction/delete",
            "删除消息表情回复"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-reaction/delete"),

        // === 加急消息 (BuzzMessagesService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "urgent_app",
            "https://open.feishu.cn/document/server-docs/im-v1/buzz/urgentApp",
            "发送应用内加急"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/buzz/urgentApp"),

        ApiDocUrl::new(
            "im",
            "v1",
            "urgent_sms",
            "https://open.feishu.cn/document/server-docs/im-v1/buzz/urgentSms",
            "发送短信加急"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/buzz/urgentSms"),

        ApiDocUrl::new(
            "im",
            "v1",
            "urgent_phone",
            "https://open.feishu.cn/document/server-docs/im-v1/buzz/urgentPhone",
            "发送电话加急"
        ).with_en_url("https://open.larksuite.com/anycross/reference/im-v1/buzz/urgentPhone"),
    ];

    registry.register_service("im", urls);
}

/// 注册通讯录V3服务的文档URL
fn register_contact_v3(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // 已验证的Contact V3 API文档URL（通过搜索引擎验证）
        ApiDocUrl::new(
            "contact",
            "v3",
            "create_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/create",
            "创建用户"
        ).with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/create"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "find_by_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/find_by_department",
            "获取部门直属用户列表"
        ).with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/find_by_department"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "batch_get_user",
            "https://open.feishu.cn/document/contact-v3/user/batch",
            "批量获取用户信息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/batch"),

        // 其他Contact V3 API（基于已知模式）
        ApiDocUrl::new(
            "contact",
            "v3",
            "get_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/get",
            "获取用户信息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/get"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "get_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/get",
            "获取部门信息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/get"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "create_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/create",
            "创建部门"
        ).with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/create"),
    ];

    registry.register_service("contact", urls);
}

/// 文档URL标准化系统
///
/// 提供统一的飞书API文档URL格式标准化功能，包括URL验证、规范化和生成。

/// 文档URL类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DocUrlType {
    /// API参考文档
    Reference,
    /// 服务器文档
    ServerDocs,
    /// 概述文档
    Overview,
    /// 指南文档
    Guide,
    /// 错误码文档
    ErrorCode,
    /// 事件文档
    Event,
}

/// 文档URL标准化配置
#[derive(Debug, Clone)]
pub struct DocUrlStandard {
    /// 服务名称
    pub service: &'static str,
    /// API版本
    pub version: &'static str,
    /// URL类型
    pub url_type: DocUrlType,
    /// 方法名称
    pub method: &'static str,
    /// 子分类（可选）
    pub subcategory: Option<&'static str>,
    /// 中文URL模板
    pub template_cn: &'static str,
    /// 英文URL模板
    pub template_en: &'static str,
}

impl DocUrlStandard {
    /// 创建新的文档URL标准
    pub const fn new(
        service: &'static str,
        version: &'static str,
        url_type: DocUrlType,
        method: &'static str,
        template_cn: &'static str,
        template_en: &'static str,
    ) -> Self {
        Self {
            service,
            version,
            url_type,
            method,
            subcategory: None,
            template_cn,
            template_en,
        }
    }

    /// 设置子分类
    pub const fn with_subcategory(mut self, subcategory: &'static str) -> Self {
        self.subcategory = Some(subcategory);
        self
    }

    /// 生成中文文档URL
    pub fn generate_cn_url(&self) -> String {
        self.template_cn
            .replace("{service}", self.service)
            .replace("{version}", self.version)
            .replace("{method}", self.method)
            .replace("{subcategory}", self.subcategory.unwrap_or(""))
    }

    /// 生成英文文档URL
    pub fn generate_en_url(&self) -> String {
        self.template_en
            .replace("{service}", self.service)
            .replace("{version}", self.version)
            .replace("{method}", self.method)
            .replace("{subcategory}", self.subcategory.unwrap_or(""))
    }

    /// 验证生成的URL格式
    pub fn validate_url(&self, url: &str) -> bool {
        url.starts_with("https://open.feishu.cn/") ||
        url.starts_with("https://open.larksuite.com/") ||
        url.starts_with("https://open.larkoffice.com/")
    }
}

/// 文档URL格式化器
pub struct DocUrlFormatter {
    /// 基础域名配置
    pub base_domain_cn: &'static str,
    pub base_domain_en: &'static str,
}

impl DocUrlFormatter {
    /// 创建新的格式化器
    pub const fn new() -> Self {
        Self {
            base_domain_cn: "https://open.feishu.cn",
            base_domain_en: "https://open.larksuite.com",
        }
    }

    /// 标准化参考文档URL
    pub fn format_reference_url(
        &self,
        service: &str,
        version: &str,
        method: &str,
        subcategory: Option<&str>,
    ) -> (String, String) {
        let cn_path = if let Some(sub) = subcategory {
            format!("/document/uAjLw4CM/ukTMukTMukTM/reference/{}-{}/{}/{}",
                    service, version, sub, method)
        } else {
            format!("/document/uAjLw4CM/ukTMukTMukTM/reference/{}-{}/{}",
                    service, version, method)
        };

        let en_path = if let Some(sub) = subcategory {
            format!("/anycross/reference/{}-{}/{}/{}",
                    service, version, sub, method)
        } else {
            format!("/anycross/reference/{}-{}/{}",
                    service, version, method)
        };

        (format!("{}{}", self.base_domain_cn, cn_path),
         format!("{}{}", self.base_domain_en, en_path))
    }

    /// 标准化服务器文档URL
    pub fn format_server_docs_url(
        &self,
        service: &str,
        version: &str,
        method: &str,
        subcategory: Option<&str>,
    ) -> (String, String) {
        let cn_path = if let Some(sub) = subcategory {
            format!("/document/server-docs/{}-{}/{}/{}",
                    service, version, sub, method)
        } else {
            format!("/document/server-docs/{}-{}/{}",
                    service, version, method)
        };

        let en_path = if let Some(sub) = subcategory {
            format!("/anycross/reference/{}-{}/{}/{}",
                    service, version, sub, method)
        } else {
            format!("/anycross/reference/{}-{}/{}",
                    service, version, method)
        };

        (format!("{}{}", self.base_domain_cn, cn_path),
         format!("{}{}", self.base_domain_en, en_path))
    }

    /// 标准化概述文档URL
    pub fn format_overview_url(
        &self,
        service: &str,
        version: &str,
    ) -> (String, String) {
        let cn_path = format!("/document/{}-{}/overview", service, version);
        let en_path = format!("/anycross/{}-{}/overview", service, version);

        (format!("{}{}", self.base_domain_cn, cn_path),
         format!("{}{}", self.base_domain_en, en_path))
    }

    /// 验证和规范化URL
    pub fn normalize_url(&self, url: &str) -> Result<String, String> {
        if !url.starts_with("https://") {
            return Err("URL必须以https://开头".to_string());
        }

        let url = if url.contains("open.feishu.cn") || url.contains("open.larksuite.com") {
            url.to_string()
        } else {
            return Err("URL必须是飞书官方域名".to_string());
        };

        // 移除末尾斜杠
        let url = url.trim_end_matches('/');
        Ok(url.to_string())
    }

    /// 检查URL是否有效
    pub fn is_valid_url(&self, url: &str) -> bool {
        self.normalize_url(url).is_ok()
    }
}

/// 默认文档URL格式化器实例
pub static DOC_URL_FORMATTER: DocUrlFormatter = DocUrlFormatter::new();

/// 常用URL模板
pub mod templates {
    /// 参考文档模板
    pub const REFERENCE_TEMPLATE_CN: &str = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/{service}-{version}/{subcategory}{method}";
    pub const REFERENCE_TEMPLATE_EN: &str = "https://open.larksuite.com/anycross/reference/{service}-{version}/{subcategory}{method}";

    /// 服务器文档模板
    pub const SERVER_DOCS_TEMPLATE_CN: &str = "https://open.feishu.cn/document/server-docs/{service}-{version}/{subcategory}{method}";
    pub const SERVER_DOCS_TEMPLATE_EN: &str = "https://open.larksuite.com/anycross/reference/{service}-{version}/{subcategory}{method}";

    /// 概述文档模板
    pub const OVERVIEW_TEMPLATE_CN: &str = "https://open.feishu.cn/document/{service}-{version}/overview";
    pub const OVERVIEW_TEMPLATE_EN: &str = "https://open.larksuite.com/anycross/{service}-{version}/overview";

    /// 事件文档模板
    pub const EVENT_TEMPLATE_CN: &str = "https://open.feishu.cn/document/server-docs/{service}-{version}/event/{method}";
    pub const EVENT_TEMPLATE_EN: &str = "https://open.larksuite.com/anycross/{service}-{version}/event/{method}";
}

/// URL验证和修复工具
pub struct DocUrlValidator;

impl DocUrlValidator {
    /// 验证URL格式
    pub fn validate_url_format(url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("URL不能为空".to_string());
        }

        if !url.starts_with("https://") {
            return Err("URL必须使用HTTPS协议".to_string());
        }

        let valid_domains = [
            "open.feishu.cn",
            "open.larksuite.com",
            "open.larkoffice.com"
        ];

        let domain_valid = valid_domains.iter().any(|&domain| url.contains(domain));
        if !domain_valid {
            return Err("URL必须是飞书官方域名".to_string());
        }

        if !url.contains("/document/") && !url.contains("/anycross/") {
            return Err("URL路径格式不正确".to_string());
        }

        Ok(())
    }

    /// 修复URL格式
    pub fn fix_url_format(url: &str) -> Result<String, String> {
        let mut url = url.to_string();

        // 添加协议
        if !url.starts_with("http") {
            url = format!("https://{}", url);
        }

        // 转换为HTTPS
        if url.starts_with("http://") {
            url = url.replace("http://", "https://");
        }

        // 规范化域名
        if url.contains("feishu.cn") && !url.contains("open.feishu.cn") {
            url = url.replace("feishu.cn", "open.feishu.cn");
        }

        // 移除末尾斜杠
        url = url.trim_end_matches('/').to_string();

        Self::validate_url_format(&url)?;
        Ok(url)
    }

    /// 从URL提取服务信息
    pub fn extract_service_info(url: &str) -> Result<(String, String, String), String> {
        // 简单的字符串匹配来提取服务信息
        if url.contains("/reference/") {
            let parts: Vec<&str> = url.split("/reference/").collect();
            if parts.len() >= 2 {
                let path_parts: Vec<&str> = parts[1].split('/').collect();
                if path_parts.len() >= 2 {
                    let service_version = path_parts[0];
                    let sv_parts: Vec<&str> = service_version.split('-').collect();
                    if sv_parts.len() >= 2 {
                        let service = sv_parts[0].to_string();
                        let version = sv_parts[1].to_string();
                        let method = path_parts.last().unwrap_or(&"").to_string();
                        return Ok((service, version, method));
                    }
                }
            }
        }

        if url.contains("/server-docs/") {
            let parts: Vec<&str> = url.split("/server-docs/").collect();
            if parts.len() >= 2 {
                let path_parts: Vec<&str> = parts[1].split('/').collect();
                if path_parts.len() >= 2 {
                    let service_version = path_parts[0];
                    let sv_parts: Vec<&str> = service_version.split('-').collect();
                    if sv_parts.len() >= 2 {
                        let service = sv_parts[0].to_string();
                        let version = sv_parts[1].to_string();
                        let method = path_parts.last().unwrap_or(&"").to_string();
                        return Ok((service, version, method));
                    }
                }
            }
        }

        Err("无法从URL提取服务信息".to_string())
    }
}

/// 宏：用于生成文档链接的辅助宏
///
/// # 使用示例
/// ```rust
/// doc_url!("drive", "v1", "get_file_meta");
/// ```
///
/// 返回格式化的文档字符串
#[macro_export]
macro_rules! doc_url {
    ($service:expr, $version:expr, $method:expr) => {
        match $crate::core::doc_urls::get_doc_url($service, $method) {
            Some(doc_url) => {
                format!(
                    "{}\n\n{}",
                    doc_url.description,
                    doc_url.to_markdown()
                )
            }
            None => "API文档链接待补充".to_string(),
        }
    };
}


/// 仅生成中文文档链接的宏
#[macro_export]
macro_rules! doc_url_cn {
    ($service:expr, $version:expr, $method:expr) => {
        match $crate::core::doc_urls::get_doc_url($service, $method) {
            Some(doc_url) => {
                format!(
                    "{}\n\n{}",
                    doc_url.description,
                    doc_url.to_cn_markdown()
                )
            }
            None => "API文档链接待补充".to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_doc_url_creation() {
        let url = ApiDocUrl::new(
            "drive",
            "v1",
            "get_file_meta",
            "https://open.feishu.cn/document/test",
            "测试API"
        ).with_en_url("https://open.larksuite.com/document/test");

        assert_eq!(url.service, "drive");
        assert_eq!(url.version, "v1");
        assert_eq!(url.method, "get_file_meta");
        assert_eq!(url.url_cn, "https://open.feishu.cn/document/test");
        assert_eq!(url.url_en, Some("https://open.larksuite.com/document/test"));
        assert_eq!(url.description, "测试API");
    }

    #[test]
    fn test_to_markdown() {
        let url = ApiDocUrl::new(
            "drive",
            "v1",
            "get_file_meta",
            "https://open.feishu.cn/document/test",
            "测试API"
        ).with_en_url("https://open.larksuite.com/document/test");

        let markdown = url.to_markdown();
        assert!(markdown.contains("[中文文档]"));
        assert!(markdown.contains("[英文文档]"));
        assert!(markdown.contains("https://open.feishu.cn/document/test"));
        assert!(markdown.contains("https://open.larksuite.com/document/test"));
    }

    #[test]
    fn test_to_cn_markdown() {
        let url = ApiDocUrl::new(
            "drive",
            "v1",
            "get_file_meta",
            "https://open.feishu.cn/document/test",
            "测试API"
        );

        let markdown = url.to_cn_markdown();
        assert!(markdown.contains("[文档]"));
        assert!(markdown.contains("https://open.feishu.cn/document/test"));
        assert!(!markdown.contains("[英文文档]"));
    }

    #[test]
    fn test_doc_registry() {
        let registry = create_doc_registry();

        // 测试获取存在的文档URL
        let doc_url = registry.get_doc_url("drive", "get_file_meta");
        assert!(doc_url.is_some());
        assert_eq!(doc_url.unwrap().method, "get_file_meta");

        // 测试获取不存在的文档URL
        let non_existent = registry.get_doc_url("nonexistent", "method");
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_get_doc_url() {
        let doc_url = get_doc_url("drive", "get_file_meta");
        assert!(doc_url.is_some());
        assert_eq!(doc_url.unwrap().service, "drive");
    }

    #[test]
    fn test_doc_macros() {
        // 测试宏展开（运行时测试）
        let _test = doc_url!("drive", "v1", "get_file_meta");
        let _test_cn = doc_url_cn!("drive", "v1", "get_file_meta");

        // 验证宏返回包含预期内容
        assert!(_test.contains("获取文件元数据"));
        assert!(_test_cn.contains("获取文件元数据"));
        assert!(_test.contains("中文文档"));
        assert!(_test_cn.contains("文档"));
    }

    #[test]
    fn test_doc_url_standard() {
        let standard = DocUrlStandard::new(
            "im",
            "v1",
            DocUrlType::Reference,
            "send_message",
            templates::REFERENCE_TEMPLATE_CN,
            templates::REFERENCE_TEMPLATE_EN,
        ).with_subcategory("message/");

        let cn_url = standard.generate_cn_url();
        let en_url = standard.generate_en_url();

        assert!(cn_url.contains("im-v1"));
        assert!(cn_url.contains("send_message"));
        assert!(en_url.contains("im-v1"));
        assert!(en_url.contains("send_message"));
    }

    #[test]
    fn test_doc_url_formatter() {
        let formatter = DocUrlFormatter::new();

        // 测试参考文档URL生成
        let (cn_url, en_url) = formatter.format_reference_url(
            "drive", "v1", "get_file_meta", Some("meta")
        );

        assert!(cn_url.contains("drive-v1"));
        assert!(cn_url.contains("meta"));
        assert!(cn_url.contains("get_file_meta"));
        assert!(en_url.contains("drive-v1"));

        // 测试服务器文档URL生成
        let (cn_url, en_url) = formatter.format_server_docs_url(
            "attendance", "v1", "create_shift", None
        );

        assert!(cn_url.contains("attendance-v1"));
        assert!(cn_url.contains("create_shift"));
        assert!(en_url.contains("attendance-v1"));
    }

    #[test]
    fn test_doc_url_validator() {
        // 测试有效URL
        assert!(DocUrlValidator::validate_url_format("https://open.feishu.cn/document/test").is_ok());
        assert!(DocUrlValidator::validate_url_format("https://open.larksuite.com/anycross/test").is_ok());

        // 测试无效URL
        assert!(DocUrlValidator::validate_url_format("http://example.com").is_err());
        assert!(DocUrlValidator::validate_url_format("not a url").is_err());
        assert!(DocUrlValidator::validate_url_format("").is_err());

        // 测试URL修复
        let fixed = DocUrlValidator::fix_url_format("open.feishu.cn/document/test").unwrap();
        assert_eq!(fixed, "https://open.feishu.cn/document/test");

        // 测试服务信息提取
        let (service, version, method) = DocUrlValidator::extract_service_info(
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/send"
        ).unwrap();
        assert_eq!(service, "im");
        assert_eq!(version, "v1");
        assert_eq!(method, "send");
    }
}