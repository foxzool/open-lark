//! 薪资计算服务
//!
//! 提供薪酬计算的完整功能：
//! - 薪资计算引擎
//! - 薪资项目计算
//! - 扣除项计算
//! - 税务和社保计算
//! - 薪资模拟和预测

use crate::core::config::Config;
use crate::core::SDKResult;
use crate::service::payroll::models::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 薪资计算服务
#[derive(Debug, Clone)]
pub struct CalculationService {
    config: Config,
}

impl CalculationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 薪资计算 ====================

    /// 执行薪资计算
    pub async fn calculate(
        &self,
        request: &SalaryCalculationRequest,
    ) -> SDKResult<SalaryCalculationResponse> {
        // 模拟实现
        let calculation_id = format!("calc_{}", chrono::Utc::now().timestamp());

        // 模拟计算结果
        let gross_salary =
            request.base_salary + request.allowances.iter().map(|a| a.amount).sum::<f64>();
        let tax_deduction = gross_salary * 0.1; // 简化的税收计算
        let social_insurance = gross_salary * 0.08; // 简化的社保计算
        let net_salary = gross_salary
            - tax_deduction
            - social_insurance
            - request.deductions.iter().map(|d| d.amount).sum::<f64>();

        Ok(SalaryCalculationResponse {
            calculation_id,
            employee_id: request.employee_id.clone(),
            calculation_period: request.calculation_period.clone(),
            gross_salary,
            net_salary,
            tax_deduction,
            social_insurance,
            other_deductions: request.deductions.iter().map(|d| d.amount).sum(),
            allowance_total: request.allowances.iter().map(|a| a.amount).sum(),
            calculation_details: vec![
                CalculationDetail {
                    item_name: "基本工资".to_string(),
                    item_code: "base_salary".to_string(),
                    amount: request.base_salary,
                    item_type: CalculationItemType::Income,
                    description: Some("员工基本工资".to_string()),
                },
                CalculationDetail {
                    item_name: "个人所得税".to_string(),
                    item_code: "income_tax".to_string(),
                    amount: tax_deduction,
                    item_type: CalculationItemType::Deduction,
                    description: Some("个人所得税扣除".to_string()),
                },
                CalculationDetail {
                    item_name: "社会保险".to_string(),
                    item_code: "social_insurance".to_string(),
                    amount: social_insurance,
                    item_type: CalculationItemType::Deduction,
                    description: Some("社会保险个人部分".to_string()),
                },
            ],
            calculated_at: Some(chrono::Utc::now()),
        })
    }

    /// 批量薪资计算
    pub async fn batch_calculate(
        &self,
        request: &BatchSalaryCalculationRequest,
    ) -> SDKResult<BatchSalaryCalculationResponse> {
        // 模拟批量计算
        let mut results = Vec::new();

        for employee_id in &request.employee_ids {
            let calc_request = SalaryCalculationRequest {
                employee_id: employee_id.clone(),
                calculation_period: request.calculation_period.clone(),
                base_salary: 8000.0, // 模拟数据
                allowances: vec![],
                deductions: vec![],
            };

            if let Ok(response) = self.calculate(&calc_request).await {
                results.push(response);
            }
        }

        Ok(BatchSalaryCalculationResponse {
            batch_id: format!("batch_{}", chrono::Utc::now().timestamp()),
            total_count: results.len() as i32,
            success_count: results.len() as i32,
            failure_count: 0,
            results,
            calculated_at: Some(chrono::Utc::now()),
        })
    }

    /// 薪资模拟计算
    pub async fn simulate(
        &self,
        request: &SalarySimulationRequest,
    ) -> SDKResult<SalarySimulationResponse> {
        // 模拟不同场景下的薪资计算
        let mut scenarios = Vec::new();

        // 场景1：当前薪资
        let current_calc = self
            .calculate(&SalaryCalculationRequest {
                employee_id: request.employee_id.clone(),
                calculation_period: request.calculation_period.clone(),
                base_salary: request.current_base_salary,
                allowances: request.current_allowances.clone(),
                deductions: request.current_deductions.clone(),
            })
            .await?;

        scenarios.push(SalaryScenario {
            scenario_name: "当前薪资".to_string(),
            scenario_type: ScenarioType::Current,
            gross_salary: current_calc.gross_salary,
            net_salary: current_calc.net_salary,
            description: Some("当前薪资结构".to_string()),
        });

        // 场景2：加薪10%
        let raise_calc = self
            .calculate(&SalaryCalculationRequest {
                employee_id: request.employee_id.clone(),
                calculation_period: request.calculation_period.clone(),
                base_salary: request.current_base_salary * 1.1,
                allowances: request.current_allowances.clone(),
                deductions: request.current_deductions.clone(),
            })
            .await?;

        scenarios.push(SalaryScenario {
            scenario_name: "加薪10%".to_string(),
            scenario_type: ScenarioType::Projection,
            gross_salary: raise_calc.gross_salary,
            net_salary: raise_calc.net_salary,
            description: Some("预期加薪10%后的薪资".to_string()),
        });

        Ok(SalarySimulationResponse {
            employee_id: request.employee_id.clone(),
            calculation_period: request.calculation_period.clone(),
            scenarios,
            simulated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取计算历史
    pub async fn get_calculation_history(
        &self,
        employee_id: &str,
        period: Option<&str>,
        page_size: i32,
        page_token: Option<&str>,
    ) -> SDKResult<CalculationHistoryResponse> {
        // 模拟历史记录
        let history = vec![
            CalculationHistory {
                calculation_id: "calc_001".to_string(),
                employee_id: employee_id.to_string(),
                calculation_period: "2024-01".to_string(),
                gross_salary: 10000.0,
                net_salary: 7500.0,
                calculation_date: "2024-01-15T00:00:00Z".to_string(),
                status: CalculationStatus::Completed,
            },
            CalculationHistory {
                calculation_id: "calc_002".to_string(),
                employee_id: employee_id.to_string(),
                calculation_period: "2024-02".to_string(),
                gross_salary: 10200.0,
                net_salary: 7650.0,
                calculation_date: "2024-02-15T00:00:00Z".to_string(),
                status: CalculationStatus::Completed,
            },
        ];

        Ok(CalculationHistoryResponse {
            history,
            total_count: history.len() as i32,
            has_more: Some(false),
            next_page_token: None,
        })
    }
}

// ==================== 请求数据模型 ====================

/// 薪资计算请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryCalculationRequest {
    /// 员工ID
    pub employee_id: String,
    /// 计算周期
    pub calculation_period: String,
    /// 基本工资
    pub base_salary: f64,
    /// 津贴项目
    pub allowances: Vec<AllowanceItem>,
    /// 扣除项目
    pub deductions: Vec<DeductionItem>,
}

/// 批量薪资计算请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSalaryCalculationRequest {
    /// 员工ID列表
    pub employee_ids: Vec<String>,
    /// 计算周期
    pub calculation_period: String,
}

/// 薪资模拟请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalarySimulationRequest {
    /// 员工ID
    pub employee_id: String,
    /// 计算周期
    pub calculation_period: String,
    /// 当前基本工资
    pub current_base_salary: f64,
    /// 当前津贴
    pub current_allowances: Vec<AllowanceItem>,
    /// 当前扣除
    pub current_deductions: Vec<DeductionItem>,
}

// ==================== 响应数据模型 ====================

/// 薪资计算响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryCalculationResponse {
    /// 计算ID
    pub calculation_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 计算周期
    pub calculation_period: String,
    /// 税前工资
    pub gross_salary: f64,
    /// 税后工资
    pub net_salary: f64,
    /// 税收扣除
    pub tax_deduction: f64,
    /// 社保扣除
    pub social_insurance: f64,
    /// 其他扣除
    pub other_deductions: f64,
    /// 津贴总额
    pub allowance_total: f64,
    /// 计算明细
    pub calculation_details: Vec<CalculationDetail>,
    /// 计算时间
    pub calculated_at: Option<DateTime<Utc>>,
}

/// 批量薪资计算响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSalaryCalculationResponse {
    /// 批次ID
    pub batch_id: String,
    /// 总数量
    pub total_count: i32,
    /// 成功数量
    pub success_count: i32,
    /// 失败数量
    pub failure_count: i32,
    /// 计算结果
    pub results: Vec<SalaryCalculationResponse>,
    /// 计算时间
    pub calculated_at: Option<DateTime<Utc>>,
}

/// 薪资模拟响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalarySimulationResponse {
    /// 员工ID
    pub employee_id: String,
    /// 计算周期
    pub calculation_period: String,
    /// 模拟场景
    pub scenarios: Vec<SalaryScenario>,
    /// 模拟时间
    pub simulated_at: Option<DateTime<Utc>>,
}

/// 计算历史响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationHistoryResponse {
    /// 历史记录
    pub history: Vec<CalculationHistory>,
    /// 总数量
    pub total_count: i32,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 下一页标记
    pub next_page_token: Option<String>,
}

// ==================== 数据模型 ====================

/// 计算明细
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationDetail {
    /// 项目名称
    pub item_name: String,
    /// 项目代码
    pub item_code: String,
    /// 金额
    pub amount: f64,
    /// 项目类型
    pub item_type: CalculationItemType,
    /// 描述
    pub description: Option<String>,
}

/// 薪资场景
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryScenario {
    /// 场景名称
    pub scenario_name: String,
    /// 场景类型
    pub scenario_type: ScenarioType,
    /// 税前工资
    pub gross_salary: f64,
    /// 税后工资
    pub net_salary: f64,
    /// 描述
    pub description: Option<String>,
}

/// 计算历史
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationHistory {
    /// 计算ID
    pub calculation_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 计算周期
    pub calculation_period: String,
    /// 税前工资
    pub gross_salary: f64,
    /// 税后工资
    pub net_salary: f64,
    /// 计算日期
    pub calculation_date: String,
    /// 计算状态
    pub status: CalculationStatus,
}

/// 津贴项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowanceItem {
    /// 项目名称
    pub name: String,
    /// 项目代码
    pub code: String,
    /// 金额
    pub amount: f64,
    /// 是否应税
    pub taxable: bool,
}

/// 扣除项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeductionItem {
    /// 项目名称
    pub name: String,
    /// 项目代码
    pub code: String,
    /// 金额
    pub amount: f64,
    /// 扣除类型
    pub deduction_type: String,
}

/// 计算项目类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculationItemType {
    /// 收入项目
    Income,
    /// 扣除项目
    Deduction,
}

/// 场景类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioType {
    /// 当前薪资
    Current,
    /// 预测薪资
    Projection,
    /// 模拟场景
    Simulation,
}

/// 计算状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculationStatus {
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 失败
    Failed,
}

// 实现Default trait
impl Default for SalaryCalculationRequest {
    fn default() -> Self {
        Self {
            employee_id: String::new(),
            calculation_period: String::new(),
            base_salary: 0.0,
            allowances: vec![],
            deductions: vec![],
        }
    }
}

impl Default for BatchSalaryCalculationRequest {
    fn default() -> Self {
        Self {
            employee_ids: vec![],
            calculation_period: String::new(),
        }
    }
}

impl Default for SalarySimulationRequest {
    fn default() -> Self {
        Self {
            employee_id: String::new(),
            calculation_period: String::new(),
            current_base_salary: 0.0,
            current_allowances: vec![],
            current_deductions: vec![],
        }
    }
}
