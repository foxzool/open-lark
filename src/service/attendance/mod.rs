//! 考勤（Attendance）服务
//!
//! 提供飞书考勤管理的完整功能集，支持考勤数据管理、统计分析、
//! 假期管理、班次设置等企业级考勤管理能力。是企业人事管理的重要组成部分。
//!
//! # 核心功能
//!
//! ## 考勤数据管理
//! - 📊 考勤记录查询和管理
//! - ⏰ 打卡数据统计和分析
//! - 📅 考勤日历和排班管理
//! - 🔍 考勤异常检测和处理
//! - 📈 考勤报表生成和导出
//!
//! ## 假期管理
//! - 🏖️ 假期类型定义和管理
//! - 📝 请假申请和审批流程
//! - 📊 假期余额查询和统计
//! - 📅 假期计划和安排
//! - 🔔 假期提醒和通知
//!
//! ## 班次设置
//! - ⏰ 工作时间和班次配置
//! - 🔄 轮班制度和排班规则
//! - 📍 考勤地点和范围设置
//! - 🎯 弹性工作时间管理
//! - 📊 加班时间统计和管理
//!
//! ## 考勤统计
//! - 📈 个人和团队考勤统计
//! - 📊 出勤率和工时分析
//! - 🎯 考勤目标和绩效评估
//! - 📋 考勤月报和年报
//! - 💰 工资结算相关数据
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取考勤服务
//! let attendance = &client.attendance;
//!
//! // 查询用户考勤记录
//! // let attendance_request = GetUserAttendanceRequest::builder()
//! //     .user_id("user_id")
//! //     .start_date("2024-07-01")
//! //     .end_date("2024-07-31")
//! //     .build();
//! // let records = attendance.v1.user_attendance.get(attendance_request, None).await?;
//!
//! // 查询假期余额
//! // let leave_request = GetLeaveBalanceRequest::builder()
//! //     .user_id("user_id")
//! //     .build();
//! // let balance = attendance.v1.leave.get_balance(leave_request, None).await?;
//!
//! // 获取班次信息
//! // let shift_request = ListShiftRequest::builder()
//! //     .page_size(20)
//! //     .build();
//! // let shifts = attendance.v1.shift.list(shift_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v1版本，提供基础的考勤管理功能：
//! - 考勤数据查询和统计
//! - 假期管理和请假流程
//! - 班次设置和排班管理
//! - 考勤报表和分析
//!
//! # 考勤管理特性
//!
//! - ⏰ 多种打卡方式支持
//! - 📱 移动端考勤应用
//! - 🔍 智能考勤异常检测
//! - 📊 实时数据同步更新
//! - 🔐 企业级权限控制
//!
//! # 人事集成
//!
//! - 👥 员工信息同步
//! - 💰 薪酬计算数据对接
//! - 📋 绩效考核数据支持
//! - 🔄 HR系统集成
//! - 📊 组织架构联动

use crate::core::config::Config;

pub mod v1;

/// 考勤服务
///
/// 企业级考勤管理的统一入口，提供考勤数据管理、假期管理、
/// 班次设置、统计分析等完整的考勤管理能力。
///
/// # 服务架构
///
/// - **v1**: 考勤管理API v1版本，提供基础功能集
///
/// # 核心特性
///
/// - ⏰ 全面的考勤数据管理
/// - 🏖️ 灵活的假期管理系统
/// - 📊 丰富的统计分析功能
/// - 🔄 智能的排班调度
/// - 📱 移动办公支持
///
/// # 适用场景
///
/// - 企业员工考勤管理
/// - 假期和请假管理
/// - 工时统计和分析
/// - 薪酬计算数据支持
/// - 人事管理系统集成
///
/// # 最佳实践
///
/// - 合理设置考勤规则
/// - 定期分析考勤数据
/// - 及时处理考勤异常
/// - 优化排班和调度
/// - 保护员工隐私数据
pub struct AttendanceService {
    /// v1版本API服务
    pub v1: v1::V1,
}

impl AttendanceService {
    /// 创建新的考勤服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的考勤服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}
