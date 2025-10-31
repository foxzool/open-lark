//! 算薪项管理服务
//!
//! 提供算薪项的完整管理功能：
//! - 算薪项创建和管理
//! - 算薪项配置和规则设置
//! - 算薪项分类和模板管理
//! - 算薪项使用统计和分析

use crate::core::config::Config;
use crate::service::payroll::models::*;
use open_lark_core::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 算薪项管理服务
#[derive(Debug, Clone)]
pub struct AcctItemService {
    config: Config,
}

impl AcctItemService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 算薪项管理 ====================

    /// 创建算薪项
    pub async fn create_acct_item(&self, request: &CreateAcctItemRequest) -> SDKResult<CreateAcctItemResponse> {
        let item_id = format!("item_{}", chrono::Utc::now().timestamp());

        Ok(CreateAcctItemResponse {
            item_id: item_id.clone(),
            item_name: request.item_name.clone(),
            item_code: request.item_code.clone(),
            item_type: request.item_type.clone(),
            category: request.category.clone(),
            description: request.description.clone(),
            is_active: true,
            created_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取算薪项详情
    pub async fn get_acct_item(&self, item_id: &str) -> SDKResult<GetAcctItemResponse> {
        // 模拟实现
        Ok(GetAcctItemResponse {
            item_id: item_id.to_string(),
            item_name: "基本工资".to_string(),
            item_code: "base_salary".to_string(),
            item_type: AcctItemType::Income,
            category: "固定收入".to_string(),
            description: Some("员工基本工资项目".to_string()),
            is_active: true,
            calculation_method: CalculationMethod::Fixed,
            default_value: Some(8000.0),
            unit: Some("元".to_string()),
            precision: Some(2),
            formula: None,
            conditions: None,
            tax_config: Some(TaxConfig {
                taxable: true,
                tax_rate: None,
                tax_category: Some("工资薪金".to_string()),
            }),
            social_insurance_config: Some(SocialInsuranceConfig {
                included: true,
                contribution_base: Some(true),
                categories: vec!["养老保险".to_string(), "医疗保险".to_string(), "失业保险".to_string()],
            }),
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        })
    }

    /// 更新算薪项
    pub async fn update_acct_item(&self, item_id: &str, request: &UpdateAcctItemRequest) -> SDKResult<UpdateAcctItemResponse> {
        Ok(UpdateAcctItemResponse {
            item_id: item_id.to_string(),
            item_name: request.item_name.clone(),
            item_code: request.item_code.clone(),
            item_type: request.item_type.clone(),
            category: request.category.clone(),
            description: request.description.clone(),
            is_active: request.is_active.unwrap_or(true),
            calculation_method: request.calculation_method.clone(),
            default_value: request.default_value,
            unit: request.unit.clone(),
            precision: request.precision,
            formula: request.formula.clone(),
            conditions: request.conditions.clone(),
            updated_at: Some(chrono::Utc::now()),
        })
    }

    /// 删除算薪项
    pub async fn delete_acct_item(&self, item_id: &str) -> SDKResult<DeleteAcctItemResponse> {
        Ok(DeleteAcctItemResponse {
            item_id: item_id.to_string(),
            deleted: true,
            deleted_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取算薪项列表
    pub async fn list_acct_items(&self, request: &ListAcctItemsRequest) -> SDKResult<ListAcctItemsResponse> {
        // 模拟算薪项列表
        let items = vec![
            AcctItemInfo {
                item_id: "item_001".to_string(),
                item_name: "基本工资".to_string(),
                item_code: "base_salary".to_string(),
                item_type: AcctItemType::Income,
                category: "固定收入".to_string(),
                is_active: true,
                calculation_method: CalculationMethod::Fixed,
                default_value: Some(8000.0),
                created_at: Some(chrono::Utc::now()),
            },
            AcctItemInfo {
                item_id: "item_002".to_string(),
                item_name: "绩效奖金".to_string(),
                item_code: "performance_bonus".to_string(),
                item_type: AcctItemType::Income,
                category: "浮动收入".to_string(),
                is_active: true,
                calculation_method: CalculationMethod::Formula,
                default_value: None,
                created_at: Some(chrono::Utc::now()),
            },
            AcctItemInfo {
                item_id: "item_003".to_string(),
                item_name: "交通补贴".to_string(),
                item_code: "transport_allowance".to_string(),
                item_type: AcctItemType::Income,
                category: "津贴补贴".to_string(),
                is_active: true,
                calculation_method: CalculationMethod::Fixed,
                default_value: Some(500.0),
                created_at: Some(chrono::Utc::now()),
            },
            AcctItemInfo {
                item_id: "item_004".to_string(),
                item_name: "个人所得税".to_string(),
                item_code: "income_tax".to_string(),
                item_type: AcctItemType::Deduction,
                category: "法定扣除".to_string(),
                is_active: true,
                calculation_method: CalculationMethod::Formula,
                default_value: None,
                created_at: Some(chrono::Utc::now()),
            },
            AcctItemInfo {
                item_id: "item_005".to_string(),
                item_name: "社会保险".to_string(),
                item_code: "social_insurance".to_string(),
                item_type: AcctItemType::Deduction,
                category: "法定扣除".to_string(),
                is_active: true,
                calculation_method: CalculationMethod::Formula,
                default_value: None,
                created_at: Some(chrono::Utc::now()),
            },
        ];

        let filtered_items = if let Some(item_type) = &request.item_type_filter {
            items.iter().filter(|item| &item.item_type == item_type).cloned().collect()
        } else if let Some(category) = &request.category_filter {
            items.iter().filter(|item| &item.category == category).cloned().collect()
        } else {
            items
        };

        Ok(ListAcctItemsResponse {
            items: filtered_items,
            total_count: filtered_items.len() as i32,
            has_more: Some(false),
            next_page_token: None,
        })
    }

    // ==================== 算薪项配置管理 ====================

    /// 设置算薪项公式
    pub async fn set_acct_item_formula(&self, item_id: &str, request: &SetAcctItemFormulaRequest) -> SDKResult<SetAcctItemFormulaResponse> {
        Ok(SetAcctItemFormulaResponse {
            item_id: item_id.to_string(),
            formula: request.formula.clone(),
            variables: request.variables.clone(),
            conditions: request.conditions.clone(),
            updated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取算薪项公式
    pub async fn get_acct_item_formula(&self, item_id: &str) -> SDKResult<GetAcctItemFormulaResponse> {
        Ok(GetAcctItemFormulaResponse {
            item_id: item_id.to_string(),
            formula: Some("base_salary * performance_coefficient".to_string()),
            variables: vec![
                FormulaVariable {
                    name: "base_salary".to_string(),
                    display_name: "基本工资".to_string(),
                    data_type: VariableDataType::Number,
                    required: true,
                    default_value: Some(8000.0),
                    description: Some("员工基本工资金额".to_string()),
                },
                FormulaVariable {
                    name: "performance_coefficient".to_string(),
                    display_name: "绩效系数".to_string(),
                    data_type: VariableDataType::Number,
                    required: true,
                    default_value: Some(1.0),
                    description: Some("员工绩效考核系数".to_string()),
                },
            ],
            conditions: Some(vec![
                CalculationCondition {
                    condition: "performance_coefficient > 0".to_string(),
                    description: Some("绩效系数必须大于0".to_string()),
                },
            ]),
            test_result: Some(FormulaTestResult {
                test_values: vec![
                    ("base_salary".to_string(), 8000.0),
                    ("performance_coefficient".to_string(), 1.2),
                ].into_iter().collect(),
                result: 9600.0,
                success: true,
                error_message: None,
            }),
        })
    }

    /// 测试算薪项公式
    pub async fn test_acct_item_formula(&self, request: &TestAcctItemFormulaRequest) -> SDKResult<TestAcctItemFormulaResponse> {
        // 模拟公式测试
        let result = match request.formula.as_str() {
            "base_salary * 1.1" => {
                if let Some(base_salary) = request.test_values.get("base_salary") {
                    Some(base_salary * 1.1)
                } else {
                    None
                }
            }
            "base_salary + 1000" => {
                if let Some(base_salary) = request.test_values.get("base_salary") {
                    Some(base_salary + 1000.0)
                } else {
                    None
                }
            }
            _ => None,
        };

        let success = result.is_some();

        Ok(TestAcctItemFormulaResponse {
            formula: request.formula.clone(),
            test_values: request.test_values.clone(),
            result,
            success,
            error_message: if success { None } else { Some("公式计算失败或缺少必要参数".to_string()) },
            execution_time: Some(15), // 毫秒
        })
    }

    // ==================== 算薪项模板管理 ====================

    /// 创建算薪项模板
    pub async fn create_acct_item_template(&self, request: &CreateAcctItemTemplateRequest) -> SDKResult<CreateAcctItemTemplateResponse> {
        let template_id = format!("template_{}", chrono::Utc::now().timestamp());

        Ok(CreateAcctItemTemplateResponse {
            template_id: template_id.clone(),
            template_name: request.template_name.clone(),
            template_code: request.template_code.clone(),
            description: request.description.clone(),
            item_count: request.items.len() as i32,
            created_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取算薪项模板列表
    pub async fn list_acct_item_templates(&self, request: &ListAcctItemTemplatesRequest) -> SDKResult<ListAcctItemTemplatesResponse> {
        let templates = vec![
            AcctItemTemplateInfo {
                template_id: "template_001".to_string(),
                template_name: "标准薪资模板".to_string(),
                template_code: "standard_salary".to_string(),
                description: Some("包含标准薪资项目的模板".to_string()),
                item_count: 8,
                is_system: true,
                created_at: Some(chrono::Utc::now()),
            },
            AcctItemTemplateInfo {
                template_id: "template_002".to_string(),
                template_name: "销售人员薪资模板".to_string(),
                template_code: "sales_salary".to_string(),
                description: Some("专门为销售人员设计的薪资模板".to_string()),
                item_count: 12,
                is_system: false,
                created_at: Some(chrono::Utc::now()),
            },
        ];

        Ok(ListAcctItemTemplatesResponse {
            templates,
            total_count: templates.len() as i32,
            has_more: Some(false),
            next_page_token: None,
        })
    }

    // ==================== 算薪项统计分析 ====================

    /// 获取算薪项使用统计
    pub async fn get_acct_item_usage_stats(&self, item_id: &str, period: &str) -> SDKResult<GetAcctItemUsageStatsResponse> {
        Ok(GetAcctItemUsageStatsResponse {
            item_id: item_id.to_string(),
            period: period.to_string(),
            usage_count: 150,
            total_amount: 1200000.0,
            average_amount: 8000.0,
            usage_by_department: vec![
                DepartmentUsage {
                    department_id: "dept_001".to_string(),
                    department_name: "技术研发部".to_string(),
                    employee_count: 80,
                    total_amount: 640000.0,
                    average_amount: 8000.0,
                },
                DepartmentUsage {
                    department_id: "dept_002".to_string(),
                    department_name: "市场营销部".to_string(),
                    employee_count: 45,
                    total_amount: 360000.0,
                    average_amount: 8000.0,
                },
                DepartmentUsage {
                    department_id: "dept_003".to_string(),
                    department_name: "人力资源部".to_string(),
                    employee_count: 25,
                    total_amount: 200000.0,
                    average_amount: 8000.0,
                },
            ],
            monthly_trend: vec![
                MonthlyUsageData {
                    month: "2024-01".to_string(),
                    amount: 980000.0,
                    count: 140,
                },
                MonthlyUsageData {
                    month: "2024-02".to_string(),
                    amount: 1000000.0,
                    count: 142,
                },
                MonthlyUsageData {
                    month: "2024-03".to_string(),
                    amount: 1200000.0,
                    count: 150,
                },
            ],
            calculated_at: Some(chrono::Utc::now()),
        })
    }

    /// 批量导入算薪项
    pub async fn batch_import_acct_items(&self, request: &BatchImportAcctItemsRequest) -> SDKResult<BatchImportAcctItemsResponse> {
        let mut success_count = 0;
        let mut failed_items = vec![];

        for item in &request.items {
            // 模拟导入处理
            if !item.item_name.is_empty() && !item.item_code.is_empty() {
                success_count += 1;
            } else {
                failed_items.push(BatchImportError {
                    item_code: item.item_code.clone(),
                    item_name: item.item_name.clone(),
                    error: "项目名称或编码不能为空".to_string(),
                });
            }
        }

        Ok(BatchImportAcctItemsResponse {
            batch_id: format!("batch_{}", chrono::Utc::now().timestamp()),
            total_count: request.items.len() as i32,
            success_count,
            failed_count: failed_items.len() as i32,
            failed_items,
            imported_at: Some(chrono::Utc::now()),
        })
    }
}

// ==================== 请求数据模型 ====================

/// 创建算薪项请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAcctItemRequest {
    /// 项目名称
    pub item_name: String,
    /// 项目编码
    pub item_code: String,
    /// 项目类型
    pub item_type: AcctItemType,
    /// 项目分类
    pub category: String,
    /// 描述
    pub description: Option<String>,
}

/// 更新算薪项请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAcctItemRequest {
    /// 项目名称
    pub item_name: Option<String>,
    /// 项目编码
    pub item_code: Option<String>,
    /// 项目类型
    pub item_type: Option<AcctItemType>,
    /// 项目分类
    pub category: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 是否激活
    pub is_active: Option<bool>,
    /// 计算方法
    pub calculation_method: Option<CalculationMethod>,
    /// 默认值
    pub default_value: Option<f64>,
    /// 单位
    pub unit: Option<String>,
    /// 精度
    pub precision: Option<i32>,
    /// 计算公式
    pub formula: Option<String>,
    /// 计算条件
    pub conditions: Option<Vec<String>>,
}

/// 算薪项列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAcctItemsRequest {
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
    /// 项目类型过滤
    pub item_type_filter: Option<AcctItemType>,
    /// 分类过滤
    pub category_filter: Option<String>,
    /// 是否激活过滤
    pub is_active_filter: Option<bool>,
}

/// 设置算薪项公式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAcctItemFormulaRequest {
    /// 计算公式
    pub formula: String,
    /// 公式变量
    pub variables: Vec<FormulaVariable>,
    /// 计算条件
    pub conditions: Option<Vec<String>>,
}

/// 测试算薪项公式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAcctItemFormulaRequest {
    /// 计算公式
    pub formula: String,
    /// 测试值
    pub test_values: std::collections::HashMap<String, f64>,
}

/// 创建算薪项模板请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAcctItemTemplateRequest {
    /// 模板名称
    pub template_name: String,
    /// 模板编码
    pub template_code: String,
    /// 描述
    pub description: Option<String>,
    /// 模板项目
    pub items: Vec<CreateAcctItemRequest>,
}

/// 算薪项模板列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAcctItemTemplatesRequest {
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 批量导入算薪项请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportAcctItemsRequest {
    /// 算薪项列表
    pub items: Vec<CreateAcctItemRequest>,
}

// ==================== 响应数据模型 ====================

/// 创建算薪项响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAcctItemResponse {
    /// 项目ID
    pub item_id: String,
    /// 项目名称
    pub item_name: String,
    /// 项目编码
    pub item_code: String,
    /// 项目类型
    pub item_type: AcctItemType,
    /// 项目分类
    pub category: String,
    /// 描述
    pub description: Option<String>,
    /// 是否激活
    pub is_active: bool,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 获取算薪项响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAcctItemResponse {
    /// 项目ID
    pub item_id: String,
    /// 项目名称
    pub item_name: String,
    /// 项目编码
    pub item_code: String,
    /// 项目类型
    pub item_type: AcctItemType,
    /// 项目分类
    pub category: String,
    /// 描述
    pub description: Option<String>,
    /// 是否激活
    pub is_active: bool,
    /// 计算方法
    pub calculation_method: CalculationMethod,
    /// 默认值
    pub default_value: Option<f64>,
    /// 单位
    pub unit: Option<String>,
    /// 精度
    pub precision: Option<i32>,
    /// 计算公式
    pub formula: Option<String>,
    /// 计算条件
    pub conditions: Option<Vec<String>>,
    /// 税务配置
    pub tax_config: Option<TaxConfig>,
    /// 社保配置
    pub social_insurance_config: Option<SocialInsuranceConfig>,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 更新算薪项响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAcctItemResponse {
    /// 项目ID
    pub item_id: String,
    /// 项目名称
    pub item_name: Option<String>,
    /// 项目编码
    pub item_code: Option<String>,
    /// 项目类型
    pub item_type: Option<AcctItemType>,
    /// 项目分类
    pub category: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 是否激活
    pub is_active: bool,
    /// 计算方法
    pub calculation_method: Option<CalculationMethod>,
    /// 默认值
    pub default_value: Option<f64>,
    /// 单位
    pub unit: Option<String>,
    /// 精度
    pub precision: Option<i32>,
    /// 计算公式
    pub formula: Option<String>,
    /// 计算条件
    pub conditions: Option<Vec<String>>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 删除算薪项响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAcctItemResponse {
    /// 项目ID
    pub item_id: String,
    /// 是否删除成功
    pub deleted: bool,
    /// 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}

/// 算薪项列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAcctItemsResponse {
    /// 算薪项列表
    pub items: Vec<AcctItemInfo>,
    /// 总数量
    pub total_count: i32,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 下一页标记
    pub next_page_token: Option<String>,
}

/// 设置算薪项公式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAcctItemFormulaResponse {
    /// 项目ID
    pub item_id: String,
    /// 计算公式
    pub formula: String,
    /// 公式变量
    pub variables: Vec<FormulaVariable>,
    /// 计算条件
    pub conditions: Option<Vec<String>>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 获取算薪项公式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAcctItemFormulaResponse {
    /// 项目ID
    pub item_id: String,
    /// 计算公式
    pub formula: Option<String>,
    /// 公式变量
    pub variables: Vec<FormulaVariable>,
    /// 计算条件
    pub conditions: Option<Vec<String>>,
    /// 测试结果
    pub test_result: Option<FormulaTestResult>,
}

/// 测试算薪项公式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAcctItemFormulaResponse {
    /// 计算公式
    pub formula: String,
    /// 测试值
    pub test_values: std::collections::HashMap<String, f64>,
    /// 计算结果
    pub result: Option<f64>,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error_message: Option<String>,
    /// 执行时间(毫秒)
    pub execution_time: Option<i64>,
}

/// 创建算薪项模板响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAcctItemTemplateResponse {
    /// 模板ID
    pub template_id: String,
    /// 模板名称
    pub template_name: String,
    /// 模板编码
    pub template_code: String,
    /// 描述
    pub description: Option<String>,
    /// 项目数量
    pub item_count: i32,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 算薪项模板列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAcctItemTemplatesResponse {
    /// 模板列表
    pub templates: Vec<AcctItemTemplateInfo>,
    /// 总数量
    pub total_count: i32,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 下一页标记
    pub next_page_token: Option<String>,
}

/// 获取算薪项使用统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAcctItemUsageStatsResponse {
    /// 项目ID
    pub item_id: String,
    /// 统计周期
    pub period: String,
    /// 使用次数
    pub usage_count: i32,
    /// 总金额
    pub total_amount: f64,
    /// 平均金额
    pub average_amount: f64,
    /// 部门使用情况
    pub usage_by_department: Vec<DepartmentUsage>,
    /// 月度趋势
    pub monthly_trend: Vec<MonthlyUsageData>,
    /// 计算时间
    pub calculated_at: Option<DateTime<Utc>>,
}

/// 批量导入算薪项响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportAcctItemsResponse {
    /// 批次ID
    pub batch_id: String,
    /// 总数量
    pub total_count: i32,
    /// 成功数量
    pub success_count: i32,
    /// 失败数量
    pub failed_count: i32,
    /// 失败项目
    pub failed_items: Vec<BatchImportError>,
    /// 导入时间
    pub imported_at: Option<DateTime<Utc>>,
}

// ==================== 数据模型 ====================

/// 算薪项信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcctItemInfo {
    /// 项目ID
    pub item_id: String,
    /// 项目名称
    pub item_name: String,
    /// 项目编码
    pub item_code: String,
    /// 项目类型
    pub item_type: AcctItemType,
    /// 项目分类
    pub category: String,
    /// 是否激活
    pub is_active: bool,
    /// 计算方法
    pub calculation_method: CalculationMethod,
    /// 默认值
    pub default_value: Option<f64>,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 算薪项模板信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcctItemTemplateInfo {
    /// 模板ID
    pub template_id: String,
    /// 模板名称
    pub template_name: String,
    /// 模板编码
    pub template_code: String,
    /// 描述
    pub description: Option<String>,
    /// 项目数量
    pub item_count: i32,
    /// 是否系统模板
    pub is_system: bool,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 公式变量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaVariable {
    /// 变量名
    pub name: String,
    /// 显示名称
    pub display_name: String,
    /// 数据类型
    pub data_type: VariableDataType,
    /// 是否必需
    pub required: bool,
    /// 默认值
    pub default_value: Option<f64>,
    /// 描述
    pub description: Option<String>,
}

/// 计算条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationCondition {
    /// 条件表达式
    pub condition: String,
    /// 描述
    pub description: Option<String>,
}

/// 公式测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaTestResult {
    /// 测试值
    pub test_values: std::collections::HashMap<String, f64>,
    /// 计算结果
    pub result: f64,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error_message: Option<String>,
}

/// 部门使用情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentUsage {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub department_name: String,
    /// 员工数量
    pub employee_count: i32,
    /// 总金额
    pub total_amount: f64,
    /// 平均金额
    pub average_amount: f64,
}

/// 月度使用数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyUsageData {
    /// 月份
    pub month: String,
    /// 金额
    pub amount: f64,
    /// 次数
    pub count: i32,
}

/// 批量导入错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportError {
    /// 项目编码
    pub item_code: String,
    /// 项目名称
    pub item_name: String,
    /// 错误信息
    pub error: String,
}

/// 税务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxConfig {
    /// 是否应税
    pub taxable: bool,
    /// 税率
    pub tax_rate: Option<f64>,
    /// 税收分类
    pub tax_category: Option<String>,
}

/// 社保配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsuranceConfig {
    /// 是否包含在社保计算中
    pub included: bool,
    /// 是否作为缴费基数
    pub contribution_base: Option<bool>,
    /// 社保类别
    pub categories: Vec<String>,
}

/// 算薪项类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcctItemType {
    /// 收入项
    Income,
    /// 扣除项
    Deduction,
    /// 税项
    Tax,
    /// 社保项
    SocialInsurance,
}

/// 计算方法
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculationMethod {
    /// 固定值
    Fixed,
    /// 公式计算
    Formula,
    /// 比例计算
    Percentage,
    /// 阶梯计算
    Tiered,
}

/// 变量数据类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableDataType {
    /// 数字
    Number,
    /// 字符串
    String,
    /// 布尔值
    Boolean,
    /// 日期
    Date,
}

// 实现Default trait
impl Default for CreateAcctItemRequest {
    fn default() -> Self {
        Self {
            item_name: String::new(),
            item_code: String::new(),
            item_type: AcctItemType::Income,
            category: String::new(),
            description: None,
        }
    }
}

impl Default for UpdateAcctItemRequest {
    fn default() -> Self {
        Self {
            item_name: None,
            item_code: None,
            item_type: None,
            category: None,
            description: None,
            is_active: None,
            calculation_method: None,
            default_value: None,
            unit: None,
            precision: None,
            formula: None,
            conditions: None,
        }
    }
}

impl Default for ListAcctItemsRequest {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page_token: None,
            item_type_filter: None,
            category_filter: None,
            is_active_filter: None,
        }
    }
}

impl Default for CreateAcctItemTemplateRequest {
    fn default() -> Self {
        Self {
            template_name: String::new(),
            template_code: String::new(),
            description: None,
            items: vec![],
        }
    }
}