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

use crate::hire::models::{CommonResponse, PageResponse, Talent};

/// 猎头服务
pub struct AgencyService {
    pub config: Config,
}

/// 猎头机构信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Agency {
    /// 猎头机构ID
    pub id: String,
    /// 机构名称
    pub name: String,
    /// 机构描述
    pub description: Option<String>,
    /// 联系人姓名
    pub contact_name: String,
    /// 联系邮箱
    pub contact_email: String,
    /// 联系电话
    pub contact_phone: Option<String>,
    /// 机构地址
    pub address: Option<String>,
    /// 机构网站
    pub website: Option<String>,
    /// 机构状态
    pub status: String,
    /// 合作模式
    pub cooperation_mode: String,
    /// 费率信息
    pub fee_rate: Option<String>,
    /// 专业领域
    pub specialties: Vec<String>,
    /// 服务区域
    pub service_areas: Vec<String>,
    /// 合作开始时间
    pub cooperation_start_time: Option<String>,
    /// 合作结束时间
    pub cooperation_end_time: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 猎头推荐记录
#[derive(Debug, Serialize, Deserialize)]
pub struct AgencyRecommendation {
    /// 推荐记录ID
    pub id: String,
    /// 猎头机构ID
    pub agency_id: String,
    /// 猎头顾问ID
    pub consultant_id: Option<String>,
    /// 人才ID
    pub talent_id: String,
    /// 职位ID
    pub job_id: String,
    /// 推荐状态
    pub status: String,
    /// 推荐时间
    pub recommendation_time: String,
    /// 推荐理由
    pub recommendation_reason: Option<String>,
    /// 费用信息
    pub fee_info: Option<AgencyFeeInfo>,
    /// 猎头机构信息
    pub agency: Option<Agency>,
    /// 人才信息
    pub talent: Option<Talent>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 猎头费用信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AgencyFeeInfo {
    /// 费用类型
    pub fee_type: String,
    /// 费用金额
    pub fee_amount: Option<String>,
    /// 费率
    pub fee_rate: Option<f32>,
    /// 币种
    pub currency: String,
    /// 付款条件
    pub payment_terms: Option<String>,
    /// 发票要求
    pub invoice_requirements: Option<String>,
}

/// 猎头顾问信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AgencyConsultant {
    /// 顾问ID
    pub id: String,
    /// 猎头机构ID
    pub agency_id: String,
    /// 顾问姓名
    pub name: String,
    /// 顾问邮箱
    pub email: String,
    /// 顾问电话
    pub phone: Option<String>,
    /// 职位
    pub position: Option<String>,
    /// 专业领域
    pub specialties: Vec<String>,
    /// 工作年限
    pub experience_years: Option<u32>,
    /// 顾问状态
    pub status: String,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 猎头机构创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AgencyCreateRequest {
    /// 机构名称
    pub name: String,
    /// 机构描述
    pub description: Option<String>,
    /// 联系人姓名
    pub contact_name: String,
    /// 联系邮箱
    pub contact_email: String,
    /// 联系电话
    pub contact_phone: Option<String>,
    /// 机构地址
    pub address: Option<String>,
    /// 机构网站
    pub website: Option<String>,
    /// 合作模式
    pub cooperation_mode: String,
    /// 费率信息
    pub fee_rate: Option<String>,
    /// 专业领域
    pub specialties: Vec<String>,
    /// 服务区域
    pub service_areas: Vec<String>,
    /// 合作开始时间
    pub cooperation_start_time: Option<String>,
    /// 合作结束时间
    pub cooperation_end_time: Option<String>,
}

/// 猎头推荐创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AgencyRecommendationCreateRequest {
    /// 猎头机构ID
    pub agency_id: String,
    /// 猎头顾问ID
    pub consultant_id: Option<String>,
    /// 人才ID
    pub talent_id: String,
    /// 职位ID
    pub job_id: String,
    /// 推荐理由
    pub recommendation_reason: Option<String>,
    /// 费用信息
    pub fee_info: Option<AgencyFeeInfo>,
}

/// 猎头顾问创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AgencyConsultantCreateRequest {
    /// 猎头机构ID
    pub agency_id: String,
    /// 顾问姓名
    pub name: String,
    /// 顾问邮箱
    pub email: String,
    /// 顾问电话
    pub phone: Option<String>,
    /// 职位
    pub position: Option<String>,
    /// 专业领域
    pub specialties: Vec<String>,
    /// 工作年限
    pub experience_years: Option<u32>,
}

/// 猎头机构列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AgencyListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 机构状态
    pub status: Option<String>,
    /// 合作模式
    pub cooperation_mode: Option<String>,
    /// 专业领域
    pub specialty: Option<String>,
    /// 服务区域
    pub service_area: Option<String>,
}

/// 猎头推荐列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AgencyRecommendationListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 猎头机构ID
    pub agency_id: Option<String>,
    /// 职位ID
    pub job_id: Option<String>,
    /// 推荐状态
    pub status: Option<String>,
    /// 推荐时间开始
    pub recommendation_start_time: Option<String>,
    /// 推荐时间结束
    pub recommendation_end_time: Option<String>,
}

/// 猎头机构列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AgencyListResponse {
    /// 猎头机构列表
    #[serde(flatten)]
    pub agencies: PageResponse<Agency>,
}

impl ApiResponseTrait for AgencyListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 猎头推荐列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AgencyRecommendationListResponse {
    /// 猎头推荐列表
    #[serde(flatten)]
    pub recommendations: PageResponse<AgencyRecommendation>,
}

impl ApiResponseTrait for AgencyRecommendationListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 猎头顾问列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AgencyConsultantListResponse {
    /// 猎头顾问列表
    #[serde(flatten)]
    pub consultants: PageResponse<AgencyConsultant>,
}

impl ApiResponseTrait for AgencyConsultantListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 猎头操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AgencyOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 相关ID
    pub id: Option<String>,
}

impl ApiResponseTrait for AgencyOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AgencyService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建猎头机构
    ///
    /// 该接口用于创建新的猎头机构档案，记录机构基本信息、
    /// 联系方式、合作模式、专业领域等详细信息。创建成功后
    /// 可以建立合作关系并接收人才推荐。
    ///
    /// # 参数
    ///
    /// - `request`: 猎头机构创建请求参数，包括：
    ///   - `name`: 机构名称（必填）
    ///   - `contact_name`: 联系人姓名（必填）
    ///   - `contact_email`: 联系邮箱（必填）
    ///   - `cooperation_mode`: 合作模式（必填）
    ///   - `description`: 机构描述
    ///   - `contact_phone`: 联系电话
    ///   - `address`: 机构地址
    ///   - `website`: 机构网站
    ///   - `fee_rate`: 费率信息
    ///   - `specialties`: 专业领域
    ///   - `service_areas`: 服务区域
    ///   - `cooperation_start_time`: 合作开始时间
    ///   - `cooperation_end_time`: 合作结束时间
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回猎头机构创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `id`: 创建的机构ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::agency::AgencyCreateRequest;
    ///
    /// let request = AgencyCreateRequest {
    ///     name: "优秀猎头公司".to_string(),
    ///     contact_name: "张经理".to_string(),
    ///     contact_email: "zhang@headhunter.com".to_string(),
    ///     contact_phone: Some("13800138000".to_string()),
    ///     cooperation_mode: "percentage".to_string(),
    ///     fee_rate: Some("20%".to_string()),
    ///     specialties: vec!["技术".to_string(), "产品".to_string()],
    ///     service_areas: vec!["北京".to_string(), "上海".to_string()],
    ///     cooperation_start_time: Some("2024-01-01T00:00:00Z".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.get_candidates.agency.create_agency(request, None).await?;
    /// ```
    pub async fn create_agency(
        &self,
        request: AgencyCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AgencyOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_AGENCIES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取猎头机构列表
    ///
    /// 该接口用于获取企业合作的猎头机构列表，支持按状态、
    /// 合作模式、专业领域、服务区域等条件筛选。返回的
    /// 列表包含机构基本信息，可用于机构管理和选择。
    ///
    /// # 参数
    ///
    /// - `request`: 猎头机构列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `status`: 机构状态筛选
    ///   - `cooperation_mode`: 合作模式筛选
    ///   - `specialty`: 专业领域筛选
    ///   - `service_area`: 服务区域筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的猎头机构列表，包括：
    /// - 机构基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::agency::AgencyListRequest;
    ///
    /// let request = AgencyListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    ///     cooperation_mode: Some("percentage".to_string()),
    ///     specialty: Some("技术".to_string()),
    ///     service_area: Some("北京".to_string()),
    /// };
    ///
    /// let response = client.hire.get_candidates.agency.list_agencies(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("猎头机构总数: {}", data.agencies.items.len());
    ///     for agency in &data.agencies.items {
    ///         println!("机构: {} ({})", agency.name, agency.contact_name);
    ///     }
    /// }
    /// ```
    pub async fn list_agencies(
        &self,
        request: AgencyListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AgencyListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_AGENCIES.to_string());
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

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(cooperation_mode) = request.cooperation_mode {
            api_req
                .query_params
                .insert("cooperation_mode", cooperation_mode);
        }

        if let Some(specialty) = request.specialty {
            api_req.query_params.insert("specialty", specialty);
        }

        if let Some(service_area) = request.service_area {
            api_req.query_params.insert("service_area", service_area);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 创建猎头推荐
    ///
    /// 该接口用于创建猎头推荐记录，记录猎头机构推荐的
    /// 人才和职位匹配信息。创建成功后将进入评估流程。
    ///
    /// # 参数
    ///
    /// - `request`: 猎头推荐创建请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::agency::{AgencyRecommendationCreateRequest, AgencyFeeInfo};
    ///
    /// let request = AgencyRecommendationCreateRequest {
    ///     agency_id: "agency_123456".to_string(),
    ///     consultant_id: Some("consultant_789".to_string()),
    ///     talent_id: "talent_abc".to_string(),
    ///     job_id: "job_def".to_string(),
    ///     recommendation_reason: Some("候选人技能完全匹配职位要求，有丰富的项目经验".to_string()),
    ///     fee_info: Some(AgencyFeeInfo {
    ///         fee_type: "percentage".to_string(),
    ///         fee_amount: None,
    ///         fee_rate: Some(20.0),
    ///         currency: "CNY".to_string(),
    ///         payment_terms: Some("入职后30天内支付".to_string()),
    ///         invoice_requirements: Some("需要正规发票".to_string()),
    ///     }),
    /// };
    ///
    /// let response = client.hire.get_candidates.agency.create_recommendation(request, None).await?;
    /// ```
    pub async fn create_recommendation(
        &self,
        request: AgencyRecommendationCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AgencyOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_AGENCY_RECOMMENDATIONS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取猎头推荐列表
    ///
    /// 该接口用于获取猎头推荐记录列表，支持按机构、
    /// 职位、状态、时间等条件筛选。返回的列表包含
    /// 推荐基本信息，可用于推荐管理和跟踪。
    ///
    /// # 参数
    ///
    /// - `request`: 猎头推荐列表查询请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::agency::AgencyRecommendationListRequest;
    ///
    /// let request = AgencyRecommendationListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     agency_id: Some("agency_123456".to_string()),
    ///     job_id: Some("job_def".to_string()),
    ///     status: Some("pending".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.get_candidates.agency.list_recommendations(request, None).await?;
    /// ```
    pub async fn list_recommendations(
        &self,
        request: AgencyRecommendationListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AgencyRecommendationListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_AGENCY_RECOMMENDATIONS.to_string());
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

        if let Some(agency_id) = request.agency_id {
            api_req.query_params.insert("agency_id", agency_id);
        }

        if let Some(job_id) = request.job_id {
            api_req.query_params.insert("job_id", job_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(recommendation_start_time) = request.recommendation_start_time {
            api_req
                .query_params
                .insert("recommendation_start_time", recommendation_start_time);
        }

        if let Some(recommendation_end_time) = request.recommendation_end_time {
            api_req
                .query_params
                .insert("recommendation_end_time", recommendation_end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 添加猎头顾问
    ///
    /// 该接口用于为猎头机构添加顾问信息，记录顾问
    /// 的基本信息、专业领域、工作经验等数据。
    ///
    /// # 参数
    ///
    /// - `request`: 猎头顾问创建请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::agency::AgencyConsultantCreateRequest;
    ///
    /// let request = AgencyConsultantCreateRequest {
    ///     agency_id: "agency_123456".to_string(),
    ///     name: "李顾问".to_string(),
    ///     email: "li@headhunter.com".to_string(),
    ///     phone: Some("13900139000".to_string()),
    ///     position: Some("高级顾问".to_string()),
    ///     specialties: vec!["技术人才".to_string(), "高管人才".to_string()],
    ///     experience_years: Some(8),
    /// };
    ///
    /// let response = client.hire.get_candidates.agency.add_consultant(request, None).await?;
    /// ```
    pub async fn add_consultant(
        &self,
        request: AgencyConsultantCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AgencyOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_AGENCY_CONSULTANTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取猎头顾问列表
    ///
    /// 该接口用于获取指定猎头机构的顾问列表，
    /// 包含顾问基本信息和专业能力数据。
    ///
    /// # 参数
    ///
    /// - `agency_id`: 猎头机构ID
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let agency_id = "agency_123456";
    /// let response = client.hire.get_candidates.agency.list_consultants(agency_id, Some(20), None, None).await?;
    /// ```
    pub async fn list_consultants(
        &self,
        agency_id: &str,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AgencyConsultantListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_AGENCY_CONSULTANTS,
            "agency_id",
            agency_id,
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
    /// 确认猎头推荐
    ///
    /// 该接口用于确认接受猎头推荐，将推荐转换为
    /// 正式的投递记录，进入招聘流程。
    ///
    /// # 参数
    ///
    /// - `recommendation_id`: 推荐记录ID
    /// - `remark`: 确认备注（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let recommendation_id = "rec_123456";
    /// let remark = Some("候选人背景符合要求，同意进入面试流程".to_string());
    ///
    /// let response = client.hire.get_candidates.agency.confirm_recommendation(
    ///     recommendation_id,
    ///     remark,
    ///     None
    /// ).await?;
    /// ```
    pub async fn confirm_recommendation(
        &self,
        recommendation_id: &str,
        remark: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AgencyOperationResponse>> {
        #[derive(Serialize)]
        struct ConfirmRequest {
            remark: Option<String>,
        }

        let request = ConfirmRequest { remark };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_AGENCY_RECOMMENDATION_CONFIRM.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 拒绝猎头推荐
    ///
    /// 该接口用于拒绝猎头推荐，设置拒绝原因
    /// 和相关反馈信息。
    ///
    /// # 参数
    ///
    /// - `recommendation_id`: 推荐记录ID
    /// - `reason`: 拒绝原因
    /// - `feedback`: 反馈信息（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let recommendation_id = "rec_123456";
    /// let reason = "候选人技能不匹配";
    /// let feedback = Some("需要更多微服务架构经验".to_string());
    ///
    /// let response = client.hire.get_candidates.agency.reject_recommendation(
    ///     recommendation_id,
    ///     reason,
    ///     feedback,
    ///     None
    /// ).await?;
    /// ```
    pub async fn reject_recommendation(
        &self,
        recommendation_id: &str,
        reason: &str,
        feedback: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AgencyOperationResponse>> {
        #[derive(Serialize)]
        struct RejectRequest {
            reason: String,
            feedback: Option<String>,
        }

        let request = RejectRequest {
            reason: reason.to_string(),
            feedback,
        };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_AGENCY_RECOMMENDATION_REJECT.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
}
