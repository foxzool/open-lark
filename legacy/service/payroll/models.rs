use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 分页响应通用结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页的分页标记
    pub page_token: Option<String>,
}

/// I18n 多语言文本
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct I18nText {
    /// 中文
    pub zh_cn: Option<String>,
    /// 英文
    pub en_us: Option<String>,
    /// 日文
    pub ja_jp: Option<String>,
}

/// 发薪明细列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentDetailListRequest {
    /// 发薪活动ID
    pub payment_activity_id: String,
    /// 分页大小，最大值100
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 员工ID筛选
    pub employee_id: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<String>,
    /// 部门ID类型
    pub department_id_type: Option<String>,
}

/// 发薪明细批量查询请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentDetailQueryRequest {
    /// 发薪活动ID
    pub payment_activity_id: String,
    /// 员工ID列表
    pub employee_ids: Vec<String>,
    /// 用户ID类型
    pub user_id_type: Option<String>,
    /// 查询字段列表
    pub fields: Option<Vec<String>>,
}

/// 发薪明细信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetail {
    /// 员工ID
    pub employee_id: String,
    /// 员工姓名
    pub employee_name: Option<I18nText>,
    /// 员工工号
    pub employee_number: Option<String>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 部门名称
    pub department_name: Option<I18nText>,
    /// 职位ID
    pub job_id: Option<String>,
    /// 职位名称
    pub job_name: Option<I18nText>,
    /// 发薪项目详情
    pub payment_items: Vec<PaymentItem>,
    /// 总发薪金额
    pub total_amount: Option<String>,
    /// 货币类型
    pub currency: Option<String>,
    /// 发薪状态
    pub payment_status: Option<String>,
    /// 发薪时间
    pub payment_time: Option<String>,
    /// 备注
    pub remark: Option<String>,
}

/// 发薪项目
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentItem {
    /// 算薪项ID
    pub acct_item_id: String,
    /// 算薪项名称
    pub acct_item_name: Option<I18nText>,
    /// 算薪项类型
    pub acct_item_type: Option<String>,
    /// 金额
    pub amount: String,
    /// 货币类型
    pub currency: Option<String>,
    /// 计算公式
    pub formula: Option<String>,
    /// 备注
    pub remark: Option<String>,
}

/// 发薪活动列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentActivityListRequest {
    /// 分页大小，最大值100
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 活动状态筛选
    pub status: Option<String>,
    /// 薪资组ID筛选
    pub paygroup_id: Option<String>,
    /// 发薪周期开始时间
    pub period_start: Option<String>,
    /// 发薪周期结束时间
    pub period_end: Option<String>,
}

/// 发薪活动封存请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaymentActivityArchiveRequest {
    /// 发薪活动ID
    pub payment_activity_id: String,
    /// 封存原因
    pub archive_reason: Option<String>,
}

/// 发薪活动信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentActivity {
    /// 发薪活动ID
    pub payment_activity_id: String,
    /// 活动名称
    pub activity_name: I18nText,
    /// 活动状态
    pub status: String,
    /// 薪资组ID
    pub paygroup_id: String,
    /// 薪资组名称
    pub paygroup_name: Option<I18nText>,
    /// 发薪周期开始时间
    pub period_start: String,
    /// 发薪周期结束时间
    pub period_end: String,
    /// 计划发薪日期
    pub planned_payment_date: Option<String>,
    /// 实际发薪日期
    pub actual_payment_date: Option<String>,
    /// 员工总数
    pub employee_count: Option<u32>,
    /// 发薪总金额
    pub total_amount: Option<String>,
    /// 货币类型
    pub currency: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
    /// 创建人ID
    pub creator_id: Option<String>,
    /// 备注
    pub remark: Option<String>,
}

/// 外部数据源记录保存请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatasourceRecordSaveRequest {
    /// 数据源ID
    pub datasource_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 用户ID类型
    pub user_id_type: Option<String>,
    /// 数据记录
    pub records: Vec<DatasourceRecord>,
    /// 发薪周期
    pub payment_period: String,
}

/// 外部数据源记录查询请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatasourceRecordQueryRequest {
    /// 数据源ID
    pub datasource_id: String,
    /// 员工ID列表
    pub employee_ids: Vec<String>,
    /// 用户ID类型
    pub user_id_type: Option<String>,
    /// 发薪周期
    pub payment_period: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 外部数据源记录
#[derive(Debug, Serialize, Deserialize)]
pub struct DatasourceRecord {
    /// 记录ID
    pub record_id: Option<String>,
    /// 员工ID
    pub employee_id: String,
    /// 数据字段值映射
    pub field_values: HashMap<String, serde_json::Value>,
    /// 发薪周期
    pub payment_period: String,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 外部数据源配置列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatasourceListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 数据源状态
    pub status: Option<String>,
}

/// 外部数据源配置
#[derive(Debug, Serialize, Deserialize)]
pub struct Datasource {
    /// 数据源ID
    pub datasource_id: String,
    /// 数据源名称
    pub datasource_name: I18nText,
    /// 数据源类型
    pub datasource_type: String,
    /// 数据源状态
    pub status: String,
    /// 字段配置
    pub field_configs: Vec<DatasourceFieldConfig>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
    /// 描述
    pub description: Option<I18nText>,
}

/// 数据源字段配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatasourceFieldConfig {
    /// 字段ID
    pub field_id: String,
    /// 字段名称（多语言）
    pub field_name: I18nText,
    /// 字段类型
    pub field_type: String,
}

/// 算薪项列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AcctItemListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 算薪项类型筛选
    pub item_type: Option<String>,
    /// 薪资组ID筛选
    pub paygroup_id: Option<String>,
    /// 状态筛选
    pub status: Option<String>,
}

/// 算薪项信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AcctItem {
    /// 算薪项ID
    pub acct_item_id: String,
    /// 算薪项名称
    pub item_name: I18nText,
    /// 算薪项类型
    pub item_type: String,
    /// 算薪项分类
    pub category: Option<String>,
    /// 计算方式
    pub calculation_method: Option<String>,
    /// 计算公式
    pub formula: Option<String>,
    /// 是否参与个税计算
    pub tax_related: bool,
    /// 是否参与社保计算
    pub social_security_related: bool,
    /// 显示顺序
    pub display_order: Option<u32>,
    /// 状态
    pub status: String,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
    /// 描述
    pub description: Option<I18nText>,
}

/// 成本分摊报表列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CostAllocationReportListRequest {
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 成本中心ID筛选
    pub cost_center_id: Option<String>,
    /// 部门ID筛选
    pub department_id: Option<String>,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 报表类型
    pub report_type: Option<String>,
}

/// 成本分摊报表数据
#[derive(Debug, Serialize, Deserialize)]
pub struct CostAllocationReport {
    /// 报表ID
    pub report_id: String,
    /// 成本中心ID
    pub cost_center_id: String,
    /// 成本中心名称
    pub cost_center_name: Option<I18nText>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 部门名称
    pub department_name: Option<I18nText>,
    /// 员工数量
    pub employee_count: u32,
    /// 总成本金额
    pub total_cost: String,
    /// 货币类型
    pub currency: String,
    /// 分摊明细
    pub allocation_details: Vec<AllocationDetail>,
    /// 统计周期开始时间
    pub period_start: String,
    /// 统计周期结束时间
    pub period_end: String,
    /// 生成时间
    pub generated_time: Option<String>,
}

/// 分摊明细
#[derive(Debug, Serialize, Deserialize)]
pub struct AllocationDetail {
    /// 算薪项ID
    pub acct_item_id: String,
    /// 算薪项名称
    pub acct_item_name: Option<I18nText>,
    /// 分摊金额
    pub allocated_amount: String,
    /// 分摊比例
    pub allocation_ratio: Option<f64>,
    /// 员工数量
    pub employee_count: u32,
}

/// 成本分摊方案列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CostAllocationPlanListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 方案状态筛选
    pub status: Option<String>,
    /// 方案类型筛选
    pub plan_type: Option<String>,
}

/// 成本分摊方案
#[derive(Debug, Serialize, Deserialize)]
pub struct CostAllocationPlan {
    /// 方案ID
    pub plan_id: String,
    /// 方案名称
    pub plan_name: I18nText,
    /// 方案类型
    pub plan_type: String,
    /// 方案状态
    pub status: String,
    /// 分摊规则
    pub allocation_rules: Vec<AllocationRule>,
    /// 生效日期
    pub effective_date: Option<String>,
    /// 失效日期
    pub expiry_date: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
    /// 创建人ID
    pub creator_id: Option<String>,
    /// 描述
    pub description: Option<I18nText>,
}

/// 分摊规则
#[derive(Debug, Serialize, Deserialize)]
pub struct AllocationRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub rule_name: I18nText,
    /// 分摊维度
    pub allocation_dimension: String,
    /// 分摊比例
    pub allocation_ratio: f64,
    /// 目标成本中心ID
    pub target_cost_center_id: String,
    /// 目标成本中心名称
    pub target_cost_center_name: Option<I18nText>,
    /// 规则条件
    pub conditions: Option<Vec<RuleCondition>>,
}

/// 规则条件
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleCondition {
    /// 条件字段
    pub field: String,
    /// 条件操作符
    pub operator: String,
    /// 条件值
    pub value: serde_json::Value,
}

/// 薪资组列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaygroupListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 薪资组状态筛选
    pub status: Option<String>,
}

/// 薪资组基本信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Paygroup {
    /// 薪资组ID
    pub paygroup_id: String,
    /// 薪资组名称
    pub paygroup_name: I18nText,
    /// 薪资组类型
    pub paygroup_type: String,
    /// 薪资组状态
    pub status: String,
    /// 发薪周期类型
    pub payment_cycle_type: String,
    /// 发薪日设置
    pub payment_day_setting: Option<PaymentDaySetting>,
    /// 关联员工数量
    pub employee_count: Option<u32>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
    /// 描述
    pub description: Option<I18nText>,
}

/// 发薪日设置
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDaySetting {
    /// 发薪日类型
    pub payment_day_type: String,
    /// 发薪日期
    pub payment_day: Option<u32>,
    /// 是否遇节假日顺延
    pub holiday_adjustment: bool,
    /// 顺延规则
    pub adjustment_rule: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response_serialization() {
        let page = PageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            has_more: true,
            page_token: Some("token123".to_string()),
        };
        let json = serde_json::to_string(&page).unwrap();
        assert!(json.contains("item1"));
        assert!(json.contains("true"));
        assert!(json.contains("token123"));
    }

    #[test]
    fn test_i18n_text_serialization() {
        let i18n = I18nText {
            zh_cn: Some("中文".to_string()),
            en_us: Some("English".to_string()),
            ja_jp: Some("日本語".to_string()),
        };
        let json = serde_json::to_string(&i18n).unwrap();
        assert!(json.contains("中文"));
        assert!(json.contains("English"));
        assert!(json.contains("日本語"));
    }

    #[test]
    fn test_i18n_text_default() {
        let i18n = I18nText::default();
        assert_eq!(i18n.zh_cn, None);
        assert_eq!(i18n.en_us, None);
        assert_eq!(i18n.ja_jp, None);
    }

    #[test]
    fn test_payment_detail_list_request() {
        let request = PaymentDetailListRequest {
            payment_activity_id: "activity123".to_string(),
            page_size: Some(50),
            page_token: Some("token456".to_string()),
            employee_id: Some("emp789".to_string()),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("open_department_id".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("activity123"));
        assert!(json.contains("50"));
        assert!(json.contains("emp789"));
    }

    #[test]
    fn test_payment_detail_query_request() {
        let request = PaymentDetailQueryRequest {
            payment_activity_id: "activity456".to_string(),
            employee_ids: vec!["emp1".to_string(), "emp2".to_string()],
            user_id_type: Some("user_id".to_string()),
            fields: Some(vec!["salary".to_string(), "bonus".to_string()]),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("activity456"));
        assert!(json.contains("emp1"));
        assert!(json.contains("salary"));
    }

    #[test]
    fn test_payment_item() {
        let item = PaymentItem {
            acct_item_id: "item123".to_string(),
            acct_item_name: Some(I18nText {
                zh_cn: Some("基本工资".to_string()),
                en_us: Some("Base Salary".to_string()),
                ja_jp: None,
            }),
            acct_item_type: Some("basic".to_string()),
            amount: "5000.00".to_string(),
            currency: Some("CNY".to_string()),
            formula: Some("base * 1.0".to_string()),
            remark: Some("基础薪资".to_string()),
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("item123"));
        assert!(json.contains("基本工资"));
        assert!(json.contains("5000.00"));
        assert!(json.contains("CNY"));
    }

    #[test]
    fn test_payment_detail() {
        let detail = PaymentDetail {
            employee_id: "emp123".to_string(),
            employee_name: Some(I18nText {
                zh_cn: Some("张三".to_string()),
                en_us: Some("Zhang San".to_string()),
                ja_jp: None,
            }),
            employee_number: Some("E001".to_string()),
            department_id: Some("dept123".to_string()),
            department_name: Some(I18nText {
                zh_cn: Some("技术部".to_string()),
                en_us: Some("Tech Dept".to_string()),
                ja_jp: None,
            }),
            job_id: Some("job123".to_string()),
            job_name: Some(I18nText {
                zh_cn: Some("软件工程师".to_string()),
                en_us: Some("Software Engineer".to_string()),
                ja_jp: None,
            }),
            payment_items: vec![],
            total_amount: Some("8000.00".to_string()),
            currency: Some("CNY".to_string()),
            payment_status: Some("paid".to_string()),
            payment_time: Some("2024-01-31T00:00:00Z".to_string()),
            remark: Some("正常发薪".to_string()),
        };
        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("emp123"));
        assert!(json.contains("张三"));
        assert!(json.contains("E001"));
        assert!(json.contains("8000.00"));
        assert!(json.contains("paid"));
    }

    #[test]
    fn test_payment_activity_list_request() {
        let request = PaymentActivityListRequest {
            page_size: Some(20),
            page_token: Some("token789".to_string()),
            status: Some("active".to_string()),
            paygroup_id: Some("pg123".to_string()),
            period_start: Some("2024-01-01".to_string()),
            period_end: Some("2024-01-31".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("20"));
        assert!(json.contains("active"));
        assert!(json.contains("pg123"));
        assert!(json.contains("2024-01-01"));
    }

    #[test]
    fn test_payment_activity() {
        let activity = PaymentActivity {
            payment_activity_id: "pa123".to_string(),
            activity_name: I18nText {
                zh_cn: Some("1月份工资发放".to_string()),
                en_us: Some("January Salary Payment".to_string()),
                ja_jp: None,
            },
            status: "completed".to_string(),
            paygroup_id: "pg456".to_string(),
            paygroup_name: Some(I18nText {
                zh_cn: Some("技术组".to_string()),
                en_us: Some("Tech Group".to_string()),
                ja_jp: None,
            }),
            period_start: "2024-01-01".to_string(),
            period_end: "2024-01-31".to_string(),
            planned_payment_date: Some("2024-02-01".to_string()),
            actual_payment_date: Some("2024-02-01".to_string()),
            employee_count: Some(50),
            total_amount: Some("400000.00".to_string()),
            currency: Some("CNY".to_string()),
            created_time: Some("2024-01-01T00:00:00Z".to_string()),
            updated_time: Some("2024-02-01T00:00:00Z".to_string()),
            creator_id: Some("user123".to_string()),
            remark: Some("月度工资发放".to_string()),
        };
        let json = serde_json::to_string(&activity).unwrap();
        assert!(json.contains("pa123"));
        assert!(json.contains("1月份工资发放"));
        assert!(json.contains("completed"));
        assert!(json.contains("50"));
        assert!(json.contains("400000.00"));
    }

    #[test]
    fn test_datasource_record_save_request() {
        let request = DatasourceRecordSaveRequest {
            datasource_id: "ds123".to_string(),
            employee_id: "emp456".to_string(),
            user_id_type: Some("open_id".to_string()),
            records: vec![],
            payment_period: "2024-01".to_string(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ds123"));
        assert!(json.contains("emp456"));
        assert!(json.contains("2024-01"));
    }

    #[test]
    fn test_datasource_record() {
        let mut field_values = HashMap::new();
        field_values.insert(
            "overtime_hours".to_string(),
            serde_json::Value::Number(serde_json::Number::from(20)),
        );
        field_values.insert(
            "bonus_rate".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(1.5).unwrap()),
        );

        let record = DatasourceRecord {
            record_id: Some("rec123".to_string()),
            employee_id: "emp789".to_string(),
            field_values,
            payment_period: "2024-01".to_string(),
            created_time: Some("2024-01-15T00:00:00Z".to_string()),
            updated_time: Some("2024-01-16T00:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&record).unwrap();
        assert!(json.contains("rec123"));
        assert!(json.contains("emp789"));
        assert!(json.contains("overtime_hours"));
        assert!(json.contains("2024-01"));
    }

    #[test]
    fn test_datasource() {
        let datasource = Datasource {
            datasource_id: "ds456".to_string(),
            datasource_name: I18nText {
                zh_cn: Some("考勤数据源".to_string()),
                en_us: Some("Attendance Datasource".to_string()),
                ja_jp: None,
            },
            datasource_type: "attendance".to_string(),
            status: "active".to_string(),
            field_configs: vec![],
            created_time: Some("2024-01-01T00:00:00Z".to_string()),
            updated_time: Some("2024-01-02T00:00:00Z".to_string()),
            description: Some(I18nText {
                zh_cn: Some("员工考勤相关数据".to_string()),
                en_us: Some("Employee attendance data".to_string()),
                ja_jp: None,
            }),
        };
        let json = serde_json::to_string(&datasource).unwrap();
        assert!(json.contains("ds456"));
        assert!(json.contains("考勤数据源"));
        assert!(json.contains("attendance"));
        assert!(json.contains("active"));
    }

    #[test]
    fn test_datasource_field_config() {
        let field_config = DatasourceFieldConfig::default();
        let _json = serde_json::to_string(&field_config).unwrap();
        // Test should succeed without panicking
    }

    #[test]
    fn test_acct_item() {
        let item = AcctItem {
            acct_item_id: "ai123".to_string(),
            item_name: I18nText {
                zh_cn: Some("绩效奖金".to_string()),
                en_us: Some("Performance Bonus".to_string()),
                ja_jp: None,
            },
            item_type: "bonus".to_string(),
            category: Some("variable".to_string()),
            calculation_method: Some("formula".to_string()),
            formula: Some("base_salary * performance_ratio".to_string()),
            tax_related: true,
            social_security_related: false,
            display_order: Some(10),
            status: "active".to_string(),
            created_time: Some("2024-01-01T00:00:00Z".to_string()),
            updated_time: Some("2024-01-02T00:00:00Z".to_string()),
            description: Some(I18nText {
                zh_cn: Some("根据绩效计算的奖金".to_string()),
                en_us: Some("Bonus calculated based on performance".to_string()),
                ja_jp: None,
            }),
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("ai123"));
        assert!(json.contains("绩效奖金"));
        assert!(json.contains("bonus"));
        assert!(json.contains("true"));
        assert!(json.contains("false"));
    }

    #[test]
    fn test_cost_allocation_report() {
        let report = CostAllocationReport {
            report_id: "report123".to_string(),
            cost_center_id: "cc123".to_string(),
            cost_center_name: Some(I18nText {
                zh_cn: Some("研发成本中心".to_string()),
                en_us: Some("R&D Cost Center".to_string()),
                ja_jp: None,
            }),
            department_id: Some("dept456".to_string()),
            department_name: Some(I18nText {
                zh_cn: Some("技术部".to_string()),
                en_us: Some("Tech Department".to_string()),
                ja_jp: None,
            }),
            employee_count: 25,
            total_cost: "200000.00".to_string(),
            currency: "CNY".to_string(),
            allocation_details: vec![],
            period_start: "2024-01-01".to_string(),
            period_end: "2024-01-31".to_string(),
            generated_time: Some("2024-02-01T00:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("report123"));
        assert!(json.contains("研发成本中心"));
        assert!(json.contains("25"));
        assert!(json.contains("200000.00"));
        assert!(json.contains("CNY"));
    }

    #[test]
    fn test_allocation_detail() {
        let detail = AllocationDetail {
            acct_item_id: "ai456".to_string(),
            acct_item_name: Some(I18nText {
                zh_cn: Some("基本工资".to_string()),
                en_us: Some("Base Salary".to_string()),
                ja_jp: None,
            }),
            allocated_amount: "150000.00".to_string(),
            allocation_ratio: Some(0.75),
            employee_count: 20,
        };
        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("ai456"));
        assert!(json.contains("基本工资"));
        assert!(json.contains("150000.00"));
        assert!(json.contains("0.75"));
        assert!(json.contains("20"));
    }

    #[test]
    fn test_cost_allocation_plan() {
        let plan = CostAllocationPlan {
            plan_id: "plan123".to_string(),
            plan_name: I18nText {
                zh_cn: Some("技术部成本分摊方案".to_string()),
                en_us: Some("Tech Dept Cost Allocation Plan".to_string()),
                ja_jp: None,
            },
            plan_type: "department".to_string(),
            status: "active".to_string(),
            allocation_rules: vec![],
            effective_date: Some("2024-01-01".to_string()),
            expiry_date: Some("2024-12-31".to_string()),
            created_time: Some("2023-12-01T00:00:00Z".to_string()),
            updated_time: Some("2024-01-01T00:00:00Z".to_string()),
            creator_id: Some("user456".to_string()),
            description: Some(I18nText {
                zh_cn: Some("技术部门成本分摊规则".to_string()),
                en_us: Some("Cost allocation rules for tech department".to_string()),
                ja_jp: None,
            }),
        };
        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains("plan123"));
        assert!(json.contains("技术部成本分摊方案"));
        assert!(json.contains("department"));
        assert!(json.contains("active"));
    }

    #[test]
    fn test_allocation_rule() {
        let rule = AllocationRule {
            rule_id: "rule123".to_string(),
            rule_name: I18nText {
                zh_cn: Some("按人数分摊".to_string()),
                en_us: Some("Allocate by headcount".to_string()),
                ja_jp: None,
            },
            allocation_dimension: "headcount".to_string(),
            allocation_ratio: 0.8,
            target_cost_center_id: "cc456".to_string(),
            target_cost_center_name: Some(I18nText {
                zh_cn: Some("目标成本中心".to_string()),
                en_us: Some("Target Cost Center".to_string()),
                ja_jp: None,
            }),
            conditions: Some(vec![]),
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("rule123"));
        assert!(json.contains("按人数分摊"));
        assert!(json.contains("headcount"));
        assert!(json.contains("0.8"));
        assert!(json.contains("cc456"));
    }

    #[test]
    fn test_rule_condition() {
        let condition = RuleCondition {
            field: "department_id".to_string(),
            operator: "equals".to_string(),
            value: serde_json::Value::String("dept123".to_string()),
        };
        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains("department_id"));
        assert!(json.contains("equals"));
        assert!(json.contains("dept123"));
    }

    #[test]
    fn test_paygroup() {
        let paygroup = Paygroup {
            paygroup_id: "pg123".to_string(),
            paygroup_name: I18nText {
                zh_cn: Some("技术组薪资组".to_string()),
                en_us: Some("Tech Group Payroll".to_string()),
                ja_jp: None,
            },
            paygroup_type: "standard".to_string(),
            status: "active".to_string(),
            payment_cycle_type: "monthly".to_string(),
            payment_day_setting: Some(PaymentDaySetting {
                payment_day_type: "fixed".to_string(),
                payment_day: Some(15),
                holiday_adjustment: true,
                adjustment_rule: Some("advance".to_string()),
            }),
            employee_count: Some(30),
            created_time: Some("2024-01-01T00:00:00Z".to_string()),
            updated_time: Some("2024-01-15T00:00:00Z".to_string()),
            description: Some(I18nText {
                zh_cn: Some("技术部门薪资组".to_string()),
                en_us: Some("Payroll group for tech department".to_string()),
                ja_jp: None,
            }),
        };
        let json = serde_json::to_string(&paygroup).unwrap();
        assert!(json.contains("pg123"));
        assert!(json.contains("技术组薪资组"));
        assert!(json.contains("monthly"));
        assert!(json.contains("30"));
    }

    #[test]
    fn test_payment_day_setting() {
        let setting = PaymentDaySetting {
            payment_day_type: "last_day".to_string(),
            payment_day: Some(31),
            holiday_adjustment: false,
            adjustment_rule: Some("no_adjustment".to_string()),
        };
        let json = serde_json::to_string(&setting).unwrap();
        assert!(json.contains("last_day"));
        assert!(json.contains("31"));
        assert!(json.contains("false"));
        assert!(json.contains("no_adjustment"));
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_i18n = I18nText {
            zh_cn: Some("中文".to_string()),
            en_us: None,
            ja_jp: None,
        };
        let json = serde_json::to_string(&minimal_i18n).unwrap();
        assert!(json.contains("中文"));
        // Note: serde includes null fields by default, so en_us will appear as null
        assert!(json.contains("en_us"));

        let minimal_request = PaymentDetailListRequest {
            payment_activity_id: "minimal".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&minimal_request).unwrap();
        assert!(json.contains("minimal"));
    }
}
