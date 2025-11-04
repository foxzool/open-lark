use crate::core::config::Config;
pub mod chat;
pub mod chat_announcement;
pub mod chat_member;
pub mod chat_menu_tree;
pub mod chat_tab;
pub mod models;
pub use models::*;
/// 群组 v1 API 版本服务
pub struct V1 {
    pub chat: chat::ChatService,
    pub chat_member: chat_member::ChatMemberService,
    pub chat_announcement: chat_announcement::ChatAnnouncementService,
    pub chat_tab: chat_tab::ChatTabService,
    pub chat_menu_tree: chat_menu_tree::ChatMenuTreeService,
}    /// 客户端配置
    config: Config}
impl V1 {
}
/// 获取客户端配置
    pub fn w+.*{
&self.config}
}
