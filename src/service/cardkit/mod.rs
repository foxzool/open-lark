use crate::core::config::Config;

pub mod v1;

/// 飞书卡片服务
///
/// 提供飞书卡片相关功能，包括卡片创建、更新、组件管理等
///
/// # 功能模块
/// - v1: 卡片 v1 API
///
/// # 示例
/// ```rust,ignore
/// use open_lark::LarkClient;
/// 
/// let client = LarkClient::builder("app_id", "app_secret").build();
/// 
/// // 创建卡片
/// let response = client.cardkit.v1.card.create(request, None).await?;
/// 
/// // 新增组件
/// let response = client.cardkit.v1.card_element.create(request, None).await?;
/// ```
pub struct CardkitService {
    /// v1 API
    pub v1: v1::V1,
}

impl CardkitService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}