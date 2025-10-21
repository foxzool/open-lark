use crate::core::config::Config;

pub mod app_feed_card;
pub mod groups_bots;
pub mod models;

use app_feed_card::AppFeedCardService;
use groups_bots::GroupsBotsService;

/// IM v2服务
pub struct V2 {
    /// 应用消息流卡片服务
    pub app_feed_card: AppFeedCardService,
    /// 群聊或机器人消息服务
    pub groups_bots: GroupsBotsService,
}

impl V2 {
    pub fn new(config: Config) -> Self {
        Self {
            app_feed_card: AppFeedCardService::new(config.clone()),
            groups_bots: GroupsBotsService::new(config),
        }
    }
}
