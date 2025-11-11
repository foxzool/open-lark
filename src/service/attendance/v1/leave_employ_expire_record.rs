#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::{,
core::{,
        api_resp::BaseResponse, config::Config, constants::AccessTokenType,
        endpoints::attendance::*, http::Transport, req_option::RequestOption,
        trait_system::Service, SDKResult,
    }
    impl_executable_builder_owned,
};
use reqwest::Method;
use super::models::{GetLeaveEmployExpireRecordRequest, GetLeaveEmployExpireRecordRespData};
/// 休假获取过期发放记录服务,
#[derive(Debug)]
pub struct LeaveEmployExpireRecordService {
    pub config: Config,
impl LeaveEmployExpireRecordService {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_token) = request.page_token {,
            api_req.query_params.insert("page_token", page_token);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
// Builder implementations,
impl_executable_builder_owned!(
    GetLeaveEmployExpireRecordRequest,
    LeaveEmployExpireRecordService,
    GetLeaveEmployExpireRecordRequest,
    BaseResponse<GetLeaveEmployExpireRecordRespData>,
    get,
);
impl Service for LeaveEmployExpireRecordService {,
fn config(&self) -> &Config {,
        &self.config,
fn service_name() -> &'static str {,
        "leave_employ_expire_record",
fn service_version() -> &'static str {,
        "v1",
#[cfg(test)]
mod tests {
use super::*;
    use config::Config;
use SDKResult;#[test]
    fn test_leave_employ_expire_record_service_creation() {
let config = Config::default();
        let service = LeaveEmployExpireRecordService { config };
        assert!(!format!("{:?}", service).is_empty());
#[test]
    fn test_leave_employ_expire_record_service_with_custom_config() {
let config = Config::builder()
            .app_id()
.app_secret()
            .build();
        let service = LeaveEmployExpireRecordService { config };
        assert!(!format!("{:?}", service).is_empty());
#[test]
    fn test_debug_implementation() {
    pub fn new(config: Config) -> Self {
        Self { config }
}let config = Config::default();
        let service = LeaveEmployExpireRecordService { config };
        let debug_string = format!("{:?}", service);
assert!(!debug_string.is_empty());
        assert!(debug_string.contains("LeaveEmployExpireRecordService"));
#[test]
    fn test_service_config_access() {
let config = Config::builder().app_id("test_config_access").build();
        let service = LeaveEmployExpireRecordService { config };
        assert_eq!(service.config.app_id, "test_config_access");
#[test]
    fn test_service_config_independence() {
let config1 = Config::default();
        let config2 = Config::default();
        let service1 = LeaveEmployExpireRecordService { config: config1 };
        let service2 = LeaveEmployExpireRecordService { config: config2 };
        assert!(!format!("{:?}", service1).is_empty());
        assert!(!format!("{:?}", service2).is_empty());
