use reqwest::Method;
use serde_json::json;

use openlark_core::{
    impl_executable_builder_owned,
    openlark_core::{
        api::Response, config::Config, constants::AccessTokenType,
        endpoints::attendance::*, http::Transport, req_option::RequestOption,
        trait_system::Service, SDKResult,
    },
},

use super::models::{
    CreateUserTaskRemedyRequest, CreateUserTaskRemedyRespData, QueryUserAllowedRemedysRequest,
    QueryUserAllowedRemedysRespData, QueryUserTaskRemedyRequest, QueryUserTaskRemedyRespData,
},

/// 用户补卡服务
pub struct UserTaskRemedyService {
    pub config: Config,
}

impl UserTaskRemedyService {
    /// 通知补卡审批发起
    ///
    /// 该接口用于提交员工的补卡申请，启动审批流程。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/batch_create
    pub async fn create(
        &self,
        request: CreateUserTaskRemedyRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CreateUserTaskRemedyRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_USER_TASK_REMEDYS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "user_id": request.remedy_application.user_id,
            "remedy_date": request.remedy_application.remedy_date,
            "remedy_time": request.remedy_application.remedy_time,
            "remedy_type": request.remedy_application.remedy_type,
            "reason": request.remedy_application.reason,
            "comment": request.remedy_application.comment
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取可补卡时间
    ///
    /// 该接口用于查询指定用户在特定时间范围内的可补卡时间段。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/create
    pub async fn query_user_allowed_remedys(
        &self,
        request: QueryUserAllowedRemedysRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryUserAllowedRemedysRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_USER_TASK_REMEDYS_QUERY_USER_ALLOWED_REMEDYS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);
        api_req.query_params.insert("user_id", request.user_id);

        if let Some(date_from) = request.date_from {
            api_req.query_params.insert("date_from", date_from);
        }

        if let Some(date_to) = request.date_to {
            api_req.query_params.insert("date_to", date_to);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取补卡记录
    ///
    /// 该接口用于查询员工的补卡申请记录，支持按状态、时间范围等条件筛选。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query
    pub async fn query(
        &self,
        request: QueryUserTaskRemedyRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryUserTaskRemedyRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_USER_TASK_REMEDYS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(user_ids) = request.user_ids {
            api_req.query_params.insert("user_ids", user_ids.join(","));
        }

        if let Some(date_from) = request.date_from {
            api_req.query_params.insert("date_from", date_from);
        }

        if let Some(date_to) = request.date_to {
            api_req.query_params.insert("date_to", date_to);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status.to_string());
        }

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

impl Service for UserTaskRemedyService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "user_task_remedy"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

// Builder implementations
impl_executable_builder_owned!(
    CreateUserTaskRemedyRequest,
    UserTaskRemedyService,
    CreateUserTaskRemedyRequest,
    Response<CreateUserTaskRemedyRespData>,
    create
);

impl_executable_builder_owned!(
    QueryUserAllowedRemedysRequest,
    UserTaskRemedyService,
    QueryUserAllowedRemedysRequest,
    Response<QueryUserAllowedRemedysRespData>,
    query_user_allowed_remedys
);

impl_executable_builder_owned!(
    QueryUserTaskRemedyRequest,
    UserTaskRemedyService,
    QueryUserTaskRemedyRequest,
    Response<QueryUserTaskRemedyRespData>,
    query
);

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::{api::ApiRequest, config::Config};
    use crate::attendance::v1::models::UserTaskRemedyApplication;

    #[test]
    fn test_user_task_remedy_service_creation() {
        let config = Config::default();
        let service = UserTaskRemedyService {
            config: config.clone(),
        },

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_user_task_remedy_service_with_custom_config() {
        let config = Config::builder()
            .app_id("remedy_test_app")
            .app_secret("remedy_test_secret")
            .build();

        let service = UserTaskRemedyService {
            config: config.clone(),
        },

        assert_eq!(service.config.app_id, "remedy_test_app");
        assert_eq!(service.config.app_secret, "remedy_test_secret");
    }

    #[test]
    fn test_create_user_task_remedy_request_construction() {
        let remedy_application = UserTaskRemedyApplication {
            user_id: "user_123".to_string(),
            remedy_date: "2023-10-01".to_string(),
            remedy_time: "09:00".to_string(),
            remedy_type: 1,
            reason: "忘记打卡".to_string(),
            comment: Some("早上忘记打卡".to_string()),
        },

        let request = CreateUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            remedy_application,
        },

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.remedy_application.user_id, "user_123");
        assert_eq!(request.remedy_application.remedy_date, "2023-10-01");
        assert_eq!(request.remedy_application.remedy_time, "09:00");
        assert_eq!(request.remedy_application.remedy_type, 1);
        assert_eq!(request.remedy_application.reason, "忘记打卡");
        assert_eq!(
            request.remedy_application.comment,
            Some("早上忘记打卡".to_string())
        );
    }

    #[test]
    fn test_create_user_task_remedy_request_without_comment() {
        let remedy_application = UserTaskRemedyApplication {
            user_id: "user_456".to_string(),
            remedy_date: "2023-10-02".to_string(),
            remedy_time: "18:00".to_string(),
            remedy_type: 2,
            reason: "设备故障".to_string(),
            comment: None,
        },

        let request = CreateUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            remedy_application,
        },

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.remedy_application.user_id, "user_456");
        assert_eq!(request.remedy_application.remedy_date, "2023-10-02");
        assert_eq!(request.remedy_application.remedy_time, "18:00");
        assert_eq!(request.remedy_application.remedy_type, 2);
        assert_eq!(request.remedy_application.reason, "设备故障");
        assert_eq!(request.remedy_application.comment, None);
    }

    #[test]
    fn test_query_user_allowed_remedys_request_construction() {
        let request = QueryUserAllowedRemedysRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            user_id: "user_789".to_string(),
            date_from: Some("2023-10-01".to_string()),
            date_to: Some("2023-10-31".to_string()),
        },

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.user_id, "user_789");
        assert_eq!(request.date_from, Some("2023-10-01".to_string()));
        assert_eq!(request.date_to, Some("2023-10-31".to_string()));
    }

    #[test]
    fn test_query_user_allowed_remedys_request_without_dates() {
        let request = QueryUserAllowedRemedysRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            user_id: "user_101".to_string(),
            date_from: None,
            date_to: None,
        },

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.user_id, "user_101");
        assert_eq!(request.date_from, None);
        assert_eq!(request.date_to, None);
    }

    #[test]
    fn test_query_user_task_remedy_request_construction() {
        let request = QueryUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            user_ids: Some(vec![
                "user_1".to_string(),
                "user_2".to_string(),
                "user_3".to_string(),
            ]),
            date_from: Some("2023-10-01".to_string()),
            date_to: Some("2023-10-31".to_string()),
            status: Some(1),
            page_size: Some(50),
            page_token: Some("page_token_123".to_string()),
        },

        assert_eq!(request.employee_type, "1");
        assert_eq!(
            request.user_ids,
            Some(vec![
                "user_1".to_string(),
                "user_2".to_string(),
                "user_3".to_string()
            ])
        );
        assert_eq!(request.date_from, Some("2023-10-01".to_string()));
        assert_eq!(request.date_to, Some("2023-10-31".to_string()));
        assert_eq!(request.status, Some(1));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
    }

    #[test]
    fn test_query_user_task_remedy_request_with_minimal_data() {
        let request = QueryUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            user_ids: None,
            date_from: None,
            date_to: None,
            status: None,
            page_size: None,
            page_token: None,
        },

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.user_ids, None);
        assert_eq!(request.date_from, None);
        assert_eq!(request.date_to, None);
        assert_eq!(request.status, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_query_user_task_remedy_request_with_single_user() {
        let request = QueryUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            user_ids: Some(vec!["single_user".to_string()]),
            date_from: Some("2023-10-15".to_string()),
            date_to: Some("2023-10-15".to_string()),
            status: Some(0),
            page_size: Some(10),
            page_token: None,
        },

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.user_ids.as_ref().unwrap().len(), 1);
        assert_eq!(request.user_ids.as_ref().unwrap()[0], "single_user");
        assert_eq!(request.date_from, Some("2023-10-15".to_string()));
        assert_eq!(request.date_to, Some("2023-10-15".to_string()));
        assert_eq!(request.status, Some(0));
        assert_eq!(request.page_size, Some(10));
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_user_task_remedy_application_construction() {
        let application = UserTaskRemedyApplication {
            user_id: "app_user_123".to_string(),
            remedy_date: "2023-12-01".to_string(),
            remedy_time: "14:30".to_string(),
            remedy_type: 3,
            reason: "外出办事".to_string(),
            comment: Some("客户拜访".to_string()),
        },

        assert_eq!(application.user_id, "app_user_123");
        assert_eq!(application.remedy_date, "2023-12-01");
        assert_eq!(application.remedy_time, "14:30");
        assert_eq!(application.remedy_type, 3);
        assert_eq!(application.reason, "外出办事");
        assert_eq!(application.comment, Some("客户拜访".to_string()));
    }

    #[test]
    fn test_user_task_remedy_service_config_independence() {
        let config1 = Config::builder().app_id("remedy_app_1").build();

        let config2 = Config::builder().app_id("remedy_app_2").build();

        let service1 = UserTaskRemedyService { config: config1 };
        let service2 = UserTaskRemedyService { config: config2 },

        assert_eq!(service1.config.app_id, "remedy_app_1");
        assert_eq!(service2.config.app_id, "remedy_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_request_structs_debug_trait() {
        let remedy_application = UserTaskRemedyApplication {
            user_id: "debug_user".to_string(),
            remedy_date: "2023-11-01".to_string(),
            remedy_time: "10:00".to_string(),
            remedy_type: 1,
            reason: "Debug test".to_string(),
            comment: None,
        },

        let create_request = CreateUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            remedy_application,
        },

        let debug_str = format!("{:?}", create_request);
        assert!(debug_str.contains("CreateUserTaskRemedyRequest"));
        assert!(debug_str.contains("debug_user"));
        assert!(debug_str.contains("Debug test"));
    }

    #[test]
    fn test_query_user_allowed_remedys_request_edge_cases() {
        // Test with very long user ID
        let long_user_id = "a".repeat(500);
        let request_long_id = QueryUserAllowedRemedysRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            user_id: long_user_id.clone(),
            date_from: None,
            date_to: None,
        },

        assert_eq!(request_long_id.user_id, long_user_id);

        // Test with different date formats (edge case)
        let request_dates = QueryUserAllowedRemedysRequest {
            api_req: ApiRequest::default(),
            employee_type: "3".to_string(),
            user_id: "user_dates".to_string(),
            date_from: Some("2023-01-01".to_string()),
            date_to: Some("2023-12-31".to_string()),
        },

        assert_eq!(request_dates.date_from, Some("2023-01-01".to_string()));
        assert_eq!(request_dates.date_to, Some("2023-12-31".to_string()));
    }

    #[test]
    fn test_query_user_task_remedy_request_edge_cases() {
        // Test with very large user ID list
        let large_user_list: Vec<String> = (0..1000).map(|i| format!("user_{}", i)).collect();
        let request_large = QueryUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            user_ids: Some(large_user_list.clone()),
            date_from: None,
            date_to: None,
            status: None,
            page_size: Some(1000),
            page_token: None,
        },

        assert_eq!(request_large.user_ids.as_ref().unwrap().len(), 1000);
        assert_eq!(request_large.user_ids.as_ref().unwrap()[999], "user_999");
        assert_eq!(request_large.page_size, Some(1000));

        // Test with zero page size
        let request_zero_page = QueryUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            user_ids: None,
            date_from: None,
            date_to: None,
            status: Some(2),
            page_size: Some(0),
            page_token: None,
        },

        assert_eq!(request_zero_page.page_size, Some(0));
        assert_eq!(request_zero_page.status, Some(2));

        // Test with negative status (edge case)
        let request_negative_status = QueryUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            user_ids: None,
            date_from: None,
            date_to: None,
            status: Some(-1),
            page_size: None,
            page_token: None,
        },

        assert_eq!(request_negative_status.status, Some(-1));
    }

    #[test]
    fn test_user_task_remedy_application_edge_cases() {
        // Test with very long strings
        let long_reason = "长原因描述".repeat(100);
        let long_comment = "长备注内容".repeat(200);

        let application_long = UserTaskRemedyApplication {
            user_id: "edge_user".to_string(),
            remedy_date: "2023-12-31".to_string(),
            remedy_time: "23:59".to_string(),
            remedy_type: 999,
            reason: long_reason.clone(),
            comment: Some(long_comment.clone()),
        },

        assert_eq!(application_long.reason, long_reason);
        assert_eq!(application_long.comment, Some(long_comment));
        assert_eq!(application_long.remedy_type, 999);

        // Test with minimum values
        let application_min = UserTaskRemedyApplication {
            user_id: "".to_string(),
            remedy_date: "".to_string(),
            remedy_time: "".to_string(),
            remedy_type: 0,
            reason: "".to_string(),
            comment: None,
        },

        assert_eq!(application_min.user_id, "");
        assert_eq!(application_min.remedy_date, "");
        assert_eq!(application_min.remedy_time, "");
        assert_eq!(application_min.remedy_type, 0);
        assert_eq!(application_min.reason, "");
        assert_eq!(application_min.comment, None);
    }

    #[test]
    fn test_query_user_task_remedy_request_with_empty_user_list() {
        let request = QueryUserTaskRemedyRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            user_ids: Some(vec![]),
            date_from: Some("2023-10-01".to_string()),
            date_to: Some("2023-10-31".to_string()),
            status: Some(1),
            page_size: Some(20),
            page_token: Some("empty_token".to_string()),
        },

        assert!(request.user_ids.as_ref().unwrap().is_empty());
        assert_eq!(request.employee_type, "1");
        assert_eq!(request.page_size, Some(20));
    }
}
