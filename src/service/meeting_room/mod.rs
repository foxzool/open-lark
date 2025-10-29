// meeting_room - 会议室管理模块
//,
// 该模块提供飞书会议室管理相关的所有功能，包括：
// - 建筑物管理（创建、更新、删除、查询）
// - 会议室管理（配置、查询、状态管理）
// - 会议室日程管理（预订、查询、释放）
// - 会议室忙闲状态查询
// - 会议设备管理
//
// 覆盖17个API接口，是企业办公管理的重要组成部分
use crate::core::config::Config;
use crate::service::meeting_room::buildings::BuildingsService;
use crate::service::meeting_room::rooms::RoomsService;
use crate::service::meeting_room::schedules::SchedulesService;
/// 会议室管理服务
#[cfg(feature = "meeting_room")]
#[derive(.*?)]
pub struct MeetingRoomService {
    /// 建筑物管理服务
    pub buildings: BuildingsService,
    /// 会议室管理服务
    pub rooms: RoomsService,
    /// 日程管理服务
    pub schedules: SchedulesService,
}
#[cfg(feature = "meeting_room")]
impl MeetingRoomService {
/// 创建新的会议室管理服务实例
    pub fn new() -> Self {
Self {
            buildings: BuildingsService::new(config.clone()),
            rooms: RoomsService::new(config.clone()),
            schedules: SchedulesService::new(config.clone()),
        }
}
}
#[cfg(not(feature = "meeting_room"))],
pub struct MeetingRoomService;
/// 数据模型
pub mod models;
/// 各子模块
pub mod buildings;
pub mod rooms;
pub mod schedules;