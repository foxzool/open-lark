use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::Endpoints,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::apass::models::{
        PageResponse, SeatActivity, SeatActivityListRequest, SeatAssignment,
        SeatAssignmentListRequest,
    },
};

/// 席位管理服务
pub struct SeatService {
    pub config: Config,
}

/// 席位分配查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SeatAssignmentListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<SeatAssignment>,
}

impl ApiResponseTrait for SeatAssignmentListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 席位活跃查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SeatActivityListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<SeatActivity>,
}

impl ApiResponseTrait for SeatActivityListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SeatService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询席位分配详情
    ///
    /// 该接口用于查询低代码平台的席位分配详情信息。
    ///
    /// # 参数
    ///
    /// - `request`: 席位分配查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_seat_assignment(
        &self,
        request: SeatAssignmentListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SeatAssignmentListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::APASS_V1_SEAT_ASSIGNMENT_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询席位活跃详情
    ///
    /// 该接口用于查询低代码平台的席位活跃详情信息。
    ///
    /// # 参数
    ///
    /// - `request`: 席位活跃查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_seat_activity(
        &self,
        request: SeatActivityListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SeatActivityListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::APASS_V1_SEAT_ACTIVITY_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(start_time) = request.start_time {
            api_req.query_params.insert("start_time", start_time);
        }
        if let Some(end_time) = request.end_time {
            api_req.query_params.insert("end_time", end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
