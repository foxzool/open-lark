use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{
            EndpointBuilder, VC_MEETING_END, VC_MEETING_GET, VC_MEETING_INVITE, VC_MEETING_KICKOUT,
            VC_MEETING_LIST_BY_NO, VC_MEETING_SET_HOST,
        },
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::vc::models::{Meeting, UserIdType},
};

/// 会议管理服务
pub struct MeetingService {
    pub config: Config,
}

/// 邀请参会人请求
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteMeetingRequest {
    /// 邀请的用户ID列表
    pub invitees: Vec<String>,
}

/// 邀请参会人响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteMeetingResponse {
    /// 邀请结果
    pub invite_results: Vec<InviteResult>,
}

/// 邀请结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteResult {
    /// 用户ID
    pub user_id: String,
    /// 邀请状态
    pub status: String,
    /// 错误信息
    pub error_msg: Option<String>,
}

impl ApiResponseTrait for InviteMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移除参会人请求
#[derive(Debug, Serialize, Deserialize)]
pub struct KickoutMeetingRequest {
    /// 要移除的用户ID列表
    pub kickout_users: Vec<String>,
}

/// 移除参会人响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KickoutMeetingResponse {
    /// 移除结果
    pub kickout_results: Vec<KickoutResult>,
}

/// 移除结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KickoutResult {
    /// 用户ID
    pub user_id: String,
    /// 移除状态
    pub status: String,
    /// 错误信息
    pub error_msg: Option<String>,
}

impl ApiResponseTrait for KickoutMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置主持人请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SetHostRequest {
    /// 新主持人用户ID
    pub host_user_id: String,
}

/// 获取会议详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMeetingResponse {
    /// 会议信息
    pub meeting: Meeting,
}

impl ApiResponseTrait for GetMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取会议列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMeetingsByNoResponse {
    /// 会议列表
    pub meetings: Vec<Meeting>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListMeetingsByNoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会议列表查询参数
#[derive(Debug, Default)]
pub struct ListMeetingsByNoParams {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub user_id_type: Option<UserIdType>,
}

impl MeetingService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 邀请参会人
    pub async fn invite(
        &self,
        meeting_id: &str,
        request: InviteMeetingRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<InviteMeetingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(VC_MEETING_INVITE, "{meeting_id}", meeting_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 移除参会人
    pub async fn kickout(
        &self,
        meeting_id: &str,
        request: KickoutMeetingRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<KickoutMeetingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                VC_MEETING_KICKOUT,
                "{meeting_id}",
                meeting_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 设置主持人
    pub async fn set_host(
        &self,
        meeting_id: &str,
        request: SetHostRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                VC_MEETING_SET_HOST,
                "{meeting_id}",
                meeting_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 结束会议
    pub async fn end(
        &self,
        meeting_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(VC_MEETING_END, "{meeting_id}", meeting_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取会议详情
    pub async fn get(
        &self,
        meeting_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetMeetingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(VC_MEETING_GET, "{meeting_id}", meeting_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取与会议号关联的会议列表
    pub async fn list_by_no(
        &self,
        meeting_no: &str,
        start_time: &str,
        end_time: &str,
        params: Option<ListMeetingsByNoParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListMeetingsByNoResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("meeting_no", meeting_no.to_string());
        query_params.insert("start_time", start_time.to_string());
        query_params.insert("end_time", end_time.to_string());

        if let Some(params) = params {
            if let Some(page_size) = params.page_size {
                query_params.insert("page_size", page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                query_params.insert("page_token", page_token);
            }
            if let Some(user_id_type) = params.user_id_type {
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
            }
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: VC_MEETING_LIST_BY_NO.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
