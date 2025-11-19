use reqwest::Method;
use serde_json::json;

use openlark_core::{
    api::Response,
    config::Config,
    constants::AccessTokenType,
    endpoints::{attendance::*, EndpointBuilder},
    http::Transport,
    req_option::RequestOption,
    trait_system::Service,
    SDKResult,
};

use super::models::{
    CreateUserApprovalRequest, CreateUserApprovalRespData, ProcessUserApprovalRequest,
    ProcessUserApprovalRespData, QueryUserApprovalRequest, QueryUserApprovalRespData,
};

/// 用户审批服务
pub struct UserApprovalService {
    pub config: Config,
}

impl UserApprovalService {
    /// 获取审批数据
    ///
    /// 该接口用于查询假勤审批数据，支持按状态、时间范围、用户等条件筛选。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query
    pub async fn query(
        &self,
        request: QueryUserApprovalRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryUserApprovalRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_USER_APPROVALS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status.to_string());
        }

        if let Some(date_from) = request.date_from {
            api_req.query_params.insert("date_from", date_from);
        }

        if let Some(date_to) = request.date_to {
            api_req.query_params.insert("date_to", date_to);
        }

        if let Some(user_ids) = request.user_ids {
            api_req.query_params.insert("user_ids", user_ids.join(","));
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

    /// 写入审批结果
    ///
    /// 该接口用于写入假勤审批的处理结果，包括通过或拒绝审批。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/batch_create
    pub async fn create(
        &self,
        request: CreateUserApprovalRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CreateUserApprovalRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_USER_APPROVALS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let mut body = json!({
            "approval_id": request.approval_id,
            "status": request.status
        });

        if let Some(approval_note) = request.approval_note {
            body["approval_note"] = json!(approval_note);
        }

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 通知审批状态更新
    ///
    /// 该接口用于通知假勤审批状态的更新，支持审批通过、拒绝、撤回等操作。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_approval/query
    pub async fn process(
        &self,
        request: ProcessUserApprovalRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ProcessUserApprovalRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_USER_APPROVAL_PROCESS,
            "approval_id",
            &request.approval_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let mut body = json!({
            "action": request.action
        });

        if let Some(message) = request.message {
            body["message"] = json!(message);
        }

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

impl Service for UserApprovalService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "user_approval"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::{api::ApiRequest, config::Config};

    #[test]
    fn test_user_approval_service_creation() {
        let config = Config::default();
        let service = UserApprovalService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_user_approval_service_with_custom_config() {
        let config = Config::builder()
            .app_id("approval_test_app")
            .app_secret("approval_test_secret")
            .build();

        let service = UserApprovalService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, "approval_test_app");
        assert_eq!(service.config.app_secret, "approval_test_secret");
    }

    #[test]
    fn test_query_user_approval_request_construction() {
        let request = QueryUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            status: Some(1),
            date_from: Some("2023-10-01".to_string()),
            date_to: Some("2023-10-31".to_string()),
            user_ids: Some(vec![
                "user_1".to_string(),
                "user_2".to_string(),
                "user_3".to_string(),
            ]),
            page_size: Some(50),
            page_token: Some("page_token_123".to_string()),
        };

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.status, Some(1));
        assert_eq!(request.date_from, Some("2023-10-01".to_string()));
        assert_eq!(request.date_to, Some("2023-10-31".to_string()));
        assert_eq!(
            request.user_ids,
            Some(vec![
                "user_1".to_string(),
                "user_2".to_string(),
                "user_3".to_string()
            ])
        );
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
    }

    #[test]
    fn test_query_user_approval_request_with_minimal_data() {
        let request = QueryUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            status: None,
            date_from: None,
            date_to: None,
            user_ids: None,
            page_size: None,
            page_token: None,
        };

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.status, None);
        assert_eq!(request.date_from, None);
        assert_eq!(request.date_to, None);
        assert_eq!(request.user_ids, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_create_user_approval_request_construction() {
        let request = CreateUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            approval_id: "approval_123".to_string(),
            status: 1,
            approval_note: Some("审批通过".to_string()),
        };

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.approval_id, "approval_123");
        assert_eq!(request.status, 1);
        assert_eq!(request.approval_note, Some("审批通过".to_string()));
    }

    #[test]
    fn test_create_user_approval_request_without_note() {
        let request = CreateUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            approval_id: "approval_456".to_string(),
            status: 0,
            approval_note: None,
        };

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.approval_id, "approval_456");
        assert_eq!(request.status, 0);
        assert_eq!(request.approval_note, None);
    }

    #[test]
    fn test_process_user_approval_request_construction() {
        let request = ProcessUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            approval_id: "approval_789".to_string(),
            action: 1, // 审批通过
            message: Some("同意该申请".to_string()),
        };

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.approval_id, "approval_789");
        assert_eq!(request.action, 1);
        assert_eq!(request.message, Some("同意该申请".to_string()));
    }

    #[test]
    fn test_process_user_approval_request_without_message() {
        let request = ProcessUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            approval_id: "approval_101".to_string(),
            action: 2, // 审批拒绝
            message: None,
        };

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.approval_id, "approval_101");
        assert_eq!(request.action, 2);
        assert_eq!(request.message, None);
    }

    #[test]
    fn test_query_user_approval_request_with_single_user() {
        let request = QueryUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            status: Some(2),
            date_from: Some("2023-11-01".to_string()),
            date_to: Some("2023-11-01".to_string()),
            user_ids: Some(vec!["single_user".to_string()]),
            page_size: Some(10),
            page_token: None,
        };

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.status, Some(2));
        assert_eq!(request.user_ids.as_ref().unwrap().len(), 1);
        assert_eq!(request.user_ids.as_ref().unwrap()[0], "single_user");
        assert_eq!(request.page_size, Some(10));
    }

    #[test]
    fn test_user_approval_service_config_independence() {
        let config1 = Config::builder().app_id("approval_app_1").build();

        let config2 = Config::builder().app_id("approval_app_2").build();

        let service1 = UserApprovalService { config: config1 };
        let service2 = UserApprovalService { config: config2 };

        assert_eq!(service1.config.app_id, "approval_app_1");
        assert_eq!(service2.config.app_id, "approval_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_request_structs_debug_trait() {
        let query_request = QueryUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            status: Some(1),
            date_from: Some("2023-10-01".to_string()),
            date_to: Some("2023-10-31".to_string()),
            user_ids: Some(vec!["debug_user".to_string()]),
            page_size: Some(20),
            page_token: Some("debug_token".to_string()),
        };

        let debug_str = format!("{:?}", query_request);
        assert!(debug_str.contains("QueryUserApprovalRequest"));
        assert!(debug_str.contains("debug_user"));
        assert!(debug_str.contains("debug_token"));
    }

    #[test]
    fn test_query_user_approval_request_edge_cases() {
        // Test with very large user ID list
        let large_user_list: Vec<String> = (0..1000).map(|i| format!("user_{}", i)).collect();
        let request_large = QueryUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            status: Some(1),
            date_from: None,
            date_to: None,
            user_ids: Some(large_user_list.clone()),
            page_size: Some(1000),
            page_token: None,
        };

        assert_eq!(request_large.user_ids.as_ref().unwrap().len(), 1000);
        assert_eq!(request_large.user_ids.as_ref().unwrap()[999], "user_999");

        // Test with zero page size
        let request_zero_page = QueryUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            status: Some(0),
            date_from: None,
            date_to: None,
            user_ids: None,
            page_size: Some(0),
            page_token: None,
        };

        assert_eq!(request_zero_page.page_size, Some(0));

        // Test with negative status
        let request_negative_status = QueryUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            status: Some(-1),
            date_from: None,
            date_to: None,
            user_ids: None,
            page_size: None,
            page_token: None,
        };

        assert_eq!(request_negative_status.status, Some(-1));
    }

    #[test]
    fn test_create_user_approval_request_edge_cases() {
        // Test with very long approval note
        let long_note = "审批备注".repeat(100);
        let request_long_note = CreateUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            approval_id: "approval_long_note".to_string(),
            status: 1,
            approval_note: Some(long_note.clone()),
        };

        assert_eq!(request_long_note.approval_note, Some(long_note));

        // Test with very long approval ID
        let long_approval_id = "approval_".repeat(100);
        let request_long_id = CreateUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            approval_id: long_approval_id.clone(),
            status: 0,
            approval_note: None,
        };

        assert_eq!(request_long_id.approval_id, long_approval_id);

        // Test with extreme status values
        let request_extreme_status = CreateUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            approval_id: "approval_extreme".to_string(),
            status: 999,
            approval_note: Some("极端状态测试".to_string()),
        };

        assert_eq!(request_extreme_status.status, 999);
    }

    #[test]
    fn test_process_user_approval_request_edge_cases() {
        // Test with very long message
        let long_message = "处理消息".repeat(200);
        let request_long_message = ProcessUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            approval_id: "approval_long_msg".to_string(),
            action: 1, // 审批通过
            message: Some(long_message.clone()),
        };

        assert_eq!(request_long_message.message, Some(long_message));

        // Test with different action types
        let actions = vec![
            (1, "approve", "审批通过"),
            (2, "reject", "审批拒绝"),
            (3, "withdraw", "撤回申请"),
        ];
        for (action_code, action_name, action_desc) in actions {
            let request = ProcessUserApprovalRequest {
                api_req: ApiRequest::default(),
                employee_type: "1".to_string(),
                approval_id: format!("approval_{}", action_name),
                action: action_code,
                message: Some(format!("{}操作", action_desc)),
            };

            assert_eq!(request.action, action_code);
            assert_eq!(request.message, Some(format!("{}操作", action_desc)));
        }

        // Test with invalid action
        let request_invalid_action = ProcessUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            approval_id: "approval_invalid_action".to_string(),
            action: 0, // 无效的动作类型
            message: None,
        };

        assert_eq!(request_invalid_action.action, 0);
    }

    #[test]
    fn test_query_user_approval_request_with_empty_user_list() {
        let request = QueryUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            status: Some(1),
            date_from: Some("2023-10-01".to_string()),
            date_to: Some("2023-10-31".to_string()),
            user_ids: Some(vec![]),
            page_size: Some(20),
            page_token: Some("empty_users_token".to_string()),
        };

        assert!(request.user_ids.as_ref().unwrap().is_empty());
        assert_eq!(request.employee_type, "1");
        assert_eq!(request.page_size, Some(20));
    }

    #[test]
    fn test_all_request_structs_with_different_employee_types() {
        // Test all request types with different employee types
        let query_request = QueryUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "employee_id".to_string(),
            status: Some(1),
            date_from: None,
            date_to: None,
            user_ids: None,
            page_size: None,
            page_token: None,
        };

        let create_request = CreateUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "open_id".to_string(),
            approval_id: "test_approval".to_string(),
            status: 1,
            approval_note: None,
        };

        let process_request = ProcessUserApprovalRequest {
            api_req: ApiRequest::default(),
            employee_type: "union_id".to_string(),
            approval_id: "test_process".to_string(),
            action: 1, // 审批通过
            message: None,
        };

        assert_eq!(query_request.employee_type, "employee_id");
        assert_eq!(create_request.employee_type, "open_id");
        assert_eq!(process_request.employee_type, "union_id");
    }
}
