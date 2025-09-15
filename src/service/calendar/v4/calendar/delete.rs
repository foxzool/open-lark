//! # 删除共享日历
//!
//! 删除一个共享日历的API接口。
//!
//! ## 功能描述
//!
//! 此功能用于删除用户创建的共享日历。删除后，日历中的所有事件也会被删除，
//! 且该操作不可逆。
//!
//! ## API 文档
//!
//! 参考飞书开放平台文档: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/delete>
//!
//! ## 实现状态
//!
//! 🚧 **待实现** - 此功能尚未实现，敬请期待。
//!
//! ## 预期接口
//!
//! ```rust,ignore
//! // 删除指定的共享日历
//! let response = client.calendar.v4.calendar.delete(
//!     calendar_id,
//!     None  // RequestOption
//! ).await?;
//! ```
