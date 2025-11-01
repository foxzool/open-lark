//! IM 模块统一模型导出
//!
//! 重新导出 v1 和 v2 中的所有模型类型，提供统一的访问接口。

// 导入 v1 特有的类型
pub use crate::im::v1::models::{
    BatchMessage, BatchMessageStatus, EmojiType, FileInfo, ImageInfo, Message, MessageCard,
    MessageReaction, MessageReadInfo, MessageType, PageInfo, Pin, ReactionUser, ReadUser,
    ReceiveIdType, UrgentInfo, UrgentType, UrlPreview,
};
