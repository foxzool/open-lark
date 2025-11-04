//! 视频会议服务端点常量定义

// ==================== 会议室管理端点 ====================

/// 获取会议室列表
pub const VC_ROOM_LIST: &str = "/open-apis/vc/v1/rooms";

/// 获取会议室详情
pub const VC_ROOM_GET: &str = "/open-apis/vc/v1/rooms/{room_id}";

/// 创建会议室
pub const VC_ROOM_CREATE: &str = "/open-apis/vc/v1/rooms";

/// 更新会议室
pub const VC_ROOM_UPDATE: &str = "/open-apis/vc/v1/rooms/{room_id}";

/// 删除会议室
pub const VC_ROOM_DELETE: &str = "/open-apis/vc/v1/rooms/{room_id}";

/// 搜索会议室
pub const VC_ROOM_SEARCH: &str = "/open-apis/vc/v1/rooms/search";

// ==================== 会议管理端点 ====================

/// 获取会议列表
pub const VC_MEETING_LIST: &str = "/open-apis/vc/v1/meetings";

/// 获取会议详情
pub const VC_MEETING_GET: &str = "/open-apis/vc/v1/meetings/{meeting_id}";

/// 创建会议
pub const VC_MEETING_CREATE: &str = "/open-apis/vc/v1/meetings";

/// 更新会议
pub const VC_MEETING_UPDATE: &str = "/open-apis/vc/v1/meetings/{meeting_id}";

/// 结束会议
pub const VC_MEETING_END: &str = "/open-apis/vc/v1/meetings/{meeting_id}/end";

/// 邀请参会
pub const VC_MEETING_INVITE: &str = "/open-apis/vc/v1/meetings/{meeting_id}/invite";

/// 移除参会人
pub const VC_MEETING_KICKOUT: &str = "/open-apis/vc/v1/meetings/{meeting_id}/kickout";

/// 根据会议号获取会议列表
pub const VC_MEETING_LIST_BY_NO: &str = "/open-apis/vc/v1/meetings/list_by_no";

/// 设置主持人
pub const VC_MEETING_SET_HOST: &str = "/open-apis/vc/v1/meetings/{meeting_id}/set_host";

// ==================== 录制管理端点 ====================

/// 获取录制文件列表
pub const VC_RECORDING_LIST: &str = "/open-apis/vc/v1/meetings/{meeting_id}/recordings";

/// 开始录制
pub const VC_RECORDING_START: &str = "/open-apis/vc/v1/meetings/{meeting_id}/recording/start";

/// 停止录制
pub const VC_RECORDING_STOP: &str = "/open-apis/vc/v1/meetings/{meeting_id}/recording/stop";

/// 获取录制文件
pub const VC_RECORDING_GET: &str = "/open-apis/vc/v1/meetings/{meeting_id}/recording";

/// 设置录制权限
pub const VC_RECORDING_SET_PERMISSION: &str =
    "/open-apis/vc/v1/meetings/{meeting_id}/recording/set_permission";

// ==================== 预约管理端点 ====================

/// 获取预约列表
pub const VC_RESERVE_LIST: &str = "/open-apis/vc/v1/reserves";

/// 获取预约详情
pub const VC_RESERVE_GET: &str = "/open-apis/vc/v1/reserves/{reserve_id}";

/// 创建预约
pub const VC_RESERVE_CREATE: &str = "/open-apis/vc/v1/reserves";

/// 更新预约
pub const VC_RESERVE_UPDATE: &str = "/open-apis/vc/v1/reserves/{reserve_id}";

/// 删除预约
pub const VC_RESERVE_DELETE: &str = "/open-apis/vc/v1/reserves/{reserve_id}";

/// 获取活跃会议
pub const VC_RESERVE_GET_ACTIVE_MEETING: &str =
    "/open-apis/vc/v1/reserves/{reserve_id}/get_active_meeting";
