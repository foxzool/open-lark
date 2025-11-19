use reqwest::Method;
use serde_json::json;

use crate::{
    core::{
        api::ApiRequest,
        api::Response,
        config::Config,
        constants::AccessTokenType,
        endpoints::{attendance::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::models::{
    CreateShiftRequest, CreateShiftRespData, DeleteShiftRequest, EmptyResponse, GetShiftRequest,
    ListShiftRequest, QueryShiftRequest, Shift, ShiftListData,
};

pub struct ShiftService {
    pub config: Config,
}

impl ShiftService {
    /// 创建班次
    ///
    /// 创建一个班次，班次是管理考勤打卡规则的载体
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/create
    pub async fn create(
        &self,
        request: CreateShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CreateShiftRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_SHIFTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加必需的查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let mut body = json!({
            "shift_name": request.shift_name,
            "punch_times": request.punch_times,
        });

        if let Some(is_flexible) = request.is_flexible {
            body["is_flexible"] = json!(is_flexible);
        }
        if let Some(flexible_minutes) = request.flexible_minutes {
            body["flexible_minutes"] = json!(flexible_minutes);
        }
        if let Some(flexible_rule) = &request.flexible_rule {
            body["flexible_rule"] = json!(flexible_rule);
        }
        if let Some(no_need_off) = request.no_need_off {
            body["no_need_off"] = json!(no_need_off);
        }
        if let Some(punch_time_rule) = &request.punch_time_rule {
            body["punch_time_rule"] = json!(punch_time_rule);
        }
        if let Some(late_minutes_as_late) = request.late_minutes_as_late {
            body["late_minutes_as_late"] = json!(late_minutes_as_late);
        }
        if let Some(late_minutes_as_lack) = request.late_minutes_as_lack {
            body["late_minutes_as_lack"] = json!(late_minutes_as_lack);
        }
        if let Some(early_minutes_as_early) = request.early_minutes_as_early {
            body["early_minutes_as_early"] = json!(early_minutes_as_early);
        }
        if let Some(early_minutes_as_lack) = request.early_minutes_as_lack {
            body["early_minutes_as_lack"] = json!(early_minutes_as_lack);
        }
        if let Some(allow_outside_apply) = request.allow_outside_apply {
            body["allow_outside_apply"] = json!(allow_outside_apply);
        }
        if let Some(outside_apply_limit) = request.outside_apply_limit {
            body["outside_apply_limit"] = json!(outside_apply_limit);
        }
        if let Some(allow_face_punch) = request.allow_face_punch {
            body["allow_face_punch"] = json!(allow_face_punch);
        }
        if let Some(face_punch_cfg) = &request.face_punch_cfg {
            body["face_punch_cfg"] = json!(face_punch_cfg);
        }

        api_req.body = serde_json::to_vec(&body)?;

        // 调试日志：打印API请求内容
        log::debug!("创建班次API请求详情:");
        log::debug!("  路径: {}", api_req.api_path);
        log::debug!("  方法: {:?}", api_req.http_method);
        log::debug!("  查询参数: {:?}", api_req.query_params);
        log::debug!(
            "  请求体: {}",
            serde_json::to_string_pretty(&body).unwrap_or_else(|_| "无法序列化".to_string())
        );

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 删除班次
    ///
    /// 删除一个班次
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/delete
    pub async fn delete(
        &self,
        request: DeleteShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<EmptyResponse>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_SHIFT_DELETE,
            "shift_id",
            &request.shift_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 按 ID 查询班次
    ///
    /// 通过班次的 ID 获取班次详情
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/get
    pub async fn get(
        &self,
        request: GetShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<Shift>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(ATTENDANCE_V1_SHIFT_GET, "shift_id", &request.shift_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 按名称查询班次
    ///
    /// 通过班次的名称查询班次信息
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query
    pub async fn query(
        &self,
        request: QueryShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<Shift>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_SHIFTS_QUERY.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加必需的查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type.clone());
        api_req
            .query_params
            .insert("shift_name", request.shift_name.clone());

        let body = json!({
            "shift_name": request.shift_name
        });

        api_req.body = serde_json::to_vec(&body).map_err(|e| {
            log::error!("序列化请求体失败: {e:?}");
            e
        })?;

        // 调试日志：打印API请求内容
        log::debug!("查询班次API请求详情:");
        log::debug!("  路径: {}", api_req.api_path);
        log::debug!("  方法: {:?}", api_req.http_method);
        log::debug!("  查询参数: {:?}", api_req.query_params);
        log::debug!(
            "  请求体: {}",
            serde_json::to_string_pretty(&body).unwrap_or_else(|_| "无法序列化".to_string())
        );

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询所有班次
    ///
    /// 分页查询所有班次
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/list
    pub async fn list(
        &self,
        request: ListShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ShiftListData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_SHIFTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
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

impl CreateShiftRequest {
    pub fn builder() -> CreateShiftRequestBuilder {
        CreateShiftRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateShiftRequestBuilder {
    employee_type: Option<String>,
    shift_name: Option<String>,
    punch_times: Option<i32>,
    is_flexible: Option<bool>,
    flexible_minutes: Option<i32>,
    flexible_rule: Option<Vec<super::models::FlexibleRule>>,
    no_need_off: Option<bool>,
    punch_time_rule: Option<Vec<super::models::PunchTimeRule>>,
    late_minutes_as_late: Option<i32>,
    late_minutes_as_lack: Option<i32>,
    early_minutes_as_early: Option<i32>,
    early_minutes_as_lack: Option<i32>,
    allow_outside_apply: Option<bool>,
    outside_apply_limit: Option<i32>,
    allow_face_punch: Option<bool>,
    face_punch_cfg: Option<super::models::FacePunchConfig>,
}

impl CreateShiftRequestBuilder {
    pub fn employee_type<T: Into<String>>(mut self, employee_type: T) -> Self {
        self.employee_type = Some(employee_type.into());
        self
    }

    pub fn shift_name<T: Into<String>>(mut self, shift_name: T) -> Self {
        self.shift_name = Some(shift_name.into());
        self
    }

    pub fn punch_times(mut self, punch_times: i32) -> Self {
        self.punch_times = Some(punch_times);
        self
    }

    pub fn is_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }

    pub fn flexible_minutes(mut self, flexible_minutes: i32) -> Self {
        self.flexible_minutes = Some(flexible_minutes);
        self
    }

    pub fn flexible_rule(mut self, flexible_rule: Vec<super::models::FlexibleRule>) -> Self {
        self.flexible_rule = Some(flexible_rule);
        self
    }

    pub fn no_need_off(mut self, no_need_off: bool) -> Self {
        self.no_need_off = Some(no_need_off);
        self
    }

    pub fn punch_time_rule(mut self, punch_time_rule: Vec<super::models::PunchTimeRule>) -> Self {
        self.punch_time_rule = Some(punch_time_rule);
        self
    }

    pub fn late_minutes_as_late(mut self, late_minutes_as_late: i32) -> Self {
        self.late_minutes_as_late = Some(late_minutes_as_late);
        self
    }

    pub fn late_minutes_as_lack(mut self, late_minutes_as_lack: i32) -> Self {
        self.late_minutes_as_lack = Some(late_minutes_as_lack);
        self
    }

    pub fn early_minutes_as_early(mut self, early_minutes_as_early: i32) -> Self {
        self.early_minutes_as_early = Some(early_minutes_as_early);
        self
    }

    pub fn early_minutes_as_lack(mut self, early_minutes_as_lack: i32) -> Self {
        self.early_minutes_as_lack = Some(early_minutes_as_lack);
        self
    }

    pub fn allow_outside_apply(mut self, allow_outside_apply: bool) -> Self {
        self.allow_outside_apply = Some(allow_outside_apply);
        self
    }

    pub fn outside_apply_limit(mut self, outside_apply_limit: i32) -> Self {
        self.outside_apply_limit = Some(outside_apply_limit);
        self
    }

    pub fn allow_face_punch(mut self, allow_face_punch: bool) -> Self {
        self.allow_face_punch = Some(allow_face_punch);
        self
    }

    pub fn face_punch_cfg(mut self, face_punch_cfg: super::models::FacePunchConfig) -> Self {
        self.face_punch_cfg = Some(face_punch_cfg);
        self
    }

    pub fn build(self) -> CreateShiftRequest {
        CreateShiftRequest {
            api_req: ApiRequest::default(),
            employee_type: self.employee_type.expect("employee_type is required"),
            shift_name: self.shift_name.expect("shift_name is required"),
            punch_times: self.punch_times.expect("punch_times is required"),
            is_flexible: self.is_flexible,
            flexible_minutes: self.flexible_minutes,
            flexible_rule: self.flexible_rule,
            no_need_off: self.no_need_off,
            punch_time_rule: self.punch_time_rule,
            late_minutes_as_late: self.late_minutes_as_late,
            late_minutes_as_lack: self.late_minutes_as_lack,
            early_minutes_as_early: self.early_minutes_as_early,
            early_minutes_as_lack: self.early_minutes_as_lack,
            allow_outside_apply: self.allow_outside_apply,
            outside_apply_limit: self.outside_apply_limit,
            allow_face_punch: self.allow_face_punch,
            face_punch_cfg: self.face_punch_cfg,
        }
    }
}

// 应用ExecutableBuilder trait到CreateShiftRequestBuilder
impl_executable_builder_owned!(
    CreateShiftRequestBuilder,
    ShiftService,
    CreateShiftRequest,
    Response<CreateShiftRespData>,
    create
);

impl DeleteShiftRequest {
    pub fn new<T: Into<String>>(shift_id: T) -> Self {
        Self {
            api_req: ApiRequest::default(),
            shift_id: shift_id.into(),
        }
    }
}

impl GetShiftRequest {
    pub fn new<T: Into<String>>(shift_id: T) -> Self {
        Self {
            api_req: ApiRequest::default(),
            shift_id: shift_id.into(),
        }
    }
}

impl QueryShiftRequest {
    pub fn new<T: Into<String>>(employee_type: T, shift_name: T) -> Self {
        Self {
            api_req: ApiRequest::default(),
            employee_type: employee_type.into(),
            shift_name: shift_name.into(),
        }
    }
}

impl ListShiftRequest {
    pub fn new() -> Self {
        Self {
            api_req: ApiRequest::default(),
            page_size: None,
            page_token: None,
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token<T: Into<String>>(mut self, page_token: T) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
}

impl Service for ShiftService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "shift"
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
    fn test_shift_service_creation() {
        let config = Config::default();
        let service = ShiftService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_shift_service_with_custom_config() {
        let config = Config::builder()
            .app_id("shift_test_app")
            .app_secret("shift_test_secret")
            .build();

        let service = ShiftService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, "shift_test_app");
        assert_eq!(service.config.app_secret, "shift_test_secret");
    }

    #[test]
    fn test_create_shift_request_builder() {
        let request = CreateShiftRequest::builder()
            .employee_type("1")
            .shift_name("Morning Shift")
            .punch_times(2)
            .is_flexible(true)
            .flexible_minutes(30)
            .no_need_off(false)
            .late_minutes_as_late(10)
            .late_minutes_as_lack(30)
            .early_minutes_as_early(10)
            .early_minutes_as_lack(30)
            .allow_outside_apply(true)
            .outside_apply_limit(5)
            .allow_face_punch(true)
            .build();

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.shift_name, "Morning Shift");
        assert_eq!(request.punch_times, 2);
        assert_eq!(request.is_flexible, Some(true));
        assert_eq!(request.flexible_minutes, Some(30));
        assert_eq!(request.no_need_off, Some(false));
        assert_eq!(request.late_minutes_as_late, Some(10));
        assert_eq!(request.late_minutes_as_lack, Some(30));
        assert_eq!(request.early_minutes_as_early, Some(10));
        assert_eq!(request.early_minutes_as_lack, Some(30));
        assert_eq!(request.allow_outside_apply, Some(true));
        assert_eq!(request.outside_apply_limit, Some(5));
        assert_eq!(request.allow_face_punch, Some(true));
    }

    #[test]
    fn test_create_shift_request_builder_minimal() {
        let request = CreateShiftRequest::builder()
            .employee_type("2")
            .shift_name("Evening Shift")
            .punch_times(4)
            .build();

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.shift_name, "Evening Shift");
        assert_eq!(request.punch_times, 4);
        assert_eq!(request.is_flexible, None);
        assert_eq!(request.flexible_minutes, None);
        assert_eq!(request.no_need_off, None);
    }

    #[test]
    fn test_delete_shift_request_new() {
        let request = DeleteShiftRequest::new("shift_123");

        assert_eq!(request.shift_id, "shift_123");
    }

    #[test]
    fn test_get_shift_request_new() {
        let request = GetShiftRequest::new("shift_456");

        assert_eq!(request.shift_id, "shift_456");
    }

    #[test]
    fn test_query_shift_request_new() {
        let request = QueryShiftRequest::new("1", "Day Shift");

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.shift_name, "Day Shift");
    }

    #[test]
    fn test_list_shift_request_new() {
        let request = ListShiftRequest::new();

        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_shift_request_with_pagination() {
        let request = ListShiftRequest::new()
            .page_size(50)
            .page_token("token_123");

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token_123".to_string()));
    }

    #[test]
    fn test_shift_service_config_independence() {
        let config1 = Config::builder().app_id("shift_app_1").build();

        let config2 = Config::builder().app_id("shift_app_2").build();

        let service1 = ShiftService { config: config1 };
        let service2 = ShiftService { config: config2 };

        assert_eq!(service1.config.app_id, "shift_app_1");
        assert_eq!(service2.config.app_id, "shift_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_create_shift_request_builder_edge_cases() {
        // Test with zero punch times
        let request_zero = CreateShiftRequest::builder()
            .employee_type("1")
            .shift_name("Zero Punch Shift")
            .punch_times(0)
            .build();

        assert_eq!(request_zero.punch_times, 0);

        // Test with large punch times
        let request_large = CreateShiftRequest::builder()
            .employee_type("2")
            .shift_name("Many Punch Shift")
            .punch_times(100)
            .build();

        assert_eq!(request_large.punch_times, 100);

        // Test with negative flexible minutes
        let request_negative = CreateShiftRequest::builder()
            .employee_type("3")
            .shift_name("Negative Flexible Shift")
            .punch_times(2)
            .flexible_minutes(-30)
            .build();

        assert_eq!(request_negative.flexible_minutes, Some(-30));
    }

    #[test]
    fn test_request_structs_debug_trait() {
        let create_request = CreateShiftRequest::builder()
            .employee_type("1")
            .shift_name("Debug Shift")
            .punch_times(2)
            .build();

        let debug_str = format!("{:?}", create_request);
        assert!(debug_str.contains("CreateShiftRequest"));
        assert!(debug_str.contains("Debug Shift"));

        let delete_request = DeleteShiftRequest::new("debug_shift_id");
        let debug_str = format!("{:?}", delete_request);
        assert!(debug_str.contains("DeleteShiftRequest"));
        assert!(debug_str.contains("debug_shift_id"));
    }

    #[test]
    fn test_list_shift_request_edge_cases() {
        // Test with very large page size
        let request_large = ListShiftRequest::new().page_size(10000);
        assert_eq!(request_large.page_size, Some(10000));

        // Test with zero page size
        let request_zero = ListShiftRequest::new().page_size(0);
        assert_eq!(request_zero.page_size, Some(0));

        // Test with very long page token
        let long_token = "a".repeat(1000);
        let request_long_token = ListShiftRequest::new().page_token(long_token.clone());
        assert_eq!(request_long_token.page_token, Some(long_token));

        // Test with empty page token
        let request_empty_token = ListShiftRequest::new().page_token("");
        assert_eq!(request_empty_token.page_token, Some("".to_string()));
    }

    #[test]
    fn test_create_shift_request_builder_string_into() {
        // Test that Into<String> works for both &str and String
        let request1 = CreateShiftRequest::builder()
            .employee_type("1")
            .shift_name("Test Shift")
            .punch_times(2)
            .build();

        let request2 = CreateShiftRequest::builder()
            .employee_type("1".to_string())
            .shift_name("Test Shift".to_string())
            .punch_times(2)
            .build();

        assert_eq!(request1.employee_type, request2.employee_type);
        assert_eq!(request1.shift_name, request2.shift_name);
    }

    #[test]
    fn test_query_shift_request_string_into() {
        // Test that Into<String> works for QueryShiftRequest::new
        let request1 = QueryShiftRequest::new("1", "Test Query");
        let request2 = QueryShiftRequest::new("1".to_string(), "Test Query".to_string());

        assert_eq!(request1.employee_type, request2.employee_type);
        assert_eq!(request1.shift_name, request2.shift_name);
    }

    #[test]
    fn test_create_shift_request_builder_chaining() {
        let request = CreateShiftRequest::builder()
            .employee_type("chain_test")
            .shift_name("Chain Test Shift")
            .punch_times(4)
            .is_flexible(true)
            .flexible_minutes(15)
            .no_need_off(true)
            .late_minutes_as_late(5)
            .early_minutes_as_early(5)
            .allow_outside_apply(false)
            .allow_face_punch(false)
            .build();

        assert_eq!(request.employee_type, "chain_test");
        assert_eq!(request.shift_name, "Chain Test Shift");
        assert_eq!(request.punch_times, 4);
        assert_eq!(request.is_flexible, Some(true));
        assert_eq!(request.flexible_minutes, Some(15));
        assert_eq!(request.no_need_off, Some(true));
        assert_eq!(request.late_minutes_as_late, Some(5));
        assert_eq!(request.early_minutes_as_early, Some(5));
        assert_eq!(request.allow_outside_apply, Some(false));
        assert_eq!(request.allow_face_punch, Some(false));
    }
}
