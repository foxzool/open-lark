use crate::{
    core::{
        api_resp::BaseResponse, config::Config, constants::AccessTokenType, endpoints::Endpoints,
        http::Transport, req_option::RequestOption, SDKResult,
    },
    impl_executable_builder_owned,
};
use reqwest::Method;

use super::models::{GetLeaveEmployExpireRecordRequest, GetLeaveEmployExpireRecordRespData};

/// 休假获取过期发放记录服务
pub struct LeaveEmployExpireRecordService {
    pub config: Config,
}

impl LeaveEmployExpireRecordService {
    /// 通过过期时间获取发放记录
    ///
    /// 该接口用于通过过期时间范围查询员工休假发放记录，支持分页查询。
    /// 可以查询指定时间范围内即将过期或已过期的休假发放记录。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/leave_employ_expire_record/get>
    pub async fn get(
        &self,
        request: GetLeaveEmployExpireRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetLeaveEmployExpireRecordRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = Endpoints::ATTENDANCE_V1_LEAVE_EMPLOY_EXPIRE_RECORDS.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

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
    BaseResponse<GetLeaveEmployExpireRecordRespData>,
    get
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{api_req::ApiRequest, config::Config};

    #[test]
    fn test_leave_employ_expire_record_service_creation() {
        let config = Config::default();
        let service = LeaveEmployExpireRecordService { config };
        assert_eq!(format!("{:?}", service).len() > 0, true);
    }

    #[test]
    fn test_leave_employ_expire_record_service_with_custom_config() {
        let mut config = Config::default();
        config.app_id = "test_app_id".to_string();
        config.app_secret = "test_secret".to_string();
        let service = LeaveEmployExpireRecordService { config };
        assert_eq!(format!("{:?}", service).len() > 0, true);
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
        let mut config = Config::default();
        config.app_id = "test_config_access".to_string();
        let service = LeaveEmployExpireRecordService { config };
        assert_eq!(service.config.app_id, "test_config_access");
    }

    #[test]
    fn test_service_config_independence() {
        let config1 = Config::default();
        let config2 = Config::default();
        let service1 = LeaveEmployExpireRecordService { config: config1 };
        let service2 = LeaveEmployExpireRecordService { config: config2 };
        assert_eq!(format!("{:?}", service1).len() > 0, true);
        assert_eq!(format!("{:?}", service2).len() > 0, true);
    }
}
