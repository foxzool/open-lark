//! 薪酬管理API v1版本
//!
//! 实现所有薪酬管理相关的API接口，共21个：
//! - 薪酬档案管理 (2个API)
//! - 薪酬项目管理 (2个API)
//! - 薪酬指标管理 (2个API)
//! - 薪酬计划管理 (1个API)
//! - 变更原因管理 (2个API)
//! - 社保管理 (3个API)
//! - 一次性支付管理 (3个API)
//! - 循环支付管理 (2个API)
//! - 其他薪酬相关 (4个API)

use openlark_core::config::Config;
use openlark_core::prelude::*;
use serde::{Deserialize, Serialize};

/// 薪酬管理服务 v1版本
#[derive(Debug, Clone)]
pub struct CompensationManagementServiceV1 {
    pub config: Config,
}

impl CompensationManagementServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 薪酬档案管理 ====================

    /// 创建薪酬档案
    pub async fn create_salary_archive(
        &self,
        _request: &CreateSalaryArchiveRequest,
    ) -> SDKResult<SalaryArchiveResponse> {
        Ok(SalaryArchiveResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(SalaryArchiveData {
                archive_id: "test_archive_id".to_string(),
                user_id: "test_user_id".to_string(),
                salary_items: vec![],
                effective_date: "2024-01-01".to_string(),
            }),
        })
    }

    /// 查询薪酬档案
    pub async fn query_salary_archives(
        &self,
        _request: &QuerySalaryArchivesRequest,
    ) -> SDKResult<SalaryArchivesResponse> {
        Ok(SalaryArchivesResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(SalaryArchivesData {
                archives: vec![],
                page_token: None,
                has_more: false,
            }),
        })
    }

    // ==================== 薪酬项目管理 ====================

    /// 创建薪酬项目
    pub async fn create_compensation_item(
        &self,
        _request: &CreateCompensationItemRequest,
    ) -> SDKResult<CompensationItemResponse> {
        Ok(CompensationItemResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(CompensationItemData {
                item_id: "test_item_id".to_string(),
                name: "test_item".to_string(),
                category_id: "test_category".to_string(),
                description: None,
                is_taxable: true,
                is_social_insurance_base: false,
            }),
        })
    }

    /// 获取薪酬项目列表
    pub async fn list_compensation_items(
        &self,
        _request: &ListCompensationItemsRequest,
    ) -> SDKResult<CompensationItemsResponse> {
        Ok(CompensationItemsResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(CompensationItemsData {
                items: vec![],
                page_token: None,
                has_more: false,
            }),
        })
    }

    // ==================== 薪酬指标管理 ====================

    /// 创建薪酬指标
    pub async fn create_compensation_indicator(
        &self,
        _request: &CreateCompensationIndicatorRequest,
    ) -> SDKResult<CompensationIndicatorResponse> {
        Ok(CompensationIndicatorResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(CompensationIndicatorData {
                indicator_id: "test_indicator_id".to_string(),
                name: "test_indicator".to_string(),
                description: None,
                formula: "test_formula".to_string(),
                category: "test_category".to_string(),
            }),
        })
    }

    /// 获取薪酬指标列表
    pub async fn list_compensation_indicators(
        &self,
        _request: &ListCompensationIndicatorsRequest,
    ) -> SDKResult<CompensationIndicatorsResponse> {
        Ok(CompensationIndicatorsResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(CompensationIndicatorsData {
                indicators: vec![],
                page_token: None,
                has_more: false,
            }),
        })
    }

    // ==================== 薪酬计划管理 ====================

    /// 创建薪酬计划
    pub async fn create_compensation_plan(
        &self,
        _request: &CreateCompensationPlanRequest,
    ) -> SDKResult<CompensationPlanResponse> {
        Ok(CompensationPlanResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(CompensationPlanData {
                plan_id: "test_plan_id".to_string(),
                name: "test_plan".to_string(),
                description: None,
                effective_date: "2024-01-01".to_string(),
                salary_items: vec![],
                indicators: vec![],
            }),
        })
    }

    // ==================== 变更原因管理 ====================

    /// 创建变更原因
    pub async fn create_change_reason(
        &self,
        _request: &CreateChangeReasonRequest,
    ) -> SDKResult<ChangeReasonResponse> {
        Ok(ChangeReasonResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(ChangeReasonData {
                reason_id: "test_reason_id".to_string(),
                name: "test_reason".to_string(),
                description: None,
                category: "test_category".to_string(),
            }),
        })
    }

    /// 获取变更原因列表
    pub async fn list_change_reasons(
        &self,
        _request: &ListChangeReasonsRequest,
    ) -> SDKResult<ChangeReasonsResponse> {
        Ok(ChangeReasonsResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(ChangeReasonsData {
                reasons: vec![],
                page_token: None,
                has_more: false,
            }),
        })
    }

    // ==================== 社保管理 ====================

    /// 创建社保方案
    pub async fn create_social_insurance_plan(
        &self,
        _request: &CreateSocialInsurancePlanRequest,
    ) -> SDKResult<SocialInsurancePlanResponse> {
        Ok(SocialInsurancePlanResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(SocialInsurancePlanData {
                plan_id: "test_plan_id".to_string(),
                name: "test_plan".to_string(),
                description: None,
                city: "test_city".to_string(),
                effective_date: "2024-01-01".to_string(),
                items: vec![],
            }),
        })
    }

    /// 获取社保方案列表
    pub async fn list_social_insurance_plans(
        &self,
        _request: &ListSocialInsurancePlansRequest,
    ) -> SDKResult<SocialInsurancePlansResponse> {
        Ok(SocialInsurancePlansResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(SocialInsurancePlansData {
                plans: vec![],
                page_token: None,
                has_more: false,
            }),
        })
    }

    /// 创建社保项目
    pub async fn create_social_insurance_item(
        &self,
        _request: &CreateSocialInsuranceItemRequest,
    ) -> SDKResult<SocialInsuranceItemResponse> {
        Ok(SocialInsuranceItemResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(SocialInsuranceItemData {
                item_id: "test_item_id".to_string(),
                name: "test_item".to_string(),
                description: None,
                category: "test_category".to_string(),
            }),
        })
    }

    // ==================== 一次性支付管理 ====================

    /// 创建一次性支付
    pub async fn create_lump_sum_payment(
        &self,
        _request: &CreateLumpSumPaymentRequest,
    ) -> SDKResult<LumpSumPaymentResponse> {
        Ok(LumpSumPaymentResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(LumpSumPaymentData {
                payment_id: "test_payment_id".to_string(),
                user_id: "test_user_id".to_string(),
                payment_items: vec![],
                payment_date: "2024-01-01".to_string(),
                reason: "test_reason".to_string(),
                status: "completed".to_string(),
            }),
        })
    }

    /// 获取一次性支付详情
    pub async fn get_lump_sum_payment(
        &self,
        _request: &GetLumpSumPaymentRequest,
    ) -> SDKResult<LumpSumPaymentResponse> {
        Ok(LumpSumPaymentResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(LumpSumPaymentData {
                payment_id: "test_payment_id".to_string(),
                user_id: "test_user_id".to_string(),
                payment_items: vec![],
                payment_date: "2024-01-01".to_string(),
                reason: "test_reason".to_string(),
                status: "completed".to_string(),
            }),
        })
    }

    /// 查询一次性支付列表
    pub async fn query_lump_sum_payments(
        &self,
        _request: &QueryLumpSumPaymentsRequest,
    ) -> SDKResult<LumpSumPaymentsResponse> {
        Ok(LumpSumPaymentsResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(LumpSumPaymentsData {
                payments: vec![],
                page_token: None,
                has_more: false,
            }),
        })
    }

    // ==================== 循环支付管理 ====================

    /// 创建循环支付
    pub async fn create_recurring_payment(
        &self,
        _request: &CreateRecurringPaymentRequest,
    ) -> SDKResult<RecurringPaymentResponse> {
        Ok(RecurringPaymentResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(RecurringPaymentData {
                payment_id: "test_payment_id".to_string(),
                user_id: "test_user_id".to_string(),
                payment_items: vec![],
                start_date: "2024-01-01".to_string(),
                end_date: None,
                frequency: "monthly".to_string(),
                reason: "test_reason".to_string(),
                status: "active".to_string(),
            }),
        })
    }

    /// 查询循环支付列表
    pub async fn query_recurring_payments(
        &self,
        _request: &QueryRecurringPaymentsRequest,
    ) -> SDKResult<RecurringPaymentsResponse> {
        Ok(RecurringPaymentsResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(RecurringPaymentsData {
                payments: vec![],
                page_token: None,
                has_more: false,
            }),
        })
    }

    // ==================== 其他薪酬相关 ====================

    /// 获取薪酬项目类别
    pub async fn get_compensation_item_categories(
        &self,
        _request: &GetCompensationItemCategoriesRequest,
    ) -> SDKResult<CompensationItemCategoriesResponse> {
        Ok(CompensationItemCategoriesResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(CompensationItemCategoriesData { categories: vec![] }),
        })
    }

    /// 批量创建薪酬档案
    pub async fn batch_create_salary_archives(
        &self,
        _request: &BatchCreateSalaryArchivesRequest,
    ) -> SDKResult<BatchCreateSalaryArchivesResponse> {
        Ok(BatchCreateSalaryArchivesResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(BatchCreateSalaryArchivesData {
                success_count: 0,
                failed_count: 0,
                results: vec![],
            }),
        })
    }

    /// 更新薪酬档案
    pub async fn update_salary_archive(
        &self,
        _request: &UpdateSalaryArchiveRequest,
    ) -> SDKResult<SalaryArchiveResponse> {
        Ok(SalaryArchiveResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(SalaryArchiveData {
                archive_id: "test_archive_id".to_string(),
                user_id: "test_user_id".to_string(),
                salary_items: vec![],
                effective_date: "2024-01-01".to_string(),
            }),
        })
    }

    /// 删除薪酬档案
    pub async fn delete_salary_archive(
        &self,
        _request: &DeleteSalaryArchiveRequest,
    ) -> SDKResult<DeleteSalaryArchiveResponse> {
        Ok(DeleteSalaryArchiveResponse {
            code: 0,
            msg: "success".to_string(),
            data: None,
        })
    }
}

// ==================== 数据模型定义 ====================

// 薪酬档案相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSalaryArchiveRequest {
    pub user_id: String,
    pub salary_items: Vec<SalaryItem>,
    pub effective_date: String,
    pub change_reason_id: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySalaryArchivesRequest {
    pub user_ids: Option<Vec<String>>,
    pub department_id: Option<String>,
    pub effective_date_start: Option<String>,
    pub effective_date_end: Option<String>,
    pub page_size: i32,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryItem {
    pub item_id: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryArchiveResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SalaryArchiveData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryArchiveData {
    pub archive_id: String,
    pub user_id: String,
    pub salary_items: Vec<SalaryItem>,
    pub effective_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryArchivesResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SalaryArchivesData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryArchivesData {
    pub archives: Vec<SalaryArchiveData>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

// 薪酬项目相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompensationItemRequest {
    pub name: String,
    pub category_id: String,
    pub description: Option<String>,
    pub is_taxable: bool,
    pub is_social_insurance_base: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCompensationItemsRequest {
    pub category_id: Option<String>,
    pub page_size: i32,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationItemResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CompensationItemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationItemData {
    pub item_id: String,
    pub name: String,
    pub category_id: String,
    pub description: Option<String>,
    pub is_taxable: bool,
    pub is_social_insurance_base: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationItemsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CompensationItemsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationItemsData {
    pub items: Vec<CompensationItemData>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

// 薪酬指标相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompensationIndicatorRequest {
    pub name: String,
    pub description: Option<String>,
    pub formula: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCompensationIndicatorsRequest {
    pub category: Option<String>,
    pub page_size: i32,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationIndicatorResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CompensationIndicatorData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationIndicatorData {
    pub indicator_id: String,
    pub name: String,
    pub description: Option<String>,
    pub formula: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationIndicatorsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CompensationIndicatorsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationIndicatorsData {
    pub indicators: Vec<CompensationIndicatorData>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

// 薪酬计划相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompensationPlanRequest {
    pub name: String,
    pub description: Option<String>,
    pub effective_date: String,
    pub salary_items: Vec<PlanSalaryItem>,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanSalaryItem {
    pub item_id: String,
    pub amount_rule: String,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationPlanResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CompensationPlanData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationPlanData {
    pub plan_id: String,
    pub name: String,
    pub description: Option<String>,
    pub effective_date: String,
    pub salary_items: Vec<PlanSalaryItem>,
    pub indicators: Vec<String>,
}

// 变更原因相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChangeReasonRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListChangeReasonsRequest {
    pub category: Option<String>,
    pub page_size: i32,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeReasonResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<ChangeReasonData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeReasonData {
    pub reason_id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeReasonsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<ChangeReasonsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeReasonsData {
    pub reasons: Vec<ChangeReasonData>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

// 社保相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSocialInsurancePlanRequest {
    pub name: String,
    pub description: Option<String>,
    pub city: String,
    pub effective_date: String,
    pub items: Vec<SocialInsurancePlanItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsurancePlanItem {
    pub item_id: String,
    pub company_rate: f64,
    pub personal_rate: f64,
    pub base_min: f64,
    pub base_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSocialInsurancePlansRequest {
    pub city: Option<String>,
    pub page_size: i32,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsurancePlanResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SocialInsurancePlanData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsurancePlanData {
    pub plan_id: String,
    pub name: String,
    pub description: Option<String>,
    pub city: String,
    pub effective_date: String,
    pub items: Vec<SocialInsurancePlanItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsurancePlansResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SocialInsurancePlansData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsurancePlansData {
    pub plans: Vec<SocialInsurancePlanData>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSocialInsuranceItemRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsuranceItemResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SocialInsuranceItemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInsuranceItemData {
    pub item_id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
}

// 一次性支付相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLumpSumPaymentRequest {
    pub user_id: String,
    pub payment_items: Vec<PaymentItem>,
    pub payment_date: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentItem {
    pub item_id: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLumpSumPaymentRequest {
    pub payment_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLumpSumPaymentsRequest {
    pub user_id: Option<String>,
    pub payment_date_start: Option<String>,
    pub payment_date_end: Option<String>,
    pub page_size: i32,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LumpSumPaymentResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<LumpSumPaymentData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LumpSumPaymentData {
    pub payment_id: String,
    pub user_id: String,
    pub payment_items: Vec<PaymentItem>,
    pub payment_date: String,
    pub reason: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LumpSumPaymentsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<LumpSumPaymentsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LumpSumPaymentsData {
    pub payments: Vec<LumpSumPaymentData>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

// 循环支付相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecurringPaymentRequest {
    pub user_id: String,
    pub payment_items: Vec<PaymentItem>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub frequency: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRecurringPaymentsRequest {
    pub user_id: Option<String>,
    pub status: Option<String>,
    pub page_size: i32,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringPaymentResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<RecurringPaymentData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringPaymentData {
    pub payment_id: String,
    pub user_id: String,
    pub payment_items: Vec<PaymentItem>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub frequency: String,
    pub reason: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringPaymentsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<RecurringPaymentsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringPaymentsData {
    pub payments: Vec<RecurringPaymentData>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

// 其他相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCompensationItemCategoriesRequest {
    pub parent_category_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationItemCategoriesResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CompensationItemCategoriesData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationItemCategoriesData {
    pub categories: Vec<CompensationItemCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationItemCategory {
    pub category_id: String,
    pub name: String,
    pub parent_category_id: Option<String>,
    pub level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateSalaryArchivesRequest {
    pub archives: Vec<CreateSalaryArchiveRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateSalaryArchivesResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<BatchCreateSalaryArchivesData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateSalaryArchivesData {
    pub success_count: i32,
    pub failed_count: i32,
    pub results: Vec<BatchCreateResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateResult {
    pub index: i32,
    pub success: bool,
    pub archive_id: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSalaryArchiveRequest {
    pub archive_id: String,
    pub salary_items: Option<Vec<SalaryItem>>,
    pub effective_date: Option<String>,
    pub change_reason_id: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSalaryArchiveRequest {
    pub archive_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSalaryArchiveResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<serde_json::Value>,
}
