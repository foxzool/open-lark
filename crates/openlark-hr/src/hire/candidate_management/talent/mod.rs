use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
        api::ApiRequest,
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::hire::*,
        endpoints::EndpointBuilder,
        error::LarkAPIError,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    };
use crate::hire::models::{
    Application, CommonResponse, PageResponse, Talent, TalentCreateRequest,};

mod builders;

// Re-export builders for easier access
pub use builders::{TalentCreateRequestBuilder, TalentListRequestBuilder};

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests;

/// 人才服务
pub struct TalentService {
    pub config: Config,
}

/// 人才列表请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TalentListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 姓名关键词
    pub name_keyword: Option<String>,
    /// 邮箱关键词
    pub email_keyword: Option<String>,
    /// 电话关键词
    pub phone_keyword: Option<String>,
    /// 工作年限筛选
    pub work_experience: Option<u32>,
    /// 学历筛选
    pub education: Option<String>,
    /// 人才标签
    pub tags: Vec<String>,
    /// 创建时间开始
    pub created_start_time: Option<String>,
    /// 创建时间结束
    pub created_end_time: Option<String>,
}

/// 人才列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentListResponse {
    /// 人才列表
    #[serde(flatten)]
    pub talents: PageResponse<Talent>,
}

impl ApiResponseTrait for TalentListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人才详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentDetailResponse {
    /// 人才信息
    pub talent: Talent,
}

impl ApiResponseTrait for TalentDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人才操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 人才ID
    pub talent_id: Option<String>,
}

impl ApiResponseTrait for TalentOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人才投递历史响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentApplicationHistoryResponse {
    /// 投递历史列表
    #[serde(flatten)]
    pub applications: PageResponse<Application>,
}

impl ApiResponseTrait for TalentApplicationHistoryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TalentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建人才档案
    ///
    /// 该接口用于创建新的人才档案，录入人才的基本信息、
    /// 工作经历、教育背景、技能标签等详细数据。
    /// 创建的人才档案可用于后续的匹配和推荐。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/combined_create
    ///
    /// # 参数
    ///
    /// - `request`: 人才创建请求参数，包括：
    ///   - `name`: 姓名（必填）
    ///   - `email`: 邮箱
    ///   - `phone`: 电话
    ///   - `gender`: 性别
    ///   - `birthday`: 生日
    ///   - `work_experience`: 工作年限
    ///   - `education`: 学历
    ///   - `current_company`: 当前公司
    ///   - `current_position`: 当前职位
    ///   - `expected_salary`: 期望薪资
    ///   - `resume_attachment_ids`: 简历附件ID列表
    ///   - `tags`: 人才标签
    ///   - `custom_fields`: 自定义字段
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回人才创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `talent_id`: 创建的人才ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::TalentCreateRequest;
    ///
    /// let request = TalentCreateRequest {
    ///     name: "张三".to_string(),
    ///     email: Some("zhangsan@example.com".to_string()),
    ///     phone: Some("13800138000".to_string()),
    ///     gender: Some("male".to_string()),
    ///     work_experience: Some(5),
    ///     education: Some("本科".to_string()),
    ///     current_company: Some("某科技公司".to_string()),
    ///     current_position: Some("高级Java工程师".to_string()),
    ///     expected_salary: Some("20-30K".to_string()),
    ///     tags: vec!["Java".to_string(), "Spring".to_string(), "微服务".to_string()],
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.candidate_management.talent.create_talent(request, None).await?;
    /// ```
    pub async fn create_talent(
        &self,
        request: TalentCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_TALENTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取人才详情
    ///
    /// 该接口用于获取指定人才的详细信息，包括人才
    /// 基本信息、工作经历、教育背景、投递历史等
    /// 完整数据。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/get
    ///
    /// # 参数
    ///
    /// - `talent_id`: 人才ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回人才详细信息，包括：
    /// - 人才基本信息（姓名、邮箱、电话等）
    /// - 工作经历和教育背景
    /// - 技能标签和期望薪资
    /// - 简历附件信息
    /// - 自定义字段数据
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let talent_id = "talent_123456";
    /// let response = client.hire.candidate_management.talent.get_talent_detail(talent_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("人才姓名: {}", data.talent.name);
    ///     println!("工作年限: {}年", data.talent.work_experience.unwrap_or(0));
    ///     println!("当前公司: {:?}", data.talent.current_company);
    /// }
    /// ```
    pub async fn get_talent_detail(
        &self,
        talent_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_TALENT_GET, "talent_id", talent_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取人才列表
    ///
    /// 该接口用于获取企业的人才列表，支持按姓名、邮箱、
    /// 电话、工作年限、学历、标签等条件筛选。返回的
    /// 列表包含人才基本信息，可用于人才管理和匹配。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/list
    ///
    /// # 参数
    ///
    /// - `request`: 人才列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `name_keyword`: 姓名关键词筛选
    ///   - `email_keyword`: 邮箱关键词筛选
    ///   - `phone_keyword`: 电话关键词筛选
    ///   - `work_experience`: 工作年限筛选
    ///   - `education`: 学历筛选
    ///   - `tags`: 人才标签筛选
    ///   - `created_start_time`: 创建时间开始
    ///   - `created_end_time`: 创建时间结束
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的人才列表，包括：
    /// - 人才基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::talent::TalentListRequest;
    ///
    /// let request = TalentListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     name_keyword: Some("张".to_string()),
    ///     work_experience: Some(3),
    ///     education: Some("本科".to_string()),
    ///     tags: vec!["Java".to_string(), "Python".to_string()],
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.candidate_management.talent.list_talents(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("符合条件的人才数: {}", data.talents.items.len());
    ///     for talent in &data.talents.items {
    ///         println!("人才: {} ({})", talent.name, talent.current_position.as_deref().unwrap_or("未知"));
    ///     }
    /// }
    /// ```
    pub async fn list_talents(
        &self,
        request: TalentListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_TALENTS.to_string());
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

        if let Some(name_keyword) = request.name_keyword {
            api_req.query_params.insert("name_keyword", name_keyword);
        }

        if let Some(email_keyword) = request.email_keyword {
            api_req.query_params.insert("email_keyword", email_keyword);
        }

        if let Some(phone_keyword) = request.phone_keyword {
            api_req.query_params.insert("phone_keyword", phone_keyword);
        }

        if let Some(work_experience) = request.work_experience {
            api_req
                .query_params
                .insert("work_experience", work_experience.to_string());
        }

        if let Some(education) = request.education {
            api_req.query_params.insert("education", education);
        }

        if !request.tags.is_empty() {
            api_req.query_params.insert("tags", request.tags.join(","));
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

    /// 更新人才档案
    ///
    /// 该接口用于更新现有人才档案的信息，支持修改
    /// 人才的基本信息、工作经历、技能标签等数据。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/combined_update
    ///
    /// # 参数
    ///
    /// - `talent_id`: 人才ID
    /// - `request`: 人才更新请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::TalentCreateRequest;
    ///
    /// let talent_id = "talent_123456";
    /// let request = TalentCreateRequest {
    ///     name: "张三".to_string(),
    ///     work_experience: Some(6),
    ///     current_company: Some("新科技公司".to_string()),
    ///     current_position: Some("技术专家".to_string()),
    ///     expected_salary: Some("25-35K".to_string()),
    ///     tags: vec!["Java".to_string(), "Spring".to_string(), "微服务".to_string(), "架构设计".to_string()],
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.candidate_management.talent.update_talent(talent_id, request, None).await?;
    /// ```
    pub async fn update_talent(
        &self,
        talent_id: &str,
        request: TalentCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_TALENT_GET, "talent_id", talent_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 删除人才档案
    ///
    /// 该接口用于删除指定的人才档案。删除后的人才
    /// 档案将不再可用，相关的投递记录也会受到影响。
    ///
    /// # API文档
    ///
    /// 注意：此功能在官方API文档中未找到对应的删除接口，可能不支持直接删除人才档案
    ///
    /// # 参数
    ///
    /// - `talent_id`: 人才ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let talent_id = "talent_123456";
    /// let response = client.hire.candidate_management.talent.delete_talent(talent_id, None).await?;
    /// ```
    pub async fn delete_talent(
        &self,
        talent_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_TALENT_GET, "talent_id", talent_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取人才投递历史
    ///
    /// 该接口用于获取指定人才的投递历史记录，
    /// 包括投递的职位、状态、时间等信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview/get_by_talent
    ///
    /// # 参数
    ///
    /// - `talent_id`: 人才ID
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的投递历史列表，包括：
    /// - 投递记录列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let talent_id = "talent_123456";
    /// let response = client.hire.candidate_management.talent.get_talent_application_history(
    ///     talent_id,
    ///     Some(20),
    ///     None,
    ///     None
    /// ).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("投递历史数: {}", data.applications.items.len());
    ///     for application in &data.applications.items {
    ///         println!("职位: {} 状态: {}", application.job_id, application.status);
    ///     }
    /// }
    /// ```
    pub async fn get_talent_application_history(
        &self,
        talent_id: &str,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentApplicationHistoryResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_TALENT_APPLICATIONS,
            "talent_id",
            talent_id,
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
    /// 批量导入人才
    ///
    /// 该接口用于批量导入人才档案，支持一次性
    /// 导入多个人才的基本信息。
    ///
    /// # 参数
    ///
    /// - `talents`: 人才信息列表
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::TalentCreateRequest;
    ///
    /// let talents = vec![
    ///     TalentCreateRequest {
    ///         name: "李四".to_string(),
    ///         email: Some("lisi@example.com".to_string()),
    ///         tags: vec!["Python".to_string()],
    ///         ..Default::default()
    ///     },
    ///     TalentCreateRequest {
    ///         name: "王五".to_string(),
    ///         email: Some("wangwu@example.com".to_string()),
    ///         tags: vec!["Go".to_string()],
    ///         ..Default::default()
    ///     },
    /// ];
    ///
    /// let response = client.hire.candidate_management.talent.batch_import_talents(talents, None).await?;
    /// ```
    pub async fn batch_import_talents(
        &self,
        talents: Vec<TalentCreateRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentOperationResponse>> {
        #[derive(Serialize)]
        struct BatchImportRequest {
            talents: Vec<TalentCreateRequest>,
        }

        let request = BatchImportRequest { talents };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_TALENTS_BATCH_IMPORT.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 创建人才档案（使用构建器模式）
    ///
    /// 该接口使用构建器模式创建新的人才档案，提供链式调用和内置验证功能。
    ///
    /// # 参数
    ///
    /// - `builder_result`: 人才创建请求构建器的结果
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回人才创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `talent_id`: 创建的人才ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let builder = client.hire.candidate_management.talent.create_talent_builder()
    ///     .with_name("张三")
    ///     .with_email("zhangsan@example.com")
    ///     .with_phone("13800138000")
    ///     .with_work_experience(5)
    ///     .with_education("本科")
    ///     .with_tags(vec!["Java".to_string(), "Spring".to_string()]);
    ///
    /// let response = client.hire.candidate_management.talent.create_talent_with_builder(builder.build(), None).await?;
    /// ```
    pub async fn create_talent_with_builder(
        &self,
        builder_result: SDKResult<TalentCreateRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentOperationResponse>> {
        let request = builder_result?;
        self.create_talent(request, option).await
    }

    /// 更新人才档案（使用构建器模式）
    ///
    /// 该接口使用构建器模式更新现有人才档案的信息，
    /// 支持修改人才的基本信息、工作经历、技能标签等数据。
    ///
    /// # 参数
    ///
    /// - `talent_id`: 人才ID
    /// - `builder_result`: 人才更新请求构建器的结果
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let builder = client.hire.candidate_management.talent.create_talent_builder()
    ///     .with_name("张三")
    ///     .with_work_experience(6)
    ///     .with_current_company("新科技公司")
    ///     .with_current_position("技术专家")
    ///     .with_tags(vec!["Java".to_string(), "架构设计".to_string()]);
    ///
    /// let response = client.hire.candidate_management.talent.update_talent_with_builder(
    ///     "talent_123456",
    ///     builder.build(),
    ///     None
    /// ).await?;
    /// ```
    pub async fn update_talent_with_builder(
        &self,
        talent_id: &str,
        builder_result: SDKResult<TalentCreateRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentOperationResponse>> {
        let request = builder_result?;
        self.update_talent(talent_id, request, option).await
    }

    /// 批量导入人才（使用构建器模式）
    ///
    /// 该接口使用构建器模式批量导入人才档案，支持一次性
    /// 导入多个人才的基本信息，并提供验证功能。
    ///
    /// # 参数
    ///
    /// - `builder_results`: 人才创建请求构建器结果列表
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let builders = vec![
    ///     client.hire.candidate_management.talent.create_talent_builder()
    ///         .with_name("李四")
    ///         .with_email("lisi@example.com"),
    ///     client.hire.candidate_management.talent.create_talent_builder()
    ///         .with_name("王五")
    ///         .with_email("wangwu@example.com"),
    /// ];
    ///
    /// let requests: Result<Vec<_>, _> = builders.into_iter().map(|b| b.build()).collect();
    /// let response = client.hire.candidate_management.talent.batch_import_talents_with_builder(
    ///     requests,
    ///     None
    /// ).await?;
    /// ```
    pub async fn batch_import_talents_with_builder(
        &self,
        builder_results: Result<Vec<TalentCreateRequest>, LarkAPIError>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentOperationResponse>> {
        let talents = builder_results?;
        self.batch_import_talents(talents, option).await
    }

    /// 创建人才档案构建器
    ///
    /// 返回一个人才创建请求的构建器，支持链式调用设置各种属性。
    ///
    /// # 返回值
    ///
    /// 返回 `TalentCreateRequestBuilder` 实例，可用于链式调用
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let builder = client.hire.candidate_management.talent.create_talent_builder()
    ///     .with_name("张三")
    ///     .with_email("zhangsan@example.com")
    ///     .with_phone("13800138000")
    ///     .with_work_experience(5)
    ///     .with_education("本科")
    ///     .with_tags(vec!["Java".to_string(), "Spring".to_string()]);
    ///
    /// let request = builder.build()?;
    /// ```
    pub fn create_talent_builder(&self) -> TalentCreateRequestBuilder {
        TalentCreateRequestBuilder::default()
    }

    /// 创建人才列表请求构建器
    ///
    /// 返回一个人才列表查询请求的构建器，支持链式调用设置筛选条件。
    ///
    /// # 返回值
    ///
    /// 返回 `TalentListRequestBuilder` 实例，可用于链式调用
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let builder = client.hire.candidate_management.talent.list_talents_builder()
    ///     .with_page_size(50)
    ///     .with_name_keyword("张")
    ///     .with_work_experience(5)
    ///     .with_tags(vec!["Java".to_string()]);
    ///
    /// let request = builder.build();
    /// ```
    pub fn list_talents_builder(&self) -> TalentListRequestBuilder {
        TalentListRequestBuilder::default()
    }

    /// 获取人才列表（使用构建器模式）
    ///
    /// 该接口使用构建器模式获取企业的人才列表，支持按姓名、邮箱、
    /// 电话、工作年限、学历、标签等条件筛选。
    ///
    /// # 参数
    ///
    /// - `request`: 人才列表查询请求
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的人才列表，包括：
    /// - 人才基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let request = client.hire.candidate_management.talent.list_talents_builder()
    ///     .with_page_size(50)
    ///     .with_name_keyword("张")
    ///     .with_work_experience(5)
    ///     .with_tags(vec!["Java".to_string()])
    ///     .build();
    ///
    /// let response = client.hire.candidate_management.talent.list_talents_with_builder(request, None).await?;
    /// ```
    pub async fn list_talents_with_builder(
        &self,
        request: TalentListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentListResponse>> {
        self.list_talents(request, option).await
    }

    /// 批量创建人才档案（使用构建器模式）
    ///
    /// 该接口使用构建器模式批量创建人才档案，每个请求都会经过验证。
    ///
    /// # 参数
    ///
    /// - `builder_results`: 人才创建请求构建器结果列表
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回批量创建操作结果，包括每个人才的创建状态
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let builders = vec![
    ///     client.hire.candidate_management.talent.create_talent_builder()
    ///         .with_name("李四")
    ///         .with_email("lisi@example.com"),
    ///     client.hire.candidate_management.talent.create_talent_builder()
    ///         .with_name("王五")
    ///         .with_email("wangwu@example.com"),
    /// ];
    ///
    /// let responses = client.hire.candidate_management.talent.batch_create_talents_with_builder(
    ///     builders.into_iter().map(|b| b.build()).collect(),
    ///     None
    /// ).await?;
    /// ```
    pub async fn batch_create_talents_with_builder(
        &self,
        builder_results: Vec<SDKResult<TalentCreateRequest>>,
        option: Option<RequestOption>,
    ) -> Vec<SDKResult<Response<TalentOperationResponse>>> {
        let mut results = Vec::new();

        for builder_result in builder_results {
            match builder_result {
                Ok(request) => match self.create_talent(request, option.clone()).await {
                    Ok(response) => results.push(Ok(response)),
                    Err(e) => results.push(Err(e)),
                },
                Err(e) => results.push(Err(e)),
            }
        }

        results
    }
}
