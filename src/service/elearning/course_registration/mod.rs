use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::elearning::models::{CourseRegistration, LearningStatistics, PageResponse},
};

/// 课程学习进度管理服务
pub struct CourseRegistrationService {
    pub config: Config,
}

impl CourseRegistrationService {
    /// 创建课程学习进度管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建课程学习进度记录
    ///
    /// 为用户创建新的课程学习进度记录。
    ///
    /// # Arguments
    ///
    /// * `request` - 课程学习进度创建请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回创建的学习进度记录
    pub async fn create(
        &self,
        request: CourseRegistrationCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CourseRegistrationCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::ELEARNING_V2_COURSE_REGISTRATIONS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询课程学习进度列表
    ///
    /// 分页查询课程学习进度记录，支持多种筛选条件。
    ///
    /// # Arguments
    ///
    /// * `request` - 课程学习进度查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回学习进度记录列表
    pub async fn list(
        &self,
        request: CourseRegistrationListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CourseRegistrationListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::ELEARNING_V2_COURSE_REGISTRATIONS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(course_id) = request.course_id {
            api_req.query_params.insert("course_id", course_id);
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert("user_id", user_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(registration_type) = request.registration_type {
            api_req
                .query_params
                .insert("registration_type", registration_type);
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert("start_time", start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert("end_time", end_time.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取课程学习进度详情
    ///
    /// 根据学习进度记录ID获取详细信息。
    ///
    /// # Arguments
    ///
    /// * `registration_id` - 学习进度记录ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回学习进度记录详情
    pub async fn get(
        &self,
        registration_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CourseRegistrationGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::ELEARNING_V2_COURSE_REGISTRATION_OPERATION,
                "registration_id",
                registration_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新课程学习进度
    ///
    /// 更新指定的课程学习进度记录。
    ///
    /// # Arguments
    ///
    /// * `registration_id` - 学习进度记录ID
    /// * `request` - 课程学习进度更新请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回更新后的学习进度记录
    pub async fn update(
        &self,
        registration_id: &str,
        request: CourseRegistrationUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CourseRegistrationUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                Endpoints::ELEARNING_V2_COURSE_REGISTRATION_OPERATION,
                "registration_id",
                registration_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除课程学习进度记录
    ///
    /// 删除指定的课程学习进度记录。
    ///
    /// # Arguments
    ///
    /// * `registration_id` - 学习进度记录ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回删除结果
    pub async fn delete(
        &self,
        registration_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CourseRegistrationDeleteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::ELEARNING_V2_COURSE_REGISTRATION_OPERATION,
                "registration_id",
                registration_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取学习统计数据
    ///
    /// 获取用户或课程的学习统计信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 学习统计查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回学习统计数据
    pub async fn get_statistics(
        &self,
        request: CourseRegistrationStatisticsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CourseRegistrationStatisticsResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::ELEARNING_V2_COURSE_REGISTRATIONS_STATISTICS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(user_id) = request.user_id {
            api_req.query_params.insert("user_id", user_id);
        }

        if let Some(course_id) = request.course_id {
            api_req.query_params.insert("course_id", course_id);
        }

        if let Some(department_id) = request.department_id {
            api_req.query_params.insert("department_id", department_id);
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert("start_time", start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert("end_time", end_time.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }
}

/// 课程学习进度创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseRegistrationCreateRequest {
    /// 课程ID
    pub course_id: String,
    /// 用户ID
    pub user_id: String,
    /// 注册类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<String>,
    /// 初始状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 备注信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// 课程学习进度创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseRegistrationCreateResponse {
    /// 创建的学习进度记录
    pub registration: CourseRegistration,
}

impl ApiResponseTrait for CourseRegistrationCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 课程学习进度查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CourseRegistrationListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 课程ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_id: Option<String>,
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 学习状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 注册类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<String>,
    /// 开始时间戳筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 课程学习进度查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseRegistrationListResponse {
    /// 学习进度记录列表
    #[serde(flatten)]
    pub registrations: PageResponse<CourseRegistration>,
}

impl ApiResponseTrait for CourseRegistrationListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 课程学习进度详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseRegistrationGetResponse {
    /// 学习进度记录详情
    pub registration: CourseRegistration,
}

impl ApiResponseTrait for CourseRegistrationGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 课程学习进度更新请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CourseRegistrationUpdateRequest {
    /// 学习状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 学习进度（百分比）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    /// 已学习时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studied_duration: Option<i64>,
    /// 学习成绩
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// 是否通过
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
    /// 完成时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<i64>,
    /// 备注信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// 课程学习进度更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseRegistrationUpdateResponse {
    /// 更新后的学习进度记录
    pub registration: CourseRegistration,
}

impl ApiResponseTrait for CourseRegistrationUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 课程学习进度删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseRegistrationDeleteResponse {
    /// 删除是否成功
    pub success: bool,
    /// 被删除的记录ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ApiResponseTrait for CourseRegistrationDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 学习统计查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CourseRegistrationStatisticsRequest {
    /// 用户ID（获取指定用户的统计数据）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 课程ID（获取指定课程的统计数据）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_id: Option<String>,
    /// 部门ID（获取指定部门的统计数据）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 学习统计查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseRegistrationStatisticsResponse {
    /// 学习统计数据
    pub statistics: LearningStatistics,
}

impl ApiResponseTrait for CourseRegistrationStatisticsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
