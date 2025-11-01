//! Contracts v1 - 合同管理API
//!
//! 提供完整的合同生命周期管理功能，包括：
//! - 合同信息的增删改查操作
//! - 批量合同数据处理和同步
//! - 合同搜索和高级筛选功能
//! - 合同续签和终止流程管理
//! - 合同统计分析和报表
//! - 即将到期合同预警
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::feishu_people::core::v1::contracts::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 获取合同详情
//!     let response = client.feishu_people.core.v1.contracts
//!         .get_contract_builder("contract_001")
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.contracts)
//!         .await?;
//!
//!     println!("合同类型: {}", response.data.contract_type);
//!
//!     // 创建新合同
//!     let contract = ContractCreateData {
//!         user_id: "user_001".to_string(),
//!         contract_type: "full_time".to_string(),
//!         start_date: "2024-01-01".to_string(),
//!         end_date: "2026-12-31".to_string(),
//!         salary: Some(15000.0),
//!         work_location: Some("北京市".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let create_response = client.feishu_people.core.v1.contracts
//!         .create_contract_builder()
//!         .contract_data(contract)
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.contracts)
//!         .await?;
//!
//!     println!("合同创建成功，ID: {}", create_response.data.contract_id);
//!
//!     Ok(())
//! }
//! ```

use crate::core::{
    api_resp::BaseResponse,
    config::Config,
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};
use open_lark_core::core::api_req::ApiRequest; // trait_system::ExecutableBuilder temporarily disabled
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// 合同管理服务
///
/// 提供完整的合同管理功能，包括合同创建、查询、更新、终止、续签等操作。
/// 支持批量操作、高级搜索和统计分析功能。
#[derive(Debug, Clone)]
pub struct ContractsService {
    config: Config,
}

impl ContractsService {
    /// 创建新的合同管理服务实例
    ///
    /// # 参数
    /// * `config` - 飞书应用配置信息
    ///
    /// # 返回
    /// 返回合同管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取合同详情
    ///
    /// 根据合同ID获取详细的合同信息，包括基本信息、薪资、工作地点等。
    ///
    /// # 参数
    /// * `contract_id` - 合同ID
    /// * `user_id_type` - 用户ID类型，支持 open_id、user_id、union_id
    ///
    /// # 返回
    /// 返回合同详情信息
    ///
    /// # 错误
    /// * `LarkErrorCode::InvalidParameter` - 参数不合法
    /// * `LarkErrorCode::ContractNotFound` - 合同不存在
    /// * `LarkErrorCode::PermissionDenied` - 没有权限访问
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = client.feishu_people.core.v1.contracts
    ///     .get("contract_001", "open_id").await?;
    /// println!("合同状态: {}", response.data.contract_status);
    /// ```
    pub async fn get(
        &self,
        contract_id: &str,
        user_id_type: &str,
    ) -> SDKResult<BaseResponse<Contract>> {
        // 参数验证
        if contract_id.is_empty() {
            return Err(LarkAPIError::invalid_parameter("合同ID不能为空").into());
        }
        if !["open_id", "user_id", "union_id"].contains(&user_id_type) {
            return Err(LarkAPIError::invalid_parameter("用户ID类型不合法").into());
        }

        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts/{}",
            self.config.domain_url, contract_id
        );

        let mut query_params = HashMap::new();
        query_params.insert("user_id_type", user_id_type);

        let request = ApiRequest::builder()
            .method("GET")
            .url(&url)
            .query_params(query_params)
            .build()?;

        let base_response: BaseResponse<Contract> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }

    /// 批量获取合同信息
    ///
    /// 根据合同ID列表批量获取合同信息，支持分页查询。
    ///
    /// # 参数
    /// * `contract_ids` - 合同ID列表，最多支持100个
    /// * `user_id_type` - 用户ID类型，支持 open_id、user_id、union_id
    ///
    /// # 返回
    /// 返回合同列表信息，包含分页数据
    ///
    /// # 错误
    /// * `LarkErrorCode::InvalidParameter` - 参数不合法或超过数量限制
    /// * `LarkErrorCode::PermissionDenied` - 没有权限访问
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let contract_ids = vec!["contract_001".to_string(), "contract_002".to_string()];
    /// let response = client.feishu_people.core.v1.contracts
    ///     .batch_get(&contract_ids, "open_id").await?;
    /// println!("获取到 {} 个合同", response.data.total.unwrap_or(0));
    /// ```
    pub async fn batch_get(
        &self,
        contract_ids: &[String],
        user_id_type: &str,
    ) -> SDKResult<BaseResponse<PageResponse<Contract>>> {
        // 参数验证
        if contract_ids.is_empty() {
            return Err(LarkAPIError::invalid_parameter("合同ID列表不能为空").into());
        }
        if contract_ids.len() > 100 {
            return Err(LarkAPIError::invalid_parameter("合同ID数量不能超过100个").into());
        }
        if !["open_id", "user_id", "union_id"].contains(&user_id_type) {
            return Err(LarkAPIError::invalid_parameter("用户ID类型不合法").into());
        }

        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts/batchGet",
            self.config.domain_url
        );

        let mut query_params = HashMap::new();
        query_params.insert("user_id_type", user_id_type);

        let request_body = json!({
            "contract_ids": contract_ids
        });

        let request = ApiRequest::builder()
            .method("POST")
            .url(&url)
            .query_params(query_params)
            .body(request_body)
            .build()?;

        let base_response: BaseResponse<PageResponse<Contract>> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }

    /// 根据用户获取合同列表
    ///
    /// 根据用户ID获取该用户的所有合同信息，支持分页查询。
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `page_size` - 分页大小，默认20，最大100
    /// * `page_token` - 分页标记，用于获取下一页数据
    ///
    /// # 返回
    /// 返回用户的合同列表信息，包含分页数据
    ///
    /// # 错误
    /// * `LarkErrorCode::InvalidParameter` - 参数不合法
    /// * `LarkErrorCode::UserNotFound` - 用户不存在
    /// * `LarkErrorCode::PermissionDenied` - 没有权限访问
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = client.feishu_people.core.v1.contracts
    ///     .get_by_user("user_001", 20, None).await?;
    /// println!("用户合同数量: {}", response.data.total.unwrap_or(0));
    /// ```
    pub async fn get_by_user(
        &self,
        user_id: &str,
        page_size: i32,
        page_token: Option<&str>,
    ) -> SDKResult<BaseResponse<PageResponse<Contract>>> {
        // 参数验证
        if user_id.is_empty() {
            return Err(LarkAPIError::invalid_parameter("用户ID不能为空").into());
        }
        if page_size < 1 || page_size > 100 {
            return Err(LarkAPIError::invalid_parameter("分页大小必须在1-100之间").into());
        }

        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts",
            self.config.domain_url
        );

        let mut query_params = HashMap::new();
        query_params.insert("user_id", user_id);
        query_params.insert("page_size", &page_size.to_string());

        if let Some(token) = page_token {
            query_params.insert("page_token", token);
        }

        let request = ApiRequest::builder()
            .method("GET")
            .url(&url)
            .query_params(query_params)
            .build()?;

        let base_response: BaseResponse<PageResponse<Contract>> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }

    /// 搜索合同
    ///
    /// 根据关键字搜索合同信息，支持按合同类型、状态、部门等条件筛选。
    ///
    /// # 参数
    /// * `query` - 搜索关键字，支持用户姓名、合同编号等
    /// * `page_size` - 分页大小，默认20，最大100
    /// * `page_token` - 分页标记，用于获取下一页数据
    ///
    /// # 返回
    /// 返回搜索结果，包含匹配分数和分页信息
    ///
    /// # 错误
    /// * `LarkErrorCode::InvalidParameter` - 参数不合法
    /// * `LarkErrorCode::PermissionDenied` - 没有权限访问
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = client.feishu_people.core.v1.contracts
    ///     .search("技术部", 20, None).await?;
    /// println!("搜索到 {} 个合同", response.data.total.unwrap_or(0));
    /// ```
    pub async fn search(
        &self,
        query: &str,
        page_size: i32,
        page_token: Option<&str>,
    ) -> SDKResult<BaseResponse<ContractSearchData>> {
        // 参数验证
        if query.is_empty() {
            return Err(LarkAPIError::invalid_parameter("搜索关键字不能为空").into());
        }
        if page_size < 1 || page_size > 100 {
            return Err(LarkAPIError::invalid_parameter("分页大小必须在1-100之间").into());
        }

        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts/search",
            self.config.domain_url
        );

        let mut query_params = HashMap::new();
        query_params.insert("query", query);
        query_params.insert("page_size", &page_size.to_string());

        if let Some(token) = page_token {
            query_params.insert("page_token", token);
        }

        let request = ApiRequest::builder()
            .method("GET")
            .url(&url)
            .query_params(query_params)
            .build()?;

        let base_response: BaseResponse<PageResponse<Contract>> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }

    /// 创建合同
    ///
    /// 创建新的合同信息，支持全日制、兼职、实习等多种合同类型。
    ///
    /// # 参数
    /// * `create_data` - 合同创建数据，包含用户信息、合同类型、薪资等
    ///
    /// # 返回
    /// 返回创建成功的合同信息
    ///
    /// # 错误
    /// * `LarkErrorCode::InvalidParameter` - 参数不合法
    /// * `LarkErrorCode::UserNotFound` - 用户不存在
    /// * `LarkErrorCode::ContractAlreadyExists` - 合同已存在
    /// * `LarkErrorCode::PermissionDenied` - 没有权限创建
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let contract = ContractCreateData {
    ///     user_id: "user_001".to_string(),
    ///     contract_type: "full_time".to_string(),
    ///     start_date: "2024-01-01".to_string(),
    ///     end_date: "2026-12-31".to_string(),
    ///     salary: Some(15000.0),
    ///     ..Default::default()
    /// };
    /// let response = client.feishu_people.core.v1.contracts
    ///     .create(&contract).await?;
    /// println!("合同创建成功，ID: {}", response.data.contract_id);
    /// ```
    pub async fn create(
        &self,
        create_data: &ContractCreateData,
    ) -> SDKResult<BaseResponse<Contract>> {
        // 参数验证
        if create_data.user_id.is_empty() {
            return Err(LarkAPIError::invalid_parameter("用户ID不能为空").into());
        }
        if create_data.contract_type.is_empty() {
            return Err(LarkAPIError::invalid_parameter("合同类型不能为空").into());
        }
        if create_data.start_date.is_empty() || create_data.end_date.is_empty() {
            return Err(LarkAPIError::invalid_parameter("合同开始和结束日期不能为空").into());
        }

        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts",
            self.config.domain_url
        );

        let request = ApiRequest::builder()
            .method("POST")
            .url(&url)
            .body(create_data)
            .build()?;

        let base_response: BaseResponse<PageResponse<Contract>> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }

    /// 更新合同信息
    ///
    /// 更新现有合同的信息，支持部分字段更新。
    ///
    /// # 参数
    /// * `contract_id` - 合同ID
    /// * `update_data` - 合同更新数据，只包含需要更新的字段
    ///
    /// # 返回
    /// 返回更新后的合同信息
    ///
    /// # 错误
    /// * `LarkErrorCode::InvalidParameter` - 参数不合法
    /// * `LarkErrorCode::ContractNotFound` - 合同不存在
    /// * `LarkErrorCode::PermissionDenied` - 没有权限更新
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let update_data = ContractUpdateData {
    ///     salary: Some(18000.0),
    ///     work_location: Some("上海市".to_string()),
    ///     ..Default::default()
    /// };
    /// let response = client.feishu_people.core.v1.contracts
    ///     .update("contract_001", &update_data).await?;
    /// println!("合同更新成功");
    /// ```
    pub async fn update(
        &self,
        contract_id: &str,
        update_data: &ContractUpdateData,
    ) -> SDKResult<BaseResponse<Contract>> {
        // 参数验证
        if contract_id.is_empty() {
            return Err(LarkAPIError::invalid_parameter("合同ID不能为空").into());
        }

        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts/{}",
            self.config.domain_url, contract_id
        );

        let request = ApiRequest::builder()
            .method("PATCH")
            .url(&url)
            .body(update_data)
            .build()?;

        let base_response: BaseResponse<PageResponse<Contract>> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }

    /// 终止合同
    ///
    /// 终止指定合同，需要提供终止原因和终止日期。
    ///
    /// # 参数
    /// * `contract_id` - 合同ID
    /// * `reason` - 终止原因
    /// * `termination_date` - 终止日期，格式为YYYY-MM-DD
    ///
    /// # 返回
    /// 返回终止后的合同信息
    ///
    /// # 错误
    /// * `LarkErrorCode::InvalidParameter` - 参数不合法
    /// * `LarkErrorCode::ContractNotFound` - 合同不存在
    /// * `LarkErrorCode::ContractAlreadyTerminated` - 合同已终止
    /// * `LarkErrorCode::PermissionDenied` - 没有权限终止
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = client.feishu_people.core.v1.contracts
    ///     .terminate("contract_001", "员工辞职", "2024-06-30").await?;
    /// println!("合同终止成功");
    /// ```
    pub async fn terminate(
        &self,
        contract_id: &str,
        reason: &str,
        termination_date: &str,
    ) -> SDKResult<BaseResponse<Contract>> {
        // 参数验证
        if contract_id.is_empty() {
            return Err(LarkAPIError::invalid_parameter("合同ID不能为空").into());
        }
        if reason.is_empty() {
            return Err(LarkAPIError::invalid_parameter("终止原因不能为空").into());
        }
        if termination_date.is_empty() {
            return Err(LarkAPIError::invalid_parameter("终止日期不能为空").into());
        }

        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts/{}/terminate",
            self.config.domain_url, contract_id
        );

        let request_body = json!({
            "termination_reason": reason,
            "termination_date": termination_date
        });

        let request = ApiRequest::builder()
            .method("POST")
            .url(&url)
            .body(request_body)
            .build()?;

        let base_response: BaseResponse<PageResponse<Contract>> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }

    /// 续签合同
    ///
    /// 续签现有合同，更新合同结束日期和相关条款。
    ///
    /// # 参数
    /// * `contract_id` - 合同ID
    /// * `new_end_date` - 新的合同结束日期，格式为YYYY-MM-DD
    /// * `renewal_terms` - 续签条款，包含新薪资等信息
    ///
    /// # 返回
    /// 返回续签后的合同信息
    ///
    /// # 错误
    /// * `LarkErrorCode::InvalidParameter` - 参数不合法
    /// * `LarkErrorCode::ContractNotFound` - 合同不存在
    /// * `LarkErrorCode::ContractNotRenewable` - 合同不可续签
    /// * `LarkErrorCode::PermissionDenied` - 没有权限续签
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let renewal_terms = ContractRenewalData {
    ///     new_salary: Some(20000.0),
    ///     renewal_terms: Some("薪资调整，职责范围扩大".to_string()),
    ///     ..Default::default()
    /// };
    /// let response = client.feishu_people.core.v1.contracts
    ///     .renew("contract_001", "2027-12-31", &renewal_terms).await?;
    /// println!("合同续签成功");
    /// ```
    pub async fn renew(
        &self,
        contract_id: &str,
        new_end_date: &str,
        renewal_terms: &ContractRenewalData,
    ) -> SDKResult<BaseResponse<Contract>> {
        // 参数验证
        if contract_id.is_empty() {
            return Err(LarkAPIError::invalid_parameter("合同ID不能为空").into());
        }
        if new_end_date.is_empty() {
            return Err(LarkAPIError::invalid_parameter("新的结束日期不能为空").into());
        }

        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts/{}/renew",
            self.config.domain_url, contract_id
        );

        let mut request_body = json!({
            "new_end_date": new_end_date
        });

        if let Some(new_salary) = renewal_terms.new_salary {
            request_body["new_salary"] = json!(new_salary);
        }
        if let Some(terms) = &renewal_terms.renewal_terms {
            request_body["renewal_terms"] = json!(terms);
        }
        if let Some(remarks) = &renewal_terms.remarks {
            request_body["remarks"] = json!(remarks);
        }

        let request = ApiRequest::builder()
            .method("POST")
            .url(&url)
            .body(request_body)
            .build()?;

        let base_response: BaseResponse<PageResponse<Contract>> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }

    /// 获取即将到期的合同
    ///
    /// 获取指定天数内即将到期的合同列表，用于合同续签提醒。
    ///
    /// # 参数
    /// * `days` - 天数范围，如30表示30天内到期
    /// * `page_size` - 分页大小，默认20，最大100
    /// * `page_token` - 分页标记，用于获取下一页数据
    ///
    /// # 返回
    /// 返回即将到期的合同列表，包含距离到期天数
    ///
    /// # 错误
    /// * `LarkErrorCode::InvalidParameter` - 参数不合法
    /// * `LarkErrorCode::PermissionDenied` - 没有权限访问
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = client.feishu_people.core.v1.contracts
    ///     .get_expiring_contracts(30, 20, None).await?;
    /// println!("30天内到期的合同数: {}", response.data.total.unwrap_or(0));
    /// ```
    pub async fn get_expiring_contracts(
        &self,
        days: i32,
        page_size: i32,
        page_token: Option<&str>,
    ) -> SDKResult<BaseResponse<PageResponse<Contract>>> {
        // 参数验证
        if days < 1 || days > 365 {
            return Err(LarkAPIError::invalid_parameter("天数范围必须在1-365之间").into());
        }
        if page_size < 1 || page_size > 100 {
            return Err(LarkAPIError::invalid_parameter("分页大小必须在1-100之间").into());
        }

        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts/expiring",
            self.config.domain_url
        );

        let mut query_params = HashMap::new();
        query_params.insert("days", &days.to_string());
        query_params.insert("page_size", &page_size.to_string());

        if let Some(token) = page_token {
            query_params.insert("page_token", token);
        }

        let request = ApiRequest::builder()
            .method("GET")
            .url(&url)
            .query_params(query_params)
            .build()?;

        let base_response: BaseResponse<PageResponse<Contract>> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }

    /// 获取合同统计信息
    ///
    /// 获取合同统计数据，包括总数、状态分布、平均薪资等。
    ///
    /// # 参数
    /// * `department_id` - 部门ID，可选。指定后只统计该部门数据
    ///
    /// # 返回
    /// 返回合同统计信息
    ///
    /// # 错误
    /// * `LarkErrorCode::PermissionDenied` - 没有权限访问
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = client.feishu_people.core.v1.contracts
    ///     .get_statistics(Some("dept_001")).await?;
    /// println!("总合同数: {}", response.data.total_contracts);
    /// println!("平均薪资: {}", response.data.average_salary);
    /// ```
    pub async fn get_statistics(
        &self,
        department_id: Option<&str>,
    ) -> SDKResult<BaseResponse<ContractStatistics>> {
        let url = format!(
            "{}/open-apis/feishu_people/v1/contracts/statistics",
            self.config.domain_url
        );

        let mut query_params = HashMap::new();
        if let Some(dept_id) = department_id {
            query_params.insert("department_id", dept_id);
        }

        let request = ApiRequest::builder()
            .method("GET")
            .url(&url)
            .query_params(query_params)
            .build()?;

        let base_response: BaseResponse<PageResponse<Contract>> =
            Transport::request(request, &self.config, None).await?;
        Ok(base_response)
    }
}

// ==================== Builder 实现 ====================

/// 获取合同详情构建器
pub struct GetContractBuilder<'a> {
    service: &'a ContractsService,
    contract_id: String,
    user_id_type: Option<String>,
}

impl<'a> GetContractBuilder<'a> {
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<Contract>> {
        let user_id_type = self.user_id_type.unwrap_or_else(|| "open_id".to_string());
        self.service.get(&self.contract_id, &user_id_type).await
    }
}

/// 批量获取合同信息构建器
pub struct BatchGetContractsBuilder<'a> {
    service: &'a ContractsService,
    contract_ids: Vec<String>,
    user_id_type: Option<String>,
}

impl<'a> BatchGetContractsBuilder<'a> {
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<PageResponse<Contract>>> {
        let user_id_type = self.user_id_type.unwrap_or_else(|| "open_id".to_string());
        self.service
            .batch_get(&self.contract_ids, &user_id_type)
            .await
    }
}

/// 根据用户获取合同列表构建器
pub struct GetContractsByUserBuilder<'a> {
    service: &'a ContractsService,
    user_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl<'a> GetContractsByUserBuilder<'a> {
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<PageResponse<Contract>>> {
        let page_size = self.page_size.unwrap_or(20);
        let page_token = self.page_token.as_deref();
        self.service
            .get_by_user(&self.user_id, page_size, page_token)
            .await
    }
}

/// 搜索合同构建器
pub struct SearchContractsBuilder<'a> {
    service: &'a ContractsService,
    query: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl<'a> SearchContractsBuilder<'a> {
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<ContractSearchData>> {
        let page_size = self.page_size.unwrap_or(20);
        let page_token = self.page_token.as_deref();
        self.service
            .search(&self.query, page_size, page_token)
            .await
    }
}

/// 创建合同构建器
pub struct CreateContractBuilder<'a> {
    service: &'a ContractsService,
    contract_data: Option<ContractCreateData>,
}

impl<'a> CreateContractBuilder<'a> {
    pub fn contract_data(mut self, contract_data: ContractCreateData) -> Self {
        self.contract_data = Some(contract_data);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<Contract>> {
        let contract_data = self
            .contract_data
            .ok_or_else(|| LarkAPIError::invalid_parameter("合同数据不能为空"))?;
        self.service.create(&contract_data).await
    }
}

/// 更新合同信息构建器
pub struct UpdateContractBuilder<'a> {
    service: &'a ContractsService,
    contract_id: String,
    update_data: Option<ContractUpdateData>,
}

impl<'a> UpdateContractBuilder<'a> {
    pub fn update_data(mut self, update_data: ContractUpdateData) -> Self {
        self.update_data = Some(update_data);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<Contract>> {
        let update_data = self
            .update_data
            .ok_or_else(|| LarkAPIError::invalid_parameter("更新数据不能为空"))?;
        self.service.update(&self.contract_id, &update_data).await
    }
}

/// 终止合同构建器
pub struct TerminateContractBuilder<'a> {
    service: &'a ContractsService,
    contract_id: String,
    reason: Option<String>,
    termination_date: Option<String>,
}

impl<'a> TerminateContractBuilder<'a> {
    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_string());
        self
    }

    pub fn termination_date(mut self, termination_date: &str) -> Self {
        self.termination_date = Some(termination_date.to_string());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<Contract>> {
        let reason = self
            .reason
            .ok_or_else(|| LarkAPIError::invalid_parameter("终止原因不能为空"))?;
        let termination_date = self
            .termination_date
            .ok_or_else(|| LarkAPIError::invalid_parameter("终止日期不能为空"))?;
        self.service
            .terminate(&self.contract_id, &reason, &termination_date)
            .await
    }
}

/// 续签合同构建器
pub struct RenewContractBuilder<'a> {
    service: &'a ContractsService,
    contract_id: String,
    new_end_date: Option<String>,
    renewal_terms: Option<ContractRenewalData>,
}

impl<'a> RenewContractBuilder<'a> {
    pub fn new_end_date(mut self, new_end_date: &str) -> Self {
        self.new_end_date = Some(new_end_date.to_string());
        self
    }

    pub fn renewal_terms(mut self, renewal_terms: ContractRenewalData) -> Self {
        self.renewal_terms = Some(renewal_terms);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<Contract>> {
        let new_end_date = self
            .new_end_date
            .ok_or_else(|| LarkAPIError::invalid_parameter("新的结束日期不能为空"))?;
        let renewal_terms = self.renewal_terms.unwrap_or_default();
        self.service
            .renew(&self.contract_id, &new_end_date, &renewal_terms)
            .await
    }
}

/// 获取即将到期的合同构建器
pub struct GetExpiringContractsBuilder<'a> {
    service: &'a ContractsService,
    days: Option<i32>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl<'a> GetExpiringContractsBuilder<'a> {
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<PageResponse<Contract>>> {
        let days = self.days.unwrap_or(30);
        let page_size = self.page_size.unwrap_or(20);
        let page_token = self.page_token.as_deref();
        self.service
            .get_expiring_contracts(days, page_size, page_token)
            .await
    }
}

/// 获取合同统计信息构建器
pub struct GetContractStatisticsBuilder<'a> {
    service: &'a ContractsService,
    department_id: Option<String>,
}

impl<'a> GetContractStatisticsBuilder<'a> {
    pub fn department_id(mut self, department_id: &str) -> Self {
        self.department_id = Some(department_id.to_string());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BaseResponse<ContractStatistics>> {
        let department_id = self.department_id.as_deref();
        self.service.get_statistics(department_id).await
    }
}

// ==================== 便捷方法实现 ====================

impl ContractsService {
    /// 获取合同详情构建器
    pub fn get_contract_builder(&self, contract_id: &str) -> GetContractBuilder {
        GetContractBuilder {
            service: self,
            contract_id: contract_id.to_string(),
            user_id_type: None,
        }
    }

    /// 批量获取合同信息构建器
    pub fn batch_get_contracts_builder(&self, contract_ids: &[String]) -> BatchGetContractsBuilder {
        BatchGetContractsBuilder {
            service: self,
            contract_ids: contract_ids.to_vec(),
            user_id_type: None,
        }
    }

    /// 根据用户获取合同列表构建器
    pub fn get_contracts_by_user_builder(&self, user_id: &str) -> GetContractsByUserBuilder {
        GetContractsByUserBuilder {
            service: self,
            user_id: user_id.to_string(),
            page_size: None,
            page_token: None,
        }
    }

    /// 搜索合同构建器
    pub fn search_contracts_builder(&self, query: &str) -> SearchContractsBuilder {
        SearchContractsBuilder {
            service: self,
            query: query.to_string(),
            page_size: None,
            page_token: None,
        }
    }

    /// 创建合同构建器
    pub fn create_contract_builder(&self) -> CreateContractBuilder {
        CreateContractBuilder {
            service: self,
            contract_data: None,
        }
    }

    /// 更新合同信息构建器
    pub fn update_contract_builder(&self, contract_id: &str) -> UpdateContractBuilder {
        UpdateContractBuilder {
            service: self,
            contract_id: contract_id.to_string(),
            update_data: None,
        }
    }

    /// 终止合同构建器
    pub fn terminate_contract_builder(&self, contract_id: &str) -> TerminateContractBuilder {
        TerminateContractBuilder {
            service: self,
            contract_id: contract_id.to_string(),
            reason: None,
            termination_date: None,
        }
    }

    /// 续签合同构建器
    pub fn renew_contract_builder(&self, contract_id: &str) -> RenewContractBuilder {
        RenewContractBuilder {
            service: self,
            contract_id: contract_id.to_string(),
            new_end_date: None,
            renewal_terms: None,
        }
    }

    /// 获取即将到期的合同构建器
    pub fn get_expiring_contracts_builder(&self, days: i32) -> GetExpiringContractsBuilder {
        GetExpiringContractsBuilder {
            service: self,
            days: Some(days),
            page_size: None,
            page_token: None,
        }
    }

    /// 获取合同统计信息构建器
    pub fn get_contract_statistics_builder(&self) -> GetContractStatisticsBuilder {
        GetContractStatisticsBuilder {
            service: self,
            department_id: None,
        }
    }
}

// ==================== 数据模型 ====================

/// 合同搜索数据
///
/// 包含合同搜索的结果列表和分页信息，支持按匹配度排序。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSearchData {
    /// 搜索结果项，按匹配分数降序排列
    pub items: Vec<ContractSearchResult>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 总匹配结果数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// 合同搜索结果
///
/// 单个合同的搜索结果，包含合同基本信息和匹配分数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractSearchResult {
    /// 合同唯一标识符
    pub contract_id: String,
    /// 合同持有人的姓名
    pub user_name: String,
    /// 合同类型，如 full_time（全职）、part_time（兼职）、internship（实习）
    pub contract_type: String,
    /// 合同当前状态，如 draft（草稿）、active（生效）、expired（已过期）、terminated（已终止）
    pub contract_status: String,
    /// 合同所属部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 职位描述或岗位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    /// 合同开始日期，格式为 YYYY-MM-DD
    pub start_date: String,
    /// 合同结束日期，格式为 YYYY-MM-DD
    pub end_date: String,
    /// 搜索匹配分数，范围 0.0-1.0，1.0 表示完全匹配
    pub match_score: f64,
}

/// 合同统计信息
///
/// 提供合同的统计分析数据，包括数量统计、到期预警和分布情况。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractStatistics {
    /// 总合同数量（所有状态）
    pub total_contracts: i32,
    /// 当前生效的合同数量
    pub active_contracts: i32,
    /// 已过期的合同数量
    pub expired_contracts: i32,
    /// 已终止的合同数量
    pub terminated_contracts: i32,
    /// 未来30天内即将到期的合同数量
    pub expiring_next_30_days: i32,
    /// 未来90天内即将到期的合同数量
    pub expiring_next_90_days: i32,
    /// 所有合同的平均薪资（保留2位小数）
    pub average_salary: f64,
    /// 合同类型分布统计，JSON 格式，包含各种合同类型的数量和占比
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type_distribution: Option<serde_json::Value>,
    /// 部门分布统计，JSON 格式，包含各部门的合同数量分布
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_distribution: Option<serde_json::Value>,
}

/// 合同信息
///
/// 完整的合同详细信息，包含基本信息、薪资条款、工作安排和状态变更记录。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Contract {
    /// 合同唯一标识符，系统自动生成
    pub contract_id: String,
    /// 合同关联的用户ID
    pub user_id: String,
    /// 合同类型，如 full_time（全职）、part_time（兼职）、internship（实习）、contractor（外包）
    pub contract_type: String,
    /// 合同当前状态，如 draft（草稿）、active（生效）、expired（已过期）、terminated（已终止）、suspended（暂停）
    pub contract_status: String,
    /// 合同开始日期，格式为 YYYY-MM-DD
    pub start_date: String,
    /// 合同结束日期，格式为 YYYY-MM-DD
    pub end_date: String,
    /// 合同签署日期，格式为 YYYY-MM-DD，合同正式签署后填写
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_date: Option<String>,
    /// 月薪金额（税前），单位为元
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary: Option<f64>,
    /// 试用期长度，单位为月，如 3 表示3个月试用期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_period: Option<i32>,
    /// 主要工作地点，如 "北京市海淀区中关村"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
    /// 工作时间安排，如 "9:00-18:00，周末双休"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<String>,
    /// 职位描述或岗位名称，如 "高级前端工程师"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    /// 所属部门名称，如 "技术研发部"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 合同终止原因，如 "员工主动辞职"、"公司裁员"、"合同到期"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_reason: Option<String>,
    /// 合同实际终止日期，格式为 YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
    /// 合同续签次数，首次为0，每次续签递增
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_count: Option<i32>,
    /// 距离合同到期的天数，仅对生效中的合同有效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_expiry: Option<i32>,
    /// 合同创建时间，ISO 8601 格式，如 "2024-01-01T10:00:00Z"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 合同最后更新时间，ISO 8601 格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// 扩展属性，JSON 格式，用于存储自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_attributes: Option<serde_json::Value>,
}

/// 合同创建数据
///
/// 创建新合同时需要提供的数据结构，包含必要的合同条款信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractCreateData {
    /// 合同关联的用户ID，必填字段
    pub user_id: String,
    /// 合同类型，必填字段，支持 full_time、part_time、internship、contractor
    pub contract_type: String,
    /// 合同开始日期，必填字段，格式为 YYYY-MM-DD
    pub start_date: String,
    /// 合同结束日期，必填字段，格式为 YYYY-MM-DD
    pub end_date: String,
    /// 月薪金额（税前），单位为元
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary: Option<f64>,
    /// 试用期长度，单位为月
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_period: Option<i32>,
    /// 主要工作地点，如 "北京市海淀区"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
    /// 工作时间安排，如 "9:00-18:00，周末双休"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<String>,
    /// 职位描述或岗位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    /// 所属部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 扩展属性，JSON 格式，用于存储自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_attributes: Option<serde_json::Value>,
}

/// 合同更新数据
///
/// 更新现有合同时使用的数据结构，支持部分字段更新，只需要提供需要修改的字段。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractUpdateData {
    /// 合同关联的用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 合同类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    /// 合同状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_status: Option<String>,
    /// 合同开始日期，格式为 YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 合同结束日期，格式为 YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 合同签署日期，格式为 YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_date: Option<String>,
    /// 月薪金额（税前），单位为元
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary: Option<f64>,
    /// 试用期长度，单位为月
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_period: Option<i32>,
    /// 主要工作地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
    /// 工作时间安排
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<String>,
    /// 职位描述或岗位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    /// 所属部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 扩展属性，JSON 格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_attributes: Option<serde_json::Value>,
}

/// 合同续签数据
///
/// 续签合同时需要提供的数据结构，包含薪资调整和续签条款。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractRenewalData {
    /// 续签后的新薪资，如不调整则不填写
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_salary: Option<f64>,
    /// 续签条件和条款说明，如薪资调整原因、职责变化等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_terms: Option<String>,
    /// 备注信息，记录续签相关说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

/// 分页响应
///
/// 通用的分页数据结构，用于返回列表数据和分页信息。
///
/// # 泛型参数
/// * `T` - 数据项的类型，如 Contract、ContractSearchResult 等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 当前页的数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据，为空时表示没有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据，true 表示有下一页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 总记录数量，可选字段，某些查询可能不提供总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// 空响应（重新导出）
pub use crate::core::api_resp::EmptyResponse;
