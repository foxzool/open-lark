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
use crate::hire::models::{CommonResponse, I18nText, PageResponse, UserId};

/// 面试服务
pub struct InterviewService {
    pub config: Config,
}

/// 面试记录信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Interview {
    /// 面试ID
    pub id: String,
    /// 投递ID
    pub application_id: String,
    /// 面试轮次ID
    pub interview_round_id: String,
    /// 面试轮次名称
    pub round_name: I18nText,
    /// 面试类型
    pub interview_type: String,
    /// 面试状态
    pub status: String,
    /// 面试开始时间
    pub start_time: String,
    /// 面试结束时间
    pub end_time: Option<String>,
    /// 面试地点
    pub location: Option<I18nText>,
    /// 面试官列表
    pub interviewers: Vec<UserId>,
    /// 候选人ID
    pub talent_id: String,
    /// 面试备注
    pub remark: Option<String>,
    /// 面试反馈
    pub feedback: Option<String>,
    /// 面试评分
    pub score: Option<f32>,
    /// 面试结果
    pub result: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 面试评估信息
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewEvaluation {
    /// 评估ID
    pub id: String,
    /// 面试ID
    pub interview_id: String,
    /// 评估人ID
    pub evaluator_id: String,
    /// 评估维度
    pub dimension: String,
    /// 评估分数
    pub score: f32,
    /// 评估评语
    pub comment: Option<String>,
    /// 评估时间
    pub evaluation_time: String,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 面试安排信息
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewArrangement {
    /// 安排ID
    pub id: String,
    /// 投递ID
    pub application_id: String,
    /// 面试轮次ID
    pub interview_round_id: String,
    /// 计划面试时间
    pub scheduled_time: String,
    /// 面试时长（分钟）
    pub duration: u32,
    /// 面试地点
    pub location: Option<I18nText>,
    /// 面试官列表
    pub interviewers: Vec<UserId>,
    /// 安排状态
    pub status: String,
    /// 通知状态
    pub notification_status: Option<String>,
    /// 安排备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 面试创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterviewCreateRequest {
    /// 投递ID
    pub application_id: String,
    /// 面试轮次ID
    pub interview_round_id: String,
    /// 面试类型
    pub interview_type: String,
    /// 面试开始时间
    pub start_time: String,
    /// 面试结束时间
    pub end_time: Option<String>,
    /// 面试地点
    pub location: Option<I18nText>,
    /// 面试官列表
    pub interviewers: Vec<String>,
    /// 面试备注
    pub remark: Option<String>,
}

/// 面试安排请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterviewArrangementRequest {
    /// 投递ID
    pub application_id: String,
    /// 面试轮次ID
    pub interview_round_id: String,
    /// 计划面试时间
    pub scheduled_time: String,
    /// 面试时长（分钟）
    pub duration: u32,
    /// 面试地点
    pub location: Option<I18nText>,
    /// 面试官列表
    pub interviewers: Vec<String>,
    /// 安排备注
    pub remark: Option<String>,
}

/// 面试评估请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterviewEvaluationRequest {
    /// 面试ID
    pub interview_id: String,
    /// 评估维度列表
    pub evaluations: Vec<EvaluationDimension>,
    /// 整体反馈
    pub overall_feedback: Option<String>,
    /// 整体评分
    pub overall_score: Option<f32>,
    /// 面试结果
    pub result: Option<String>,
}

/// 评估维度
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EvaluationDimension {
    /// 评估维度
    pub dimension: String,
    /// 评估分数
    pub score: f32,
    /// 评估评语
    pub comment: Option<String>,
}

/// 面试列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterviewListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 投递ID
    pub application_id: Option<String>,
    /// 面试官ID
    pub interviewer_id: Option<String>,
    /// 面试状态
    pub status: Option<String>,
    /// 开始时间筛选
    pub start_time_from: Option<String>,
    /// 结束时间筛选
    pub start_time_to: Option<String>,
}

/// 面试列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewListResponse {
    /// 面试记录列表
    #[serde(flatten)]
    pub interviews: PageResponse<Interview>,
}

impl ApiResponseTrait for InterviewListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 面试详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewDetailResponse {
    /// 面试记录信息
    pub interview: Interview,
}

impl ApiResponseTrait for InterviewDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 面试评估列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewEvaluationListResponse {
    /// 面试评估列表
    #[serde(flatten)]
    pub evaluations: PageResponse<InterviewEvaluation>,
}

impl ApiResponseTrait for InterviewEvaluationListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 面试操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 相关ID
    pub id: Option<String>,
}

impl ApiResponseTrait for InterviewOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl InterviewService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建面试记录
    ///
    /// 该接口用于创建新的面试记录，安排候选人参加指定
    /// 轮次的面试。创建成功后可以设置面试官、时间地点等信息。
    ///
    /// # 参数
    ///
    /// - `request`: 面试创建请求参数，包括：
    ///   - `application_id`: 投递ID（必填）
    ///   - `interview_round_id`: 面试轮次ID（必填）
    ///   - `interview_type`: 面试类型（必填）
    ///   - `start_time`: 面试开始时间（必填）
    ///   - `end_time`: 面试结束时间
    ///   - `location`: 面试地点
    ///   - `interviewers`: 面试官列表
    ///   - `remark`: 面试备注
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回面试创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `id`: 创建的面试ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::interview::InterviewCreateRequest;
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let request = InterviewCreateRequest {
    ///     application_id: "app_123456".to_string(),
    ///     interview_round_id: "round_789".to_string(),
    ///     interview_type: "technical".to_string(),
    ///     start_time: "2024-01-15T14:00:00Z".to_string(),
    ///     end_time: Some("2024-01-15T15:30:00Z".to_string()),
    ///     location: Some(I18nText {
    ///         zh_cn: Some("会议室A".to_string()),
    ///         en_us: Some("Meeting Room A".to_string()),
    ///         ja_jp: None,
    ///     }),
    ///     interviewers: vec!["interviewer_001".to_string(), "interviewer_002".to_string()],
    ///     remark: Some("技术面试，重点考察算法和系统设计".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.interview.create_interview(request, None).await?;
    /// ```
    pub async fn create_interview(
        &self,
        request: InterviewCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_INTERVIEWS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取面试记录详情
    ///
    /// 该接口用于获取指定面试记录的详细信息，包括
    /// 面试基本信息、面试官、评估结果等完整数据。
    ///
    /// # 参数
    ///
    /// - `interview_id`: 面试记录ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回面试记录详细信息，包括：
    /// - 面试基本信息（时间、地点、类型等）
    /// - 面试官信息列表
    /// - 面试状态和结果
    /// - 评分和反馈信息
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let interview_id = "interview_123456";
    /// let response = client.hire.candidate_management.interview.get_interview_detail(interview_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("面试状态: {}", data.interview.status);
    ///     println!("面试评分: {:?}", data.interview.score);
    ///     println!("面试反馈: {:?}", data.interview.feedback);
    /// }
    /// ```
    pub async fn get_interview_detail(
        &self,
        interview_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
                api_req.set_api_path(HIRE_V1_INTERVIEW_CANCEL.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取面试记录列表
    ///
    /// 该接口用于获取企业的面试记录列表，支持按投递、
    /// 面试官、状态、时间等条件筛选。返回的列表包含
    /// 面试基本信息，可用于面试管理和统计。
    ///
    /// # 参数
    ///
    /// - `request`: 面试列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `application_id`: 投递ID筛选
    ///   - `interviewer_id`: 面试官ID筛选
    ///   - `status`: 面试状态筛选
    ///   - `start_time_from`: 开始时间筛选
    ///   - `start_time_to`: 结束时间筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的面试记录列表，包括：
    /// - 面试记录基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::interview::InterviewListRequest;
    ///
    /// let request = InterviewListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     application_id: Some("app_123456".to_string()),
    ///     interviewer_id: Some("interviewer_001".to_string()),
    ///     status: Some("scheduled".to_string()),
    ///     start_time_from: Some("2024-01-01T00:00:00Z".to_string()),
    ///     start_time_to: Some("2024-01-31T23:59:59Z".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.interview.list_interviews(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("面试记录总数: {}", data.interviews.items.len());
    ///     for interview in &data.interviews.items {
    ///         println!("面试: {} 状态: {}", interview.id, interview.status);
    ///     }
    /// }
    /// ```
    pub async fn list_interviews(
        &self,
        request: InterviewListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_INTERVIEWS.to_string());
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

        if let Some(application_id) = request.application_id {
            api_req
                .query_params
                .insert("application_id", application_id);
        }

        if let Some(interviewer_id) = request.interviewer_id {
            api_req
                .query_params
                .insert("interviewer_id", interviewer_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(start_time_from) = request.start_time_from {
            api_req
                .query_params
                .insert("start_time_from", start_time_from);
        }

        if let Some(start_time_to) = request.start_time_to {
            api_req.query_params.insert("start_time_to", start_time_to);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 安排面试
    ///
    /// 该接口用于为候选人安排面试时间和面试官，
    /// 发送面试通知并创建面试安排记录。
    ///
    /// # 参数
    ///
    /// - `request`: 面试安排请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::interview::InterviewArrangementRequest;
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let request = InterviewArrangementRequest {
    ///     application_id: "app_123456".to_string(),
    ///     interview_round_id: "round_789".to_string(),
    ///     scheduled_time: "2024-01-15T14:00:00Z".to_string(),
    ///     duration: 90,
    ///     location: Some(I18nText {
    ///         zh_cn: Some("北京总部 8楼会议室".to_string()),
    ///         en_us: Some("Beijing HQ 8F Meeting Room".to_string()),
    ///         ja_jp: None,
    ///     }),
    ///     interviewers: vec!["interviewer_001".to_string(), "interviewer_002".to_string()],
    ///     remark: Some("请候选人提前15分钟到达".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.interview.arrange_interview(request, None).await?;
    /// ```
    pub async fn arrange_interview(
        &self,
        request: InterviewArrangementRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_INTERVIEW_ARRANGEMENTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 提交面试评估
    ///
    /// 该接口用于面试官提交面试评估，包括各维度
    /// 评分、整体评价和面试结果建议。
    ///
    /// # 参数
    ///
    /// - `request`: 面试评估请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::interview::{
    ///     InterviewEvaluationRequest, EvaluationDimension
    /// };
    ///
    /// let request = InterviewEvaluationRequest {
    ///     interview_id: "interview_123456".to_string(),
    ///     evaluations: vec![
    ///         EvaluationDimension {
    ///             dimension: "技术能力".to_string(),
    ///             score: 4.5,
    ///             comment: Some("算法基础扎实，系统设计思路清晰".to_string()),
    ///         },
    ///         EvaluationDimension {
    ///             dimension: "沟通表达".to_string(),
    ///             score: 4.0,
    ///             comment: Some("表达清晰，逻辑性强".to_string()),
    ///         },
    ///     ],
    ///     overall_feedback: Some("候选人技术基础很好，建议通过".to_string()),
    ///     overall_score: Some(4.3),
    ///     result: Some("pass".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.interview.submit_interview_evaluation(request, None).await?;
    /// ```
    pub async fn submit_interview_evaluation(
        &self,
        request: InterviewEvaluationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_INTERVIEW_EVALUATIONS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 获取面试评估列表
    ///
    /// 该接口用于获取指定面试的评估记录列表，
    /// 包括各面试官的评分和反馈信息。
    ///
    /// # 参数
    ///
    /// - `interview_id`: 面试ID
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let interview_id = "interview_123456";
    /// let response = client.hire.candidate_management.interview.list_interview_evaluations(
    ///     interview_id,
    ///     Some(20),
    ///     None,
    ///     None
    /// ).await?;
    /// ```
    pub async fn list_interview_evaluations(
        &self,
        interview_id: &str,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewEvaluationListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_INTERVIEW_EVALUATIONS,
            "interview_id",
            interview_id,
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
    /// 取消面试
    ///
    /// 该接口用于取消已安排的面试，设置取消原因
    /// 并发送通知。
    ///
    /// # 参数
    ///
    /// - `interview_id`: 面试ID
    /// - `reason`: 取消原因
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let interview_id = "interview_123456";
    /// let reason = "候选人临时有事，需要重新安排时间";
    ///
    /// let response = client.hire.candidate_management.interview.cancel_interview(
    ///     interview_id,
    ///     reason,
    ///     None
    /// ).await?;
    /// ```
    pub async fn cancel_interview(
        &self,
        interview_id: &str,
        reason: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewOperationResponse>> {
        #[derive(Serialize)]
        struct CancelRequest {
            reason: String,
        }

        let request = CancelRequest {
            reason: reason.to_string(),
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_V1_INTERVIEW_CANCEL.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 重新安排面试
    ///
    /// 该接口用于重新安排面试时间或面试官，
    /// 发送更新通知。
    ///
    /// # 参数
    ///
    /// - `interview_id`: 面试ID
    /// - `new_time`: 新的面试时间
    /// - `new_interviewers`: 新的面试官列表（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let interview_id = "interview_123456";
    /// let new_time = "2024-01-16T10:00:00Z";
    /// let new_interviewers = Some(vec!["interviewer_003".to_string()]);
    ///
    /// let response = client.hire.candidate_management.interview.reschedule_interview(
    ///     interview_id,
    ///     new_time,
    ///     new_interviewers,
    ///     None
    /// ).await?;
    /// ```
    pub async fn reschedule_interview(
        &self,
        interview_id: &str,
        new_time: &str,
        new_interviewers: Option<Vec<String>>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewOperationResponse>> {
        #[derive(Serialize)]
        struct RescheduleRequest {
            new_time: String,
            new_interviewers: Option<Vec<String>>,
        }

        let request = RescheduleRequest {
            new_time: new_time.to_string(),
            new_interviewers,
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_V1_INTERVIEW_CANCEL.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
}
