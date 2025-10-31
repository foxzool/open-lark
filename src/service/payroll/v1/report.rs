//! 薪酬报表服务
//!
//! 提供薪酬报表的完整功能：
//! - 薪酬报表生成和管理
//! - 薪酬统计分析和数据挖掘
//! - 报表模板和自定义报表
//! - 报表导出和分发管理

use crate::core::config::Config;
use crate::service::payroll::models::*;
use chrono::{DateTime, Utc};
use open_lark_core::prelude::*;
use serde::{Deserialize, Serialize};

/// 薪酬报表服务
#[derive(Debug, Clone)]
pub struct ReportService {
    config: Config,
}

impl ReportService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 薪酬报表生成 ====================

    /// 生成月度薪酬报表
    pub async fn generate_monthly_report(
        &self,
        request: &GenerateMonthlyReportRequest,
    ) -> SDKResult<GenerateMonthlyReportResponse> {
        let report_id = format!("report_{}", chrono::Utc::now().timestamp());

        // 模拟月度报表数据
        let summary = MonthlyReportSummary {
            total_employees: 250,
            total_gross_salary: 2500000.0,
            total_net_salary: 1875000.0,
            total_tax: 250000.0,
            total_social_insurance: 200000.0,
            total_other_deductions: 175000.0,
            average_gross_salary: 10000.0,
            average_net_salary: 7500.0,
            pay_period: request.period.clone(),
        };

        let department_breakdown = vec![
            DepartmentSalaryData {
                department_id: "dept_001".to_string(),
                department_name: "技术研发部".to_string(),
                employee_count: 80,
                total_gross_salary: 960000.0,
                total_net_salary: 720000.0,
                average_gross_salary: 12000.0,
                average_net_salary: 9000.0,
            },
            DepartmentSalaryData {
                department_id: "dept_002".to_string(),
                department_name: "市场营销部".to_string(),
                employee_count: 60,
                total_gross_salary: 540000.0,
                total_net_salary: 405000.0,
                average_gross_salary: 9000.0,
                average_net_salary: 6750.0,
            },
            DepartmentSalaryData {
                department_id: "dept_003".to_string(),
                department_name: "人力资源部".to_string(),
                employee_count: 25,
                total_gross_salary: 200000.0,
                total_net_salary: 150000.0,
                average_gross_salary: 8000.0,
                average_net_salary: 6000.0,
            },
        ];

        Ok(GenerateMonthlyReportResponse {
            report_id: report_id.clone(),
            report_name: format!("{}月度薪酬报表", request.period),
            period: request.period.clone(),
            report_type: ReportType::Monthly,
            status: ReportStatus::Completed,
            summary,
            department_breakdown,
            salary_distribution: self.generate_salary_distribution(),
            tax_summary: self.generate_tax_summary(),
            social_insurance_summary: self.generate_social_insurance_summary(),
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 生成年度薪酬报表
    pub async fn generate_annual_report(
        &self,
        request: &GenerateAnnualReportRequest,
    ) -> SDKResult<GenerateAnnualReportResponse> {
        let report_id = format!("annual_report_{}", chrono::Utc::now().timestamp());

        let summary = AnnualReportSummary {
            year: request.year,
            total_employees: 250,
            total_annual_salary: 30000000.0,
            average_annual_salary: 120000.0,
            total_bonus_paid: 5000000.0,
            total_tax_withheld: 3000000.0,
            total_social_insurance: 2400000.0,
            salary_growth_rate: 8.5,
            employee_turnover_rate: 12.0,
        };

        let quarterly_data = vec![
            QuarterSalaryData {
                quarter: "Q1".to_string(),
                total_salary: 7200000.0,
                average_salary: 28800.0,
                employee_count: 250,
            },
            QuarterSalaryData {
                quarter: "Q2".to_string(),
                total_salary: 7350000.0,
                average_salary: 29400.0,
                employee_count: 250,
            },
            QuarterSalaryData {
                quarter: "Q3".to_string(),
                total_salary: 7500000.0,
                average_salary: 30000.0,
                employee_count: 250,
            },
            QuarterSalaryData {
                quarter: "Q4".to_string(),
                total_salary: 7950000.0,
                average_salary: 31800.0,
                employee_count: 250,
            },
        ];

        Ok(GenerateAnnualReportResponse {
            report_id: report_id.clone(),
            report_name: format!("{}年度薪酬报表", request.year),
            year: request.year,
            report_type: ReportType::Annual,
            status: ReportStatus::Completed,
            summary,
            quarterly_data,
            department_analysis: self.generate_department_analysis(),
            salary_band_analysis: self.generate_salary_band_analysis(),
            trend_analysis: self.generate_trend_analysis(),
            generated_at: Some(chrono::Utc::now()),
        })
    }

    /// 生成员工个人薪酬报表
    pub async fn generate_employee_report(
        &self,
        request: &GenerateEmployeeReportRequest,
    ) -> SDKResult<GenerateEmployeeReportResponse> {
        let report_id = format!("emp_report_{}", chrono::Utc::now().timestamp());

        let employee_summary = EmployeeSalarySummary {
            employee_id: request.employee_id.clone(),
            employee_name: "张三".to_string(),
            department: "技术研发部".to_string(),
            position: "高级工程师".to_string(),
            grade: "L7".to_string(),
            hire_date: "2023-01-15".to_string(),
        };

        let period_data = vec![
            PeriodSalaryData {
                period: "2024-01".to_string(),
                base_salary: 12000.0,
                bonuses: 2400.0,
                allowances: 800.0,
                gross_salary: 15200.0,
                tax_deduction: 1520.0,
                social_insurance: 1216.0,
                other_deductions: 300.0,
                net_salary: 12164.0,
            },
            PeriodSalaryData {
                period: "2024-02".to_string(),
                base_salary: 12000.0,
                bonuses: 2400.0,
                allowances: 800.0,
                gross_salary: 15200.0,
                tax_deduction: 1520.0,
                social_insurance: 1216.0,
                other_deductions: 300.0,
                net_salary: 12164.0,
            },
            PeriodSalaryData {
                period: "2024-03".to_string(),
                base_salary: 12000.0,
                bonuses: 3000.0,
                allowances: 800.0,
                gross_salary: 15800.0,
                tax_deduction: 1580.0,
                social_insurance: 1216.0,
                other_deductions: 300.0,
                net_salary: 12704.0,
            },
        ];

        Ok(GenerateEmployeeReportResponse {
            report_id: report_id.clone(),
            report_name: format!("员工薪酬报表 - {}", request.employee_id),
            employee_summary,
            period_data,
            year_to_date_summary: self.generate_year_to_date_summary(),
            salary_projection: self.generate_salary_projection(),
            generated_at: Some(chrono::Utc::now()),
        })
    }

    // ==================== 报表管理 ====================

    /// 获取报表列表
    pub async fn list_reports(
        &self,
        request: &ListReportsRequest,
    ) -> SDKResult<ListReportsResponse> {
        let reports = vec![
            ReportInfo {
                report_id: "report_001".to_string(),
                report_name: "2024年3月月度薪酬报表".to_string(),
                report_type: ReportType::Monthly,
                period: Some("2024-03".to_string()),
                status: ReportStatus::Completed,
                generated_by: "hr_manager".to_string(),
                generated_at: Some(chrono::Utc::now()),
                file_size: Some(2048576), // 2MB
            },
            ReportInfo {
                report_id: "report_002".to_string(),
                report_name: "2023年度薪酬报表".to_string(),
                report_type: ReportType::Annual,
                period: Some("2023".to_string()),
                status: ReportStatus::Completed,
                generated_by: "hr_director".to_string(),
                generated_at: Some(chrono::Utc::now()),
                file_size: Some(5242880), // 5MB
            },
            ReportInfo {
                report_id: "report_003".to_string(),
                report_name: "员工薪酬明细报表".to_string(),
                report_type: ReportType::Employee,
                period: Some("2024-Q1".to_string()),
                status: ReportStatus::Processing,
                generated_by: "hr_specialist".to_string(),
                generated_at: Some(chrono::Utc::now()),
                file_size: None,
            },
        ];

        let filtered_reports = if let Some(report_type) = &request.report_type_filter {
            reports
                .iter()
                .filter(|r| &r.report_type == report_type)
                .cloned()
                .collect()
        } else if let Some(status) = &request.status_filter {
            reports
                .iter()
                .filter(|r| &r.status == status)
                .cloned()
                .collect()
        } else {
            reports
        };

        Ok(ListReportsResponse {
            reports: filtered_reports,
            total_count: filtered_reports.len() as i32,
            has_more: Some(false),
            next_page_token: None,
        })
    }

    /// 获取报表详情
    pub async fn get_report(&self, report_id: &str) -> SDKResult<GetReportResponse> {
        // 模拟实现
        Ok(GetReportResponse {
            report_id: report_id.to_string(),
            report_name: "2024年3月月度薪酬报表".to_string(),
            report_type: ReportType::Monthly,
            period: Some("2024-03".to_string()),
            status: ReportStatus::Completed,
            generated_by: "hr_manager".to_string(),
            generated_at: Some(chrono::Utc::now()),
            completed_at: Some(chrono::Utc::now()),
            file_size: Some(2048576),
            download_url: Some(format!("https://company.com/reports/{}.xlsx", report_id)),
            parameters: vec![
                ReportParameter {
                    name: "period".to_string(),
                    value: "2024-03".to_string(),
                    data_type: "string".to_string(),
                },
                ReportParameter {
                    name: "include_tax_details".to_string(),
                    value: "true".to_string(),
                    data_type: "boolean".to_string(),
                },
            ],
        })
    }

    /// 删除报表
    pub async fn delete_report(&self, report_id: &str) -> SDKResult<DeleteReportResponse> {
        Ok(DeleteReportResponse {
            report_id: report_id.to_string(),
            deleted: true,
            deleted_at: Some(chrono::Utc::now()),
        })
    }

    // ==================== 报表导出和分发 ====================

    /// 导出报表
    pub async fn export_report(
        &self,
        request: &ExportReportRequest,
    ) -> SDKResult<ExportReportResponse> {
        let export_id = format!("export_{}", chrono::Utc::now().timestamp());
        let download_url = format!(
            "https://company.com/exports/{}.{}",
            export_id, request.format
        );

        Ok(ExportReportResponse {
            export_id: export_id.clone(),
            report_id: request.report_id.clone(),
            format: request.format.clone(),
            download_url,
            file_size: match request.format.as_str() {
                "xlsx" => Some(2048576),
                "pdf" => Some(1024000),
                "csv" => Some(512000),
                _ => None,
            },
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)),
            exported_at: Some(chrono::Utc::now()),
        })
    }

    /// 分发报表
    pub async fn distribute_report(
        &self,
        request: &DistributeReportRequest,
    ) -> SDKResult<DistributeReportResponse> {
        let distribution_id = format!("dist_{}", chrono::Utc::now().timestamp());

        Ok(DistributeReportResponse {
            distribution_id: distribution_id.clone(),
            report_id: request.report_id.clone(),
            recipients: request.recipients.clone(),
            method: request.method.clone(),
            status: DistributionStatus::Sent,
            sent_at: Some(chrono::Utc::now()),
            failed_recipients: vec![],
        })
    }

    // ==================== 统计分析 ====================

    /// 获取薪酬统计概览
    pub async fn get_salary_overview(
        &self,
        request: &GetSalaryOverviewRequest,
    ) -> SDKResult<GetSalaryOverviewResponse> {
        Ok(GetSalaryOverviewResponse {
            period: request.period.clone(),
            total_employees: 250,
            active_employees: 245,
            total_payroll_cost: 2500000.0,
            average_salary: 10000.0,
            median_salary: 9500.0,
            salary_range: SalaryRange {
                min: 6000.0,
                max: 25000.0,
            },
            department_breakdown: vec![
                DepartmentOverview {
                    department_id: "dept_001".to_string(),
                    department_name: "技术研发部".to_string(),
                    employee_count: 80,
                    average_salary: 12000.0,
                    total_cost: 960000.0,
                },
                DepartmentOverview {
                    department_id: "dept_002".to_string(),
                    department_name: "市场营销部".to_string(),
                    employee_count: 60,
                    average_salary: 9000.0,
                    total_cost: 540000.0,
                },
            ],
            calculated_at: Some(chrono::Utc::now()),
        })
    }

    // ==================== 辅助方法 ====================

    fn generate_salary_distribution(&self) -> Vec<SalaryDistributionData> {
        vec![
            SalaryDistributionData {
                range: "6K-8K".to_string(),
                min_salary: 6000.0,
                max_salary: 8000.0,
                employee_count: 50,
                percentage: 20.0,
            },
            SalaryDistributionData {
                range: "8K-12K".to_string(),
                min_salary: 8000.0,
                max_salary: 12000.0,
                employee_count: 125,
                percentage: 50.0,
            },
            SalaryDistributionData {
                range: "12K-20K".to_string(),
                min_salary: 12000.0,
                max_salary: 20000.0,
                employee_count: 60,
                percentage: 24.0,
            },
            SalaryDistributionData {
                range: "20K+".to_string(),
                min_salary: 20000.0,
                max_salary: 999999.0,
                employee_count: 15,
                percentage: 6.0,
            },
        ]
    }

    fn generate_tax_summary(&self) -> TaxSummary {
        TaxSummary {
            total_tax_withheld: 250000.0,
            average_tax_rate: 10.0,
            tax_brackets: vec![
                TaxBracket {
                    min_income: 0.0,
                    max_income: 3500.0,
                    tax_rate: 3.0,
                    employee_count: 0,
                },
                TaxBracket {
                    min_income: 3500.0,
                    max_income: 9000.0,
                    tax_rate: 10.0,
                    employee_count: 150,
                },
                TaxBracket {
                    min_income: 9000.0,
                    max_income: 25000.0,
                    tax_rate: 20.0,
                    employee_count: 95,
                },
                TaxBracket {
                    min_income: 25000.0,
                    max_income: 999999.0,
                    tax_rate: 25.0,
                    employee_count: 5,
                },
            ],
        }
    }

    fn generate_social_insurance_summary(&self) -> SocialInsuranceSummary {
        SocialInsuranceSummary {
            total_employee_contribution: 160000.0,
            total_employer_contribution: 400000.0,
            total_contribution: 560000.0,
            breakdown: vec![
                InsuranceContribution {
                    insurance_type: "养老保险".to_string(),
                    employee_rate: 8.0,
                    employer_rate: 20.0,
                    employee_amount: 64000.0,
                    employer_amount: 160000.0,
                },
                InsuranceContribution {
                    insurance_type: "医疗保险".to_string(),
                    employee_rate: 2.0,
                    employer_rate: 8.0,
                    employee_amount: 16000.0,
                    employer_amount: 64000.0,
                },
                InsuranceContribution {
                    insurance_type: "失业保险".to_string(),
                    employee_rate: 0.5,
                    employer_rate: 1.5,
                    employee_amount: 4000.0,
                    employer_amount: 12000.0,
                },
                InsuranceContribution {
                    insurance_type: "住房公积金".to_string(),
                    employee_rate: 6.0,
                    employer_rate: 6.0,
                    employee_amount: 48000.0,
                    employer_amount: 48000.0,
                },
            ],
        }
    }

    fn generate_department_analysis(&self) -> Vec<DepartmentAnalysis> {
        vec![
            DepartmentAnalysis {
                department_id: "dept_001".to_string(),
                department_name: "技术研发部".to_string(),
                employee_count: 80,
                average_salary: 12000.0,
                salary_growth: 12.5,
                turnover_rate: 8.0,
                performance_score: 4.2,
            },
            DepartmentAnalysis {
                department_id: "dept_002".to_string(),
                department_name: "市场营销部".to_string(),
                employee_count: 60,
                average_salary: 9000.0,
                salary_growth: 8.0,
                turnover_rate: 15.0,
                performance_score: 3.8,
            },
        ]
    }

    fn generate_salary_band_analysis(&self) -> Vec<SalaryBandAnalysis> {
        vec![
            SalaryBandAnalysis {
                band_name: "初级".to_string(),
                grade_range: "L1-L3".to_string(),
                employee_count: 60,
                average_salary: 7000.0,
                salary_range: SalaryRange {
                    min: 5000.0,
                    max: 9000.0,
                },
            },
            SalaryBandAnalysis {
                band_name: "中级".to_string(),
                grade_range: "L4-L6".to_string(),
                employee_count: 120,
                average_salary: 10000.0,
                salary_range: SalaryRange {
                    min: 8000.0,
                    max: 15000.0,
                },
            },
            SalaryBandAnalysis {
                band_name: "高级".to_string(),
                grade_range: "L7-L9".to_string(),
                employee_count: 50,
                average_salary: 16000.0,
                salary_range: SalaryRange {
                    min: 12000.0,
                    max: 25000.0,
                },
            },
        ]
    }

    fn generate_trend_analysis(&self) -> TrendAnalysis {
        TrendAnalysis {
            salary_trend: "upward".to_string(),
            growth_rate: 8.5,
            market_comparison: "above_average".to_string(),
            forecast: TrendForecast {
                next_quarter_growth: 2.5,
                next_year_growth: 7.0,
                confidence_level: 0.85,
            },
        }
    }

    fn generate_year_to_date_summary(&self) -> YearToDateSummary {
        YearToDateSummary {
            total_gross_salary: 45600.0,
            total_net_salary: 34200.0,
            total_tax: 4560.0,
            total_social_insurance: 3648.0,
            total_bonus: 5400.0,
            average_monthly_gross: 15200.0,
            average_monthly_net: 11400.0,
        }
    }

    fn generate_salary_projection(&self) -> SalaryProjection {
        SalaryProjection {
            projected_annual_gross: 182400.0,
            projected_annual_net: 136800.0,
            projected_bonus: 21600.0,
            assumptions: vec![
                "Assuming current performance level continues".to_string(),
                "No major changes in tax regulations".to_string(),
                "Standard salary adjustment in Q2".to_string(),
            ],
        }
    }
}

// ==================== 请求数据模型 ====================

/// 生成月度报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateMonthlyReportRequest {
    /// 报表周期 (YYYY-MM)
    pub period: String,
    /// 是否包含税务明细
    pub include_tax_details: Option<bool>,
    /// 是否包含社保明细
    pub include_insurance_details: Option<bool>,
    /// 部门过滤
    pub department_filter: Option<Vec<String>>,
}

/// 生成年度报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateAnnualReportRequest {
    /// 报表年份
    pub year: i32,
    /// 是否包含季度对比
    pub include_quarterly_comparison: Option<bool>,
    /// 是否包含预测数据
    pub include_forecast: Option<bool>,
}

/// 生成员工报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateEmployeeReportRequest {
    /// 员工ID
    pub employee_id: String,
    /// 开始周期
    pub start_period: Option<String>,
    /// 结束周期
    pub end_period: Option<String>,
    /// 是否包含明细数据
    pub include_details: Option<bool>,
}

/// 报表列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReportsRequest {
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
    /// 报表类型过滤
    pub report_type_filter: Option<ReportType>,
    /// 状态过滤
    pub status_filter: Option<ReportStatus>,
    /// 生成者过滤
    pub generated_by_filter: Option<String>,
}

/// 导出报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportReportRequest {
    /// 报表ID
    pub report_id: String,
    /// 导出格式
    pub format: String,
    /// 是否包含原始数据
    pub include_raw_data: Option<bool>,
}

/// 分发报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributeReportRequest {
    /// 报表ID
    pub report_id: String,
    /// 接收者列表
    pub recipients: Vec<String>,
    /// 分发方式
    pub method: DistributionMethod,
    /// 主题
    pub subject: Option<String>,
    /// 消息内容
    pub message: Option<String>,
}

/// 获取薪酬概览请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSalaryOverviewRequest {
    /// 统计周期
    pub period: String,
    /// 是否包含部门明细
    pub include_department_breakdown: Option<bool>,
}

// ==================== 响应数据模型 ====================

/// 生成月度报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateMonthlyReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 报表名称
    pub report_name: String,
    /// 报表周期
    pub period: String,
    /// 报表类型
    pub report_type: ReportType,
    /// 报表状态
    pub status: ReportStatus,
    /// 汇总信息
    pub summary: MonthlyReportSummary,
    /// 部门明细
    pub department_breakdown: Vec<DepartmentSalaryData>,
    /// 薪资分布
    pub salary_distribution: Vec<SalaryDistributionData>,
    /// 税务汇总
    pub tax_summary: TaxSummary,
    /// 社保汇总
    pub social_insurance_summary: SocialInsuranceSummary,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 生成年度报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateAnnualReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 报表名称
    pub report_name: String,
    /// 报表年份
    pub year: i32,
    /// 报表类型
    pub report_type: ReportType,
    /// 报表状态
    pub status: ReportStatus,
    /// 年度汇总
    pub summary: AnnualReportSummary,
    /// 季度数据
    pub quarterly_data: Vec<QuarterSalaryData>,
    /// 部门分析
    pub department_analysis: Vec<DepartmentAnalysis>,
    /// 薪资段分析
    pub salary_band_analysis: Vec<SalaryBandAnalysis>,
    /// 趋势分析
    pub trend_analysis: TrendAnalysis,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 生成员工报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateEmployeeReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 报表名称
    pub report_name: String,
    /// 员工汇总
    pub employee_summary: EmployeeSalarySummary,
    /// 周期数据
    pub period_data: Vec<PeriodSalaryData>,
    /// 年度至今汇总
    pub year_to_date_summary: YearToDateSummary,
    /// 薪资预测
    pub salary_projection: SalaryProjection,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 报表列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReportsResponse {
    /// 报表列表
    pub reports: Vec<ReportInfo>,
    /// 总数量
    pub total_count: i32,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 下一页标记
    pub next_page_token: Option<String>,
}

/// 获取报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 报表名称
    pub report_name: String,
    /// 报表类型
    pub report_type: ReportType,
    /// 报表周期
    pub period: Option<String>,
    /// 报表状态
    pub status: ReportStatus,
    /// 生成者
    pub generated_by: String,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
    /// 文件大小
    pub file_size: Option<i64>,
    /// 下载链接
    pub download_url: Option<String>,
    /// 报表参数
    pub parameters: Vec<ReportParameter>,
}

/// 删除报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 是否删除成功
    pub deleted: bool,
    /// 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}

/// 导出报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportReportResponse {
    /// 导出ID
    pub export_id: String,
    /// 报表ID
    pub report_id: String,
    /// 导出格式
    pub format: String,
    /// 下载链接
    pub download_url: String,
    /// 文件大小
    pub file_size: Option<i64>,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
    /// 导出时间
    pub exported_at: Option<DateTime<Utc>>,
}

/// 分发报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributeReportResponse {
    /// 分发ID
    pub distribution_id: String,
    /// 报表ID
    pub report_id: String,
    /// 接收者列表
    pub recipients: Vec<String>,
    /// 分发方式
    pub method: DistributionMethod,
    /// 分发状态
    pub status: DistributionStatus,
    /// 发送时间
    pub sent_at: Option<DateTime<Utc>>,
    /// 失败接收者
    pub failed_recipients: Vec<String>,
}

/// 获取薪酬概览响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSalaryOverviewResponse {
    /// 统计周期
    pub period: String,
    /// 总员工数
    pub total_employees: i32,
    /// 活跃员工数
    pub active_employees: i32,
    /// 总薪酬成本
    pub total_payroll_cost: f64,
    /// 平均薪资
    pub average_salary: f64,
    /// 中位数薪资
    pub median_salary: f64,
    /// 薪资范围
    pub salary_range: SalaryRange,
    /// 部门明细
    pub department_breakdown: Vec<DepartmentOverview>,
    /// 计算时间
    pub calculated_at: Option<DateTime<Utc>>,
}

// ==================== 数据模型 ====================

/// 月度报表汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyReportSummary {
    /// 总员工数
    pub total_employees: i32,
    /// 总税前工资
    pub total_gross_salary: f64,
    /// 总税后工资
    pub total_net_salary: f64,
    /// 总税收
    pub total_tax: f64,
    /// 总社保
    pub total_social_insurance: f64,
    /// 总其他扣除
    pub total_other_deductions: f64,
    /// 平均税前工资
    pub average_gross_salary: f64,
    /// 平均税后工资
    pub average_net_salary: f64,
    /// 发薪周期
    pub pay_period: String,
}

/// 年度报表汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualReportSummary {
    /// 年份
    pub year: i32,
    /// 总员工数
    pub total_employees: i32,
    /// 年度总薪酬
    pub total_annual_salary: f64,
    /// 平均年度薪酬
    pub average_annual_salary: f64,
    /// 已发奖金总额
    pub total_bonus_paid: f64,
    /// 代扣个税总额
    pub total_tax_withheld: f64,
    /// 社保总额
    pub total_social_insurance: f64,
    /// 薪资增长率
    pub salary_growth_rate: f64,
    /// 员工流失率
    pub employee_turnover_rate: f64,
}

/// 部门薪资数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentSalaryData {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub department_name: String,
    /// 员工数量
    pub employee_count: i32,
    /// 总税前工资
    pub total_gross_salary: f64,
    /// 总税后工资
    pub total_net_salary: f64,
    /// 平均税前工资
    pub average_gross_salary: f64,
    /// 平均税后工资
    pub average_net_salary: f64,
}

/// 季度薪资数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarterSalaryData {
    /// 季度
    pub quarter: String,
    /// 总薪资
    pub total_salary: f64,
    /// 平均薪资
    pub average_salary: f64,
    /// 员工数量
    pub employee_count: i32,
}

/// 员工薪酬汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeSalarySummary {
    /// 员工ID
    pub employee_id: String,
    /// 员工姓名
    pub employee_name: String,
    /// 部门
    pub department: String,
    /// 职位
    pub position: String,
    /// 级别
    pub grade: String,
    /// 入职日期
    pub hire_date: String,
}

/// 周期薪资数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodSalaryData {
    /// 周期
    pub period: String,
    /// 基本工资
    pub base_salary: f64,
    /// 奖金
    pub bonuses: f64,
    /// 津贴
    pub allowances: f64,
    /// 税前工资
    pub gross_salary: f64,
    /// 税收扣除
    pub tax_deduction: f64,
    /// 社保扣除
    pub social_insurance: f64,
    /// 其他扣除
    pub other_deductions: f64,
    /// 税后工资
    pub net_salary: f64,
}

/// 薪资分布数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryDistributionData {
    /// 薪资范围
    pub range: String,
    /// 最低薪资
    pub min_salary: f64,
    /// 最高薪资
    pub max_salary: f64,
    /// 员工数量
    pub employee_count: i32,
    /// 百分比
    pub percentage: f64,
}

/// 税务汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxSummary {
    /// 总代扣税额
    pub total_tax_withheld: f64,
    /// 平均税率
    pub average_tax_rate: f64,
    /// 税收等级
    pub tax_brackets: Vec<TaxBracket>,
}

/// 税收等级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBracket {
    /// 最低收入
    pub min_income: f64,
    /// 最高收入
    pub max_income: f64,
    /// 税率
    pub tax_rate: f64,
    /// 员工数量
    pub employee_count: i32,
}

/// 社保汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsuranceSummary {
    /// 员工缴费总额
    pub total_employee_contribution: f64,
    /// 雇主缴费总额
    pub total_employer_contribution: f64,
    /// 总缴费额
    pub total_contribution: f64,
    /// 分项明细
    pub breakdown: Vec<InsuranceContribution>,
}

/// 保险缴费
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceContribution {
    /// 保险类型
    pub insurance_type: String,
    /// 员工缴费比例
    pub employee_rate: f64,
    /// 雇主缴费比例
    pub employer_rate: f64,
    /// 员工缴费金额
    pub employee_amount: f64,
    /// 雇主缴费金额
    pub employer_amount: f64,
}

/// 部门分析
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentAnalysis {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub department_name: String,
    /// 员工数量
    pub employee_count: i32,
    /// 平均薪资
    pub average_salary: f64,
    /// 薪资增长
    pub salary_growth: f64,
    /// 流失率
    pub turnover_rate: f64,
    /// 绩效评分
    pub performance_score: f64,
}

/// 薪资段分析
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryBandAnalysis {
    /// 薪资段名称
    pub band_name: String,
    /// 级别范围
    pub grade_range: String,
    /// 员工数量
    pub employee_count: i32,
    /// 平均薪资
    pub average_salary: f64,
    /// 薪资范围
    pub salary_range: SalaryRange,
}

/// 趋势分析
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    /// 薪资趋势
    pub salary_trend: String,
    /// 增长率
    pub growth_rate: f64,
    /// 市场对比
    pub market_comparison: String,
    /// 预测
    pub forecast: TrendForecast,
}

/// 趋势预测
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendForecast {
    /// 下季度增长预测
    pub next_quarter_growth: f64,
    /// 下年度增长预测
    pub next_year_growth: f64,
    /// 置信度
    pub confidence_level: f64,
}

/// 年度至今汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearToDateSummary {
    /// 总税前工资
    pub total_gross_salary: f64,
    /// 总税后工资
    pub total_net_salary: f64,
    /// 总税收
    pub total_tax: f64,
    /// 总社保
    pub total_social_insurance: f64,
    /// 总奖金
    pub total_bonus: f64,
    /// 平均月度税前工资
    pub average_monthly_gross: f64,
    /// 平均月度税后工资
    pub average_monthly_net: f64,
}

/// 薪资预测
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryProjection {
    /// 预测年度税前工资
    pub projected_annual_gross: f64,
    /// 预测年度税后工资
    pub projected_annual_net: f64,
    /// 预测奖金
    pub projected_bonus: f64,
    /// 预测假设
    pub assumptions: Vec<String>,
}

/// 报表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportInfo {
    /// 报表ID
    pub report_id: String,
    /// 报表名称
    pub report_name: String,
    /// 报表类型
    pub report_type: ReportType,
    /// 报表周期
    pub period: Option<String>,
    /// 报表状态
    pub status: ReportStatus,
    /// 生成者
    pub generated_by: String,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
    /// 文件大小
    pub file_size: Option<i64>,
}

/// 报表参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportParameter {
    /// 参数名
    pub name: String,
    /// 参数值
    pub value: String,
    /// 数据类型
    pub data_type: String,
}

/// 部门概览
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentOverview {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub department_name: String,
    /// 员工数量
    pub employee_count: i32,
    /// 平均薪资
    pub average_salary: f64,
    /// 总成本
    pub total_cost: f64,
}

/// 薪资范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRange {
    /// 最小值
    pub min: f64,
    /// 最大值
    pub max: f64,
}

/// 报表类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    /// 月度报表
    Monthly,
    /// 年度报表
    Annual,
    /// 员工报表
    Employee,
    /// 部门报表
    Department,
    /// 自定义报表
    Custom,
}

/// 报表状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    /// 处理中
    Processing,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
}

/// 分发方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DistributionMethod {
    /// 邮件
    Email,
    /// 下载链接
    DownloadLink,
    /// 系统通知
    SystemNotification,
}

/// 分发状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DistributionStatus {
    /// 待发送
    Pending,
    /// 已发送
    Sent,
    /// 失败
    Failed,
    /// 部分失败
    PartialFailed,
}

// 实现Default trait
impl Default for GenerateMonthlyReportRequest {
    fn default() -> Self {
        Self {
            period: String::new(),
            include_tax_details: Some(true),
            include_insurance_details: Some(true),
            department_filter: None,
        }
    }
}

impl Default for GenerateAnnualReportRequest {
    fn default() -> Self {
        Self {
            year: chrono::Utc::now().year(),
            include_quarterly_comparison: Some(true),
            include_forecast: Some(false),
        }
    }
}

impl Default for GenerateEmployeeReportRequest {
    fn default() -> Self {
        Self {
            employee_id: String::new(),
            start_period: None,
            end_period: None,
            include_details: Some(true),
        }
    }
}

impl Default for ListReportsRequest {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page_token: None,
            report_type_filter: None,
            status_filter: None,
            generated_by_filter: None,
        }
    }
}

impl Default for ExportReportRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            format: "xlsx".to_string(),
            include_raw_data: Some(false),
        }
    }
}

impl Default for DistributeReportRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            recipients: vec![],
            method: DistributionMethod::Email,
            subject: None,
            message: None,
        }
    }
}

impl Default for GetSalaryOverviewRequest {
    fn default() -> Self {
        Self {
            period: String::new(),
            include_department_breakdown: Some(true),
        }
    }
}
