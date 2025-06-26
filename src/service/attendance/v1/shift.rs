use reqwest::Method;
use serde_json::json;

use crate::{
    core::{
        api_req::ApiRequest, api_resp::BaseResponse, config::Config, constants::AccessTokenType,
        http::Transport, req_option::RequestOption, SDKResult,
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
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/shift/create>
    pub async fn create(
        &self,
        request: CreateShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateShiftRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/shifts".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加必需的查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

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
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/shift/delete>
    pub async fn delete(
        &self,
        request: DeleteShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::DELETE;
        api_req.api_path = format!("/open-apis/attendance/v1/shifts/{}", request.shift_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 按 ID 查询班次
    ///
    /// 通过班次的 ID 获取班次详情
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/shift/get>
    pub async fn get(
        &self,
        request: GetShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<Shift>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = format!("/open-apis/attendance/v1/shifts/{}", request.shift_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 按名称查询班次
    ///
    /// 通过班次的名称查询班次信息
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/shift/query>
    pub async fn query(
        &self,
        request: QueryShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<Shift>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/shifts/query".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加必需的查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type.clone());
        api_req
            .query_params
            .insert("shift_name".to_string(), request.shift_name.clone());

        let body = json!({
            "shift_name": request.shift_name
        });

        api_req.body = serde_json::to_vec(&body).map_err(|e| {
            log::error!("序列化请求体失败: {:?}", e);
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
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/shift/list>
    pub async fn list(
        &self,
        request: ListShiftRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ShiftListData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/attendance/v1/shifts".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
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
    BaseResponse<CreateShiftRespData>,
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
