use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::hire::*,
    endpoints::EndpointBuilder,
    http::Transport,
    query_params::QueryParams,
    req_option::RequestOption,
    SDKResult,
};

use crate::hire::models::{CommonResponse, PageResponse, ReferralAccount};

/// 内推账户服务
pub struct ReferralAccountService {
    pub config: Config,
}

/// 内推账户余额信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralAccountBalance {
    /// 用户ID
    pub user_id: String,
    /// 可用余额
    pub available_balance: String,
    /// 冻结余额
    pub frozen_balance: String,
    /// 总余额
    pub total_balance: String,
    /// 币种
    pub currency: String,
    /// 最后更新时间
    pub last_updated: Option<String>,
}

/// 内推收入记录
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralIncomeRecord {
    /// 记录ID
    pub id: String,
    /// 用户ID
    pub user_id: String,
    /// 内推记录ID
    pub referral_id: String,
    /// 收入类型
    pub income_type: String,
    /// 收入金额
    pub amount: String,
    /// 币种
    pub currency: String,
    /// 收入时间
    pub income_time: String,
    /// 收入描述
    pub description: Option<String>,
    /// 状态
    pub status: String,
    /// 创建时间
    pub created_time: Option<String>,
}

/// 提现记录
#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRecord {
    /// 提现记录ID
    pub id: String,
    /// 用户ID
    pub user_id: String,
    /// 提现金额
    pub amount: String,
    /// 币种
    pub currency: String,
    /// 提现方式
    pub withdrawal_method: String,
    /// 提现账户信息
    pub account_info: WithdrawalAccountInfo,
    /// 提现状态
    pub status: String,
    /// 申请时间
    pub application_time: String,
    /// 处理时间
    pub processing_time: Option<String>,
    /// 完成时间
    pub completion_time: Option<String>,
    /// 手续费
    pub fee: Option<String>,
    /// 实际到账金额
    pub actual_amount: Option<String>,
    /// 备注信息
    pub remark: Option<String>,
    /// 拒绝原因
    pub rejection_reason: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 提现账户信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WithdrawalAccountInfo {
    /// 账户类型
    pub account_type: String,
    /// 账户号码
    pub account_number: String,
    /// 账户名称
    pub account_name: String,
    /// 银行名称
    pub bank_name: Option<String>,
    /// 开户行
    pub bank_branch: Option<String>,
}

/// 内推账户创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReferralAccountCreateRequest {
    /// 用户ID
    pub user_id: String,
    /// 真实姓名
    pub real_name: String,
    /// 身份证号
    pub id_card: Option<String>,
    /// 银行卡号
    pub bank_card: Option<String>,
    /// 银行名称
    pub bank_name: Option<String>,
    /// 开户行
    pub bank_branch: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
}

/// 提现申请请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WithdrawalApplicationRequest {
    /// 用户ID
    pub user_id: String,
    /// 提现金额
    pub amount: String,
    /// 提现方式
    pub withdrawal_method: String,
    /// 提现账户信息
    pub account_info: WithdrawalAccountInfo,
    /// 申请备注
    pub remark: Option<String>,
}

/// 内推账户列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReferralAccountListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 账户状态
    pub status: Option<String>,
    /// 用户ID
    pub user_id: Option<String>,
}

/// 收入记录列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IncomeRecordListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 用户ID
    pub user_id: Option<String>,
    /// 收入类型
    pub income_type: Option<String>,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}

/// 提现记录列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WithdrawalRecordListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 用户ID
    pub user_id: Option<String>,
    /// 提现状态
    pub status: Option<String>,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}

/// 内推账户列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralAccountListResponse {
    /// 内推账户列表
    #[serde(flatten)]
    pub accounts: PageResponse<ReferralAccount>,
}

impl ApiResponseTrait for ReferralAccountListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 账户余额响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralAccountBalanceResponse {
    /// 账户余额信息
    pub balance: ReferralAccountBalance,
}

impl ApiResponseTrait for ReferralAccountBalanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 收入记录列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeRecordListResponse {
    /// 收入记录列表
    #[serde(flatten)]
    pub records: PageResponse<ReferralIncomeRecord>,
}

impl ApiResponseTrait for IncomeRecordListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 提现记录列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRecordListResponse {
    /// 提现记录列表
    #[serde(flatten)]
    pub records: PageResponse<WithdrawalRecord>,
}

impl ApiResponseTrait for WithdrawalRecordListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 内推账户操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralAccountOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 相关ID
    pub id: Option<String>,
}

impl ApiResponseTrait for ReferralAccountOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ReferralAccountService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建内推账户
    ///
    /// 该接口用于为用户创建内推账户，设置银行卡信息
    /// 和身份验证信息，用于后续的奖励发放和提现操作。
    ///
    /// # 参数
    ///
    /// - `request`: 内推账户创建请求参数，包括：
    ///   - `user_id`: 用户ID（必填）
    ///   - `real_name`: 真实姓名（必填）
    ///   - `id_card`: 身份证号
    ///   - `bank_card`: 银行卡号
    ///   - `bank_name`: 银行名称
    ///   - `bank_branch`: 开户行
    ///   - `phone`: 手机号
    ///   - `email`: 邮箱
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回内推账户创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `id`: 创建的账户ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::referral_account::ReferralAccountCreateRequest;
    ///
    /// let request = ReferralAccountCreateRequest {
    ///     user_id: "user_123456".to_string(),
    ///     real_name: "张三".to_string(),
    ///     id_card: Some("123456789012345678".to_string()),
    ///     bank_card: Some("6228480000000000000".to_string()),
    ///     bank_name: Some("招商银行".to_string()),
    ///     bank_branch: Some("北京分行".to_string()),
    ///     phone: Some("13800138000".to_string()),
    ///     email: Some("zhangsan@example.com".to_string()),
    /// };
    ///
    /// let response = client.hire.referral_account.create_account(request, None).await?;
    /// ```
    pub async fn create_account(
        &self,
        request: ReferralAccountCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralAccountOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_REFERRAL_ACCOUNTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取内推账户列表
    ///
    /// 该接口用于获取企业的内推账户列表，支持按状态、
    /// 用户等条件筛选。返回的列表包含账户基本信息，
    /// 可用于账户管理和统计。
    ///
    /// # 参数
    ///
    /// - `request`: 内推账户列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `status`: 账户状态筛选
    ///   - `user_id`: 用户ID筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的内推账户列表，包括：
    /// - 内推账户基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::referral_account::ReferralAccountListRequest;
    ///
    /// let request = ReferralAccountListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    ///     user_id: Some("user_123456".to_string()),
    /// };
    ///
    /// let response = client.hire.referral_account.list_accounts(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("账户总数: {}", data.accounts.items.len());
    ///     for account in &data.accounts.items {
    ///         println!("账户: {} 状态: {}", account.user_id, account.status);
    ///     }
    /// }
    /// ```
    pub async fn list_accounts(
        &self,
        request: ReferralAccountListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralAccountListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_REFERRAL_ACCOUNTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert(QueryParams::STATUS, status);
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取账户余额
    ///
    /// 该接口用于获取指定用户的内推账户余额信息，
    /// 包括可用余额、冻结余额和总余额。
    ///
    /// # 参数
    ///
    /// - `user_id`: 用户ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回账户余额详细信息，包括：
    /// - 可用余额和冻结余额
    /// - 总余额和币种信息
    /// - 最后更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let user_id = "user_123456";
    /// let response = client.hire.referral_account.get_balance(user_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("可用余额: {}", data.balance.available_balance);
    ///     println!("冻结余额: {}", data.balance.frozen_balance);
    ///     println!("总余额: {}", data.balance.total_balance);
    /// }
    /// ```
    pub async fn get_balance(
        &self,
        user_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralAccountBalanceResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
                api_req.set_api_path(HIRE_REFERRAL_ACCOUNT_BALANCE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取收入记录列表
    ///
    /// 该接口用于获取用户的内推收入记录列表，支持按
    /// 用户、收入类型、时间等条件筛选。返回的列表包含
    /// 收入记录详细信息，可用于收入管理和统计。
    ///
    /// # 参数
    ///
    /// - `request`: 收入记录列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `user_id`: 用户ID筛选
    ///   - `income_type`: 收入类型筛选
    ///   - `start_time`: 开始时间筛选
    ///   - `end_time`: 结束时间筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的收入记录列表，包括：
    /// - 收入记录基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::referral_account::IncomeRecordListRequest;
    ///
    /// let request = IncomeRecordListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     user_id: Some("user_123456".to_string()),
    ///     income_type: Some("referral_bonus".to_string()),
    ///     start_time: Some("2024-01-01T00:00:00Z".to_string()),
    ///     end_time: Some("2024-01-31T23:59:59Z".to_string()),
    /// };
    ///
    /// let response = client.hire.referral_account.list_income_records(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("收入记录总数: {}", data.records.items.len());
    ///     for record in &data.records.items {
    ///         println!("收入: {} 金额: {}", record.id, record.amount);
    ///     }
    /// }
    /// ```
    pub async fn list_income_records(
        &self,
        request: IncomeRecordListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<IncomeRecordListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_REFERRAL_INCOME_RECORDS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        if let Some(income_type) = request.income_type {
            api_req
                .query_params
                .insert(QueryParams::INCOME_TYPE, income_type);
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert(QueryParams::START_TIME, start_time);
        }

        if let Some(end_time) = request.end_time {
            api_req.query_params.insert(QueryParams::END_TIME, end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 申请提现
    ///
    /// 该接口用于用户申请提现，设置提现金额、
    /// 提现方式和账户信息。申请成功后将进入审核流程。
    ///
    /// # 参数
    ///
    /// - `request`: 提现申请请求参数，包括：
    ///   - `user_id`: 用户ID（必填）
    ///   - `amount`: 提现金额（必填）
    ///   - `withdrawal_method`: 提现方式（必填）
    ///   - `account_info`: 提现账户信息（必填）
    ///   - `remark`: 申请备注
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回提现申请操作结果，包括：
    /// - `success`: 申请是否成功
    /// - `id`: 创建的提现记录ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::referral_account::{
    ///     WithdrawalApplicationRequest, WithdrawalAccountInfo
    /// };
    ///
    /// let request = WithdrawalApplicationRequest {
    ///     user_id: "user_123456".to_string(),
    ///     amount: "1000.00".to_string(),
    ///     withdrawal_method: "bank_transfer".to_string(),
    ///     account_info: WithdrawalAccountInfo {
    ///         account_type: "bank_card".to_string(),
    ///         account_number: "6228480000000000000".to_string(),
    ///         account_name: "张三".to_string(),
    ///         bank_name: Some("招商银行".to_string()),
    ///         bank_branch: Some("北京分行".to_string()),
    ///     },
    ///     remark: Some("第一次提现".to_string()),
    /// };
    ///
    /// let response = client.hire.referral_account.apply_withdrawal(request, None).await?;
    /// ```
    pub async fn apply_withdrawal(
        &self,
        request: WithdrawalApplicationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralAccountOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_REFERRAL_WITHDRAWALS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取提现记录列表
    ///
    /// 该接口用于获取用户的提现记录列表，支持按
    /// 用户、状态、时间等条件筛选。返回的列表包含
    /// 提现记录详细信息，可用于提现管理和统计。
    ///
    /// # 参数
    ///
    /// - `request`: 提现记录列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `user_id`: 用户ID筛选
    ///   - `status`: 提现状态筛选
    ///   - `start_time`: 开始时间筛选
    ///   - `end_time`: 结束时间筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的提现记录列表，包括：
    /// - 提现记录基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::referral_account::WithdrawalRecordListRequest;
    ///
    /// let request = WithdrawalRecordListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     user_id: Some("user_123456".to_string()),
    ///     status: Some("pending".to_string()),
    ///     start_time: Some("2024-01-01T00:00:00Z".to_string()),
    ///     end_time: Some("2024-01-31T23:59:59Z".to_string()),
    /// };
    ///
    /// let response = client.hire.referral_account.list_withdrawal_records(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("提现记录总数: {}", data.records.items.len());
    ///     for record in &data.records.items {
    ///         println!("提现: {} 金额: {}", record.id, record.amount);
    ///     }
    /// }
    /// ```
    pub async fn list_withdrawal_records(
        &self,
        request: WithdrawalRecordListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<WithdrawalRecordListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_REFERRAL_WITHDRAWALS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert(QueryParams::STATUS, status);
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert(QueryParams::START_TIME, start_time);
        }

        if let Some(end_time) = request.end_time {
            api_req.query_params.insert(QueryParams::END_TIME, end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }
    /// 审批提现申请
    ///
    /// 该接口用于审批提现申请，设置审批结果
    /// 和相关备注信息。
    ///
    /// # 参数
    ///
    /// - `withdrawal_id`: 提现记录ID
    /// - `approved`: 是否批准
    /// - `remark`: 审批备注（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let withdrawal_id = "withdrawal_123456";
    /// let approved = true;
    /// let remark = Some("审批通过".to_string());
    ///
    /// let response = client.hire.referral_account.approve_withdrawal(
    ///     withdrawal_id,
    ///     approved,
    ///     remark,
    ///     None
    /// ).await?;
    /// ```
    pub async fn approve_withdrawal(
        &self,
        withdrawal_id: &str,
        approved: bool,
        remark: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralAccountOperationResponse>> {
        #[derive(Serialize)]
        struct ApprovalRequest {
            approved: bool,
            remark: Option<String>,
        }

        let request = ApprovalRequest { approved, remark };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_REFERRAL_ACCOUNT_BALANCE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 启用内推账户
    ///
    /// 该接口用于启用被停用的内推账户，
    /// 恢复账户的正常功能。
    ///
    /// # 参数
    ///
    /// - `user_id`: 用户ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let user_id = "user_123456";
    /// let response = client.hire.referral_account.enable_account(user_id, None).await?;
    /// ```
    pub async fn enable_account(
        &self,
        user_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralAccountOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_REFERRAL_ACCOUNT_BALANCE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 停用内推账户
    ///
    /// 该接口用于停用内推账户，暂停账户的
    /// 收入和提现功能。
    ///
    /// # 参数
    ///
    /// - `user_id`: 用户ID
    /// - `reason`: 停用原因
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let user_id = "user_123456";
    /// let reason = "违反内推规则";
    ///
    /// let response = client.hire.referral_account.disable_account(user_id, reason, None).await?;
    /// ```
    pub async fn disable_account(
        &self,
        user_id: &str,
        reason: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralAccountOperationResponse>> {
        #[derive(Serialize)]
        struct DisableRequest {
            reason: String,
        }

        let request = DisableRequest {
            reason: reason.to_string(),
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_REFERRAL_ACCOUNT_BALANCE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取内推统计数据
    ///
    /// 该接口用于获取内推相关的统计数据，包括
    /// 总收入、提现金额、内推人数等关键指标。
    ///
    /// # 参数
    ///
    /// - `start_date`: 统计开始日期（可选）
    /// - `end_date`: 统计结束日期（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let start_date = Some("2024-01-01".to_string());
    /// let end_date = Some("2024-01-31".to_string());
    ///
    /// let response = client.hire.referral_account.get_referral_statistics(
    ///     start_date,
    ///     end_date,
    ///     None
    /// ).await?;
    /// ```
    pub async fn get_referral_statistics(
        &self,
        start_date: Option<String>,
        end_date: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<serde_json::Value>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_REFERRAL_STATISTICS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(start_date) = start_date {
            api_req
                .query_params
                .insert(QueryParams::START_DATE, start_date);
        }

        if let Some(end_date) = end_date {
            api_req.query_params.insert(QueryParams::END_DATE, end_date);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
