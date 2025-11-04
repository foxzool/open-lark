//! 会议模型定义
use serde::{Deserialize, Serialize};
/// 会议室状态
#[derive(Debug, Deserialize, Serialize)]
pub enum BuildingStatus {
    Available,
    Occupied,
    Maintenance,
/// 会议室信息
#[derive(Debug, Deserialize, Serialize)]
pub struct MeetingRoom {
}
    pub room_id: String,
    pub name: String,
    pub building_id: String,
    pub floor: i32,
    pub capacity: i32,
    pub status: BuildingStatus,
/// 会议室响应
#[derive(Debug, Deserialize, Serialize)]
pub struct MeetingRoomResponse {
}
    pub code: i32,
    pub msg: String,
    pub data: Option<MeetingRoom>,
/// 分页响应
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedMeetingRoomResponse {
}
    pub code: i32,
    pub msg: String,
    pub data: Option<PaginatedMeetingRoomData>,
/// 分页数据
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedMeetingRoomData {
    pub items: Vec<MeetingRoom>,
    pub page_token: Option<String>,
    pub has_more: bool,
}
}