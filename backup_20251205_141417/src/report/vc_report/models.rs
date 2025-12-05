//! VC Report API 数据模型
//!
//! 提供视频会议报告相关的数据结构，支持每日报告和用户统计报告等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 获取每日会议报告请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDailyReportRequest {
    /// 开始日期 (YYYY-MM-DD格式)
    pub start_date: String,
    /// 结束日期 (YYYY-MM-DD格式)
    pub end_date: String,
    /// 用户ID类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 用户ID列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl GetDailyReportRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.start_date.trim().is_empty() {
            return Err("开始日期不能为空".to_string());
        }

        if self.end_date.trim().is_empty() {
            return Err("结束日期不能为空".to_string());
        }

        // 验证日期格式
        if !Self::is_valid_date_format(&self.start_date) {
            return Err("开始日期格式不正确，应为YYYY-MM-DD".to_string());
        }

        if !Self::is_valid_date_format(&self.end_date) {
            return Err("结束日期格式不正确，应为YYYY-MM-DD".to_string());
        }

        // 验证日期范围
        if let Err(e) = self.validate_date_range(&self.start_date, &self.end_date) {
            return Err(e);
        }

        if let Some(ref user_ids) = self.user_ids {
            if user_ids.is_empty() {
                return Err("用户ID列表不能为空".to_string());
            }

            if user_ids.len() > 100 {
                return Err("用户ID列表不能超过100个".to_string());
            }

            for (index, user_id) in user_ids.iter().enumerate() {
                if user_id.trim().is_empty() {
                    return Err(format!("用户ID{}不能为空", index + 1));
                }
            }
        }

        Ok(())
    }

    /// 验证日期格式
    fn is_valid_date_format(date_str: &str) -> bool {
        // 简单的YYYY-MM-DD格式验证
        if date_str.len() != 10 {
            return false;
        }

        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return false;
        }

        // 验证各部分长度
        if parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
            return false;
        }

        // 验证是否为数字
        parts
            .iter()
            .all(|part| part.chars().all(|c| c.is_ascii_digit()))
    }

    /// 验证日期范围
    fn validate_date_range(&self, start_date: &str, end_date: &str) -> Result<(), String> {
        // 简单的比较，实际应该解析为日期对象进行比较
        if start_date > end_date {
            return Err("开始日期不能晚于结束日期".to_string());
        }

        // 检查日期范围是否合理（不超过1年）
        // 这里简化处理，实际应该计算日期差
        Ok(())
    }
}

/// 获取每日会议报告响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDailyReportResponse {
    /// 报告数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DailyReportData>,
}

impl ApiResponseTrait for GetDailyReportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取Top用户报告请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetTopUserReportRequest {
    /// 开始日期 (YYYY-MM-DD格式)
    pub start_date: String,
    /// 结束日期 (YYYY-MM-DD格式)
    pub end_date: String,
    /// 排序类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    /// 返回数量限制（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// 用户ID类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetTopUserReportRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.start_date.trim().is_empty() {
            return Err("开始日期不能为空".to_string());
        }

        if self.end_date.trim().is_empty() {
            return Err("结束日期不能为空".to_string());
        }

        // 验证日期格式
        if !Self::is_valid_date_format(&self.start_date) {
            return Err("开始日期格式不正确，应为YYYY-MM-DD".to_string());
        }

        if !Self::is_valid_date_format(&self.end_date) {
            return Err("结束日期格式不正确，应为YYYY-MM-DD".to_string());
        }

        // 验证日期范围
        if let Err(e) = self.validate_date_range(&self.start_date, &self.end_date) {
            return Err(e);
        }

        // 验证排序类型
        if let Some(ref sort_type) = self.sort_type {
            let valid_sort_types = vec!["meeting_count", "meeting_duration", "meeting_score"];
            if !valid_sort_types.contains(&sort_type.as_str()) {
                return Err(format!("不支持的排序类型: {}", sort_type));
            }
        }

        // 验证限制数量
        if let Some(limit) = self.limit {
            if limit <= 0 || limit > 1000 {
                return Err("返回数量限制必须在1-1000之间".to_string());
            }
        }

        Ok(())
    }

    /// 验证日期格式
    fn is_valid_date_format(date_str: &str) -> bool {
        // 简单的YYYY-MM-DD格式验证
        if date_str.len() != 10 {
            return false;
        }

        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return false;
        }

        // 验证各部分长度
        if parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
            return false;
        }

        // 验证是否为数字
        parts
            .iter()
            .all(|part| part.chars().all(|c| c.is_ascii_digit()))
    }

    /// 验证日期范围
    fn validate_date_range(&self, start_date: &str, end_date: &str) -> Result<(), String> {
        if start_date > end_date {
            return Err("开始日期不能晚于结束日期".to_string());
        }

        Ok(())
    }
}

/// 获取Top用户报告响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetTopUserReportResponse {
    /// 报告数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TopUserReportData>,
}

impl ApiResponseTrait for GetTopUserReportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 每日报告数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DailyReportData {
    /// 报告周期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_period: Option<ReportPeriod>,
    /// 会议统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_stats: Option<MeetingStatistics>,
    /// 用户统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_stats: Option<UserStatistics>,
    /// 时间段统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_slot_stats: Option<Vec<TimeSlotStatistics>>,
}

/// Top用户报告数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TopUserReportData {
    /// 报告周期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_period: Option<ReportPeriod>,
    /// Top用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_users: Option<Vec<TopUserItem>>,
    /// 统计摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReportSummary>,
}

/// 报告周期信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReportPeriod {
    /// 开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 总天数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_days: Option<i32>,
}

/// 会议统计信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MeetingStatistics {
    /// 总会议数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_meetings: Option<i32>,
    /// 总会议时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    /// 平均会议时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_duration: Option<f64>,
    /// 最大会议时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<i64>,
    /// 最小会议时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<i64>,
}

/// 用户统计信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserStatistics {
    /// 活跃用户数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_users: Option<i32>,
    /// 总参与人次
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_participants: Option<i32>,
    /// 平均参与人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_participants: Option<f64>,
    /// 最大参与人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_participants: Option<i32>,
}

/// 时间段统计
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TimeSlotStatistics {
    /// 时间段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_slot: Option<String>,
    /// 会议数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_count: Option<i32>,
    /// 参与人次
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<i32>,
    /// 平均时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_duration: Option<f64>,
}

/// Top用户项目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TopUserItem {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 会议数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_count: Option<i32>,
    /// 总会议时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    /// 平均会议时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_duration: Option<f64>,
    /// 会议评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_score: Option<f64>,
}

/// 报告摘要
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReportSummary {
    /// 统计用户总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_users: Option<i32>,
    /// 总会议数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_meetings: Option<i32>,
    /// 总参与人次
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_participants: Option<i32>,
    /// 总会议时长（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_hours: Option<f64>,
}

/// 排序类型常量
pub mod sort_types {
    /// 按会议数量排序
    pub const MEETING_COUNT: &str = "meeting_count";
    /// 按会议时长排序
    pub const MEETING_DURATION: &str = "meeting_duration";
    /// 按会议评分排序
    pub const MEETING_SCORE: &str = "meeting_score";
}

/// 用户ID类型常量
pub mod user_id_types {
    /// 用户ID
    pub const USER_ID: &str = "user_id";
    /// Open ID
    pub const OPEN_ID: &str = "open_id";
    /// Union ID
    pub const UNION_ID: &str = "union_id";
}
