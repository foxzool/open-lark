use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 课程学习进度记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseRegistration {
    /// 学习进度记录ID
    pub registration_id: String,
    /// 课程ID
    pub course_id: String,
    /// 用户ID
    pub user_id: String,
    /// 注册类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<String>,
    /// 学习状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 学习进度（百分比）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    /// 开始学习时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 完成学习时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<i64>,
    /// 总学习时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    /// 已学习时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studied_duration: Option<i64>,
    /// 学习成绩
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// 是否通过
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
    /// 课程信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_info: Option<CourseInfo>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    /// 学习记录详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_records: Option<Vec<LearningRecord>>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 课程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseInfo {
    /// 课程ID
    pub course_id: String,
    /// 课程名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_name: Option<String>,
    /// 课程描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 课程类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_type: Option<String>,
    /// 课程分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 课程标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 课程时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// 课程难度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<String>,
    /// 课程封面URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    /// 课程创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 课程状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 用户部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 用户职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// 学习记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRecord {
    /// 记录ID
    pub record_id: String,
    /// 学习章节ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_id: Option<String>,
    /// 章节名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_name: Option<String>,
    /// 学习进度（百分比）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    /// 学习时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// 是否完成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// 学习开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 学习结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 学习次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_count: Option<i32>,
}

/// 学习统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStatistics {
    /// 总课程数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_courses: Option<i64>,
    /// 已完成课程数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_courses: Option<i64>,
    /// 进行中课程数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_courses: Option<i64>,
    /// 总学习时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_study_time: Option<i64>,
    /// 平均学习进度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_progress: Option<f64>,
    /// 平均成绩
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_score: Option<f64>,
    /// 通过率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_rate: Option<f64>,
}

/// 学习进度事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseRegistrationEvent {
    /// 事件类型
    pub event_type: String,
    /// 学习进度记录
    pub registration: CourseRegistration,
    /// 变更前数据（更新事件时提供）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_registration: Option<CourseRegistration>,
    /// 事件时间戳
    pub timestamp: i64,
    /// 事件来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
