//! 飞书考勤模块
//!
//! 提供考勤打卡、考勤组管理、排班查询等功能。
//!
//! # 权限要求
//! 使用本模块的API需要在飞书开放平台申请以下权限：
//! - `attendance:readonly` - 基础考勤数据读取权限
//! - `attendance:user.read` - 用户考勤数据访问权限  
//! - `attendance:write` - 修改考勤组、更新审批状态
//! - `attendance:group.write` - 考勤组增删改
//! - `attendance:approval` - 审批状态同步
//!
//! # 示例
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use chrono::NaiveDate;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let app_id = "your_app_id";
//!     let app_secret = "your_app_secret";
//!     let client = LarkClient::builder(app_id, app_secret).build();
//!
//!     // 查询用户考勤记录
//!     let req = UserTaskQueryRequest::builder()
//!         .user_ids(vec!["user_id_1".to_string()])
//!         .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
//!         .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
//!         .build();
//!
//!     let records = client.attendance.v1.user_task.query(req, None).await?;
//!     Ok(())
//! }
//! ```

use crate::core::config::Config;

pub mod v1;

/// 考勤服务
pub struct AttendanceService {
    pub v1: v1::V1,
}

impl AttendanceService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}
