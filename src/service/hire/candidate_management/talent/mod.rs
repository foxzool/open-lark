use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::hire::models::{
        Application, CommonResponse, PageResponse, Talent, TalentCreateRequest,
    },
};

/// 人才服务
pub struct TalentService {
    pub config: Config,
}

/// 人才列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
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
    /// use open_lark::service::hire::models::TalentCreateRequest;
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
    ) -> SDKResult<BaseResponse<TalentOperationResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/hire/v1/talents".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取人才详情
    ///
    /// 该接口用于获取指定人才的详细信息，包括人才
    /// 基本信息、工作经历、教育背景、投递历史等
    /// 完整数据。
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
    ) -> SDKResult<BaseResponse<TalentDetailResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/hire/v1/talents/{}", talent_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取人才列表
    ///
    /// 该接口用于获取企业的人才列表，支持按姓名、邮箱、
    /// 电话、工作年限、学历、标签等条件筛选。返回的
    /// 列表包含人才基本信息，可用于人才管理和匹配。
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
    ) -> SDKResult<BaseResponse<TalentListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/hire/v1/talents".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }

        if let Some(name_keyword) = request.name_keyword {
            api_req
                .query_params
                .insert("name_keyword".to_string(), name_keyword);
        }

        if let Some(email_keyword) = request.email_keyword {
            api_req
                .query_params
                .insert("email_keyword".to_string(), email_keyword);
        }

        if let Some(phone_keyword) = request.phone_keyword {
            api_req
                .query_params
                .insert("phone_keyword".to_string(), phone_keyword);
        }

        if let Some(work_experience) = request.work_experience {
            api_req
                .query_params
                .insert("work_experience".to_string(), work_experience.to_string());
        }

        if let Some(education) = request.education {
            api_req
                .query_params
                .insert("education".to_string(), education);
        }

        if !request.tags.is_empty() {
            api_req
                .query_params
                .insert("tags".to_string(), request.tags.join(","));
        }

        if let Some(created_start_time) = request.created_start_time {
            api_req
                .query_params
                .insert("created_start_time".to_string(), created_start_time);
        }

        if let Some(created_end_time) = request.created_end_time {
            api_req
                .query_params
                .insert("created_end_time".to_string(), created_end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新人才档案
    ///
    /// 该接口用于更新现有人才档案的信息，支持修改
    /// 人才的基本信息、工作经历、技能标签等数据。
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
    /// use open_lark::service::hire::models::TalentCreateRequest;
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
    ) -> SDKResult<BaseResponse<TalentOperationResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("/open-apis/hire/v1/talents/{}", talent_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除人才档案
    ///
    /// 该接口用于删除指定的人才档案。删除后的人才
    /// 档案将不再可用，相关的投递记录也会受到影响。
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
    ) -> SDKResult<BaseResponse<TalentOperationResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: format!("/open-apis/hire/v1/talents/{}", talent_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取人才投递历史
    ///
    /// 该接口用于获取指定人才的投递历史记录，
    /// 包括投递的职位、状态、时间等信息。
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
    ) -> SDKResult<BaseResponse<TalentApplicationHistoryResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/hire/v1/talents/{}/applications", talent_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(page_token) = page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
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
    /// use open_lark::service::hire::models::TalentCreateRequest;
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
    ) -> SDKResult<BaseResponse<TalentOperationResponse>> {
        #[derive(Serialize)]
        struct BatchImportRequest {
            talents: Vec<TalentCreateRequest>,
        }

        let request = BatchImportRequest { talents };

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/hire/v1/talents/batch_import".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
