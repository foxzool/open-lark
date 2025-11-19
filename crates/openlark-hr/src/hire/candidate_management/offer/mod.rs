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
use crate::hire::models::{CommonResponse, I18nText, PageResponse};

/// Offer服务
pub struct OfferService {
    pub config: Config,
}

/// Offer信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    /// Offer ID
    pub id: String,
    /// 投递ID
    pub application_id: String,
    /// 候选人ID
    pub talent_id: String,
    /// 职位ID
    pub job_id: String,
    /// Offer状态
    pub status: String,
    /// Offer类型
    pub offer_type: String,
    /// 薪资包信息
    pub salary_package: SalaryPackage,
    /// 工作地点
    pub work_location: Option<I18nText>,
    /// 入职时间
    pub start_date: Option<String>,
    /// 试用期（月）
    pub probation_period: Option<u32>,
    /// Offer有效期
    pub expiration_date: Option<String>,
    /// 备注信息
    pub remark: Option<String>,
    /// 创建人
    pub creator_id: String,
    /// 审批状态
    pub approval_status: Option<String>,
    /// 发送状态
    pub send_status: Option<String>,
    /// 候选人回复状态
    pub reply_status: Option<String>,
    /// 候选人回复时间
    pub reply_time: Option<String>,
    /// 候选人回复备注
    pub reply_remark: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 薪资包信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SalaryPackage {
    /// 基本薪资
    pub base_salary: String,
    /// 薪资币种
    pub currency: String,
    /// 薪资周期
    pub pay_period: String,
    /// 奖金
    pub bonus: Option<String>,
    /// 股票期权
    pub stock_options: Option<String>,
    /// 其他福利
    pub benefits: Vec<String>,
}

/// 入职信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Onboarding {
    /// 入职记录ID
    pub id: String,
    /// Offer ID
    pub offer_id: String,
    /// 候选人ID
    pub talent_id: String,
    /// 实际入职时间
    pub actual_start_date: String,
    /// 入职状态
    pub status: String,
    /// 入职部门
    pub department: Option<String>,
    /// 直属领导
    pub manager_id: Option<String>,
    /// 工号
    pub employee_id: Option<String>,
    /// 座位安排
    pub seat_arrangement: Option<String>,
    /// 设备分配
    pub equipment_allocation: Vec<String>,
    /// 入职培训计划
    pub training_plan: Option<String>,
    /// 入职进度
    pub onboarding_progress: Vec<OnboardingProgress>,
    /// 备注信息
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 入职进度项
#[derive(Debug, Serialize, Deserialize)]
pub struct OnboardingProgress {
    /// 进度项ID
    pub id: String,
    /// 进度项名称
    pub name: String,
    /// 负责人
    pub owner_id: Option<String>,
    /// 完成状态
    pub completed: bool,
    /// 计划完成时间
    pub planned_completion_date: Option<String>,
    /// 实际完成时间
    pub actual_completion_date: Option<String>,
    /// 备注
    pub remark: Option<String>,
}

/// Offer创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OfferCreateRequest {
    /// 投递ID
    pub application_id: String,
    /// Offer类型
    pub offer_type: String,
    /// 薪资包信息
    pub salary_package: SalaryPackage,
    /// 工作地点
    pub work_location: Option<I18nText>,
    /// 入职时间
    pub start_date: Option<String>,
    /// 试用期（月）
    pub probation_period: Option<u32>,
    /// Offer有效期
    pub expiration_date: Option<String>,
    /// 备注信息
    pub remark: Option<String>,
}

/// Offer更新请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OfferUpdateRequest {
    /// 薪资包信息
    pub salary_package: Option<SalaryPackage>,
    /// 工作地点
    pub work_location: Option<I18nText>,
    /// 入职时间
    pub start_date: Option<String>,
    /// 试用期（月）
    pub probation_period: Option<u32>,
    /// Offer有效期
    pub expiration_date: Option<String>,
    /// 备注信息
    pub remark: Option<String>,
}

/// 入职创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OnboardingCreateRequest {
    /// Offer ID
    pub offer_id: String,
    /// 实际入职时间
    pub actual_start_date: String,
    /// 入职部门
    pub department: Option<String>,
    /// 直属领导
    pub manager_id: Option<String>,
    /// 工号
    pub employee_id: Option<String>,
    /// 座位安排
    pub seat_arrangement: Option<String>,
    /// 设备分配
    pub equipment_allocation: Vec<String>,
    /// 入职培训计划
    pub training_plan: Option<String>,
    /// 备注信息
    pub remark: Option<String>,
}

/// Offer列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OfferListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 候选人ID
    pub talent_id: Option<String>,
    /// 职位ID
    pub job_id: Option<String>,
    /// Offer状态
    pub status: Option<String>,
    /// 审批状态
    pub approval_status: Option<String>,
    /// 创建时间开始
    pub created_start_time: Option<String>,
    /// 创建时间结束
    pub created_end_time: Option<String>,
}

/// Offer列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferListResponse {
    /// Offer列表
    #[serde(flatten)]
    pub offers: PageResponse<Offer>,
}

impl ApiResponseTrait for OfferListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Offer详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferDetailResponse {
    /// Offer信息
    pub offer: Offer,
}

impl ApiResponseTrait for OfferDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 入职列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OnboardingListResponse {
    /// 入职记录列表
    #[serde(flatten)]
    pub onboardings: PageResponse<Onboarding>,
}

impl ApiResponseTrait for OnboardingListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Offer操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 相关ID
    pub id: Option<String>,
}

impl ApiResponseTrait for OfferOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl OfferService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建Offer
    ///
    /// 该接口用于为候选人创建Offer，包括薪资包、
    /// 工作地点、入职时间等信息。创建成功后可以
    /// 发起审批流程。
    ///
    /// # 参数
    ///
    /// - `request`: Offer创建请求参数，包括：
    ///   - `application_id`: 投递ID（必填）
    ///   - `offer_type`: Offer类型（必填）
    ///   - `salary_package`: 薪资包信息（必填）
    ///   - `work_location`: 工作地点
    ///   - `start_date`: 入职时间
    ///   - `probation_period`: 试用期
    ///   - `expiration_date`: Offer有效期
    ///   - `remark`: 备注信息
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回Offer创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `id`: 创建的Offer ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::offer::{OfferCreateRequest, SalaryPackage};
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let request = OfferCreateRequest {
    ///     application_id: "app_123456".to_string(),
    ///     offer_type: "formal".to_string(),
    ///     salary_package: SalaryPackage {
    ///         base_salary: "300000".to_string(),
    ///         currency: "CNY".to_string(),
    ///         pay_period: "yearly".to_string(),
    ///         bonus: Some("50000".to_string()),
    ///         stock_options: Some("10000".to_string()),
    ///         benefits: vec!["五险一金".to_string(), "年度体检".to_string()],
    ///     },
    ///     work_location: Some(I18nText {
    ///         zh_cn: Some("北京市朝阳区".to_string()),
    ///         en_us: Some("Chaoyang District, Beijing".to_string()),
    ///         ja_jp: None,
    ///     }),
    ///     start_date: Some("2024-02-01".to_string()),
    ///     probation_period: Some(3),
    ///     expiration_date: Some("2024-01-31T23:59:59Z".to_string()),
    ///     remark: Some("欢迎加入我们的团队！".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.offer.create_offer(request, None).await?;
    /// ```
    pub async fn create_offer(
        &self,
        request: OfferCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OfferOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_OFFERS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取Offer详情
    ///
    /// 该接口用于获取指定Offer的详细信息，包括
    /// 薪资包、审批状态、候选人回复状态等完整数据。
    ///
    /// # 参数
    ///
    /// - `offer_id`: Offer ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回Offer详细信息，包括：
    /// - Offer基本信息（薪资、地点、时间等）
    /// - 审批状态和进度
    /// - 发送状态和候选人回复
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let offer_id = "offer_123456";
    /// let response = client.hire.candidate_management.offer.get_offer_detail(offer_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("Offer状态: {}", data.offer.status);
    ///     println!("审批状态: {:?}", data.offer.approval_status);
    ///     println!("候选人回复: {:?}", data.offer.reply_status);
    /// }
    /// ```
    pub async fn get_offer_detail(
        &self,
        offer_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OfferDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_OFFER_GET, "offer_id", offer_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取Offer列表
    ///
    /// 该接口用于获取企业的Offer列表，支持按候选人、
    /// 职位、状态、时间等条件筛选。返回的列表包含
    /// Offer基本信息，可用于Offer管理和统计。
    ///
    /// # 参数
    ///
    /// - `request`: Offer列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `talent_id`: 候选人ID筛选
    ///   - `job_id`: 职位ID筛选
    ///   - `status`: Offer状态筛选
    ///   - `approval_status`: 审批状态筛选
    ///   - `created_start_time`: 创建时间开始
    ///   - `created_end_time`: 创建时间结束
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的Offer列表，包括：
    /// - Offer基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::offer::OfferListRequest;
    ///
    /// let request = OfferListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     talent_id: Some("talent_789".to_string()),
    ///     status: Some("approved".to_string()),
    ///     approval_status: Some("approved".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.candidate_management.offer.list_offers(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("Offer总数: {}", data.offers.items.len());
    ///     for offer in &data.offers.items {
    ///         println!("Offer: {} 状态: {}", offer.id, offer.status);
    ///     }
    /// }
    /// ```
    pub async fn list_offers(
        &self,
        request: OfferListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OfferListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_OFFERS.to_string());
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

        if let Some(talent_id) = request.talent_id {
            api_req.query_params.insert("talent_id", talent_id);
        }

        if let Some(job_id) = request.job_id {
            api_req.query_params.insert("job_id", job_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(approval_status) = request.approval_status {
            api_req
                .query_params
                .insert("approval_status", approval_status);
        }

        if let Some(created_start_time) = request.created_start_time {
            api_req
                .query_params
                .insert("created_start_time", created_start_time);
        }

        if let Some(created_end_time) = request.created_end_time {
            api_req
                .query_params
                .insert("created_end_time", created_end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新Offer
    ///
    /// 该接口用于更新Offer信息，包括薪资包、
    /// 工作地点、入职时间等内容。
    ///
    /// # 参数
    ///
    /// - `offer_id`: Offer ID
    /// - `request`: Offer更新请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::offer::{OfferUpdateRequest, SalaryPackage};
    ///
    /// let offer_id = "offer_123456";
    /// let request = OfferUpdateRequest {
    ///     salary_package: Some(SalaryPackage {
    ///         base_salary: "320000".to_string(),
    ///         currency: "CNY".to_string(),
    ///         pay_period: "yearly".to_string(),
    ///         bonus: Some("60000".to_string()),
    ///         stock_options: Some("12000".to_string()),
    ///         benefits: vec!["五险一金".to_string(), "年度体检".to_string(), "带薪年假".to_string()],
    ///     }),
    ///     start_date: Some("2024-02-15".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.candidate_management.offer.update_offer(offer_id, request, None).await?;
    /// ```
    pub async fn update_offer(
        &self,
        offer_id: &str,
        request: OfferUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OfferOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_OFFER_GET, "offer_id", offer_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 发送Offer
    ///
    /// 该接口用于将审批通过的Offer发送给候选人，
    /// 发送邮件或短信通知。
    ///
    /// # 参数
    ///
    /// - `offer_id`: Offer ID
    /// - `send_method`: 发送方式（email, sms, both）
    /// - `message`: 附加消息（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let offer_id = "offer_123456";
    /// let send_method = "email";
    /// let message = Some("期待您的加入！".to_string());
    ///
    /// let response = client.hire.candidate_management.offer.send_offer(
    ///     offer_id,
    ///     send_method,
    ///     message,
    ///     None
    /// ).await?;
    /// ```
    pub async fn send_offer(
        &self,
        offer_id: &str,
        send_method: &str,
        message: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OfferOperationResponse>> {
        #[derive(Serialize)]
        struct SendOfferRequest {
            send_method: String,
            message: Option<String>,
        }

        let request = SendOfferRequest {
            send_method: send_method.to_string(),
            message,
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_OFFER_SEND, "offer_id", offer_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 撤回Offer
    ///
    /// 该接口用于撤回已发送的Offer，设置撤回原因
    /// 并发送通知。
    ///
    /// # 参数
    ///
    /// - `offer_id`: Offer ID
    /// - `reason`: 撤回原因
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let offer_id = "offer_123456";
    /// let reason = "职位需求变更";
    ///
    /// let response = client.hire.candidate_management.offer.withdraw_offer(offer_id, reason, None).await?;
    /// ```
    pub async fn withdraw_offer(
        &self,
        offer_id: &str,
        reason: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OfferOperationResponse>> {
        #[derive(Serialize)]
        struct WithdrawRequest {
            reason: String,
        }

        let request = WithdrawRequest {
            reason: reason.to_string(),
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_OFFER_WITHDRAW, "offer_id", offer_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 创建入职记录
    ///
    /// 该接口用于为接受Offer的候选人创建入职记录，
    /// 设置实际入职时间、部门、领导等信息。
    ///
    /// # 参数
    ///
    /// - `request`: 入职创建请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::offer::OnboardingCreateRequest;
    ///
    /// let request = OnboardingCreateRequest {
    ///     offer_id: "offer_123456".to_string(),
    ///     actual_start_date: "2024-02-01".to_string(),
    ///     department: Some("技术部".to_string()),
    ///     manager_id: Some("manager_001".to_string()),
    ///     employee_id: Some("EMP2024001".to_string()),
    ///     seat_arrangement: Some("A区08工位".to_string()),
    ///     equipment_allocation: vec!["笔记本电脑".to_string(), "显示器".to_string()],
    ///     training_plan: Some("新员工培训计划".to_string()),
    ///     remark: Some("新员工入职".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.offer.create_onboarding(request, None).await?;
    /// ```
    pub async fn create_onboarding(
        &self,
        request: OnboardingCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OfferOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_ONBOARDINGS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取入职记录列表
    ///
    /// 该接口用于获取企业的入职记录列表，支持按
    /// 状态、部门、入职时间等条件筛选。
    ///
    /// # 参数
    ///
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `status`: 入职状态筛选（可选）
    /// - `department`: 部门筛选（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.hire.candidate_management.offer.list_onboardings(
    ///     Some(20),
    ///     None,
    ///     Some("in_progress".to_string()),
    ///     Some("技术部".to_string()),
    ///     None
    /// ).await?;
    /// ```
    pub async fn list_onboardings(
        &self,
        page_size: Option<u32>,
        page_token: Option<String>,
        status: Option<String>,
        department: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OnboardingListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_ONBOARDINGS.to_string());
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

        if let Some(status) = status {
            api_req.query_params.insert("status", status);
        }

        if let Some(department) = department {
            api_req.query_params.insert("department", department);
        }
        Transport::request(api_req, &self.config, option).await
    }

    /// 更新入职进度
    ///
    /// 该接口用于更新入职进度项的完成状态，
    /// 记录入职流程的执行情况。
    ///
    /// # 参数
    ///
    /// - `onboarding_id`: 入职记录ID
    /// - `progress_id`: 进度项ID
    /// - `completed`: 是否完成
    /// - `remark`: 备注（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let onboarding_id = "onb_123456";
    /// let progress_id = "prog_001";
    /// let completed = true;
    /// let remark = Some("已完成设备配置".to_string());
    ///
    /// let response = client.hire.candidate_management.offer.update_onboarding_progress(
    ///     onboarding_id,
    ///     progress_id,
    ///     completed,
    ///     remark,
    ///     None
    /// ).await?;
    /// ```
    pub async fn update_onboarding_progress(
        &self,
        onboarding_id: &str,
        progress_id: &str,
        completed: bool,
        remark: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OfferOperationResponse>> {
        #[derive(Serialize)]
        struct UpdateProgressRequest {
            completed: bool,
            remark: Option<String>,
        }

        let request = UpdateProgressRequest { completed, remark };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_ONBOARDING_PROGRESS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();

        Transport::request(api_req, &self.config, option).await
    }
}
