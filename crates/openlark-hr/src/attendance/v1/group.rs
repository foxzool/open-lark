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
    CreateGroupRequest, CreateGroupRespData, DeleteGroupRequest, EmptyResponse, GetGroupRequest,
    Group, ListGroupRequest, ListGroupRespData, ListGroupUserRequest, ListGroupUserRespData,
    SearchGroupRequest, SearchGroupRespData,
};

/// 考勤组服务
pub struct GroupService {
    pub config: Config,
}

impl GroupService {
    /// 查询考勤组下所有成员
    ///
    /// 该接口用于查询考勤组下所有成员的信息，支持分页查询。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/attendance-v1/group/list_user
    pub async fn list_user(
        &self,
        request: ListGroupUserRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ListGroupUserRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_GROUP_USERS,
            "group_id",
            &request.group_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type.clone());

        if let Some(dept_type) = &request.dept_type {
            api_req.query_params.insert("dept_type", dept_type.clone());
        }

        if let Some(page_size) = &request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = &request.page_token {
            api_req
                .query_params
                .insert("page_token", page_token.clone());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 创建或修改考勤组
    ///
    /// 该接口用于创建或修改考勤组，支持设置考勤规则、工作日安排、成员管理等。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/batch_create
    pub async fn create(
        &self,
        request: CreateGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CreateGroupRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_GROUPS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type.clone());

        if let Some(dept_type) = &request.dept_type {
            api_req.query_params.insert("dept_type", dept_type.clone());
        }

        // 构建请求体
        let mut body = json!({
            "group_name": request.group_name.clone()
        });

        if let Some(time_zone) = &request.time_zone {
            body["time_zone"] = json!(time_zone.clone());
        }
        if let Some(bind_dept_ids) = &request.bind_dept_ids {
            body["bind_dept_ids"] = json!(bind_dept_ids.clone());
        }
        if let Some(except_date_rule) = &request.except_date_rule {
            body["except_date_rule"] = json!(except_date_rule.clone());
        }
        if let Some(attendance_type) = &request.attendance_type {
            body["attendance_type"] = json!(attendance_type.clone());
        }
        if let Some(punch_type) = &request.punch_type {
            body["punch_type"] = json!(punch_type.clone());
        }
        if let Some(allow_late_minutes) = &request.allow_late_minutes {
            body["allow_late_minutes"] = json!(allow_late_minutes.clone());
        }
        if let Some(allow_early_leave_minutes) = &request.allow_early_leave_minutes {
            body["allow_early_leave_minutes"] = json!(allow_early_leave_minutes.clone());
        }
        if let Some(work_day_rule) = &request.work_day_rule {
            body["work_day_rule"] = json!(work_day_rule.clone());
        }
        if let Some(shift_rule) = &request.shift_rule {
            body["shift_rule"] = json!(shift_rule.clone());
        }
        if let Some(member_rule) = &request.member_rule {
            body["member_rule"] = json!(member_rule.clone());
        }

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 删除考勤组
    ///
    /// 删除指定的考勤组。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/list_user
    pub async fn delete(
        &self,
        request: DeleteGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<EmptyResponse>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_GROUP_DELETE,
            "group_id",
            &request.group_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 按 ID 查询考勤组
    ///
    /// 根据考勤组 ID 查询考勤组的详细信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/list_user
    pub async fn get(
        &self,
        request: GetGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<Group>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_GROUP_DELETE,
            "group_id",
            &request.group_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(dept_type) = request.dept_type {
            api_req.query_params.insert("dept_type", dept_type);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 按名称查询考勤组
    ///
    /// 根据考勤组名称查询考勤组信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/list_user
    pub async fn search(
        &self,
        request: SearchGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<SearchGroupRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_GROUPS_SEARCH.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(dept_type) = request.dept_type {
            api_req.query_params.insert("dept_type", dept_type);
        }

        // 构建请求体
        let body = json!({
            "group_name": request.group_name
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询所有考勤组
    ///
    /// 查询企业内所有考勤组的信息，支持分页查询。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/list_user
    pub async fn list(
        &self,
        request: ListGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ListGroupRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_GROUPS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(dept_type) = request.dept_type {
            api_req.query_params.insert("dept_type", dept_type);
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

impl Service for GroupService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "attendance_group"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::{api::ApiRequest, config::Config};
    use crate::service::attendance::v1::models::{
        ExceptDateRule, MemberRule, ShiftRule, WorkDayRule,
    };

    #[test]
    fn test_group_service_creation() {
        let config = Config::default();
        let service = GroupService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_group_service_with_custom_config() {
        let config = Config::builder()
            .app_id("group_test_app")
            .app_secret("group_test_secret")
            .build();

        let service = GroupService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, "group_test_app");
        assert_eq!(service.config.app_secret, "group_test_secret");
    }

    #[test]
    fn test_list_group_user_request_construction() {
        let request = ListGroupUserRequest {
            api_req: ApiRequest::default(),
            group_id: "group_123".to_string(),
            employee_type: "1".to_string(),
            dept_type: Some("dept_id".to_string()),
            page_size: Some(50),
            page_token: Some("user_page_token".to_string()),
        };

        assert_eq!(request.group_id, "group_123");
        assert_eq!(request.employee_type, "1");
        assert_eq!(request.dept_type, Some("dept_id".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("user_page_token".to_string()));
    }

    #[test]
    fn test_list_group_user_request_with_none_values() {
        let request = ListGroupUserRequest {
            api_req: ApiRequest::default(),
            group_id: "group_456".to_string(),
            employee_type: "2".to_string(),
            dept_type: None,
            page_size: None,
            page_token: None,
        };

        assert_eq!(request.group_id, "group_456");
        assert_eq!(request.employee_type, "2");
        assert_eq!(request.dept_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_create_group_request_construction() {
        let request = CreateGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: Some("department_id".to_string()),
            group_name: "开发组".to_string(),
            time_zone: Some("Asia/Shanghai".to_string()),
            bind_dept_ids: Some(vec!["dept_1".to_string(), "dept_2".to_string()]),
            except_date_rule: Some(vec![ExceptDateRule {
                date: "2023-12-25".to_string(),
                except_type: 2,
                shift_id: None,
            }]),
            attendance_type: Some(1),
            punch_type: Some(1),
            allow_late_minutes: Some(10),
            allow_early_leave_minutes: Some(5),
            work_day_rule: Some(vec![WorkDayRule {
                week_day: 1,
                shift_id: "shift_001".to_string(),
            }]),
            shift_rule: Some(vec![ShiftRule {
                shift_id: "shift_001".to_string(),
                shift_name: Some("normal".to_string()),
            }]),
            member_rule: Some(MemberRule {
                member_type: 1,
                member_ids: vec!["dept_1".to_string(), "dept_2".to_string()],
            }),
        };

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.dept_type, Some("department_id".to_string()));
        assert_eq!(request.group_name, "开发组");
        assert_eq!(request.time_zone, Some("Asia/Shanghai".to_string()));
        assert_eq!(
            request.bind_dept_ids,
            Some(vec!["dept_1".to_string(), "dept_2".to_string()])
        );
        assert!(request.except_date_rule.is_some());
        assert_eq!(request.attendance_type, Some(1));
        assert_eq!(request.punch_type, Some(1));
        assert_eq!(request.allow_late_minutes, Some(10));
        assert_eq!(request.allow_early_leave_minutes, Some(5));
        assert!(request.work_day_rule.is_some());
        assert!(request.shift_rule.is_some());
        assert!(request.member_rule.is_some());
    }

    #[test]
    fn test_create_group_request_with_minimal_data() {
        let request = CreateGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            dept_type: None,
            group_name: "测试组".to_string(),
            time_zone: None,
            bind_dept_ids: None,
            except_date_rule: None,
            attendance_type: None,
            punch_type: None,
            allow_late_minutes: None,
            allow_early_leave_minutes: None,
            work_day_rule: None,
            shift_rule: None,
            member_rule: None,
        };

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.dept_type, None);
        assert_eq!(request.group_name, "测试组");
        assert_eq!(request.time_zone, None);
        assert_eq!(request.bind_dept_ids, None);
        assert!(request.except_date_rule.is_none());
        assert_eq!(request.attendance_type, None);
        assert_eq!(request.punch_type, None);
        assert_eq!(request.allow_late_minutes, None);
        assert_eq!(request.allow_early_leave_minutes, None);
        assert!(request.work_day_rule.is_none());
        assert!(request.shift_rule.is_none());
        assert!(request.member_rule.is_none());
    }

    #[test]
    fn test_delete_group_request_construction() {
        let request = DeleteGroupRequest {
            api_req: ApiRequest::default(),
            group_id: "group_to_delete".to_string(),
        };

        assert_eq!(request.group_id, "group_to_delete");
    }

    #[test]
    fn test_get_group_request_construction() {
        let request = GetGroupRequest {
            api_req: ApiRequest::default(),
            group_id: "group_to_get".to_string(),
            employee_type: "1".to_string(),
            dept_type: Some("open_id".to_string()),
        };

        assert_eq!(request.group_id, "group_to_get");
        assert_eq!(request.employee_type, "1");
        assert_eq!(request.dept_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_group_request_without_dept_type() {
        let request = GetGroupRequest {
            api_req: ApiRequest::default(),
            group_id: "group_no_dept".to_string(),
            employee_type: "2".to_string(),
            dept_type: None,
        };

        assert_eq!(request.group_id, "group_no_dept");
        assert_eq!(request.employee_type, "2");
        assert_eq!(request.dept_type, None);
    }

    #[test]
    fn test_search_group_request_construction() {
        let request = SearchGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: Some("department_id".to_string()),
            group_name: "研发组".to_string(),
        };

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.dept_type, Some("department_id".to_string()));
        assert_eq!(request.group_name, "研发组");
    }

    #[test]
    fn test_search_group_request_without_dept_type() {
        let request = SearchGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            dept_type: None,
            group_name: "销售组".to_string(),
        };

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.dept_type, None);
        assert_eq!(request.group_name, "销售组");
    }

    #[test]
    fn test_list_group_request_construction() {
        let request = ListGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: Some("union_id".to_string()),
            page_size: Some(100),
            page_token: Some("list_page_token".to_string()),
        };

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.dept_type, Some("union_id".to_string()));
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token, Some("list_page_token".to_string()));
    }

    #[test]
    fn test_list_group_request_with_minimal_data() {
        let request = ListGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            dept_type: None,
            page_size: None,
            page_token: None,
        };

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.dept_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_group_service_config_independence() {
        let config1 = Config::builder().app_id("group_app_1").build();

        let config2 = Config::builder().app_id("group_app_2").build();

        let service1 = GroupService { config: config1 };
        let service2 = GroupService { config: config2 };

        assert_eq!(service1.config.app_id, "group_app_1");
        assert_eq!(service2.config.app_id, "group_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_request_structs_debug_trait() {
        let create_request = CreateGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: Some("debug_dept".to_string()),
            group_name: "Debug Group".to_string(),
            time_zone: Some("UTC".to_string()),
            bind_dept_ids: Some(vec!["debug_dept_1".to_string()]),
            except_date_rule: None,
            attendance_type: Some(1),
            punch_type: None,
            allow_late_minutes: None,
            allow_early_leave_minutes: None,
            work_day_rule: None,
            shift_rule: None,
            member_rule: None,
        };

        let debug_str = format!("{:?}", create_request);
        assert!(debug_str.contains("CreateGroupRequest"));
        assert!(debug_str.contains("Debug Group"));
        assert!(debug_str.contains("debug_dept"));
    }

    #[test]
    fn test_create_group_request_edge_cases() {
        // Test with very long group name
        let long_name = "长考勤组名称".repeat(50);
        let request_long_name = CreateGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: None,
            group_name: long_name.clone(),
            time_zone: None,
            bind_dept_ids: None,
            except_date_rule: None,
            attendance_type: None,
            punch_type: None,
            allow_late_minutes: None,
            allow_early_leave_minutes: None,
            work_day_rule: None,
            shift_rule: None,
            member_rule: None,
        };

        assert_eq!(request_long_name.group_name, long_name);

        // Test with large number of bind department IDs
        let large_dept_list: Vec<String> = (0..1000).map(|i| format!("dept_{}", i)).collect();
        let request_large_depts = CreateGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: None,
            group_name: "Large Depts Group".to_string(),
            time_zone: None,
            bind_dept_ids: Some(large_dept_list.clone()),
            except_date_rule: None,
            attendance_type: None,
            punch_type: None,
            allow_late_minutes: None,
            allow_early_leave_minutes: None,
            work_day_rule: None,
            shift_rule: None,
            member_rule: None,
        };

        assert_eq!(
            request_large_depts.bind_dept_ids.as_ref().unwrap().len(),
            1000
        );
        assert_eq!(
            request_large_depts.bind_dept_ids.as_ref().unwrap()[999],
            "dept_999"
        );

        // Test with extreme late minutes values
        let request_extreme_late = CreateGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: None,
            group_name: "Extreme Late Group".to_string(),
            time_zone: None,
            bind_dept_ids: None,
            except_date_rule: None,
            attendance_type: None,
            punch_type: None,
            allow_late_minutes: Some(99999),
            allow_early_leave_minutes: Some(99999),
            work_day_rule: None,
            shift_rule: None,
            member_rule: None,
        };

        assert_eq!(request_extreme_late.allow_late_minutes, Some(99999));
        assert_eq!(request_extreme_late.allow_early_leave_minutes, Some(99999));
    }

    #[test]
    fn test_list_group_user_request_edge_cases() {
        // Test with very long group ID
        let long_group_id = "group_".repeat(100);
        let request_long_id = ListGroupUserRequest {
            api_req: ApiRequest::default(),
            group_id: long_group_id.clone(),
            employee_type: "1".to_string(),
            dept_type: None,
            page_size: None,
            page_token: None,
        };

        assert_eq!(request_long_id.group_id, long_group_id);

        // Test with very large page size
        let request_large_page = ListGroupUserRequest {
            api_req: ApiRequest::default(),
            group_id: "group_large_page".to_string(),
            employee_type: "1".to_string(),
            dept_type: None,
            page_size: Some(10000),
            page_token: None,
        };

        assert_eq!(request_large_page.page_size, Some(10000));

        // Test with zero page size
        let request_zero_page = ListGroupUserRequest {
            api_req: ApiRequest::default(),
            group_id: "group_zero_page".to_string(),
            employee_type: "1".to_string(),
            dept_type: None,
            page_size: Some(0),
            page_token: None,
        };

        assert_eq!(request_zero_page.page_size, Some(0));
    }

    #[test]
    fn test_search_group_request_edge_cases() {
        // Test with empty group name
        let request_empty_name = SearchGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: None,
            group_name: "".to_string(),
        };

        assert_eq!(request_empty_name.group_name, "");

        // Test with very long group name
        let long_search_name = "搜索组名".repeat(100);
        let request_long_search = SearchGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            dept_type: Some("dept_long".to_string()),
            group_name: long_search_name.clone(),
        };

        assert_eq!(request_long_search.group_name, long_search_name);
    }

    #[test]
    fn test_all_request_structs_with_different_employee_types() {
        // Test all request types with different employee types
        let list_user_request = ListGroupUserRequest {
            api_req: ApiRequest::default(),
            group_id: "test_group".to_string(),
            employee_type: "employee_id".to_string(),
            dept_type: None,
            page_size: None,
            page_token: None,
        };

        let create_request = CreateGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "open_id".to_string(),
            dept_type: None,
            group_name: "Test Group".to_string(),
            time_zone: None,
            bind_dept_ids: None,
            except_date_rule: None,
            attendance_type: None,
            punch_type: None,
            allow_late_minutes: None,
            allow_early_leave_minutes: None,
            work_day_rule: None,
            shift_rule: None,
            member_rule: None,
        };

        let get_request = GetGroupRequest {
            api_req: ApiRequest::default(),
            group_id: "test_get_group".to_string(),
            employee_type: "union_id".to_string(),
            dept_type: None,
        };

        let search_request = SearchGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "user_id".to_string(),
            dept_type: None,
            group_name: "Search Group".to_string(),
        };

        let list_request = ListGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "department_id".to_string(),
            dept_type: None,
            page_size: None,
            page_token: None,
        };

        assert_eq!(list_user_request.employee_type, "employee_id");
        assert_eq!(create_request.employee_type, "open_id");
        assert_eq!(get_request.employee_type, "union_id");
        assert_eq!(search_request.employee_type, "user_id");
        assert_eq!(list_request.employee_type, "department_id");
    }

    #[test]
    fn test_create_group_request_with_empty_bind_dept_ids() {
        let request = CreateGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: None,
            group_name: "Empty Depts Group".to_string(),
            time_zone: None,
            bind_dept_ids: Some(vec![]),
            except_date_rule: None,
            attendance_type: None,
            punch_type: None,
            allow_late_minutes: None,
            allow_early_leave_minutes: None,
            work_day_rule: None,
            shift_rule: None,
            member_rule: None,
        };

        assert!(request.bind_dept_ids.as_ref().unwrap().is_empty());
        assert_eq!(request.group_name, "Empty Depts Group");
    }

    #[test]
    fn test_create_group_request_with_negative_values() {
        let request = CreateGroupRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            dept_type: None,
            group_name: "Negative Values Group".to_string(),
            time_zone: None,
            bind_dept_ids: None,
            except_date_rule: None,
            attendance_type: Some(-1),
            punch_type: Some(-1),
            allow_late_minutes: Some(-10),
            allow_early_leave_minutes: Some(-5),
            work_day_rule: None,
            shift_rule: None,
            member_rule: None,
        };

        assert_eq!(request.attendance_type, Some(-1));
        assert_eq!(request.punch_type, Some(-1));
        assert_eq!(request.allow_late_minutes, Some(-10));
        assert_eq!(request.allow_early_leave_minutes, Some(-5));
    }
}
