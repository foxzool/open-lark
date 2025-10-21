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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChatConfig {
    /// 是否可加入
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joinable: Option<bool>,
    /// 是否可搜索
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    /// 是否允许分享
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_allowed: Option<bool>,
    /// 仅群主可编辑
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_owner_edit: Option<bool>,
    /// 仅群主可发起视频
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_owner_video_call: Option<bool>,
    /// 仅群主可发言
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

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_type_enum() {
        assert_eq!(
            serde_json::to_string(&UserIdType::UserId).unwrap(),
            "\"user_id\""
        );
        assert_eq!(
            serde_json::to_string(&UserIdType::UnionId).unwrap(),
            "\"union_id\""
        );
        assert_eq!(
            serde_json::to_string(&UserIdType::OpenId).unwrap(),
            "\"open_id\""
        );
    }

    #[test]
    fn test_chat_id_type_enum() {
        assert_eq!(
            serde_json::to_string(&ChatIdType::ChatId).unwrap(),
            "\"chat_id\""
        );
        assert_eq!(
            serde_json::to_string(&ChatIdType::OpenChatId).unwrap(),
            "\"open_chat_id\""
        );
    }

    #[test]
    fn test_chat_type_enum() {
        assert_eq!(serde_json::to_string(&ChatType::P2p).unwrap(), "1");
        assert_eq!(serde_json::to_string(&ChatType::Group).unwrap(), "2");
    }

    #[test]
    fn test_chat_mode_enum() {
        assert_eq!(
            serde_json::to_string(&ChatMode::Group).unwrap(),
            "\"group\""
        );
        assert_eq!(
            serde_json::to_string(&ChatMode::Topic).unwrap(),
            "\"topic\""
        );
    }

    #[test]
    fn test_member_type_enum() {
        assert_eq!(
            serde_json::to_string(&MemberType::User).unwrap(),
            "\"user\""
        );
        assert_eq!(serde_json::to_string(&MemberType::Bot).unwrap(), "\"bot\"");
    }

    #[test]
    fn test_member_role_enum() {
        assert_eq!(
            serde_json::to_string(&MemberRole::Owner).unwrap(),
            "\"owner\""
        );
        assert_eq!(
            serde_json::to_string(&MemberRole::Admin).unwrap(),
            "\"admin\""
        );
        assert_eq!(
            serde_json::to_string(&MemberRole::Member).unwrap(),
            "\"member\""
        );
    }

    #[test]
    fn test_chat_tab_type_enum() {
        assert_eq!(
            serde_json::to_string(&ChatTabType::Message).unwrap(),
            "\"message\""
        );
        assert_eq!(serde_json::to_string(&ChatTabType::Doc).unwrap(), "\"doc\"");
        assert_eq!(
            serde_json::to_string(&ChatTabType::Meeting).unwrap(),
            "\"meeting\""
        );
        assert_eq!(
            serde_json::to_string(&ChatTabType::File).unwrap(),
            "\"file\""
        );
        assert_eq!(serde_json::to_string(&ChatTabType::Bot).unwrap(), "\"bot\"");
        assert_eq!(
            serde_json::to_string(&ChatTabType::Custom).unwrap(),
            "\"custom\""
        );
    }

    #[test]
    fn test_chat_menu_type_enum() {
        assert_eq!(
            serde_json::to_string(&ChatMenuType::Link).unwrap(),
            "\"link\""
        );
        assert_eq!(
            serde_json::to_string(&ChatMenuType::Callback).unwrap(),
            "\"callback\""
        );
        assert_eq!(
            serde_json::to_string(&ChatMenuType::Submenu).unwrap(),
            "\"submenu\""
        );
    }

    #[test]
    fn test_chat_config_full() {
        let config = ChatConfig {
            joinable: Some(true),
            searchable: Some(false),
            share_allowed: Some(true),
            only_owner_edit: Some(false),
            only_owner_video_call: Some(true),
            only_owner_send_msg: Some(false),
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"joinable\":true"));
        assert!(json.contains("\"searchable\":false"));
        assert!(json.contains("\"share_allowed\":true"));
        assert!(json.contains("\"only_owner_edit\":false"));
        assert!(json.contains("\"only_owner_video_call\":true"));
        assert!(json.contains("\"only_owner_send_msg\":false"));
    }

    #[test]
    fn test_chat_config_minimal() {
        let config = ChatConfig {
            searchable: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"searchable\":true"));
        assert!(!json.contains("joinable"));
        assert!(!json.contains("share_allowed"));
    }

    #[test]
    fn test_chat_full() {
        let chat_config = ChatConfig::default();

        let chat = Chat {
            chat_id: Some("oc_123456".to_string()),
            name: Some("技术讨论群".to_string()),
            description: Some("团队技术交流群组".to_string()),
            avatar: Some("avatar_key".to_string()),
            chat_type: Some(ChatType::Group),
            chat_mode: Some(ChatMode::Group),
            config: Some(chat_config),
            owner_id: Some("ou_owner123".to_string()),
            creator_id: Some("ou_creator123".to_string()),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            update_time: Some("2024-01-01T12:00:00Z".to_string()),
            member_count: Some(25),
            external: Some(false),
            tenant_key: Some("tenant123".to_string()),
        };

        let json = serde_json::to_string(&chat).unwrap();
        assert!(json.contains("oc_123456"));
        assert!(json.contains("技术讨论群"));
        assert!(json.contains("团队技术交流群组"));
        assert!(json.contains("ou_owner123"));
        assert!(json.contains("25"));
        assert!(json.contains("tenant123"));
    }

    #[test]
    fn test_chat_member_full() {
        let member = ChatMember {
            member_id: Some("ou_member123".to_string()),
            member_type: Some(MemberType::User),
            role: Some(MemberRole::Admin),
            name: Some("张三".to_string()),
            avatar: Some("avatar_key123".to_string()),
            join_time: Some("2024-01-01T00:00:00Z".to_string()),
            tenant_key: Some("tenant123".to_string()),
        };

        let json = serde_json::to_string(&member).unwrap();
        assert!(json.contains("ou_member123"));
        assert!(json.contains("\"user\""));
        assert!(json.contains("\"admin\""));
        assert!(json.contains("张三"));
        assert!(json.contains("avatar_key123"));
        assert!(json.contains("2024-01-01T00:00:00Z"));
    }

    #[test]
    fn test_chat_member_bot() {
        let bot_member = ChatMember {
            member_id: Some("cli_bot123".to_string()),
            member_type: Some(MemberType::Bot),
            role: Some(MemberRole::Member),
            name: Some("助手机器人".to_string()),
            avatar: None,
            join_time: Some("2024-01-02T00:00:00Z".to_string()),
            tenant_key: Some("tenant123".to_string()),
        };

        let json = serde_json::to_string(&bot_member).unwrap();
        assert!(json.contains("cli_bot123"));
        assert!(json.contains("\"bot\""));
        assert!(json.contains("助手机器人"));
        assert!(!json.contains("avatar"));
    }

    #[test]
    fn test_chat_top_notice() {
        let notice = ChatTopNotice {
            chat_id: Some("oc_123456".to_string()),
            notice_id: Some("notice123".to_string()),
            content: Some("重要通知：明天团建活动".to_string()),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            update_time: Some("2024-01-01T10:00:00Z".to_string()),
            creator_id: Some("ou_creator123".to_string()),
        };

        let json = serde_json::to_string(&notice).unwrap();
        assert!(json.contains("oc_123456"));
        assert!(json.contains("notice123"));
        assert!(json.contains("重要通知：明天团建活动"));
        assert!(json.contains("ou_creator123"));
    }

    #[test]
    fn test_chat_announcement() {
        let meta = serde_json::json!({
            "priority": "high",
            "category": "general"
        });

        let announcement = ChatAnnouncement {
            announcement_id: Some("announcement123".to_string()),
            chat_id: Some("oc_123456".to_string()),
            title: Some("群公告标题".to_string()),
            content: Some("这是一条重要的群公告内容".to_string()),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            update_time: Some("2024-01-01T12:00:00Z".to_string()),
            creator_id: Some("ou_creator123".to_string()),
            revision: Some(1),
            meta: Some(meta),
        };

        let json = serde_json::to_string(&announcement).unwrap();
        assert!(json.contains("announcement123"));
        assert!(json.contains("群公告标题"));
        assert!(json.contains("这是一条重要的群公告内容"));
        assert!(json.contains("\"revision\":1"));
        assert!(json.contains("priority"));
    }

    #[test]
    fn test_chat_announcement_block() {
        let content = serde_json::json!({
            "text": "文本内容",
            "format": "rich_text"
        });

        let block = ChatAnnouncementBlock {
            block_id: Some("block123".to_string()),
            parent_id: Some("parent_block123".to_string()),
            block_type: Some("text".to_string()),
            content: Some(content),
            children: Some(vec!["child1".to_string(), "child2".to_string()]),
        };

        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("block123"));
        assert!(json.contains("parent_block123"));
        assert!(json.contains("\"text\""));
        assert!(json.contains("文本内容"));
        assert!(json.contains("child1"));
        assert!(json.contains("child2"));
    }

    #[test]
    fn test_chat_tab() {
        let tab = ChatTab {
            tab_id: Some("tab123".to_string()),
            name: Some("会议纪要".to_string()),
            tab_type: Some(ChatTabType::Meeting),
            url: Some("https://meeting.example.com".to_string()),
            icon: Some("meeting_icon".to_string()),
            sort_order: Some(1),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            update_time: Some("2024-01-01T12:00:00Z".to_string()),
            creator_id: Some("ou_creator123".to_string()),
        };

        let json = serde_json::to_string(&tab).unwrap();
        assert!(json.contains("tab123"));
        assert!(json.contains("会议纪要"));
        assert!(json.contains("\"meeting\""));
        assert!(json.contains("https://meeting.example.com"));
        assert!(json.contains("\"sort_order\":1"));
    }

    #[test]
    fn test_chat_menu_simple() {
        let menu = ChatMenu {
            menu_id: Some("menu123".to_string()),
            parent_id: None,
            name: Some("快捷操作".to_string()),
            menu_type: Some(ChatMenuType::Link),
            icon: Some("link_icon".to_string()),
            value: Some("https://example.com".to_string()),
            sort_order: Some(1),
            visible: Some(true),
            children: None,
        };

        let json = serde_json::to_string(&menu).unwrap();
        assert!(json.contains("menu123"));
        assert!(json.contains("快捷操作"));
        assert!(json.contains("\"link\""));
        assert!(json.contains("https://example.com"));
        assert!(json.contains("\"visible\":true"));
        assert!(!json.contains("parent_id"));
        assert!(!json.contains("children"));
    }

    #[test]
    fn test_chat_menu_with_submenu() {
        let child_menu = ChatMenu {
            menu_id: Some("child_menu123".to_string()),
            parent_id: Some("menu123".to_string()),
            name: Some("子菜单".to_string()),
            menu_type: Some(ChatMenuType::Callback),
            icon: Some("callback_icon".to_string()),
            value: Some("callback_data".to_string()),
            sort_order: Some(1),
            visible: Some(true),
            children: None,
        };

        let parent_menu = ChatMenu {
            menu_id: Some("menu123".to_string()),
            parent_id: None,
            name: Some("主菜单".to_string()),
            menu_type: Some(ChatMenuType::Submenu),
            icon: Some("submenu_icon".to_string()),
            value: None,
            sort_order: Some(0),
            visible: Some(true),
            children: Some(vec![child_menu]),
        };

        let json = serde_json::to_string(&parent_menu).unwrap();
        assert!(json.contains("menu123"));
        assert!(json.contains("主菜单"));
        assert!(json.contains("\"submenu\""));
        assert!(json.contains("child_menu123"));
        assert!(json.contains("子菜单"));
        assert!(json.contains("\"callback\""));
    }

    #[test]
    fn test_page_info() {
        let page_info = PageInfo {
            page_token: Some("token123".to_string()),
            has_more: Some(true),
        };

        let json = serde_json::to_string(&page_info).unwrap();
        assert!(json.contains("token123"));
        assert!(json.contains("\"has_more\":true"));
    }

    #[test]
    fn test_page_info_minimal() {
        let page_info = PageInfo {
            page_token: None,
            has_more: Some(false),
        };

        let json = serde_json::to_string(&page_info).unwrap();
        assert!(json.contains("\"has_more\":false"));
        assert!(!json.contains("page_token"));
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_chat = Chat {
            chat_id: Some("oc_minimal".to_string()),
            name: None,
            description: None,
            avatar: None,
            chat_type: None,
            chat_mode: None,
            config: None,
            owner_id: None,
            creator_id: None,
            create_time: None,
            update_time: None,
            member_count: None,
            external: None,
            tenant_key: None,
        };

        let json = serde_json::to_string(&minimal_chat).unwrap();
        assert!(json.contains("oc_minimal"));
        assert!(!json.contains("name"));
        assert!(!json.contains("description"));
        assert!(!json.contains("config"));
    }
}
