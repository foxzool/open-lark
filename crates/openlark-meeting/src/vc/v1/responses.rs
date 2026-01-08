//! 会议相关响应结构
//!
//! 定义视频会议 API 的响应数据类型。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomResponse {
    pub room_id: String,
}

impl ApiResponseTrait for CreateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoomRequest {
    pub user_id_type: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoomResponse {
    pub room_id: String,
    pub room_name: String,
    pub capacity: u32,
    pub devices: Option<Vec<DeviceInfo>>,
    pub floor: Option<String>,
    pub description: Option<String>,
}

impl ApiResponseTrait for GetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRoomResponse {
    pub rooms: Vec<RoomInfo>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomInfo {
    pub room_id: String,
    pub room_name: String,
    pub capacity: u32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgetRoomResponse {
    pub rooms: Vec<RoomInfo>,
}

impl ApiResponseTrait for MgetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRoomResponse {
    pub room_id: String,
}

impl ApiResponseTrait for PatchRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRoomResponse {
    pub rooms: Vec<RoomInfo>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoomResponse {}

impl ApiResponseTrait for DeleteRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
