use reqwest::Method;
use serde_json::json;
use crate::core::{
    api_resp::BaseResponse, config::Config, constants::AccessTokenType, endpoints::attendance::*,
    http::Transport, req_option::RequestOption, trait_system::Service, SDKResult,
};
use super::models::{,
    BatchCreateTempUserDailyShiftRequest, BatchCreateTempUserDailyShiftRespData,
    BatchCreateUserDailyShiftRequest, BatchCreateUserDailyShiftRespData,
    QueryUserDailyShiftRequest, QueryUserDailyShiftRespData,
};
/// 用户排班服务,
#[derive(Debug)]
pub struct UserDailyShiftService {
    pub config: Config,
impl UserDailyShiftService {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl Service for UserDailyShiftService {,
    fn config(&self) -> &Config {,
&self.config,
    }
fn service_name() -> &'static str {,
        "user_daily_shift",
fn service_version() -> &'static str {,
        "v1",
#[cfg(test)]
mod tests {
use super::*;
    use crate::core::config::Config;
#[test]
    fn test_user_daily_shift_service_creation() {
let config = Config::default();
        let service = UserDailyShiftService { config };
        assert!(!format!("{:?}", service).is_empty());
#[test]
    fn test_user_daily_shift_service_with_custom_config() {
let config = Config::builder()
            .app_id()
.app_secret()
            .build();
        let service = UserDailyShiftService { config };
        assert!(!format!("{:?}", service).is_empty());
#[test]
    fn test_debug_implementation() {
    pub fn new(config: Config) -> Self {
        Self { config }
}let config = Config::default();
        let service = UserDailyShiftService { config };
        let debug_string = format!("{:?}", service);
assert!(!debug_string.is_empty());
        assert!(debug_string.contains("UserDailyShiftService"));
#[test]
    fn test_service_config_access() {
let config = Config::builder().app_id("test_config_access").build();
        let service = UserDailyShiftService { config };
        assert_eq!(service.config.app_id, "test_config_access");
#[test]
    fn test_service_config_independence() {
let config1 = Config::default();
        let config2 = Config::default();
        let service1 = UserDailyShiftService { config: config1 };
        let service2 = UserDailyShiftService { config: config2 };
        assert!(!format!("{:?}", service1).is_empty());
        assert!(!format!("{:?}", service2).is_empty());
