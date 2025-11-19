use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::hire::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::hire::models::PageResponse;

/// 候选人配置服务
///
/// 提供候选人相关配置功能，包括人才标签管理、信息登记表等配置服务。
pub struct ApplicationConfigService {
    pub config: Config,
}

/// 人才标签列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentTagListResponse {
    /// 标签列表
    #[serde(flatten)]
    pub tags: PageResponse<TalentTag>,
}

impl ApiResponseTrait for TalentTagListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 信息登记表列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationFormListResponse {
    /// 登记表列表
    #[serde(flatten)]
    pub forms: PageResponse<RegistrationForm>,
}

impl ApiResponseTrait for RegistrationFormListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApplicationConfigService {
    /// 创建候选人配置服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取人才标签信息列表
    ///
    /// 查询系统中可用的人才标签列表，用于对候选人进行分类和标记。
    ///
    /// # Arguments
    ///
    /// * `request` - 查询请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回包含人才标签列表的响应
    pub async fn list_talent_tags(
        &self,
        request: TalentTagListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentTagListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_TALENT_TAGS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(tag_type) = request.tag_type {
            api_req
                .query_params
                .insert("tag_type", format!("{tag_type:?}"));
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
    /// 获取信息登记表列表
    ///
    /// 查询系统中可用的信息登记表模板，用于收集候选人的详细信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 查询请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回包含信息登记表列表的响应
    pub async fn list_registration_forms(
        &self,
        request: RegistrationFormListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<RegistrationFormListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_REGISTRATION_FORMS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(form_type) = request.form_type {
            api_req
                .query_params
                .insert("form_type", format!("{form_type:?}"));
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
}

/// 人才标签查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TalentTagListRequest {
    /// 标签类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_type: Option<TalentTagType>,
    /// 页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 人才标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentTag {
    /// 标签ID
    pub tag_id: String,
    /// 标签名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 标签类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_type: Option<TalentTagType>,
    /// 标签描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否激活
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

/// 人才标签类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TalentTagType {
    /// 技能标签
    Skill,
    /// 经验标签
    Experience,
    /// 教育背景标签
    Education,
    /// 个人特质标签
    Personality,
    /// 自定义标签
    Custom,
}

/// 信息登记表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegistrationFormListRequest {
    /// 登记表类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_type: Option<RegistrationFormType>,
    /// 页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 信息登记表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationForm {
    /// 登记表ID
    pub form_id: String,
    /// 登记表名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 登记表类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_type: Option<RegistrationFormType>,
    /// 表单字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FormField>>,
    /// 是否激活
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// 信息登记表类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistrationFormType {
    /// 候选人基本信息
    BasicInfo,
    /// 工作经历
    WorkExperience,
    /// 教育背景
    Education,
    /// 项目经验
    ProjectExperience,
    /// 技能证书
    Skills,
    /// 自定义表单
    Custom,
}

/// 表单字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    /// 字段ID
    pub field_id: String,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<FormFieldType>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
}

/// 表单字段类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormFieldType {
    /// 文本输入
    Text,
    /// 数字输入
    Number,
    /// 日期选择
    Date,
    /// 单选框
    SingleSelect,
    /// 多选框
    MultiSelect,
    /// 文件上传
    File,
}
