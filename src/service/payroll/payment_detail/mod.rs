#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::payroll::models::{
        PageResponse, PaymentDetail, PaymentDetailListRequest, PaymentDetailQueryRequest,
    },
};
use open_lark_core::core::api_req::ApiRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 发薪明细服务
pub struct PaymentDetailService {
    pub config: Config,
}

/// 发薪明细列表响应
#[derive(Debug, Clone)]
pub struct PaymentDetailListResponse {
    /// 发薪明细列表
    pub items: Vec<PaymentDetail>,
    /// 分页信息
    pub page: PageResponse<PaymentDetail>,
}

/// 发薪明细响应
#[derive(Debug, Clone)]
pub struct PaymentDetailResponse {
    /// 发薪明细信息
    pub data: PaymentDetail,
}

impl Service for PaymentDetailService {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl PaymentDetailService {
    /// 创建发薪明细服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询发薪明细列表
    ///
    /// # API文档
    ///
    /// 根据发薪活动ID查询员工的发薪明细列表，支持分页和筛选功能。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请求参数
    ///
    /// # 返回值
    ///
    /// 返回发薪明细列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::payment_detail::*;
    ///
    /// let request = PaymentDetailListRequest {
    ///     payment_activity_id: "pa_123456".to_string(),
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     employee_id: Some("emp_789".to_string()),
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("open_department_id".to_string()),
    /// };
    ///
    /// let response = service.list_payment_details(&request).await?;
    /// println!("找到 {} 条发薪明细", response.page.items.len());
    /// ```
    pub async fn list_payment_details(
        &self,
        request: &PaymentDetailListRequest,
    ) -> SDKResult<BaseResponse<PaymentDetailListResponse>> {
        let mut query_params = std::collections::HashMap::new();
        query_params.insert(
            "payment_activity_id".to_string(),
            request.payment_activity_id.clone(),
        );

        if let Some(page_size) = &request.page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token".to_string(), page_token.clone());
        }
        if let Some(employee_id) = &request.employee_id {
            query_params.insert("employee_id".to_string(), employee_id.clone());
        }
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type".to_string(), user_id_type.clone());
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.insert("department_id_type".to_string(), department_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: self
                .config
                .build_url("/open-apis/payroll/v4/payment_details"),
            query_params,
            path_params: std::collections::HashMap::new(),
            body: None,
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 批量查询发薪明细
    ///
    /// # API文档
    ///
    /// 根据员工ID列表批量查询发薪明细，适用于需要查询多个员工薪资的场景。
    ///
    /// # 参数
    ///
    /// * `request` - 批量查询请求参数
    ///
    /// # 返回值
    ///
    /// 返回指定员工的发薪明细列表
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::payment_detail::*;
    ///
    /// let request = PaymentDetailQueryRequest {
    ///     payment_activity_id: "pa_123456".to_string(),
    ///     employee_ids: vec!["emp_001".to_string(), "emp_002".to_string()],
    ///     user_id_type: Some("open_id".to_string()),
    ///     fields: Some(vec!["salary".to_string(), "bonus".to_string()]),
    /// };
    ///
    /// let response = service.query_payment_details(&request).await?;
    /// println!("查询到 {} 条明细", response.data.items.len());
    /// ```
    pub async fn query_payment_details(
        &self,
        request: &PaymentDetailQueryRequest,
    ) -> SDKResult<BaseResponse<PaymentDetailListResponse>> {
        let mut query_params = std::collections::HashMap::new();
        query_params.insert(
            "payment_activity_id".to_string(),
            request.payment_activity_id.clone(),
        );
        query_params.insert("employee_ids".to_string(), request.employee_ids.join(","));

        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type".to_string(), user_id_type.clone());
        }
        if let Some(fields) = &request.fields {
            query_params.insert("fields".to_string(), fields.join(","));
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: self
                .config
                .build_url("/open-apis/payroll/v4/payment_details/query"),
            query_params,
            path_params: std::collections::HashMap::new(),
            body: None,
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 获取发薪明细详情
    ///
    /// # API文档
    ///
    /// 根据发薪活动ID和员工ID获取详细的发薪明细信息。
    ///
    /// # 参数
    ///
    /// * `payment_activity_id` - 发薪活动ID
    /// * `employee_id` - 员工ID
    /// * `user_id_type` - 用户ID类型（可选）
    ///
    /// # 返回值
    ///
    /// 返回员工的详细发薪明细信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::payment_detail::*;
    ///
    /// let response = service.get_payment_detail("pa_123456", "emp_001", Some("open_id")).await?;
    /// println!("员工姓名: {:?}", response.data.employee_name);
    /// println!("总金额: {}", response.data.total_amount.as_ref().unwrap_or(&"0".to_string()));
    /// ```
    pub async fn get_payment_detail(
        &self,
        payment_activity_id: &str,
        employee_id: &str,
        user_id_type: Option<&str>,
    ) -> SDKResult<BaseResponse<PaymentDetailResponse>> {
        let mut query_params = std::collections::HashMap::new();
        query_params.insert(
            "payment_activity_id".to_string(),
            payment_activity_id.to_string(),
        );
        query_params.insert("employee_id".to_string(), employee_id.to_string());

        if let Some(user_id) = user_id_type {
            query_params.insert("user_id_type".to_string(), user_id.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: self
                .config
                .build_url("/open-apis/payroll/v4/payment_details/detail"),
            query_params,
            path_params: std::collections::HashMap::new(),
            body: None,
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 导出发薪明细
    ///
    /// # API文档
    ///
    /// 将发薪明细导出为Excel文件，支持自定义字段和格式。
    ///
    /// # 参数
    ///
    /// * `payment_activity_id` - 发薪活动ID
    /// * `employee_ids` - 员工ID列表（可选，不指定则导出全部）
    /// * `fields` - 导出字段列表（可选）
    /// * `format` - 导出格式（可选，默认为xlsx）
    ///
    /// # 返回值
    ///
    /// 返回导出文件的下载链接
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::payment_detail::*;
    ///
    /// let response = service.export_payment_details(
    ///     "pa_123456",
    ///     Some(&vec!["emp_001".to_string(), "emp_002".to_string()]),
    ///     Some(&vec!["employee_name".to_string(), "total_amount".to_string()]),
    ///     Some("xlsx")
    /// ).await?;
    ///
    /// println!("导出链接: {}", response.data.download_url);
    /// ```
    pub async fn export_payment_details(
        &self,
        payment_activity_id: &str,
        employee_ids: Option<&Vec<String>>,
        fields: Option<&Vec<String>>,
        format: Option<&str>,
    ) -> SDKResult<BaseResponse<ExportResponse>> {
        let mut query_params = std::collections::HashMap::new();
        query_params.insert(
            "payment_activity_id".to_string(),
            payment_activity_id.to_string(),
        );

        if let Some(employees) = employee_ids {
            query_params.insert("employee_ids".to_string(), employees.join(","));
        }
        if let Some(field_list) = fields {
            query_params.insert("fields".to_string(), field_list.join(","));
        }
        if let Some(fmt) = format {
            query_params.insert("format".to_string(), fmt.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: self
                .config
                .build_url("/open-apis/payroll/v4/payment_details/export"),
            query_params,
            path_params: std::collections::HashMap::new(),
            body: Some(serde_json::Value::Object(serde_json::Map::new())),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }
}

/// 导出响应
#[derive(Debug, Clone)]
pub struct ExportResponse {
    /// 下载链接
    pub download_api_path: String,
    /// 文件名
    pub filename: String,
    /// 文件大小（字节）
    pub file_size: u64,
    /// 过期时间
    pub expire_time: String,
}
