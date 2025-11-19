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

/// 笔试服务
pub struct ExamService {
    pub config: Config,
}

/// 笔试试卷信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamPaper {
    /// 试卷ID
    pub id: String,
    /// 试卷名称
    pub title: I18nText,
    /// 试卷描述
    pub description: Option<I18nText>,
    /// 试卷类型
    pub paper_type: String,
    /// 题目数量
    pub question_count: u32,
    /// 考试时长（分钟）
    pub duration_minutes: u32,
    /// 及格分数
    pub pass_score: f32,
    /// 满分
    pub total_score: f32,
    /// 难度等级
    pub difficulty_level: String,
    /// 适用职位
    pub suitable_positions: Vec<String>,
    /// 技能标签
    pub skill_tags: Vec<String>,
    /// 是否启用
    pub enabled: bool,
    /// 创建人
    pub creator_id: String,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 笔试记录信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamRecord {
    /// 笔试记录ID
    pub id: String,
    /// 投递ID
    pub application_id: String,
    /// 候选人ID
    pub talent_id: String,
    /// 试卷ID
    pub paper_id: String,
    /// 笔试状态
    pub status: String,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 实际用时（分钟）
    pub actual_duration: Option<u32>,
    /// 得分
    pub score: Option<f32>,
    /// 是否通过
    pub passed: Option<bool>,
    /// 答题详情
    pub answer_details: Vec<ExamAnswerDetail>,
    /// 监考记录
    pub proctoring_records: Vec<ExamProctoringRecord>,
    /// 备注信息
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 答题详情
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamAnswerDetail {
    /// 题目ID
    pub question_id: String,
    /// 题目类型
    pub question_type: String,
    /// 候选人答案
    pub candidate_answer: String,
    /// 正确答案
    pub correct_answer: Option<String>,
    /// 是否正确
    pub is_correct: Option<bool>,
    /// 得分
    pub score: Option<f32>,
    /// 答题时间
    pub answer_time: Option<String>,
}

/// 监考记录
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamProctoringRecord {
    /// 记录ID
    pub id: String,
    /// 记录类型
    pub record_type: String,
    /// 记录时间
    pub record_time: String,
    /// 记录详情
    pub details: Option<String>,
    /// 风险级别
    pub risk_level: Option<String>,
}

/// 笔试题目信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamQuestion {
    /// 题目ID
    pub id: String,
    /// 题目类型
    pub question_type: String,
    /// 题目内容
    pub content: I18nText,
    /// 选项列表（选择题）
    pub options: Option<Vec<ExamOption>>,
    /// 正确答案
    pub correct_answer: String,
    /// 题目分数
    pub score: f32,
    /// 难度等级
    pub difficulty: String,
    /// 技能标签
    pub skill_tags: Vec<String>,
    /// 解析
    pub explanation: Option<I18nText>,
}

/// 考试选项
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamOption {
    /// 选项标识
    pub key: String,
    /// 选项内容
    pub content: I18nText,
}

/// 笔试安排请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExamArrangementRequest {
    /// 投递ID
    pub application_id: String,
    /// 试卷ID
    pub paper_id: String,
    /// 考试开始时间
    pub start_time: String,
    /// 考试结束时间
    pub end_time: String,
    /// 是否开启监考
    pub enable_proctoring: Option<bool>,
    /// 允许重考次数
    pub retry_count: Option<u32>,
    /// 通知候选人
    pub notify_candidate: Option<bool>,
    /// 备注信息
    pub remark: Option<String>,
}

/// 笔试提交请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExamSubmissionRequest {
    /// 笔试记录ID
    pub exam_record_id: String,
    /// 答案列表
    pub answers: Vec<ExamAnswerSubmission>,
    /// 提交时间
    pub submit_time: String,
}

/// 答案提交
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExamAnswerSubmission {
    /// 题目ID
    pub question_id: String,
    /// 候选人答案
    pub answer: String,
}

/// 笔试试卷列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExamPaperListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 试卷类型
    pub paper_type: Option<String>,
    /// 难度等级
    pub difficulty_level: Option<String>,
    /// 技能标签
    pub skill_tag: Option<String>,
    /// 启用状态
    pub enabled: Option<bool>,
}

/// 笔试记录列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExamRecordListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 候选人ID
    pub talent_id: Option<String>,
    /// 试卷ID
    pub paper_id: Option<String>,
    /// 笔试状态
    pub status: Option<String>,
    /// 开始时间筛选
    pub start_time_from: Option<String>,
    /// 结束时间筛选
    pub start_time_to: Option<String>,
}

/// 笔试试卷列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamPaperListResponse {
    /// 试卷列表
    #[serde(flatten)]
    pub papers: PageResponse<ExamPaper>,
}

impl ApiResponseTrait for ExamPaperListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 笔试记录列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamRecordListResponse {
    /// 笔试记录列表
    #[serde(flatten)]
    pub records: PageResponse<ExamRecord>,
}

impl ApiResponseTrait for ExamRecordListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 笔试记录详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamRecordDetailResponse {
    /// 笔试记录信息
    pub record: ExamRecord,
}

impl ApiResponseTrait for ExamRecordDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 笔试操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ExamOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 相关ID
    pub id: Option<String>,
}

impl ApiResponseTrait for ExamOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ExamService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取笔试试卷列表
    ///
    /// 该接口用于获取可用的笔试试卷列表，支持按类型、
    /// 难度、技能标签等条件筛选。返回的列表包含试卷
    /// 基本信息，可用于选择合适的笔试试卷。
    ///
    /// # 参数
    ///
    /// - `request`: 试卷列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `paper_type`: 试卷类型筛选
    ///   - `difficulty_level`: 难度等级筛选
    ///   - `skill_tag`: 技能标签筛选
    ///   - `enabled`: 启用状态筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的试卷列表，包括：
    /// - 试卷基本信息列表
    /// - 题目数量和考试时长
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::ecological_docking::exam::ExamPaperListRequest;
    ///
    /// let request = ExamPaperListRequest {
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     paper_type: Some("technical".to_string()),
    ///     difficulty_level: Some("intermediate".to_string()),
    ///     skill_tag: Some("Java".to_string()),
    ///     enabled: Some(true),
    /// };
    ///
    /// let response = client.hire.ecological_docking.exam.list_papers(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("试卷总数: {}", data.papers.items.len());
    ///     for paper in &data.papers.items {
    ///         println!("试卷: {:?} 题目数: {}", paper.title.zh_cn, paper.question_count);
    ///     }
    /// }
    /// ```
    pub async fn list_papers(
        &self,
        request: ExamPaperListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExamPaperListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_EXAM_PAPERS.to_string());
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

        if let Some(paper_type) = request.paper_type {
            api_req.query_params.insert("paper_type", paper_type);
        }

        if let Some(difficulty_level) = request.difficulty_level {
            api_req
                .query_params
                .insert("difficulty_level", difficulty_level);
        }

        if let Some(skill_tag) = request.skill_tag {
            api_req.query_params.insert("skill_tag", skill_tag);
        }

        if let Some(enabled) = request.enabled {
            api_req.query_params.insert("enabled", enabled.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 安排笔试
    ///
    /// 该接口用于为候选人安排笔试，选择试卷并设置
    /// 考试时间、监考选项等配置。安排成功后将发送
    /// 通知给候选人。
    ///
    /// # 参数
    ///
    /// - `request`: 笔试安排请求参数，包括：
    ///   - `application_id`: 投递ID（必填）
    ///   - `paper_id`: 试卷ID（必填）
    ///   - `start_time`: 考试开始时间（必填）
    ///   - `end_time`: 考试结束时间（必填）
    ///   - `enable_proctoring`: 是否开启监考
    ///   - `retry_count`: 允许重考次数
    ///   - `notify_candidate`: 通知候选人
    ///   - `remark`: 备注信息
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回笔试安排操作结果，包括：
    /// - `success`: 安排是否成功
    /// - `id`: 创建的笔试记录ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::ecological_docking::exam::ExamArrangementRequest;
    ///
    /// let request = ExamArrangementRequest {
    ///     application_id: "app_123456".to_string(),
    ///     paper_id: "paper_789".to_string(),
    ///     start_time: "2024-01-15T14:00:00Z".to_string(),
    ///     end_time: "2024-01-15T16:00:00Z".to_string(),
    ///     enable_proctoring: Some(true),
    ///     retry_count: Some(1),
    ///     notify_candidate: Some(true),
    ///     remark: Some("技术笔试".to_string()),
    /// };
    ///
    /// let response = client.hire.ecological_docking.exam.arrange_exam(request, None).await?;
    /// ```
    pub async fn arrange_exam(
        &self,
        request: ExamArrangementRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExamOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_EXAM_ARRANGEMENTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取笔试记录详情
    ///
    /// 该接口用于获取指定笔试记录的详细信息，包括
    /// 考试状态、得分、答题详情、监考记录等完整数据。
    ///
    /// # 参数
    ///
    /// - `record_id`: 笔试记录ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回笔试记录详细信息，包括：
    /// - 考试基本信息（状态、时间、得分等）
    /// - 答题详情和正确率
    /// - 监考记录和风险信息
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let record_id = "record_123456";
    /// let response = client.hire.ecological_docking.exam.get_record_detail(record_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("考试状态: {}", data.record.status);
    ///     println!("考试得分: {:?}", data.record.score);
    ///     println!("是否通过: {:?}", data.record.passed);
    /// }
    /// ```
    pub async fn get_record_detail(
        &self,
        record_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExamRecordDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_EXAM_RECORD_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取笔试记录列表
    ///
    /// 该接口用于获取企业的笔试记录列表，支持按候选人、
    /// 试卷、状态、时间等条件筛选。返回的列表包含笔试
    /// 基本信息，可用于笔试管理和统计。
    ///
    /// # 参数
    ///
    /// - `request`: 笔试记录列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `talent_id`: 候选人ID筛选
    ///   - `paper_id`: 试卷ID筛选
    ///   - `status`: 笔试状态筛选
    ///   - `start_time_from`: 开始时间筛选
    ///   - `start_time_to`: 结束时间筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的笔试记录列表，包括：
    /// - 笔试记录基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::ecological_docking::exam::ExamRecordListRequest;
    ///
    /// let request = ExamRecordListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     talent_id: Some("talent_789".to_string()),
    ///     status: Some("completed".to_string()),
    ///     start_time_from: Some("2024-01-01T00:00:00Z".to_string()),
    ///     start_time_to: Some("2024-01-31T23:59:59Z".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.ecological_docking.exam.list_records(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("笔试记录总数: {}", data.records.items.len());
    ///     for record in &data.records.items {
    ///         println!("笔试: {} 状态: {}", record.id, record.status);
    ///     }
    /// }
    /// ```
    pub async fn list_records(
        &self,
        request: ExamRecordListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExamRecordListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_EXAM_RECORDS.to_string());
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

        if let Some(paper_id) = request.paper_id {
            api_req.query_params.insert("paper_id", paper_id);
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

    /// 提交笔试答案
    ///
    /// 该接口用于候选人提交笔试答案，完成考试并
    /// 触发自动评分流程。
    ///
    /// # 参数
    ///
    /// - `request`: 笔试提交请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::ecological_docking::exam::{
    ///     ExamSubmissionRequest, ExamAnswerSubmission
    /// };
    ///
    /// let request = ExamSubmissionRequest {
    ///     exam_record_id: "record_123456".to_string(),
    ///     answers: vec![
    ///         ExamAnswerSubmission {
    ///             question_id: "q1".to_string(),
    ///             answer: "A".to_string(),
    ///         },
    ///         ExamAnswerSubmission {
    ///             question_id: "q2".to_string(),
    ///             answer: "public class HelloWorld { }".to_string(),
    ///         },
    ///     ],
    ///     submit_time: "2024-01-15T15:30:00Z".to_string(),
    /// };
    ///
    /// let response = client.hire.ecological_docking.exam.submit_exam(request, None).await?;
    /// ```
    pub async fn submit_exam(
        &self,
        request: ExamSubmissionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExamOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_EXAM_SUBMISSIONS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 取消笔试
    ///
    /// 该接口用于取消已安排的笔试，设置取消原因
    /// 并发送通知。
    ///
    /// # 参数
    ///
    /// - `record_id`: 笔试记录ID
    /// - `reason`: 取消原因
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let record_id = "record_123456";
    /// let reason = "候选人时间冲突，需要重新安排";
    ///
    /// let response = client.hire.ecological_docking.exam.cancel_exam(record_id, reason, None).await?;
    /// ```
    pub async fn cancel_exam(
        &self,
        record_id: &str,
        reason: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExamOperationResponse>> {
        #[derive(Serialize)]
        struct CancelExamRequest {
            reason: String,
        }

        let request = CancelExamRequest {
            reason: reason.to_string(),
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_EXAM_RECORD_CANCEL.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 重新安排笔试
    ///
    /// 该接口用于重新安排笔试时间，为候选人
    /// 提供新的考试时间窗口。
    ///
    /// # 参数
    ///
    /// - `record_id`: 笔试记录ID
    /// - `new_start_time`: 新的开始时间
    /// - `new_end_time`: 新的结束时间
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let record_id = "record_123456";
    /// let new_start_time = "2024-01-16T14:00:00Z";
    /// let new_end_time = "2024-01-16T16:00:00Z";
    ///
    /// let response = client.hire.ecological_docking.exam.reschedule_exam(
    ///     record_id,
    ///     new_start_time,
    ///     new_end_time,
    ///     None
    /// ).await?;
    /// ```
    pub async fn reschedule_exam(
        &self,
        record_id: &str,
        new_start_time: &str,
        new_end_time: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExamOperationResponse>> {
        #[derive(Serialize)]
        struct RescheduleExamRequest {
            new_start_time: String,
            new_end_time: String,
        }

        let request = RescheduleExamRequest {
            new_start_time: new_start_time.to_string(),
            new_end_time: new_end_time.to_string(),
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_EXAM_RECORD_RESCHEDULE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取笔试统计数据
    ///
    /// 该接口用于获取笔试相关的统计数据，包括
    /// 通过率、平均分、参考人数等关键指标。
    ///
    /// # 参数
    ///
    /// - `paper_id`: 试卷ID（可选）
    /// - `start_date`: 统计开始日期（可选）
    /// - `end_date`: 统计结束日期（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let paper_id = Some("paper_789".to_string());
    /// let start_date = Some("2024-01-01".to_string());
    /// let end_date = Some("2024-01-31".to_string());
    ///
    /// let response = client.hire.ecological_docking.exam.get_exam_statistics(
    ///     paper_id,
    ///     start_date,
    ///     end_date,
    ///     None
    /// ).await?;
    /// ```
    pub async fn get_exam_statistics(
        &self,
        paper_id: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<serde_json::Value>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_EXAM_STATISTICS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(paper_id) = paper_id {
            api_req.query_params.insert("paper_id", paper_id);
        }

        if let Some(start_date) = start_date {
            api_req.query_params.insert("start_date", start_date);
        }

        if let Some(end_date) = end_date {
            api_req.query_params.insert("end_date", end_date);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
