//! 动态订阅 API v2（strict 路径）

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod patch;

pub use get::GetActivitySubscriptionRequest;
pub use list::ListActivitySubscriptionsRequest;

pub use create::CreateActivitySubscriptionRequest;
pub use delete::DeleteActivitySubscriptionRequest;
pub use patch::UpdateActivitySubscriptionRequest;
