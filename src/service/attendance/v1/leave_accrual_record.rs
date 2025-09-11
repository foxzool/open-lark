use reqwest::Method;
use std::sync::Arc;
use serde_json::json;

use crate::{
    core::{
        api_resp::BaseResponse, config::Config, constants::AccessTokenType, http::Transport,
        req_option::RequestOption, SDKResult,
    },
    impl_executable_builder_owned,
};

use super::models::{PatchLeaveAccrualRecordRequest, PatchLeaveAccrualRecordRespData};

/// 休假发放记录服务
pub struct LeaveAccrualRecordService {
    pub config: Arc<Config>,
}

impl LeaveAccrualRecordService {
    /// 修改发放记录
    ///
    /// 该接口用于修改指定的休假发放记录信息，包括发放数量、过期时间、发放原因等。
    /// 支持部分字段更新，只需要传入需要修改的字段即可。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/leave_accrual_record/patch>
    pub async fn patch(
        &self,
        request: PatchLeaveAccrualRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PatchLeaveAccrualRecordRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::PATCH;
        api_req.api_path = format!(
            "/open-apis/attendance/v1/leave_accrual_records/{}",
            request.leave_accrual_record_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        // 构建请求体
        let body = json!({
            "leave_accrual_record": request.leave_accrual_record
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// Builder implementations
impl_executable_builder_owned!(
    PatchLeaveAccrualRecordRequest,
    LeaveAccrualRecordService,
    PatchLeaveAccrualRecordRequest,
    BaseResponse<PatchLeaveAccrualRecordRespData>,
    patch
);
