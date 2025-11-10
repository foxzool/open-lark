#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! IM API v2版本
use crate::config::Config;
/// 应用订阅卡片服务
pub struct AppFeedCardService {
    _config: Config,
}
impl AppFeedCardService {
}
    /// 群机器人服务
pub struct GroupsBotsService {
    _config: Config,
}
impl GroupsBotsService {
}
    pub mod app_feed_card {
    use serde::{Deserialize, Serialize
};
    use crate::api_resp::{ApiResponseTrait, ResponseFormat};
    #[derive(Debug, Deserialize, Serialize)]
    pub struct CreateAppFeedCardResponse;
    impl ApiResponseTrait for CreateAppFeedCardResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
}
pub mod groups_bots {
    use serde::{Deserialize, Serialize};
    use crate::api_resp::{ApiResponseTrait, ResponseFormat};
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ButtonInfo;
    pub type UserIdType = String;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct CreateBotResponse;
    impl ApiResponseTrait for CreateBotResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
}
pub mod models {
    // 简单的模型定义
    pub struct MessageCard;
}
}}}}