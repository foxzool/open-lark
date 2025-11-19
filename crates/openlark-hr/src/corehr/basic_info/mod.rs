use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api::ApiRequest,
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::corehr::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::corehr::models::{
        CountryRegion, CountryRegionSearchRequest, EnumInfo, EnumSearchRequest, IdConvertRequest,
        IdConvertResult, Nationality, NationalitySearchRequest, PageResponse,
    },
};

/// 基础数据服务
pub struct BasicInfoService {
    pub config: Config,
}

/// 查询枚举信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EnumSearchResponse {
    /// 枚举信息列表
    #[serde(flatten)]
    pub enums: PageResponse<EnumInfo>,
}

impl ApiResponseTrait for EnumSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询国家/地区信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryRegionSearchResponse {
    /// 国家/地区信息列表
    #[serde(flatten)]
    pub country_regions: PageResponse<CountryRegion>,
}

impl ApiResponseTrait for CountryRegionSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询国籍信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct NationalitySearchResponse {
    /// 国籍信息列表
    #[serde(flatten)]
    pub nationalities: PageResponse<Nationality>,
}

impl ApiResponseTrait for NationalitySearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// ID转换响应
#[derive(Debug, Serialize, Deserialize)]
pub struct IdConvertResponse {
    /// 转换结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<IdConvertResult>>,
}

impl ApiResponseTrait for IdConvertResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BasicInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询枚举信息
    ///
    /// 该接口用于查询系统中的枚举信息，支持按枚举类型查询。
    /// 枚举信息包括性别、婚姻状况、员工状态等系统预定义的选项值。
    ///
    /// # 参数
    ///
    /// - `request`: 枚举查询请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的枚举信息列表，包括：
    /// - 枚举值和名称
    /// - 多语言支持
    /// - 枚举状态
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::EnumSearchRequest;
    ///
    /// let request = EnumSearchRequest {
    ///     enum_type: "gender".to_string(),
    ///     page_size: Some(50),
    ///     page_token: None,
    /// };
    ///
    /// let response = client.corehr.basic_info.search_enum(request, None).await?;
    /// ```
    pub async fn search_enum(
        &self,
        request: EnumSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<EnumSearchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_BASIC_INFO_ENUM_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询国家/地区信息
    ///
    /// 该接口用于查询系统中的国家/地区基础信息，包括国家代码、
    /// 时区等信息，用于地址填写等场景。
    ///
    /// # 参数
    ///
    /// - `request`: 国家/地区查询请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的国家/地区信息列表，包括：
    /// - 国家/地区名称（多语言）
    /// - 国家代码
    /// - 时区信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::CountryRegionSearchRequest;
    ///
    /// let request = CountryRegionSearchRequest {
    ///     page_size: Some(100),
    ///     page_token: None,
    /// };
    ///
    /// let response = client.corehr.basic_info.search_country_region(request, None).await?;
    /// ```
    pub async fn search_country_region(
        &self,
        request: CountryRegionSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CountryRegionSearchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_BASIC_INFO_LOCATION_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询国籍信息
    ///
    /// 该接口用于查询系统中的国籍基础信息，包括国籍代码、
    /// 国籍名称等信息，用于员工个人信息填写。
    ///
    /// # 参数
    ///
    /// - `request`: 国籍查询请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的国籍信息列表，包括：
    /// - 国籍名称（多语言）
    /// - 国籍代码
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::NationalitySearchRequest;
    ///
    /// let request = NationalitySearchRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    /// };
    ///
    /// let response = client.corehr.basic_info.search_nationality(request, None).await?;
    /// ```
    pub async fn search_nationality(
        &self,
        request: NationalitySearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<NationalitySearchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_BASIC_INFO_NATIONALITY_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// ID转换
    ///
    /// 该接口用于在不同ID类型之间进行转换，例如将person_id转换为
    /// employee_id，或将user_id转换为employee_id等。
    ///
    /// # 参数
    ///
    /// - `request`: ID转换请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回ID转换结果列表，包括：
    /// - 源ID和目标ID的对应关系
    /// - 转换成功的ID列表
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::IdConvertRequest;
    ///
    /// let request = IdConvertRequest {
    ///     source_id_type: "person_id".to_string(),
    ///     target_id_type: "employee_id".to_string(),
    ///     ids: vec!["person_123".to_string(), "person_456".to_string()],
    /// };
    ///
    /// let response = client.corehr.basic_info.convert_id(request, None).await?;
    /// ```
    pub async fn convert_id(
        &self,
        request: IdConvertRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<IdConvertResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_COMMON_DATA_ID_CONVERT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }
}
