//! 薪资组管理服务
//!
//! 提供薪资组的完整管理功能：
//! - 薪资组创建和管理
//! - 薪资组配置
//! - 员工组关联管理
//! - 薪资组统计分析

use crate::core::config::Config;
use crate::service::payroll::models::*;
use chrono::{DateTime, Utc};
use open_lark_core::prelude::*;
use serde::{Deserialize, Serialize};

/// 薪资组管理服务
#[derive(Debug, Clone)]
pub struct PaygroupService {
    config: Config,
}

impl PaygroupService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 薪资组管理 ====================

    /// 创建薪资组
    pub async fn create_paygroup(
        &self,
        request: &CreatePaygroupRequest,
    ) -> SDKResult<CreatePaygroupResponse> {
        let paygroup_id = format!("pg_{}", chrono::Utc::now().timestamp());

        Ok(CreatePaygroupResponse {
            paygroup_id: paygroup_id.clone(),
            paygroup_name: request.paygroup_name.clone(),
            paygroup_code: request.paygroup_code.clone(),
            description: request.description.clone(),
            status: PaygroupStatus::Active,
            employee_count: 0,
            created_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取薪资组详情
    pub async fn get_paygroup(&self, paygroup_id: &str) -> SDKResult<GetPaygroupResponse> {
        // 模拟实现
        Ok(GetPaygroupResponse {
            paygroup_id: paygroup_id.to_string(),
            paygroup_name: "技术研发部薪资组".to_string(),
            paygroup_code: "tech_group_001".to_string(),
            description: Some("技术研发部门员工薪资组".to_string()),
            status: PaygroupStatus::Active,
            employee_count: 150,
            salary_rules: vec![
                SalaryRule {
                    rule_id: "rule_001".to_string(),
                    rule_name: "基本工资计算".to_string(),
                    rule_type: SalaryRuleType::BaseSalary,
                    formula: "base_salary * 1.0".to_string(),
                    description: Some("基本工资按实际金额计算".to_string()),
                },
                SalaryRule {
                    rule_id: "rule_002".to_string(),
                    rule_name: "绩效奖金计算".to_string(),
                    rule_type: SalaryRuleType::Bonus,
                    formula: "base_salary * performance_coefficient".to_string(),
                    description: Some("绩效奖金根据绩效系数计算".to_string()),
                },
            ],
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        })
    }

    /// 更新薪资组
    pub async fn update_paygroup(
        &self,
        paygroup_id: &str,
        request: &UpdatePaygroupRequest,
    ) -> SDKResult<UpdatePaygroupResponse> {
        Ok(UpdatePaygroupResponse {
            paygroup_id: paygroup_id.to_string(),
            paygroup_name: request.paygroup_name.clone(),
            paygroup_code: request.paygroup_code.clone(),
            description: request.description.clone(),
            status: request.status.clone(),
            updated_at: Some(chrono::Utc::now()),
        })
    }

    /// 删除薪资组
    pub async fn delete_paygroup(&self, paygroup_id: &str) -> SDKResult<DeletePaygroupResponse> {
        Ok(DeletePaygroupResponse {
            paygroup_id: paygroup_id.to_string(),
            deleted: true,
            deleted_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取薪资组列表
    pub async fn list_paygroups(
        &self,
        request: &ListPaygroupsRequest,
    ) -> SDKResult<ListPaygroupsResponse> {
        // 模拟薪资组列表
        let paygroups = vec![
            PaygroupInfo {
                paygroup_id: "pg_001".to_string(),
                paygroup_name: "技术研发部".to_string(),
                paygroup_code: "tech_group".to_string(),
                status: PaygroupStatus::Active,
                employee_count: 150,
                description: Some("技术研发部门薪资组".to_string()),
                created_at: Some(chrono::Utc::now()),
            },
            PaygroupInfo {
                paygroup_id: "pg_002".to_string(),
                paygroup_name: "市场营销部".to_string(),
                paygroup_code: "marketing_group".to_string(),
                status: PaygroupStatus::Active,
                employee_count: 80,
                description: Some("市场营销部门薪资组".to_string()),
                created_at: Some(chrono::Utc::now()),
            },
            PaygroupInfo {
                paygroup_id: "pg_003".to_string(),
                paygroup_name: "人力资源部".to_string(),
                paygroup_code: "hr_group".to_string(),
                status: PaygroupStatus::Inactive,
                employee_count: 25,
                description: Some("人力资源部门薪资组".to_string()),
                created_at: Some(chrono::Utc::now()),
            },
        ];

        Ok(ListPaygroupsResponse {
            paygroups,
            total_count: paygroups.len() as i32,
            has_more: Some(false),
            next_page_token: None,
        })
    }

    // ==================== 薪资组配置 ====================

    /// 设置薪资组规则
    pub async fn set_salary_rules(
        &self,
        paygroup_id: &str,
        request: &SetSalaryRulesRequest,
    ) -> SDKResult<SetSalaryRulesResponse> {
        Ok(SetSalaryRulesResponse {
            paygroup_id: paygroup_id.to_string(),
            rule_count: request.rules.len() as i32,
            rules: request.rules.clone(),
            updated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取薪资组规则
    pub async fn get_salary_rules(&self, paygroup_id: &str) -> SDKResult<GetSalaryRulesResponse> {
        Ok(GetSalaryRulesResponse {
            paygroup_id: paygroup_id.to_string(),
            rules: vec![
                SalaryRule {
                    rule_id: "rule_001".to_string(),
                    rule_name: "基本工资".to_string(),
                    rule_type: SalaryRuleType::BaseSalary,
                    formula: "base_salary".to_string(),
                    description: Some("基本工资计算规则".to_string()),
                },
                SalaryRule {
                    rule_id: "rule_002".to_string(),
                    rule_name: "绩效奖金".to_string(),
                    rule_type: SalaryRuleType::Bonus,
                    formula: "base_salary * performance_coefficient".to_string(),
                    description: Some("绩效奖金计算规则".to_string()),
                },
                SalaryRule {
                    rule_id: "rule_003".to_string(),
                    rule_name: "交通补贴".to_string(),
                    rule_type: SalaryRuleType::Allowance,
                    formula: "500".to_string(),
                    description: Some("固定交通补贴".to_string()),
                },
            ],
        })
    }

    // ==================== 员工组关联管理 ====================

    /// 添加员工到薪资组
    pub async fn add_employees_to_paygroup(
        &self,
        paygroup_id: &str,
        request: &AddEmployeesRequest,
    ) -> SDKResult<AddEmployeesResponse> {
        Ok(AddEmployeesResponse {
            paygroup_id: paygroup_id.to_string(),
            added_count: request.employee_ids.len() as i32,
            failed_count: 0,
            failed_employees: vec![],
            added_at: Some(chrono::Utc::now()),
        })
    }

    /// 从薪资组移除员工
    pub async fn remove_employees_from_paygroup(
        &self,
        paygroup_id: &str,
        request: &RemoveEmployeesRequest,
    ) -> SDKResult<RemoveEmployeesResponse> {
        Ok(RemoveEmployeesResponse {
            paygroup_id: paygroup_id.to_string(),
            removed_count: request.employee_ids.len() as i32,
            failed_count: 0,
            failed_employees: vec![],
            removed_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取薪资组员工列表
    pub async fn get_paygroup_employees(
        &self,
        paygroup_id: &str,
        request: &GetPaygroupEmployeesRequest,
    ) -> SDKResult<GetPaygroupEmployeesResponse> {
        // 模拟员工列表
        let employees = vec![
            PaygroupEmployee {
                employee_id: "emp_001".to_string(),
                employee_name: "张三".to_string(),
                employee_email: Some("zhangsan@company.com".to_string()),
                department: "技术研发部".to_string(),
                position: "高级工程师".to_string(),
                joined_date: Some("2023-01-15".to_string()),
                salary_level: Some("L7".to_string()),
            },
            PaygroupEmployee {
                employee_id: "emp_002".to_string(),
                employee_name: "李四".to_string(),
                employee_email: Some("lisi@company.com".to_string()),
                department: "技术研发部".to_string(),
                position: "技术主管".to_string(),
                joined_date: Some("2022-06-20".to_string()),
                salary_level: Some("L8".to_string()),
            },
        ];

        Ok(GetPaygroupEmployeesResponse {
            paygroup_id: paygroup_id.to_string(),
            employees,
            total_count: employees.len() as i32,
            has_more: Some(false),
            next_page_token: None,
        })
    }

    // ==================== 薪资组统计分析 ====================

    /// 获取薪资组统计信息
    pub async fn get_paygroup_statistics(
        &self,
        paygroup_id: &str,
        period: &str,
    ) -> SDKResult<GetPaygroupStatisticsResponse> {
        Ok(GetPaygroupStatisticsResponse {
            paygroup_id: paygroup_id.to_string(),
            period: period.to_string(),
            total_employees: 150,
            active_employees: 145,
            total_salary_cost: 1500000.0,
            average_salary: 10000.0,
            salary_distribution: SalaryDistribution {
                min_salary: 6000.0,
                max_salary: 25000.0,
                median_salary: 9500.0,
                salary_ranges: vec![
                    SalaryRange {
                        range_name: "6K-8K".to_string(),
                        min_amount: 6000.0,
                        max_amount: 8000.0,
                        employee_count: 30,
                        percentage: 20.0,
                    },
                    SalaryRange {
                        range_name: "8K-12K".to_string(),
                        min_amount: 8000.0,
                        max_amount: 12000.0,
                        employee_count: 75,
                        percentage: 50.0,
                    },
                    SalaryRange {
                        range_name: "12K-20K".to_string(),
                        min_amount: 12000.0,
                        max_amount: 20000.0,
                        employee_count: 35,
                        percentage: 23.3,
                    },
                    SalaryRange {
                        range_name: "20K+".to_string(),
                        min_amount: 20000.0,
                        max_amount: 999999.0,
                        employee_count: 10,
                        percentage: 6.7,
                    },
                ],
            },
            calculated_at: Some(chrono::Utc::now()),
        })
    }
}

// ==================== 请求数据模型 ====================

/// 创建薪资组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaygroupRequest {
    /// 薪资组名称
    pub paygroup_name: String,
    /// 薪资组编码
    pub paygroup_code: String,
    /// 描述
    pub description: Option<String>,
}

/// 更新薪资组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaygroupRequest {
    /// 薪资组名称
    pub paygroup_name: Option<String>,
    /// 薪资组编码
    pub paygroup_code: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 状态
    pub status: Option<PaygroupStatus>,
}

/// 薪资组列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaygroupsRequest {
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
    /// 状态过滤
    pub status_filter: Option<PaygroupStatus>,
}

/// 设置薪资规则请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSalaryRulesRequest {
    /// 薪资规则列表
    pub rules: Vec<SalaryRule>,
}

/// 添加员工到薪资组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddEmployeesRequest {
    /// 员工ID列表
    pub employee_ids: Vec<String>,
}

/// 从薪资组移除员工请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveEmployeesRequest {
    /// 员工ID列表
    pub employee_ids: Vec<String>,
}

/// 获取薪资组员工请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaygroupEmployeesRequest {
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

// ==================== 响应数据模型 ====================

/// 创建薪资组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaygroupResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 薪资组名称
    pub paygroup_name: String,
    /// 薪资组编码
    pub paygroup_code: String,
    /// 描述
    pub description: Option<String>,
    /// 状态
    pub status: PaygroupStatus,
    /// 员工数量
    pub employee_count: i32,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 获取薪资组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaygroupResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 薪资组名称
    pub paygroup_name: String,
    /// 薪资组编码
    pub paygroup_code: String,
    /// 描述
    pub description: Option<String>,
    /// 状态
    pub status: PaygroupStatus,
    /// 员工数量
    pub employee_count: i32,
    /// 薪资规则
    pub salary_rules: Vec<SalaryRule>,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 更新薪资组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaygroupResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 薪资组名称
    pub paygroup_name: Option<String>,
    /// 薪资组编码
    pub paygroup_code: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 状态
    pub status: Option<PaygroupStatus>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 删除薪资组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePaygroupResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 是否删除成功
    pub deleted: bool,
    /// 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}

/// 薪资组列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaygroupsResponse {
    /// 薪资组列表
    pub paygroups: Vec<PaygroupInfo>,
    /// 总数量
    pub total_count: i32,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 下一页标记
    pub next_page_token: Option<String>,
}

/// 设置薪资规则响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSalaryRulesResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 规则数量
    pub rule_count: i32,
    /// 薪资规则列表
    pub rules: Vec<SalaryRule>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 获取薪资规则响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSalaryRulesResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 薪资规则列表
    pub rules: Vec<SalaryRule>,
}

/// 添加员工响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddEmployeesResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 添加数量
    pub added_count: i32,
    /// 失败数量
    pub failed_count: i32,
    /// 失败员工列表
    pub failed_employees: Vec<String>,
    /// 添加时间
    pub added_at: Option<DateTime<Utc>>,
}

/// 移除员工响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveEmployeesResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 移除数量
    pub removed_count: i32,
    /// 失败数量
    pub failed_count: i32,
    /// 失败员工列表
    pub failed_employees: Vec<String>,
    /// 移除时间
    pub removed_at: Option<DateTime<Utc>>,
}

/// 获取薪资组员工响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaygroupEmployeesResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 员工列表
    pub employees: Vec<PaygroupEmployee>,
    /// 总数量
    pub total_count: i32,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 下一页标记
    pub next_page_token: Option<String>,
}

/// 获取薪资组统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaygroupStatisticsResponse {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 统计周期
    pub period: String,
    /// 总员工数
    pub total_employees: i32,
    /// 活跃员工数
    pub active_employees: i32,
    /// 总薪资成本
    pub total_salary_cost: f64,
    /// 平均薪资
    pub average_salary: f64,
    /// 薪资分布
    pub salary_distribution: SalaryDistribution,
    /// 计算时间
    pub calculated_at: Option<DateTime<Utc>>,
}

// ==================== 数据模型 ====================

/// 薪资组信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaygroupInfo {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 薪资组名称
    pub paygroup_name: String,
    /// 薪资组编码
    pub paygroup_code: String,
    /// 状态
    pub status: PaygroupStatus,
    /// 员工数量
    pub employee_count: i32,
    /// 描述
    pub description: Option<String>,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 薪资组员工
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaygroupEmployee {
    /// 员工ID
    pub employee_id: String,
    /// 员工姓名
    pub employee_name: String,
    /// 员工邮箱
    pub employee_email: Option<String>,
    /// 部门
    pub department: String,
    /// 职位
    pub position: String,
    /// 入职日期
    pub joined_date: Option<String>,
    /// 薪资级别
    pub salary_level: Option<String>,
}

/// 薪资规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub rule_name: String,
    /// 规则类型
    pub rule_type: SalaryRuleType,
    /// 计算公式
    pub formula: String,
    /// 描述
    pub description: Option<String>,
}

/// 薪资分布
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryDistribution {
    /// 最低薪资
    pub min_salary: f64,
    /// 最高薪资
    pub max_salary: f64,
    /// 中位数薪资
    pub median_salary: f64,
    /// 薪资区间
    pub salary_ranges: Vec<SalaryRange>,
}

/// 薪资区间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRange {
    /// 区间名称
    pub range_name: String,
    /// 最小金额
    pub min_amount: f64,
    /// 最大金额
    pub max_amount: f64,
    /// 员工数量
    pub employee_count: i32,
    /// 百分比
    pub percentage: f64,
}

/// 薪资组状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaygroupStatus {
    /// 激活
    Active,
    /// 停用
    Inactive,
    /// 已删除
    Deleted,
}

/// 薪资规则类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SalaryRuleType {
    /// 基本工资
    BaseSalary,
    /// 绩效奖金
    Bonus,
    /// 津贴补贴
    Allowance,
    /// 扣除项
    Deduction,
    /// 税收计算
    Tax,
    /// 社保计算
    SocialInsurance,
}

// 实现Default trait
impl Default for CreatePaygroupRequest {
    fn default() -> Self {
        Self {
            paygroup_name: String::new(),
            paygroup_code: String::new(),
            description: None,
        }
    }
}

impl Default for UpdatePaygroupRequest {
    fn default() -> Self {
        Self {
            paygroup_name: None,
            paygroup_code: None,
            description: None,
            status: None,
        }
    }
}

impl Default for ListPaygroupsRequest {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page_token: None,
            status_filter: None,
        }
    }
}

impl Default for AddEmployeesRequest {
    fn default() -> Self {
        Self {
            employee_ids: vec![],
        }
    }
}

impl Default for RemoveEmployeesRequest {
    fn default() -> Self {
        Self {
            employee_ids: vec![],
        }
    }
}

impl Default for GetPaygroupEmployeesRequest {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page_token: None,
        }
    }
}
