//! 消息表情回复相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 表情类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionType {
    pub emoji_type: String,
}

/// 操作者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionOperator {
    pub operator_id: String,
    pub operator_type: String,
}

/// 表情回复
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReaction {
    pub reaction_id: String,
    pub operator: ReactionOperator,
    pub action_time: String,
    pub reaction_type: ReactionType,
}

impl ApiResponseTrait for MessageReaction {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加消息表情回复请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageReactionBody {
    pub reaction_type: ReactionType,
}

/// 获取消息表情回复列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMessageReactionsResponse {
    #[serde(default)]
    pub items: Option<Vec<MessageReaction>>,
    pub has_more: bool,
    #[serde(default)]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListMessageReactionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

