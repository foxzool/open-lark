//! VC (视频会议) 端点定义
//!
//! 视频会议系统 - 会议室管理、会议创建、参会控制
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::vc::*;
//!
//! let rooms_endpoint = VC_V1_ROOMS;
//! let meetings_endpoint = VC_V1_MEETINGS;
//! ```

// ==================== VC (视频会议) v1 ====================
// 视频会议系统 - 会议室管理、会议创建、参会控制

/// VC会议室管理
/// 会议室的创建、查询、管理
pub const VC_V1_ROOMS: &str = "/open-apis/vc/v1/rooms";
pub const VC_V1_ROOM_GET: &str = "/open-apis/vc/v1/rooms/{room_id}";
pub const VC_V1_ROOM_CREATE: &str = "/open-apis/vc/v1/rooms";
pub const VC_V1_ROOM_UPDATE: &str = "/open-apis/vc/v1/rooms/{room_id}";
pub const VC_V1_ROOM_DELETE: &str = "/open-apis/vc/v1/rooms/{room_id}";
pub const VC_V1_ROOM_SEARCH: &str = "/open-apis/vc/v1/rooms/search";

/// VC会议管理
/// 会议的创建、管理、控制
pub const VC_V1_MEETINGS: &str = "/open-apis/vc/v1/meetings";
pub const VC_V1_MEETING_GET: &str = "/open-apis/vc/v1/meetings/{meeting_id}";
pub const VC_V1_MEETING_CREATE: &str = "/open-apis/vc/v1/meetings";
pub const VC_V1_MEETING_UPDATE: &str = "/open-apis/vc/v1/meetings/{meeting_id}";
pub const VC_V1_MEETING_END: &str = "/open-apis/vc/v1/meetings/{meeting_id}/end";
pub const VC_V1_MEETING_INVITE: &str = "/open-apis/vc/v1/meetings/{meeting_id}/invite";
pub const VC_V1_MEETING_KICKOUT: &str = "/open-apis/vc/v1/meetings/{meeting_id}/kickout";
pub const VC_V1_MEETING_LIST_BY_NO: &str = "/open-apis/vc/v1/meetings/list_by_no";
pub const VC_V1_MEETING_SET_HOST: &str = "/open-apis/vc/v1/meetings/{meeting_id}/set_host";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vc_endpoints() {
        assert!(VC_V1_ROOMS.starts_with("/open-apis/vc/v1/"));
        assert!(VC_V1_ROOMS.contains("rooms"));
        assert!(VC_V1_ROOM_SEARCH.contains("search"));
        assert!(VC_V1_MEETINGS.contains("meetings"));
        assert!(VC_V1_MEETING_CREATE.contains("meetings"));
        assert!(VC_V1_MEETING_INVITE.contains("invite"));
        assert!(VC_V1_MEETING_KICKOUT.contains("kickout"));
        assert!(VC_V1_MEETING_SET_HOST.contains("set_host"));
}
}
