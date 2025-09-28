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
            vc::{
                VC_ROOM_CREATE, VC_ROOM_DELETE, VC_ROOM_GET, VC_ROOM_LIST, VC_ROOM_SEARCH,
                VC_ROOM_UPDATE,
            },
            EndpointBuilder,
        },
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::vc::models::{Room, RoomIdType, UserIdType},
};

/// 会议室管理服务
pub struct RoomService {
    pub config: Config,
}

/// 创建会议室请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    /// 会议室名称
    pub name: String,
    /// 会议室描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 会议室容量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    /// 会议室位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// 创建会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomResponse {
    /// 会议室信息
    pub room: Room,
}

impl ApiResponseTrait for CreateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新会议室请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoomRequest {
    /// 会议室名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 会议室描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 会议室容量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    /// 会议室位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// 更新会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoomResponse {
    /// 会议室信息
    pub room: Room,
}

impl ApiResponseTrait for UpdateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoomResponse {
    /// 会议室信息
    pub room: Room,
}

impl ApiResponseTrait for GetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会议室列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRoomsResponse {
    /// 会议室列表
    pub rooms: Vec<Room>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListRoomsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRoomsResponse {
    /// 会议室列表
    pub rooms: Vec<Room>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchRoomsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索会议室参数
#[derive(Debug, Default)]
pub struct SearchRoomsParams {
    pub keyword: Option<String>,
    pub room_ids: Option<Vec<String>>,
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub room_id_type: Option<RoomIdType>,
    pub user_id_type: Option<UserIdType>,
}

impl RoomService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建会议室
    pub async fn create(
        &self,
        request: CreateRoomRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateRoomResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: VC_ROOM_CREATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新会议室
    pub async fn update(
        &self,
        room_id: &str,
        request: UpdateRoomRequest,
        room_id_type: Option<RoomIdType>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateRoomResponse>> {
        let mut query_params = HashMap::new();
        if let Some(room_id_type) = room_id_type {
            query_params.insert("room_id_type", room_id_type.as_str().to_string());
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(VC_ROOM_UPDATE, "{room_id}", room_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除会议室
    pub async fn delete(
        &self,
        room_id: &str,
        room_id_type: Option<RoomIdType>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(room_id_type) = room_id_type {
            query_params.insert("room_id_type", room_id_type.as_str().to_string());
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(VC_ROOM_DELETE, "{room_id}", room_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取会议室
    pub async fn get(
        &self,
        room_id: &str,
        room_id_type: Option<RoomIdType>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetRoomResponse>> {
        let mut query_params = HashMap::new();
        if let Some(room_id_type) = room_id_type {
            query_params.insert("room_id_type", room_id_type.as_str().to_string());
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(VC_ROOM_GET, "{room_id}", room_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取会议室列表
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
        room_id_type: Option<RoomIdType>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListRoomsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }
        if let Some(room_id_type) = room_id_type {
            query_params.insert("room_id_type", room_id_type.as_str().to_string());
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: VC_ROOM_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 搜索会议室
    pub async fn search(
        &self,
        params: Option<SearchRoomsParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchRoomsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(params) = params {
            if let Some(keyword) = params.keyword {
                query_params.insert("keyword", keyword);
            }
            if let Some(room_ids) = params.room_ids {
                query_params.insert("room_ids", room_ids.join(","));
            }
            if let Some(page_size) = params.page_size {
                query_params.insert("page_size", page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                query_params.insert("page_token", page_token);
            }
            if let Some(room_id_type) = params.room_id_type {
                query_params.insert("room_id_type", room_id_type.as_str().to_string());
            }
            if let Some(user_id_type) = params.user_id_type {
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
            }
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: VC_ROOM_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
