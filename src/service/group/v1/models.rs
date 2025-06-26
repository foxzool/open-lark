use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    UserId,
    UnionId,
    OpenId,
}

/// 群ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatIdType {
    ChatId,
    OpenChatId,
}

/// 群类型
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChatType {
    /// 私聊
    P2p = 1,
    /// 群聊
    Group = 2,
}

/// 群模式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatMode {
    /// 群组
    Group,
    /// 话题
    Topic,
}

/// 群配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatConfig {
    /// 是否允许加入群聊
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joinable: Option<bool>,
    /// 是否允许搜索到群聊
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    /// 是否允许成员分享群链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_allowed: Option<bool>,
    /// 是否仅群主可编辑群信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_owner_edit: Option<bool>,
    /// 是否仅群主和群管理员可发起视频会议
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_owner_video_call: Option<bool>,
    /// 是否仅群主和群管理员可发送消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_owner_send_msg: Option<bool>,
}

/// 群基本信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    /// 群 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// 群名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 群描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 群头像 key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 群类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<ChatType>,
    /// 群模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_mode: Option<ChatMode>,
    /// 群配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ChatConfig>,
    /// 群主 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// 群创建者 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 群创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 群更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 群成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// 群外部标识符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    /// 租户 key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

/// 群成员类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberType {
    /// 用户
    User,
    /// 机器人
    Bot,
}

/// 群成员身份
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberRole {
    /// 群主
    Owner,
    /// 群管理员
    Admin,
    /// 群成员
    Member,
}

/// 群成员信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMember {
    /// 成员 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<MemberType>,
    /// 成员身份
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<MemberRole>,
    /// 成员名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 成员头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 加入时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    /// 租户 key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

/// 群置顶信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatTopNotice {
    /// 群 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// 置顶信息 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_id: Option<String>,
    /// 置顶内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}

/// 群公告信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatAnnouncement {
    /// 群公告 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement_id: Option<String>,
    /// 群 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// 公告标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 公告内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// 群公告块信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatAnnouncementBlock {
    /// 块 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    /// 父块 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 块类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    /// 块内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    /// 子块列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
}

/// 会话标签页类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatTabType {
    /// 消息
    Message,
    /// 文档
    Doc,
    /// 会议纪要
    Meeting,
    /// 文件
    File,
    /// 机器人
    Bot,
    /// 自定义
    Custom,
}

/// 会话标签页信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatTab {
    /// 标签页 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<String>,
    /// 标签页名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 标签页类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_type: Option<ChatTabType>,
    /// 标签页链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 标签页图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 排序号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}

/// 群菜单项类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatMenuType {
    /// 链接
    Link,
    /// 回调
    Callback,
    /// 子菜单
    Submenu,
}

/// 群菜单项信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMenu {
    /// 菜单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_id: Option<String>,
    /// 父菜单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 菜单名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 菜单类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_type: Option<ChatMenuType>,
    /// 菜单图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 菜单链接或回调数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 排序号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    /// 是否可见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    /// 子菜单列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ChatMenu>>,
}

/// 分页信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageInfo {
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}
