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
    Application, ApplicationCreateRequest, ApplicationListRequest, CommonResponse, Interview,
    Offer, OfferCreateRequest, PageResponse,
};

/// 投递服务
pub struct ApplicationService {
    pub config: Config,
}

/// 投递阶段推进请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApplicationAdvanceRequest {
    /// 投递ID
    pub application_id: String,
    /// 目标阶段ID
    pub target_stage_id: String,
    /// 推进原因
    pub reason: Option<String>,
    /// 备注
    pub remark: Option<String>,
}

/// 投递评价请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApplicationEvaluationRequest {
    /// 投递ID
    pub application_id: String,
    /// 评价内容
    pub evaluation: String,
    /// 评分
    pub score: Option<f32>,
    /// 评价人ID
    pub evaluator_id: String,
    /// 评价时间
    pub evaluation_time: Option<String>,
}

/// 投递列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationListResponse {
    /// 投递列表
    #[serde(flatten)]
    pub applications: PageResponse<Application>,
}

impl ApiResponseTrait for ApplicationListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 投递详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationDetailResponse {
    /// 投递信息
    pub application: Application,
}

impl ApiResponseTrait for ApplicationDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 投递操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 投递ID
    pub application_id: Option<String>,
}

impl ApiResponseTrait for ApplicationOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 面试列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewListResponse {
    /// 面试列表
    #[serde(flatten)]
    pub interviews: PageResponse<Interview>,
}

impl ApiResponseTrait for InterviewListResponse {
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

impl ApplicationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建投递
    ///
    /// 该接口用于创建新的投递记录，将人才与职位关联，
    /// 并设置投递的初始阶段和相关信息。创建成功后
    /// 投递将进入招聘流程的第一个阶段。
    ///
    /// # 参数
    ///
    /// - `request`: 投递创建请求参数，包括：
    ///   - `talent_id`: 人才ID（必填）
    ///   - `job_id`: 职位ID（必填）
    ///   - `stage_id`: 阶段ID（必填）
    ///   - `source`: 投递渠道
    ///   - `apply_time`: 投递时间
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回投递创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `application_id`: 创建的投递ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::ApplicationCreateRequest;
    ///
    /// let request = ApplicationCreateRequest {
    ///     talent_id: "talent_123456".to_string(),
    ///     job_id: "job_789".to_string(),
    ///     stage_id: "stage_001".to_string(),
    ///     source: Some("内推".to_string()),
    ///     apply_time: Some("2024-01-15T10:00:00Z".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.application.create_application(request, None).await?;
    /// ```
    pub async fn create_application(
        &self,
        request: ApplicationCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ApplicationOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_APPLICATIONS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取投递详情
    ///
    /// 该接口用于获取指定投递的详细信息，包括投递
    /// 基本信息、当前阶段、人才信息、职位信息等
    /// 完整数据。
    ///
    /// # 参数
    ///
    /// - `application_id`: 投递ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回投递详细信息，包括：
    /// - 投递基本信息（人才、职位、阶段等）
    /// - 投递状态和渠道
    /// - 关联的人才和职位详情
    /// - 投递时间信息
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let application_id = "app_123456";
    /// let response = client.hire.candidate_management.application.get_application_detail(application_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("投递状态: {}", data.application.status);
    ///     println!("当前阶段: {}", data.application.stage_id);
    ///     println!("投递渠道: {:?}", data.application.source);
    /// }
    /// ```
    pub async fn get_application_detail(
        &self,
        application_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ApplicationDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
                api_req.set_api_path(HIRE_V1_APPLICATION_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取投递列表
    ///
    /// 该接口用于获取企业的投递列表，支持按职位、状态、
    /// 阶段、渠道、时间等条件筛选。返回的列表包含投递
    /// 基本信息，可用于投递管理和跟踪。
    ///
    /// # 参数
    ///
    /// - `request`: 投递列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `job_id`: 职位ID筛选
    ///   - `status`: 投递状态筛选
    ///   - `stage_id`: 阶段ID筛选
    ///   - `source`: 投递渠道筛选
    ///   - `created_start_time`: 创建时间开始
    ///   - `created_end_time`: 创建时间结束
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的投递列表，包括：
    /// - 投递基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::ApplicationListRequest;
    ///
    /// let request = ApplicationListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     job_id: Some("job_789".to_string()),
    ///     status: Some("active".to_string()),
    ///     stage_id: Some("stage_001".to_string()),
    ///     source: Some("内推".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.candidate_management.application.list_applications(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("投递总数: {}", data.applications.items.len());
    ///     for application in &data.applications.items {
    ///         println!("投递: {} 状态: {}", application.id, application.status);
    ///     }
    /// }
    /// ```
    pub async fn list_applications(
        &self,
        request: ApplicationListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ApplicationListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_APPLICATIONS.to_string());
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

        if let Some(job_id) = request.job_id {
            api_req.query_params.insert("job_id", job_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(stage_id) = request.stage_id {
            api_req.query_params.insert("stage_id", stage_id);
        }

        if let Some(source) = request.source {
            api_req.query_params.insert("source", source);
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

    /// 推进投递到下一阶段
    ///
    /// 该接口用于将投递推进到指定的下一个阶段，
    /// 支持跨阶段推进和添加推进原因。
    ///
    /// # 参数
    ///
    /// - `request`: 投递推进请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::application::ApplicationAdvanceRequest;
    ///
    /// let request = ApplicationAdvanceRequest {
    ///     application_id: "app_123456".to_string(),
    ///     target_stage_id: "stage_002".to_string(),
    ///     reason: Some("简历筛选通过".to_string()),
    ///     remark: Some("候选人技能匹配度高".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.application.advance_application(request, None).await?;
    /// ```
    pub async fn advance_application(
        &self,
        request: ApplicationAdvanceRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ApplicationOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_V1_APPLICATION_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 淘汰投递
    ///
    /// 该接口用于淘汰指定的投递，设置淘汰原因
    /// 和相关备注信息。
    ///
    /// # 参数
    ///
    /// - `application_id`: 投递ID
    /// - `reason`: 淘汰原因
    /// - `remark`: 备注信息（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let application_id = "app_123456";
    /// let reason = "技能不匹配";
    /// let remark = Some("缺乏相关项目经验".to_string());
    ///
    /// let response = client.hire.candidate_management.application.reject_application(
    ///     application_id,
    ///     reason,
    ///     remark,
    ///     None
    /// ).await?;
    /// ```
    pub async fn reject_application(
        &self,
        application_id: &str,
        reason: &str,
        remark: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ApplicationOperationResponse>> {
        #[derive(Serialize)]
        struct RejectRequest {
            reason: String,
            remark: Option<String>,
        }

        let request = RejectRequest {
            reason: reason.to_string(),
            remark,
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_V1_APPLICATION_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取投递的面试列表
    ///
    /// 该接口用于获取指定投递的面试记录列表，
    /// 包括面试安排、状态、结果等信息。
    ///
    /// # 参数
    ///
    /// - `application_id`: 投递ID
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的面试列表，包括：
    /// - 面试记录列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let application_id = "app_123456";
    /// let response = client.hire.candidate_management.application.get_application_interviews(
    ///     application_id,
    ///     Some(20),
    ///     None,
    ///     None
    /// ).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("面试轮次数: {}", data.interviews.items.len());
    ///     for interview in &data.interviews.items {
    ///         println!("面试轮次: {} 状态: {}", interview.round, interview.status);
    ///     }
    /// }
    /// ```
    pub async fn get_application_interviews(
        &self,
        application_id: &str,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_APPLICATION_INTERVIEWS,
            "application_id",
            application_id,
        ));
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
    /// 创建Offer
    ///
    /// 该接口用于为通过面试的投递创建Offer，
    /// 设置薪资结构、入职时间等相关信息。
    ///
    /// # 参数
    ///
    /// - `request`: Offer创建请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::OfferCreateRequest;
    ///
    /// let request = OfferCreateRequest {
    ///     application_id: "app_123456".to_string(),
    ///     job_level: Some("P6".to_string()),
    ///     basic_salary: Some("25000".to_string()),
    ///     performance_bonus: Some("3-6个月".to_string()),
    ///     onboard_date: Some("2024-03-01".to_string()),
    ///     expire_time: Some("2024-02-15T23:59:59Z".to_string()),
    ///     remark: Some("优秀候选人，建议尽快入职".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.candidate_management.application.create_offer(request, None).await?;
    /// ```
    pub async fn create_offer(
        &self,
        request: OfferCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ApplicationOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_OFFERS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 获取投递的Offer信息
    ///
    /// 该接口用于获取指定投递的Offer详细信息，
    /// 包括薪资结构、有效期、签约状态等数据。
    ///
    /// # 参数
    ///
    /// - `application_id`: 投递ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回Offer详细信息，包括：
    /// - Offer基本信息（薪资、入职时间等）
    /// - Offer状态和有效期
    /// - 自定义字段数据
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let application_id = "app_123456";
    /// let response = client.hire.candidate_management.application.get_application_offer(application_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("Offer状态: {}", data.offer.status);
    ///     println!("基本薪资: {:?}", data.offer.basic_salary);
    ///     println!("入职时间: {:?}", data.offer.onboard_date);
    /// }
    /// ```
    pub async fn get_application_offer(
        &self,
        application_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OfferDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
                api_req.set_api_path(HIRE_V1_APPLICATION_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 添加投递评价
    ///
    /// 该接口用于为投递添加评价信息，包括评价内容、
    /// 评分等，用于记录面试官或HR的评价意见。
    ///
    /// # 参数
    ///
    /// - `request`: 投递评价请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::application::ApplicationEvaluationRequest;
    ///
    /// let request = ApplicationEvaluationRequest {
    ///     application_id: "app_123456".to_string(),
    ///     evaluation: "候选人技术能力强，沟通能力良好，推荐录用".to_string(),
    ///     score: Some(4.5),
    ///     evaluator_id: "user_789".to_string(),
    ///     evaluation_time: Some("2024-01-20T15:30:00Z".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.application.add_application_evaluation(request, None).await?;
    /// ```
    pub async fn add_application_evaluation(
        &self,
        request: ApplicationEvaluationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ApplicationOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_V1_APPLICATION_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
}
