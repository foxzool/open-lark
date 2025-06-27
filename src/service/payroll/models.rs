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
#[derive(Debug, Serialize, Deserialize)]
pub struct DatasourceFieldConfig {
    /// 字段ID
    pub field_id: String,
    /// 字段名称
    pub field_name: I18nText,
    /// 字段类型
    pub field_type: String,
    /// 是否必填
    pub required: bool,
    /// 默认值
    pub default_value: Option<serde_json::Value>,
    /// 字段描述
    pub description: Option<I18nText>,
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
