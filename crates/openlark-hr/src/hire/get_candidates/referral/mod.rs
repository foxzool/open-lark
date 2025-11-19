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
        req_option::RequestOption,
        SDKResult,
    };

use crate::hire::models::{
    Application, CommonResponse, PageResponse, ReferralAccount, ReferralAccountCreateRequest,
    Talent, UserId,
};

/// 内推服务
pub struct ReferralService {
    pub config: Config,
}

/// 内推记录信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralRecord {
    /// 内推记录ID
    pub id: String,
    /// 内推人ID
    pub referrer_id: String,
    /// 被内推人才ID
    pub talent_id: String,
    /// 内推职位ID
    pub job_id: String,
    /// 内推状态
    pub status: String,
    /// 内推时间
    pub referral_time: String,
    /// 内推备注
    pub remark: Option<String>,
    /// 奖励金额
    pub reward_amount: Option<String>,
    /// 奖励状态
    pub reward_status: Option<String>,
    /// 内推人信息
    pub referrer: Option<UserId>,
    /// 被内推人才信息
    pub talent: Option<Talent>,
    /// 关联投递信息
    pub application: Option<Application>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 内推创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReferralCreateRequest {
    /// 内推人ID
    pub referrer_id: String,
    /// 被内推人才ID
    pub talent_id: String,
    /// 内推职位ID
    pub job_id: String,
    /// 内推备注
    pub remark: Option<String>,
    /// 内推渠道
    pub source: Option<String>,
}

/// 内推列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReferralListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 内推人ID
    pub referrer_id: Option<String>,
    /// 职位ID
    pub job_id: Option<String>,
    /// 内推状态
    pub status: Option<String>,
    /// 奖励状态
    pub reward_status: Option<String>,
    /// 内推时间开始
    pub referral_start_time: Option<String>,
    /// 内推时间结束
    pub referral_end_time: Option<String>,
}

/// 内推奖励设置
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralRewardSettings {
    /// 设置ID
    pub id: String,
    /// 职位类型
    pub job_type: String,
    /// 奖励金额
    pub reward_amount: String,
    /// 奖励币种
    pub currency: String,
    /// 奖励条件
    pub reward_conditions: Vec<String>,
    /// 奖励发放时机
    pub reward_timing: String,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 内推奖励设置创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReferralRewardSettingsCreateRequest {
    /// 职位类型
    pub job_type: String,
    /// 奖励金额
    pub reward_amount: String,
    /// 奖励币种
    pub currency: String,
    /// 奖励条件
    pub reward_conditions: Vec<String>,
    /// 奖励发放时机
    pub reward_timing: String,
    /// 是否启用
    pub enabled: Option<bool>,
}

/// 内推记录列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralListResponse {
    /// 内推记录列表
    #[serde(flatten)]
    pub referrals: PageResponse<ReferralRecord>,
}

impl ApiResponseTrait for ReferralListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 内推记录详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralDetailResponse {
    /// 内推记录信息
    pub referral: ReferralRecord,
}

impl ApiResponseTrait for ReferralDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 内推操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 内推记录ID
    pub referral_id: Option<String>,
}

impl ApiResponseTrait for ReferralOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 内推账户详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralAccountDetailResponse {
    /// 内推账户信息
    pub account: ReferralAccount,
}

impl ApiResponseTrait for ReferralAccountDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 内推奖励设置列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralRewardSettingsListResponse {
    /// 奖励设置列表
    #[serde(flatten)]
    pub settings: PageResponse<ReferralRewardSettings>,
}

impl ApiResponseTrait for ReferralRewardSettingsListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ReferralService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建内推记录
    ///
    /// 该接口用于创建新的内推记录，记录内推人、被内推人才、
    /// 内推职位等信息。创建成功后将自动生成对应的投递记录。
    ///
    /// # 参数
    ///
    /// - `request`: 内推创建请求参数，包括：
    ///   - `referrer_id`: 内推人ID（必填）
    ///   - `talent_id`: 被内推人才ID（必填）
    ///   - `job_id`: 内推职位ID（必填）
    ///   - `remark`: 内推备注
    ///   - `source`: 内推渠道
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回内推创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `referral_id`: 创建的内推记录ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::referral::ReferralCreateRequest;
    ///
    /// let request = ReferralCreateRequest {
    ///     referrer_id: "user_123456".to_string(),
    ///     talent_id: "talent_789".to_string(),
    ///     job_id: "job_abc".to_string(),
    ///     remark: Some("朋友推荐，技术能力强".to_string()),
    ///     source: Some("员工内推".to_string()),
    /// };
    ///
    /// let response = client.hire.get_candidates.referral.create_referral(request, None).await?;
    /// ```
    pub async fn create_referral(
        &self,
        request: ReferralCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_REFERRALS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取内推记录详情
    ///
    /// 该接口用于获取指定内推记录的详细信息，包括内推
    /// 基本信息、奖励状态、关联投递等完整数据。
    ///
    /// # 参数
    ///
    /// - `referral_id`: 内推记录ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回内推记录详细信息，包括：
    /// - 内推基本信息（内推人、被内推人、职位等）
    /// - 内推状态和时间
    /// - 奖励金额和状态
    /// - 关联的投递信息
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let referral_id = "ref_123456";
    /// let response = client.hire.get_candidates.referral.get_referral_detail(referral_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("内推状态: {}", data.referral.status);
    ///     println!("奖励金额: {:?}", data.referral.reward_amount);
    ///     println!("奖励状态: {:?}", data.referral.reward_status);
    /// }
    /// ```
    pub async fn get_referral_detail(
        &self,
        referral_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
                api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_REFERRAL_GET,
            "referral_id",
            referral_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取内推记录列表
    ///
    /// 该接口用于获取企业的内推记录列表，支持按内推人、
    /// 职位、状态、奖励状态、时间等条件筛选。返回的
    /// 列表包含内推基本信息，可用于内推管理和统计。
    ///
    /// # 参数
    ///
    /// - `request`: 内推列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `referrer_id`: 内推人ID筛选
    ///   - `job_id`: 职位ID筛选
    ///   - `status`: 内推状态筛选
    ///   - `reward_status`: 奖励状态筛选
    ///   - `referral_start_time`: 内推时间开始
    ///   - `referral_end_time`: 内推时间结束
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的内推记录列表，包括：
    /// - 内推记录基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::referral::ReferralListRequest;
    ///
    /// let request = ReferralListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     referrer_id: Some("user_123456".to_string()),
    ///     status: Some("active".to_string()),
    ///     reward_status: Some("pending".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.get_candidates.referral.list_referrals(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("内推记录总数: {}", data.referrals.items.len());
    ///     for referral in &data.referrals.items {
    ///         println!("内推: {} 状态: {}", referral.id, referral.status);
    ///     }
    /// }
    /// ```
    pub async fn list_referrals(
        &self,
        request: ReferralListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_REFERRALS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(referrer_id) = request.referrer_id {
            api_req.query_params.insert("referrer_id", referrer_id);
        }

        if let Some(job_id) = request.job_id {
            api_req.query_params.insert("job_id", job_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(reward_status) = request.reward_status {
            api_req.query_params.insert("reward_status", reward_status);
        }

        if let Some(referral_start_time) = request.referral_start_time {
            api_req
                .query_params
                .insert("referral_start_time", referral_start_time);
        }

        if let Some(referral_end_time) = request.referral_end_time {
            api_req
                .query_params
                .insert("referral_end_time", referral_end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 注册内推账户
    ///
    /// 该接口用于为用户注册内推账户，设置银行卡信息
    /// 和身份验证信息，用于后续的奖励发放。
    ///
    /// # 参数
    ///
    /// - `request`: 内推账户注册请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::ReferralAccountCreateRequest;
    ///
    /// let request = ReferralAccountCreateRequest {
    ///     user_id: "user_123456".to_string(),
    ///     id_card: Some("123456789012345678".to_string()),
    ///     bank_card: Some("6228480000000000000".to_string()),
    ///     bank_name: Some("招商银行".to_string()),
    /// };
    ///
    /// let response = client.hire.get_candidates.referral.register_referral_account(request, None).await?;
    /// ```
    pub async fn register_referral_account(
        &self,
        request: ReferralAccountCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_REFERRAL_ACCOUNTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取内推账户信息
    ///
    /// 该接口用于获取指定用户的内推账户信息，包括
    /// 账户余额、总收入、已提现金额等数据。
    ///
    /// # 参数
    ///
    /// - `user_id`: 用户ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回内推账户详细信息，包括：
    /// - 账户基本信息（用户ID、状态等）
    /// - 账户余额和总收入
    /// - 已提现金额
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let user_id = "user_123456";
    /// let response = client.hire.get_candidates.referral.get_referral_account(user_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("账户状态: {}", data.account.status);
    ///     println!("账户余额: {}", data.account.balance);
    ///     println!("总收入: {}", data.account.total_income);
    /// }
    /// ```
    pub async fn get_referral_account(
        &self,
        user_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralAccountDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
                api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_REFERRAL_ACCOUNT_GET,
            "account_id",
            user_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
    /// 发放内推奖励
    ///
    /// 该接口用于为符合条件的内推记录发放奖励，
    /// 将奖励金额计入内推人的账户余额。
    ///
    /// # 参数
    ///
    /// - `referral_id`: 内推记录ID
    /// - `reward_amount`: 奖励金额
    /// - `remark`: 发放备注（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let referral_id = "ref_123456";
    /// let reward_amount = "1000.00";
    /// let remark = Some("候选人已成功入职".to_string());
    ///
    /// let response = client.hire.get_candidates.referral.grant_referral_reward(
    ///     referral_id,
    ///     reward_amount,
    ///     remark,
    ///     None
    /// ).await?;
    /// ```
    pub async fn grant_referral_reward(
        &self,
        referral_id: &str,
        reward_amount: &str,
        remark: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralOperationResponse>> {
        #[derive(Serialize)]
        struct GrantRewardRequest {
            reward_amount: String,
            remark: Option<String>,
        }

        let request = GrantRewardRequest {
            reward_amount: reward_amount.to_string(),
            remark,
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_V1_REFERRAL_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 创建内推奖励设置
    ///
    /// 该接口用于创建内推奖励设置，定义不同职位类型
    /// 的奖励金额、发放条件和发放时机。
    ///
    /// # 参数
    ///
    /// - `request`: 内推奖励设置创建请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::referral::ReferralRewardSettingsCreateRequest;
    ///
    /// let request = ReferralRewardSettingsCreateRequest {
    ///     job_type: "software_engineer".to_string(),
    ///     reward_amount: "3000.00".to_string(),
    ///     currency: "CNY".to_string(),
    ///     reward_conditions: vec!["候选人入职".to_string(), "通过试用期".to_string()],
    ///     reward_timing: "after_probation".to_string(),
    ///     enabled: Some(true),
    /// };
    ///
    /// let response = client.hire.get_candidates.referral.create_reward_settings(request, None).await?;
    /// ```
    pub async fn create_reward_settings(
        &self,
        request: ReferralRewardSettingsCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_REFERRAL_REWARD_SETTINGS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 获取内推奖励设置列表
    ///
    /// 该接口用于获取企业的内推奖励设置列表，
    /// 包括不同职位类型的奖励配置。
    ///
    /// # 参数
    ///
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的奖励设置列表，包括：
    /// - 奖励设置基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.hire.get_candidates.referral.list_reward_settings(Some(20), None, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("奖励设置总数: {}", data.settings.items.len());
    ///     for setting in &data.settings.items {
    ///         println!("职位类型: {} 奖励: {}", setting.job_type, setting.reward_amount);
    ///     }
    /// }
    /// ```
    pub async fn list_reward_settings(
        &self,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ReferralRewardSettingsListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_REFERRAL_REWARD_SETTINGS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
