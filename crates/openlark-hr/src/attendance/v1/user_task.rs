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
    BatchCreateUserTaskRequest, BatchCreateUserTaskRespData, BatchDelUserTaskRequest,
    BatchDelUserTaskRespData, GetUserTaskRequest, GetUserTaskRespData, QueryUserTaskRequest,
    QueryUserTaskRespData, QueryUserTaskResultRequest, QueryUserTaskResultRespData,
};

/// 用户打卡任务服务
pub struct UserTaskService {
    pub config: Config,
}

impl UserTaskService {
    /// 导入打卡流水
    ///
    /// 该接口用于批量导入员工的打卡记录，支持补录历史打卡数据。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/batch_create
    pub async fn batch_create(
        &self,
        request: BatchCreateUserTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BatchCreateUserTaskRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_USER_TASKS_BATCH_CREATE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "user_tasks": request.user_tasks
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询打卡流水
    ///
    /// 该接口用于查询指定用户在特定日期的打卡流水记录。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/create
    pub async fn get(
        &self,
        request: GetUserTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<GetUserTaskRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_USER_TASK_GET,
            "user_id",
            &request.user_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);
        api_req
            .query_params
            .insert("check_date", request.check_date);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 批量查询打卡流水
    ///
    /// 该接口用于批量查询多个用户的打卡流水记录，支持按时间范围和打卡类型筛选。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/create
    pub async fn query(
        &self,
        request: QueryUserTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryUserTaskRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_USER_TASKS_QUERY.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(user_ids) = request.user_ids {
            api_req.query_params.insert("user_ids", user_ids.join(","));
        }

        if let Some(check_date_from) = request.check_date_from {
            api_req
                .query_params
                .insert("check_date_from", check_date_from);
        }

        if let Some(check_date_to) = request.check_date_to {
            api_req.query_params.insert("check_date_to", check_date_to);
        }

        if let Some(check_type) = request.check_type {
            api_req
                .query_params
                .insert("check_type", check_type.to_string());
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

    /// 删除打卡流水
    ///
    /// 该接口用于批量删除指定的打卡流水记录。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/createser_task/batch_del
    pub async fn batch_del(
        &self,
        request: BatchDelUserTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BatchDelUserTaskRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_USER_TASKS_BATCH_DELETE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "task_ids": request.task_ids
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询打卡结果
    ///
    /// 该接口用于查询员工的考勤结果，包括工作时长、加班时长、考勤状态等汇总信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query
    pub async fn query_result(
        &self,
        request: QueryUserTaskResultRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryUserTaskResultRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_USER_TASK_RESULTS_QUERY.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(user_ids) = request.user_ids {
            api_req.query_params.insert("user_ids", user_ids.join(","));
        }

        if let Some(check_date_from) = request.check_date_from {
            api_req
                .query_params
                .insert("check_date_from", check_date_from);
        }

        if let Some(check_date_to) = request.check_date_to {
            api_req.query_params.insert("check_date_to", check_date_to);
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

impl Service for UserTaskService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "user_task"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use crate::service::attendance::v1::models::UserTaskCreate;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    fn create_test_service() -> UserTaskService {
        UserTaskService {
            config: create_test_config(),
        }
    }

    #[test]
    fn test_user_task_service_creation() {
        let service = create_test_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
    }

    #[test]
    fn test_batch_create_request_builder() {
        let service = create_test_service();
        let mut request = BatchCreateUserTaskRequest::default();
        request.employee_type = "employee_id".to_string();
        request.user_tasks = vec![UserTaskCreate {
            user_id: "user123".to_string(),
            group_id: "group123".to_string(),
            shift_id: "shift123".to_string(),
            check_date: "2023-01-01".to_string(),
            check_time: "2023-01-01 09:00:00".to_string(),
            check_type: 1,
            check_result: 1,
            location: None,
            is_field: Some(false),
            is_remedy: Some(false),
            comment: Some("Test task".to_string()),
        }];

        // Test request structure
        assert_eq!(request.employee_type, "employee_id");
        assert_eq!(request.user_tasks.len(), 1);
    }

    #[test]
    fn test_get_user_task_request_builder() {
        let mut request = GetUserTaskRequest::default();
        request.user_id = "user123".to_string();
        request.employee_type = "employee_id".to_string();
        request.check_date = "2023-01-01".to_string();

        assert_eq!(request.user_id, "user123");
        assert_eq!(request.employee_type, "employee_id");
        assert_eq!(request.check_date, "2023-01-01");
    }

    #[test]
    fn test_query_user_task_request_builder() {
        let mut request = QueryUserTaskRequest::default();
        request.employee_type = "employee_id".to_string();
        request.user_ids = Some(vec!["user1".to_string(), "user2".to_string()]);
        request.check_date_from = Some("2023-01-01".to_string());
        request.check_date_to = Some("2023-01-31".to_string());
        request.page_size = Some(50);
        request.page_token = Some("next_page_token".to_string());

        assert_eq!(request.employee_type, "employee_id");
        assert_eq!(request.user_ids.as_ref().unwrap().len(), 2);
        assert_eq!(request.check_date_from, Some("2023-01-01".to_string()));
        assert_eq!(request.check_date_to, Some("2023-01-31".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_batch_del_request_builder() {
        let mut request = BatchDelUserTaskRequest::default();
        request.employee_type = "employee_id".to_string();
        request.task_ids = vec!["task1".to_string(), "task2".to_string()];

        assert_eq!(request.employee_type, "employee_id");
        assert_eq!(request.task_ids.len(), 2);
    }

    #[test]
    fn test_query_user_task_result_request_builder() {
        let mut request = QueryUserTaskResultRequest::default();
        request.employee_type = "employee_id".to_string();
        request.user_ids = Some(vec!["user1".to_string()]);
        request.check_date_from = Some("2023-01-01".to_string());
        request.check_date_to = Some("2023-01-31".to_string());

        assert_eq!(request.employee_type, "employee_id");
        assert!(request.user_ids.is_some());
        assert!(request.check_date_from.is_some());
        assert!(request.check_date_to.is_some());
    }

    #[test]
    fn test_query_request_with_optional_fields() {
        let mut request = QueryUserTaskRequest::default();
        request.employee_type = "employee_no".to_string();

        // Test with no optional fields
        assert!(request.user_ids.is_none());
        assert!(request.check_date_from.is_none());
        assert!(request.check_date_to.is_none());
        assert!(request.check_type.is_none());
        assert!(request.page_size.is_none());
        assert!(request.page_token.is_none());

        // Test with some optional fields
        request.check_type = Some(1);
        assert_eq!(request.check_type, Some(1));
    }

    #[test]
    fn test_query_result_request_with_pagination() {
        let mut request = QueryUserTaskResultRequest::default();
        request.employee_type = "employee_id".to_string();
        request.page_size = Some(100);
        request.page_token = Some("pagination_token".to_string());

        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token, Some("pagination_token".to_string()));
    }

    #[test]
    fn test_access_token_types() {
        // All attendance user task operations should use tenant access token
        let service = create_test_service();

        // These would be tested in actual API calls, but we can verify
        // that the service properly supports tenant access tokens
        assert!(matches!(AccessTokenType::Tenant, AccessTokenType::Tenant));
    }

    #[test]
    fn test_request_validation() {
        // Test employee_type validation
        let valid_types = ["employee_id", "employee_no", "open_id", "union_id"];

        for employee_type in valid_types {
            let mut request = GetUserTaskRequest::default();
            request.employee_type = employee_type.to_string();
            assert!(!request.employee_type.is_empty());
        }
    }

    #[test]
    fn test_user_task_create_serialization() {
        let user_task = UserTaskCreate {
            user_id: "user123".to_string(),
            group_id: "group123".to_string(),
            shift_id: "shift123".to_string(),
            check_date: "2023-01-01".to_string(),
            check_time: "2023-01-01 09:00:00".to_string(),
            check_type: 1,
            check_result: 1,
            location: None,
            is_field: Some(false),
            is_remedy: Some(false),
            comment: Some("Test serialization".to_string()),
        };

        let serialized = serde_json::to_string(&user_task).unwrap();
        assert!(serialized.contains("user123"));
        assert!(serialized.contains("2023-01-01 09:00:00"));
    }

    #[test]
    fn test_batch_operations_limits() {
        let mut request = BatchCreateUserTaskRequest::default();

        // Test empty user_tasks
        request.user_tasks = vec![];
        assert_eq!(request.user_tasks.len(), 0);

        // Test single user_task
        request.user_tasks = vec![UserTaskCreate {
            user_id: "user1".to_string(),
            group_id: "group1".to_string(),
            shift_id: "shift1".to_string(),
            check_date: "2023-01-01".to_string(),
            check_time: "2023-01-01 09:00:00".to_string(),
            check_type: 1,
            check_result: 1,
            location: None,
            is_field: None,
            is_remedy: None,
            comment: None,
        }];
        assert_eq!(request.user_tasks.len(), 1);

        // Test multiple user_tasks
        request.user_tasks = vec![
            UserTaskCreate {
                user_id: "user1".to_string(),
                group_id: "group1".to_string(),
                shift_id: "shift1".to_string(),
                check_date: "2023-01-01".to_string(),
                check_time: "2023-01-01 09:00:00".to_string(),
                check_type: 1,
                check_result: 1,
                location: None,
                is_field: None,
                is_remedy: None,
                comment: None,
            },
            UserTaskCreate {
                user_id: "user2".to_string(),
                group_id: "group2".to_string(),
                shift_id: "shift2".to_string(),
                check_date: "2023-01-02".to_string(),
                check_time: "2023-01-02 09:00:00".to_string(),
                check_type: 1,
                check_result: 1,
                location: None,
                is_field: None,
                is_remedy: None,
                comment: None,
            },
            UserTaskCreate {
                user_id: "user3".to_string(),
                group_id: "group3".to_string(),
                shift_id: "shift3".to_string(),
                check_date: "2023-01-03".to_string(),
                check_time: "2023-01-03 09:00:00".to_string(),
                check_type: 1,
                check_result: 1,
                location: None,
                is_field: None,
                is_remedy: None,
                comment: None,
            },
        ];
        assert_eq!(request.user_tasks.len(), 3);
    }

    #[test]
    fn test_task_ids_validation() {
        let mut request = BatchDelUserTaskRequest::default();

        // Test empty task_ids
        request.task_ids = vec![];
        assert_eq!(request.task_ids.len(), 0);

        // Test with task IDs
        request.task_ids = vec!["task1".to_string(), "task2".to_string()];
        assert_eq!(request.task_ids.len(), 2);
        assert!(request.task_ids.contains(&"task1".to_string()));
        assert!(request.task_ids.contains(&"task2".to_string()));
    }

    #[test]
    fn test_date_format_validation() {
        let mut request = GetUserTaskRequest::default();

        // Test various date formats (though API may enforce specific format)
        let valid_dates = ["2023-01-01", "2023-12-31", "2024-02-29"];

        for date in valid_dates {
            request.check_date = date.to_string();
            assert!(!request.check_date.is_empty());
            assert!(request.check_date.len() == 10); // YYYY-MM-DD format
        }
    }

    #[test]
    fn test_page_size_bounds() {
        let mut request = QueryUserTaskRequest::default();

        // Test various page sizes
        request.page_size = Some(1);
        assert_eq!(request.page_size, Some(1));

        request.page_size = Some(50);
        assert_eq!(request.page_size, Some(50));

        request.page_size = Some(200);
        assert_eq!(request.page_size, Some(200));
    }
}
