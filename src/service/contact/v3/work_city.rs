#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 工作城市服务
//!
//! 提供完整的工作城市管理功能：
//! - 获取单个工作城市信息
//! - 获取租户工作城市列表
//! - 支持分页查询

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 工作城市信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCity {
    /// 工作城市ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_city_id: Option<String>,
    /// 城市名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 城市编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_code: Option<String>,
    /// 省份名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_name: Option<String>,
    /// 国家名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for WorkCity {
    fn default() -> Self {
        Self {
            work_city_id: None,
            name: None,
            city_code: None,
            province_name: None,
            country_name: None,
            timezone: None,
            order: None,
            enabled: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 工作城市服务
#[derive(Debug, Clone)]
pub struct WorkCityService {
    config: Config,
}

impl WorkCityService {
    /// 创建新的工作城市服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取单个工作城市信息
    ///
    /// 根据工作城市ID获取工作城市的详细信息
    ///
    /// # 参数
    /// * `work_city_id` - 工作城市ID
    ///
    /// # 返回值
    /// 返回工作城市的详细信息
    pub async fn get(&self, work_city_id: &str) -> SDKResult<GetWorkCityResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_WORK_CITY_GET
            .replace("{work_city_id}", work_city_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetWorkCityResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取租户工作城市列表
    ///
    /// 获取企业内所有工作城市的列表，支持分页查询
    ///
    /// # 参数
    /// * `req` - 查询工作城市列表请求
    ///
    /// # 返回值
    /// 返回工作城市列表，支持分页
    pub async fn list(&self, req: &ListWorkCitiesRequest) -> SDKResult<ListWorkCitiesResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_WORK_CITIES.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<ListWorkCitiesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 获取单个工作城市信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetWorkCityResponse {
    /// 工作城市信息
    pub work_city: WorkCity,
}

impl ApiResponseTrait for GetWorkCityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询工作城市列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkCitiesRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for ListWorkCitiesRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }
}

/// 查询工作城市列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListWorkCitiesResponse {
    /// 工作城市列表
    pub items: Vec<WorkCity>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListWorkCitiesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 查询工作城市列表构建器
#[derive(Debug, Clone)]
pub struct ListWorkCitiesBuilder {
    request: ListWorkCitiesRequest,
}

impl Default for ListWorkCitiesBuilder {
    fn default() -> Self {
        Self {
            request: ListWorkCitiesRequest::default(),
        }
    }
}

impl ListWorkCitiesBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 执行查询
    pub async fn execute(self, service: &WorkCityService) -> SDKResult<ListWorkCitiesResponse> {
        service.list(&self.request).await
    }
}

impl WorkCityService {
    /// 创建查询构建器
    pub fn list_work_cities_builder(&self) -> ListWorkCitiesBuilder {
        ListWorkCitiesBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_city_service_creation() {
        let config = Config::default();
        let service = WorkCityService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_work_city_default_creation() {
        let work_city = WorkCity::default();
        assert_eq!(work_city.work_city_id, None);
        assert_eq!(work_city.name, None);
        assert_eq!(work_city.city_code, None);
        assert_eq!(work_city.province_name, None);
        assert_eq!(work_city.country_name, None);
        assert_eq!(work_city.timezone, None);
        assert_eq!(work_city.order, None);
        assert_eq!(work_city.enabled, None);
        assert_eq!(work_city.create_time, None);
        assert_eq!(work_city.update_time, None);
    }

    #[test]
    fn test_work_city_with_data() {
        let work_city = WorkCity {
            work_city_id: Some("city_123".to_string()),
            name: Some("北京".to_string()),
            city_code: Some("110000".to_string()),
            province_name: Some("北京市".to_string()),
            country_name: Some("中国".to_string()),
            timezone: Some("Asia/Shanghai".to_string()),
            order: Some(1),
            enabled: Some(true),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(work_city.work_city_id, Some("city_123".to_string()));
        assert_eq!(work_city.name, Some("北京".to_string()));
        assert_eq!(work_city.city_code, Some("110000".to_string()));
        assert_eq!(work_city.province_name, Some("北京市".to_string()));
        assert_eq!(work_city.country_name, Some("中国".to_string()));
        assert_eq!(work_city.timezone, Some("Asia/Shanghai".to_string()));
        assert_eq!(work_city.order, Some(1));
        assert_eq!(work_city.enabled, Some(true));
        assert_eq!(
            work_city.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            work_city.update_time,
            Some("2023-01-02T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_list_work_cities_request_default() {
        let request = ListWorkCitiesRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_work_cities_request_with_pagination() {
        let request = ListWorkCitiesRequest {
            page_size: Some(50),
            page_token: Some("token123".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_list_work_cities_builder() {
        let builder = ListWorkCitiesBuilder::new()
            .page_size(20)
            .page_token("test_token");

        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("test_token".to_string()));
    }

    #[test]
    fn test_list_work_cities_builder_default() {
        let builder = ListWorkCitiesBuilder::default();
        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
    }

    #[test]
    fn test_response_default_creation() {
        let get_response = GetWorkCityResponse::default();
        assert_eq!(get_response.work_city.work_city_id, None);

        let list_response = ListWorkCitiesResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut get_response = GetWorkCityResponse::default();
        get_response.work_city = WorkCity {
            work_city_id: Some("city_456".to_string()),
            name: Some("上海".to_string()),
            ..Default::default()
        };

        assert_eq!(
            get_response.work_city.work_city_id,
            Some("city_456".to_string())
        );
        assert_eq!(get_response.work_city.name, Some("上海".to_string()));

        let mut list_response = ListWorkCitiesResponse::default();
        list_response.items = vec![WorkCity {
            work_city_id: Some("city_789".to_string()),
            name: Some("深圳".to_string()),
            ..Default::default()
        }];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page".to_string());

        assert_eq!(list_response.items.len(), 1);
        assert_eq!(
            list_response.items[0].work_city_id,
            Some("city_789".to_string())
        );
        assert_eq!(list_response.items[0].name, Some("深圳".to_string()));
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(list_response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(GetWorkCityResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListWorkCitiesResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = ListWorkCitiesRequest {
            page_size: Some(10),
            page_token: Some("token".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ListWorkCitiesRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.page_token, deserialized.page_token);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListWorkCitiesRequest {
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        assert_eq!(query_params.len(), 2);
        assert!(query_params.contains(&"page_size=20".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_WORK_CITIES,
            "/open-apis/contact/v3/work_cities"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_WORK_CITY_GET,
            "/open-apis/contact/v3/work_cities/{work_city_id}"
        );
    }

    #[test]
    fn test_work_city_timezone_handling() {
        // Test different timezone formats
        let utc_city = WorkCity {
            work_city_id: Some("city_utc".to_string()),
            name: Some("UTC城市".to_string()),
            timezone: Some("UTC".to_string()),
            ..Default::default()
        };

        let shanghai_city = WorkCity {
            work_city_id: Some("city_shanghai".to_string()),
            name: Some("上海".to_string()),
            timezone: Some("Asia/Shanghai".to_string()),
            ..Default::default()
        };

        let new_york_city = WorkCity {
            work_city_id: Some("city_ny".to_string()),
            name: Some("纽约".to_string()),
            timezone: Some("America/New_York".to_string()),
            ..Default::default()
        };

        assert_eq!(utc_city.timezone, Some("UTC".to_string()));
        assert_eq!(shanghai_city.timezone, Some("Asia/Shanghai".to_string()));
        assert_eq!(new_york_city.timezone, Some("America/New_York".to_string()));
    }

    #[test]
    fn test_work_city_order_and_status() {
        // Test city ordering and enabled status
        let city1 = WorkCity {
            work_city_id: Some("city_1".to_string()),
            name: Some("一线城市".to_string()),
            order: Some(1),
            enabled: Some(true),
            ..Default::default()
        };

        let city2 = WorkCity {
            work_city_id: Some("city_2".to_string()),
            name: Some("二线城市".to_string()),
            order: Some(2),
            enabled: Some(false),
            ..Default::default()
        };

        assert_eq!(city1.order, Some(1));
        assert_eq!(city1.enabled, Some(true));
        assert_eq!(city2.order, Some(2));
        assert_eq!(city2.enabled, Some(false));
    }

    #[test]
    fn test_comprehensive_work_city_data() {
        // Test comprehensive work city data with all fields
        let comprehensive_city = WorkCity {
            work_city_id: Some("comprehensive_001".to_string()),
            name: Some("广州市".to_string()),
            city_code: Some("440100".to_string()),
            province_name: Some("广东省".to_string()),
            country_name: Some("中华人民共和国".to_string()),
            timezone: Some("Asia/Shanghai".to_string()),
            order: Some(3),
            enabled: Some(true),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
        };

        assert_eq!(
            comprehensive_city.work_city_id,
            Some("comprehensive_001".to_string())
        );
        assert_eq!(comprehensive_city.name, Some("广州市".to_string()));
        assert_eq!(comprehensive_city.city_code, Some("440100".to_string()));
        assert_eq!(comprehensive_city.province_name, Some("广东省".to_string()));
        assert_eq!(
            comprehensive_city.country_name,
            Some("中华人民共和国".to_string())
        );
        assert_eq!(
            comprehensive_city.timezone,
            Some("Asia/Shanghai".to_string())
        );
        assert_eq!(comprehensive_city.order, Some(3));
        assert_eq!(comprehensive_city.enabled, Some(true));
        assert_eq!(
            comprehensive_city.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_city.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
    }
}
