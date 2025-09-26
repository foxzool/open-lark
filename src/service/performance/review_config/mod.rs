use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::performance::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::performance::models::{
        Activity, AdditionalInformation, Metric, MetricField, MetricTag, MetricTemplate,
        PageResponse, ReviewItem, ReviewTemplate, Reviewee, Semester, TagQuestionConfig, UserGroup,
    },
};

/// 后台配置管理服务
pub struct ReviewConfigService {
    pub config: Config,
}

impl ReviewConfigService {
    /// 创建后台配置管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取周期列表
    ///
    /// 分页获取绩效周期列表。
    ///
    /// # Arguments
    ///
    /// * `request` - 周期列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回周期列表
    pub async fn list_semesters(
        &self,
        request: SemesterListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SemesterListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: PERFORMANCE_SEMESTER_LIST.to_string(),
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

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取项目列表
    ///
    /// 查询指定周期下的绩效项目列表。
    ///
    /// # Arguments
    ///
    /// * `request` - 项目查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回项目列表
    pub async fn query_activities(
        &self,
        request: ActivityQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ActivityQueryResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: PERFORMANCE_ACTIVITIES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(semester_id) = request.semester_id {
            api_req.query_params.insert("semester_id", semester_id);
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量查询补充信息
    ///
    /// 查询指定项目下的补充信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 补充信息查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回补充信息列表
    pub async fn query_additional_information(
        &self,
        request: AdditionalInfoQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AdditionalInfoQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_ADDITIONAL_INFO_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量导入补充信息
    ///
    /// 批量导入或更新补充信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 补充信息导入请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回导入结果
    pub async fn import_additional_information(
        &self,
        request: AdditionalInfoImportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AdditionalInfoImportResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_ADDITIONAL_INFO_IMPORT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量删除补充信息
    ///
    /// 批量删除指定的补充信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 补充信息删除请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回删除结果
    pub async fn delete_additional_information(
        &self,
        request: AdditionalInfoDeleteRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AdditionalInfoDeleteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_ADDITIONAL_INFO_DELETE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新人员组成员
    ///
    /// 更新指定人员组的成员列表。
    ///
    /// # Arguments
    ///
    /// * `request` - 人员组成员更新请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回更新结果
    pub async fn write_user_group_members(
        &self,
        request: UserGroupWriteRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserGroupWriteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_USER_GROUP_WRITE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取被评估人信息
    ///
    /// 查询指定项目下的被评估人信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 被评估人查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回被评估人信息列表
    pub async fn query_reviewees(
        &self,
        request: RevieweeQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RevieweeQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_REVIEWEES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取评估模板配置
    ///
    /// 查询指定项目的评估模板配置。
    ///
    /// # Arguments
    ///
    /// * `request` - 评估模板查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回评估模板配置
    pub async fn query_review_templates(
        &self,
        request: ReviewTemplateQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ReviewTemplateQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_REVIEW_TEMPLATES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取评估项列表
    ///
    /// 查询指定评估模板的评估项列表。
    ///
    /// # Arguments
    ///
    /// * `request` - 评估项查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回评估项列表
    pub async fn query_review_items(
        &self,
        request: ReviewItemQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ReviewItemQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_REVIEW_ITEMS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取标签填写题配置
    ///
    /// 查询标签填写题的详细配置。
    ///
    /// # Arguments
    ///
    /// * `request` - 标签题配置查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回标签题配置
    pub async fn query_tag_question_configs(
        &self,
        request: TagQuestionConfigQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<TagQuestionConfigQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_TAG_QUESTIONS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取指标列表
    ///
    /// 查询可用的指标列表。
    ///
    /// # Arguments
    ///
    /// * `request` - 指标查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回指标列表
    pub async fn query_metrics(
        &self,
        request: MetricQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_METRICS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取指标模板列表
    ///
    /// 查询指标模板配置。
    ///
    /// # Arguments
    ///
    /// * `request` - 指标模板查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回指标模板列表
    pub async fn query_metric_templates(
        &self,
        request: MetricTemplateQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricTemplateQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_METRIC_TEMPLATES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取指标字段列表
    ///
    /// 查询指标的字段配置。
    ///
    /// # Arguments
    ///
    /// * `request` - 指标字段查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回指标字段列表
    pub async fn query_metric_fields(
        &self,
        request: MetricFieldQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricFieldQueryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: PERFORMANCE_METRIC_FIELDS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取指标标签列表
    ///
    /// 查询可用的指标标签。
    ///
    /// # Arguments
    ///
    /// * `request` - 指标标签查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回指标标签列表
    pub async fn list_metric_tags(
        &self,
        request: MetricTagListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricTagListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: PERFORMANCE_METRIC_TAGS.to_string(),
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

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for ReviewConfigService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "review_config"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

/// 周期列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SemesterListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 周期列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SemesterListResponse {
    /// 周期列表
    #[serde(flatten)]
    pub semesters: PageResponse<Semester>,
}

impl ApiResponseTrait for SemesterListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 项目查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivityQueryRequest {
    /// 周期ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 项目查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityQueryResponse {
    /// 项目列表
    #[serde(flatten)]
    pub activities: PageResponse<Activity>,
}

impl ApiResponseTrait for ActivityQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 补充信息查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalInfoQueryRequest {
    /// 项目ID
    pub activity_id: String,
    /// 用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

/// 补充信息查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfoQueryResponse {
    /// 补充信息列表
    pub additional_information: Vec<AdditionalInformation>,
}

impl ApiResponseTrait for AdditionalInfoQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 补充信息导入请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalInfoImportRequest {
    /// 项目ID
    pub activity_id: String,
    /// 补充信息列表
    pub additional_information: Vec<AdditionalInformation>,
}

/// 补充信息导入响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfoImportResponse {
    /// 导入成功标识
    pub success: bool,
    /// 导入数量
    pub imported_count: i32,
    /// 失败信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<String>>,
}

impl ApiResponseTrait for AdditionalInfoImportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 补充信息删除请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalInfoDeleteRequest {
    /// 项目ID
    pub activity_id: String,
    /// 要删除的信息ID列表
    pub info_ids: Vec<String>,
}

/// 补充信息删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfoDeleteResponse {
    /// 删除成功标识
    pub success: bool,
    /// 删除数量
    pub deleted_count: i32,
}

impl ApiResponseTrait for AdditionalInfoDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人员组成员更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroupWriteRequest {
    /// 人员组ID
    pub group_id: String,
    /// 成员用户ID列表
    pub member_user_ids: Vec<String>,
    /// 操作类型 (add: 添加, remove: 移除, replace: 替换)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
}

/// 人员组成员更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserGroupWriteResponse {
    /// 更新成功标识
    pub success: bool,
    /// 人员组信息
    pub user_group: UserGroup,
}

impl ApiResponseTrait for UserGroupWriteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 被评估人查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevieweeQueryRequest {
    /// 项目ID
    pub activity_id: String,
    /// 用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

/// 被评估人查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RevieweeQueryResponse {
    /// 被评估人列表
    pub reviewees: Vec<Reviewee>,
}

impl ApiResponseTrait for RevieweeQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 评估模板查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewTemplateQueryRequest {
    /// 项目ID
    pub activity_id: String,
    /// 模板类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
}

/// 评估模板查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewTemplateQueryResponse {
    /// 评估模板列表
    pub review_templates: Vec<ReviewTemplate>,
}

impl ApiResponseTrait for ReviewTemplateQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 评估项查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewItemQueryRequest {
    /// 模板ID
    pub template_id: String,
}

/// 评估项查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewItemQueryResponse {
    /// 评估项列表
    pub review_items: Vec<ReviewItem>,
}

impl ApiResponseTrait for ReviewItemQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 标签题配置查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagQuestionConfigQueryRequest {
    /// 评估项ID列表
    pub item_ids: Vec<String>,
}

/// 标签题配置查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TagQuestionConfigQueryResponse {
    /// 标签题配置列表
    pub tag_question_configs: Vec<TagQuestionConfig>,
}

impl ApiResponseTrait for TagQuestionConfigQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 指标查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricQueryRequest {
    /// 项目ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// 是否只查询关键指标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_only: Option<bool>,
}

/// 指标查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricQueryResponse {
    /// 指标列表
    pub metrics: Vec<Metric>,
}

impl ApiResponseTrait for MetricQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 指标模板查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTemplateQueryRequest {
    /// 项目ID
    pub activity_id: String,
}

/// 指标模板查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricTemplateQueryResponse {
    /// 指标模板列表
    pub metric_templates: Vec<MetricTemplate>,
}

impl ApiResponseTrait for MetricTemplateQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 指标字段查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricFieldQueryRequest {
    /// 指标ID
    pub metric_id: String,
}

/// 指标字段查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricFieldQueryResponse {
    /// 指标字段列表
    pub metric_fields: Vec<MetricField>,
}

impl ApiResponseTrait for MetricFieldQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 指标标签列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricTagListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 指标标签列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricTagListResponse {
    /// 指标标签列表
    #[serde(flatten)]
    pub metric_tags: PageResponse<MetricTag>,
}

impl ApiResponseTrait for MetricTagListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
