use crate::{
    core::{
        api::Response, config::Config, constants::AccessTokenType,
        endpoints::attendance::*, http::Transport, req_option::RequestOption,
        trait_system::Service, SDKResult,
    },
    impl_executable_builder_owned,
};
use reqwest::Method;

use super::models::{GetLeaveEmployExpireRecordRequest, GetLeaveEmployExpireRecordRespData};

/// 休假获取过期发放记录服务
#[derive(Debug)]
pub struct LeaveEmployExpireRecordService {
    pub config: Config,
}

impl LeaveEmployExpireRecordService {
    /// 通过过期时间获取发放记录
    ///
    /// 该接口用于通过过期时间范围查询员工休假发放记录，支持分页查询。
    /// 可以查询指定时间范围内即将过期或已过期的休假发放记录。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/leave_employ_expire_record/get
    pub async fn get(
        &self,
        request: GetLeaveEmployExpireRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<GetLeaveEmployExpireRecordRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_LEAVE_EMPLOY_EXPIRE_RECORDS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);
        api_req
            .query_params
            .insert("start_time", request.start_time.to_string());
        api_req
            .query_params
            .insert("end_time", request.end_time.to_string());

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// Builder implementations
impl_executable_builder_owned!(
    GetLeaveEmployExpireRecordRequest,
    LeaveEmployExpireRecordService,
    GetLeaveEmployExpireRecordRequest,
    Response<GetLeaveEmployExpireRecordRespData>,
    get
);

impl Service for LeaveEmployExpireRecordService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "leave_employ_expire_record"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    #[test]
    fn test_leave_employ_expire_record_service_creation() {
        let config = Config::default();
        let service = LeaveEmployExpireRecordService { config };
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_leave_employ_expire_record_service_with_custom_config() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_secret")
            .build();
        let service = LeaveEmployExpireRecordService { config };
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_debug_implementation() {
        let config = Config::default();
        let service = LeaveEmployExpireRecordService { config };
        let debug_string = format!("{:?}", service);
        assert!(!debug_string.is_empty());
        assert!(debug_string.contains("LeaveEmployExpireRecordService"));
    }

    #[test]
    fn test_service_config_access() {
        let config = Config::builder().app_id("test_config_access").build();
        let service = LeaveEmployExpireRecordService { config };
        assert_eq!(service.config.app_id, "test_config_access");
    }

    #[test]
    fn test_service_config_independence() {
        let config1 = Config::default();
        let config2 = Config::default();
        let service1 = LeaveEmployExpireRecordService { config: config1 };
        let service2 = LeaveEmployExpireRecordService { config: config2 };
        assert!(!format!("{:?}", service1).is_empty());
        assert!(!format!("{:?}", service2).is_empty());
    }
}
