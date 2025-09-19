use reqwest::Method;
use serde_json::json;

use crate::core::{
    api_resp::BaseResponse, config::Config, constants::AccessTokenType, endpoints::Endpoints,
    http::Transport, req_option::RequestOption, SDKResult,
};

use super::models::{
    BatchCreateTempUserDailyShiftRequest, BatchCreateTempUserDailyShiftRespData,
    BatchCreateUserDailyShiftRequest, BatchCreateUserDailyShiftRespData,
    QueryUserDailyShiftRequest, QueryUserDailyShiftRespData,
};

/// 用户排班服务
#[derive(Debug)]
pub struct UserDailyShiftService {
    pub config: Config,
}

impl UserDailyShiftService {
    /// 创建或修改排班表
    ///
    /// 该接口用于批量创建或修改排班表。排班表是用来描述考勤组内人员在某天需要按照哪个班次进行上班。
    /// 目前排班表支持到2099年。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_daily_shift/batch_create>
    pub async fn batch_create(
        &self,
        request: BatchCreateUserDailyShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchCreateUserDailyShiftRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = Endpoints::ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加必需的查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "user_daily_shifts": request.user_daily_shifts
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询排班表
    ///
    /// 该接口用于根据日期和用户 ID 查询排班表。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_daily_shift/query>
    pub async fn query(
        &self,
        request: QueryUserDailyShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryUserDailyShiftRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = Endpoints::ATTENDANCE_V1_USER_DAILY_SHIFTS_QUERY.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加必需的查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "user_ids": request.user_ids,
            "check_date_from": request.check_date_from,
            "check_date_to": request.check_date_to
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 创建或修改临时排班
    ///
    /// 该接口用于批量创建或修改临时排班。临时排班是指在特定日期临时安排的班次，
    /// 优先级高于正常排班。
    ///
    /// <https://open.feishu.cn/document/attendance-v1/user_daily_shift/batch_create_temp>
    pub async fn batch_create_temp(
        &self,
        request: BatchCreateTempUserDailyShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchCreateTempUserDailyShiftRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = Endpoints::ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE_TEMP.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加必需的查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "user_daily_shifts": request.user_daily_shifts
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    #[test]
    fn test_user_daily_shift_service_creation() {
        let config = Config::default();
        let service = UserDailyShiftService { config };
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_user_daily_shift_service_with_custom_config() {
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_secret".to_string(),
            ..Default::default()
        };
        let service = UserDailyShiftService { config };
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_debug_implementation() {
        let config = Config::default();
        let service = UserDailyShiftService { config };
        let debug_string = format!("{:?}", service);
        assert!(!debug_string.is_empty());
        assert!(debug_string.contains("UserDailyShiftService"));
    }

    #[test]
    fn test_service_config_access() {
        let config = Config {
            app_id: "test_config_access".to_string(),
            ..Default::default()
        };
        let service = UserDailyShiftService { config };
        assert_eq!(service.config.app_id, "test_config_access");
    }

    #[test]
    fn test_service_config_independence() {
        let config1 = Config::default();
        let config2 = Config::default();
        let service1 = UserDailyShiftService { config: config1 };
        let service2 = UserDailyShiftService { config: config2 };
        assert!(!format!("{:?}", service1).is_empty());
        assert!(!format!("{:?}", service2).is_empty());
    }
}
