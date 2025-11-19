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

/// 官网服务
pub struct WebsiteService {
    pub config: Config,
}

/// 官网职位信息
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteJob {
    /// 职位ID
    pub id: String,
    /// 职位名称
    pub title: String,
    /// 职位描述
    pub description: Option<String>,
    /// 职位要求
    pub requirement: Option<String>,
    /// 工作地点
    pub location: Option<String>,
    /// 职位类型
    pub job_type: Option<String>,
    /// 薪资范围
    pub salary_range: Option<String>,
    /// 发布状态
    pub publish_status: String,
    /// 发布时间
    pub publish_time: Option<String>,
    /// 截止时间
    pub deadline: Option<String>,
    /// 浏览次数
    pub view_count: u32,
    /// 投递次数
    pub apply_count: u32,
    /// 职位标签
    pub tags: Vec<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 官网投递信息
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteApplication {
    /// 投递ID
    pub id: String,
    /// 职位ID
    pub job_id: String,
    /// 候选人姓名
    pub candidate_name: String,
    /// 候选人邮箱
    pub candidate_email: String,
    /// 候选人电话
    pub candidate_phone: Option<String>,
    /// 简历附件URL
    pub resume_url: Option<String>,
    /// 投递状态
    pub status: String,
    /// 投递时间
    pub apply_time: String,
    /// 候选人备注
    pub candidate_remark: Option<String>,
    /// 投递来源
    pub source: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 官网配置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteConfiguration {
    /// 配置ID
    pub id: String,
    /// 网站名称
    pub site_name: I18nText,
    /// 网站描述
    pub site_description: Option<I18nText>,
    /// 网站Logo URL
    pub logo_url: Option<String>,
    /// 联系邮箱
    pub contact_email: Option<String>,
    /// 联系电话
    pub contact_phone: Option<String>,
    /// 网站主题配置
    pub theme_config: Option<serde_json::Value>,
    /// SEO配置
    pub seo_config: Option<SeoConfig>,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// SEO配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SeoConfig {
    // TODO: Add fields
}

/// 官网职位发布请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebsiteJobPublishRequest {
    /// 职位ID
    pub job_id: String,
    /// 是否发布
    pub publish: bool,
    /// 发布时间
    pub publish_time: Option<String>,
    /// 截止时间
    pub deadline: Option<String>,
    /// 职位标签
    pub tags: Vec<String>,
}

/// 官网投递列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebsiteApplicationListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 职位ID
    pub job_id: Option<String>,
    /// 投递状态
    pub status: Option<String>,
    /// 候选人姓名关键词
    pub candidate_name: Option<String>,
    /// 候选人邮箱关键词
    pub candidate_email: Option<String>,
    /// 投递时间开始
    pub apply_start_time: Option<String>,
    /// 投递时间结束
    pub apply_end_time: Option<String>,
}

/// 官网配置更新请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebsiteConfigurationUpdateRequest {
    /// 网站名称
    pub site_name: Option<I18nText>,
    /// 网站描述
    pub site_description: Option<I18nText>,
    /// 网站Logo URL
    pub logo_url: Option<String>,
    /// 联系邮箱
    pub contact_email: Option<String>,
    /// 联系电话
    pub contact_phone: Option<String>,
    /// 网站主题配置
    pub theme_config: Option<serde_json::Value>,
    /// SEO配置
    pub seo_config: Option<SeoConfig>,
    /// 是否启用
    pub enabled: Option<bool>,
}

/// 官网职位列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteJobListResponse {
    /// 官网职位列表
    #[serde(flatten)]
    pub jobs: PageResponse<WebsiteJob>,
}

impl ApiResponseTrait for WebsiteJobListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 官网投递列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteApplicationListResponse {
    /// 官网投递列表
    #[serde(flatten)]
    pub applications: PageResponse<WebsiteApplication>,
}

impl ApiResponseTrait for WebsiteApplicationListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 官网配置响应
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteConfigurationResponse {
    /// 官网配置信息
    pub configuration: WebsiteConfiguration,
}

impl ApiResponseTrait for WebsiteConfigurationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 官网操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 相关ID
    pub id: Option<String>,
}

impl ApiResponseTrait for WebsiteOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl WebsiteService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取官网职位列表
    ///
    /// 该接口用于获取已发布到官网的职位列表，支持按
    /// 状态、类型、地点等条件筛选。返回的列表包含
    /// 职位基本信息和发布统计数据。
    ///
    /// # 参数
    ///
    /// - `page_size`: 分页大小（可选），最大值100
    /// - `page_token`: 分页标记（可选）
    /// - `publish_status`: 发布状态筛选（可选）
    /// - `job_type`: 职位类型筛选（可选）
    /// - `location`: 工作地点筛选（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的官网职位列表，包括：
    /// - 职位基本信息列表
    /// - 发布状态和统计数据
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.hire.get_candidates.website.list_website_jobs(
    ///     Some(20),
    ///     None,
    ///     Some("published".to_string()),
    ///     Some("full_time".to_string()),
    ///     Some("北京".to_string()),
    ///     None
    /// ).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("官网职位总数: {}", data.jobs.items.len());
    ///     for job in &data.jobs.items {
    ///         println!("职位: {} (投递{}次)", job.title, job.apply_count);
    ///     }
    /// }
    /// ```
    pub async fn list_website_jobs(
        &self,
        page_size: Option<u32>,
        page_token: Option<String>,
        publish_status: Option<String>,
        job_type: Option<String>,
        location: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<WebsiteJobListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_WEBSITE_JOBS.to_string());
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

        if let Some(publish_status) = publish_status {
            api_req
                .query_params
                .insert("publish_status", publish_status);
        }

        if let Some(job_type) = job_type {
            api_req.query_params.insert("job_type", job_type);
        }

        if let Some(location) = location {
            api_req.query_params.insert("location", location);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 发布职位到官网
    ///
    /// 该接口用于将内部职位发布到官网，设置发布时间、
    /// 截止时间、职位标签等发布配置。发布后的职位
    /// 将在官网上对外展示。
    ///
    /// # 参数
    ///
    /// - `request`: 官网职位发布请求参数，包括：
    ///   - `job_id`: 职位ID（必填）
    ///   - `publish`: 是否发布（必填）
    ///   - `publish_time`: 发布时间
    ///   - `deadline`: 截止时间
    ///   - `tags`: 职位标签
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回职位发布操作结果，包括：
    /// - `success`: 发布是否成功
    /// - `id`: 相关ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::website::WebsiteJobPublishRequest;
    ///
    /// let request = WebsiteJobPublishRequest {
    ///     job_id: "job_123456".to_string(),
    ///     publish: true,
    ///     publish_time: Some("2024-01-15T00:00:00Z".to_string()),
    ///     deadline: Some("2024-03-15T23:59:59Z".to_string()),
    ///     tags: vec!["热招".to_string(), "技术".to_string()],
    /// };
    ///
    /// let response = client.hire.get_candidates.website.publish_job_to_website(request, None).await?;
    /// ```
    pub async fn publish_job_to_website(
        &self,
        request: WebsiteJobPublishRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<WebsiteOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_WEBSITE_JOBS_PUBLISH.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 从官网下架职位
    ///
    /// 该接口用于将已发布的职位从官网下架，
    /// 下架后的职位将不再在官网上显示。
    ///
    /// # 参数
    ///
    /// - `job_id`: 职位ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let job_id = "job_123456";
    /// let response = client.hire.get_candidates.website.unpublish_job_from_website(job_id, None).await?;
    /// ```
    pub async fn unpublish_job_from_website(
        &self,
        job_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<WebsiteOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_WEBSITE_JOB_UNPUBLISH,
            "job_id",
            job_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取官网投递列表
    ///
    /// 该接口用于获取通过官网提交的投递列表，支持按
    /// 职位、状态、候选人信息、时间等条件筛选。返回
    /// 的列表包含投递基本信息，可用于投递管理。
    ///
    /// # 参数
    ///
    /// - `request`: 官网投递列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `job_id`: 职位ID筛选
    ///   - `status`: 投递状态筛选
    ///   - `candidate_name`: 候选人姓名关键词筛选
    ///   - `candidate_email`: 候选人邮箱关键词筛选
    ///   - `apply_start_time`: 投递时间开始
    ///   - `apply_end_time`: 投递时间结束
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的官网投递列表，包括：
    /// - 投递基本信息列表
    /// - 候选人信息和投递来源
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::website::WebsiteApplicationListRequest;
    ///
    /// let request = WebsiteApplicationListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     job_id: Some("job_123456".to_string()),
    ///     status: Some("pending".to_string()),
    ///     candidate_name: Some("张".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.get_candidates.website.list_website_applications(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("官网投递总数: {}", data.applications.items.len());
    ///     for application in &data.applications.items {
    ///         println!("投递: {} ({})", application.candidate_name, application.candidate_email);
    ///     }
    /// }
    /// ```
    pub async fn list_website_applications(
        &self,
        request: WebsiteApplicationListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<WebsiteApplicationListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_WEBSITE_APPLICATIONS.to_string());
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

        if let Some(candidate_name) = request.candidate_name {
            api_req
                .query_params
                .insert("candidate_name", candidate_name);
        }

        if let Some(candidate_email) = request.candidate_email {
            api_req
                .query_params
                .insert("candidate_email", candidate_email);
        }

        if let Some(apply_start_time) = request.apply_start_time {
            api_req
                .query_params
                .insert("apply_start_time", apply_start_time);
        }

        if let Some(apply_end_time) = request.apply_end_time {
            api_req
                .query_params
                .insert("apply_end_time", apply_end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取官网配置
    ///
    /// 该接口用于获取官网的配置信息，包括网站
    /// 基本信息、主题配置、SEO设置等。
    ///
    /// # 参数
    ///
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回官网配置详细信息，包括：
    /// - 网站基本信息（名称、描述、Logo等）
    /// - 联系方式信息
    /// - 主题和SEO配置
    /// - 启用状态
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.hire.get_candidates.website.get_website_configuration(None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("网站名称: {:?}", data.configuration.site_name.zh_cn);
    ///     println!("启用状态: {}", data.configuration.enabled);
    ///     println!("联系邮箱: {:?}", data.configuration.contact_email);
    /// }
    /// ```
    pub async fn get_website_configuration(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<WebsiteConfigurationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_WEBSITE_CONFIGURATION.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 更新官网配置
    ///
    /// 该接口用于更新官网的配置信息，支持修改网站
    /// 基本信息、主题配置、SEO设置等内容。
    ///
    /// # 参数
    ///
    /// - `request`: 官网配置更新请求参数，包括：
    ///   - `site_name`: 网站名称
    ///   - `site_description`: 网站描述
    ///   - `logo_url`: 网站Logo URL
    ///   - `contact_email`: 联系邮箱
    ///   - `contact_phone`: 联系电话
    ///   - `theme_config`: 网站主题配置
    ///   - `seo_config`: SEO配置
    ///   - `enabled`: 是否启用
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回配置更新操作结果，包括：
    /// - `success`: 更新是否成功
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::website::{WebsiteConfigurationUpdateRequest, SeoConfig};
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let request = WebsiteConfigurationUpdateRequest {
    ///     site_name: Some(I18nText {
    ///         zh_cn: Some("某科技公司招聘官网".to_string()),
    ///         en_us: Some("TechCorp Careers".to_string()),
    ///         ja_jp: None,
    ///     }),
    ///     contact_email: Some("careers@techcorp.com".to_string()),
    ///     enabled: Some(true),
    ///     seo_config: Some(SeoConfig {
    ///         keywords: Some(I18nText {
    ///             zh_cn: Some("招聘,技术,工程师".to_string()),
    ///             en_us: Some("careers,technology,engineer".to_string()),
    ///             ja_jp: None,
    ///         }),
    ///         description: None,
    ///         favicon_url: None,
    ///     }),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.get_candidates.website.update_website_configuration(request, None).await?;
    /// ```
    pub async fn update_website_configuration(
        &self,
        request: WebsiteConfigurationUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<WebsiteOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_WEBSITE_CONFIGURATION.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 将官网投递转换为内部投递
    ///
    /// 该接口用于将通过官网提交的投递转换为内部
    /// 招聘系统的正式投递记录，便于后续流程管理。
    ///
    /// # 参数
    ///
    /// - `website_application_id`: 官网投递ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let website_application_id = "web_app_123456";
    /// let response = client.hire.get_candidates.website.convert_website_application(website_application_id, None).await?;
    /// ```
    pub async fn convert_website_application(
        &self,
        website_application_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<WebsiteOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_WEBSITE_APPLICATION_CONVERT,
            "application_id",
            website_application_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取官网职位统计数据
    ///
    /// 该接口用于获取官网职位的统计数据，包括
    /// 浏览量、投递量等关键指标。
    ///
    /// # 参数
    ///
    /// - `job_id`: 职位ID（可选）
    /// - `start_date`: 统计开始日期（可选）
    /// - `end_date`: 统计结束日期（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let job_id = Some("job_123456".to_string());
    /// let start_date = Some("2024-01-01".to_string());
    /// let end_date = Some("2024-01-31".to_string());
    ///
    /// let response = client.hire.get_candidates.website.get_website_job_statistics(
    ///     job_id,
    ///     start_date,
    ///     end_date,
    ///     None
    /// ).await?;
    /// ```
    pub async fn get_website_job_statistics(
        &self,
        job_id: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<serde_json::Value>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_WEBSITE_STATISTICS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(job_id) = job_id {
            api_req.query_params.insert("job_id", job_id);
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
