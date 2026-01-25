/// 文件订阅管理模块
pub mod create;
pub mod get;
pub mod models;
pub mod patch;

// 重新导出所有API函数
// create 模块显式导出
pub use create::{
    CreateFileSubscriptionRequest,
    CreateFileSubscriptionResponse,
    GetSubscriptionRequest,
    GetSubscriptionResponse,
    PatchSubscriptionRequest,
    PatchSubscriptionResponse,
    Subscription,
    create_file_subscription,
    get_subscription,
    is_subcribe,
    new,
    patch_subscription,
    subscription_id,
};
// get 模块显式导出
pub use get::{
    CreateFileSubscriptionRequest,
    CreateFileSubscriptionResponse,
    GetSubscriptionRequest,
    GetSubscriptionResponse,
    PatchSubscriptionRequest,
    PatchSubscriptionResponse,
    Subscription,
    create_file_subscription,
    get_subscription,
    is_subcribe,
    new,
    patch_subscription,
    subscription_id,
};
// models 模块显式导出
pub use models::{
    CreateFileSubscriptionRequest,
    CreateFileSubscriptionResponse,
    GetSubscriptionRequest,
    GetSubscriptionResponse,
    PatchSubscriptionRequest,
    PatchSubscriptionResponse,
    Subscription,
    create_file_subscription,
    get_subscription,
    is_subcribe,
    new,
    patch_subscription,
    subscription_id,
};
// patch 模块显式导出
pub use patch::{
    CreateFileSubscriptionRequest,
    CreateFileSubscriptionResponse,
    GetSubscriptionRequest,
    GetSubscriptionResponse,
    PatchSubscriptionRequest,
    PatchSubscriptionResponse,
    Subscription,
    create_file_subscription,
    get_subscription,
    is_subcribe,
    new,
    patch_subscription,
    subscription_id,
};
