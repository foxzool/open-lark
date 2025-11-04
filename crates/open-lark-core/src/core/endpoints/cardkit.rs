//! cardkit 服务端点常量定义

/// 卡片列表
pub const CARDKIT_V1_CARDS: &str = "/open-apis/cardkit/v1/cards";

/// 卡片更新
pub const CARDKIT_V1_CARD_UPDATE: &str = "/open-apis/cardkit/v1/cards/{card_id}";

/// 卡片批量更新
pub const CARDKIT_V1_CARD_BATCH_UPDATE: &str = "/open-apis/cardkit/v1/cards/batch_update";

/// 卡片设置
pub const CARDKIT_V1_CARD_SETTINGS: &str = "/open-apis/cardkit/v1/cards/{card_id}/settings";

/// 卡片元素
pub const CARDKIT_V1_CARD_ELEMENTS: &str = "/open-apis/cardkit/v1/cards/{card_id}/elements";
