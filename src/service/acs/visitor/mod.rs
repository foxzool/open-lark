use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{acs::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::acs::models::Visitor,
};

/// 访客管理服务
#[derive(Debug)]
pub struct VisitorService {
    pub config: Config,
}

impl VisitorService {
    /// 创建访客管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 添加访客
    ///
    /// 添加新的访客记录，包括访客基本信息和访问权限。
    ///
    /// # Arguments
    ///
    /// * `request` - 访客添加请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回添加的访客信息
    pub async fn create_visitor(
        &self,
        request: VisitorCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<VisitorCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: ACS_V1_VISITORS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除访客
    ///
    /// 删除指定的访客记录，撤销访客的门禁权限。
    ///
    /// # Arguments
    ///
    /// * `visitor_id` - 访客ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回删除结果
    pub async fn delete_visitor(
        &self,
        visitor_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<VisitorDeleteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(ACS_V1_VISITOR_GET, "visitor_id", visitor_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 访客添加请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorCreateRequest {
    /// 访客姓名
    pub name: String,
    /// 访客电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 访客公司
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 访问目的
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// 接待人员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_user_id: Option<String>,
    /// 访问开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 访问结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 允许访问的设备ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
}

/// 访客添加响应
#[derive(Debug, Serialize, Deserialize)]
pub struct VisitorCreateResponse {
    /// 添加的访客信息
    pub visitor: Visitor,
}

impl ApiResponseTrait for VisitorCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 访客删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct VisitorDeleteResponse {
    /// 删除成功标识
    pub success: bool,
}

impl ApiResponseTrait for VisitorDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use crate::core::constants::AppType;
    use serde_json;

    #[test]
    fn test_visitor_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = VisitorService::new(config.clone());

        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
    }

    #[test]
    fn test_visitor_service_debug() {
        let config = Config::builder()
            .app_id("app_id")
            .app_secret("app_secret")
            .build();
        let service = VisitorService::new(config);
        let debug_str = format!("{:?}", service);

        assert!(debug_str.contains("VisitorService"));
    }

    #[test]
    fn test_visitor_create_request_full() {
        let request = VisitorCreateRequest {
            name: "张三".to_string(),
            phone: Some("13800138000".to_string()),
            company: Some("阿里巴巴".to_string()),
            purpose: Some("技术交流".to_string()),
            host_user_id: Some("user123".to_string()),
            start_time: Some(1640995200),
            end_time: Some(1641081600),
            device_ids: Some(vec!["device1".to_string(), "device2".to_string()]),
        };

        assert_eq!(request.name, "张三");
        assert_eq!(request.phone, Some("13800138000".to_string()));
        assert_eq!(request.company, Some("阿里巴巴".to_string()));
        assert_eq!(request.purpose, Some("技术交流".to_string()));
        assert_eq!(request.host_user_id, Some("user123".to_string()));
        assert_eq!(request.start_time, Some(1640995200));
        assert_eq!(request.end_time, Some(1641081600));
        assert_eq!(
            request.device_ids,
            Some(vec!["device1".to_string(), "device2".to_string()])
        );
    }

    #[test]
    fn test_visitor_create_request_minimal() {
        let request = VisitorCreateRequest {
            name: "李四".to_string(),
            phone: None,
            company: None,
            purpose: None,
            host_user_id: None,
            start_time: None,
            end_time: None,
            device_ids: None,
        };

        assert_eq!(request.name, "李四");
        assert!(request.phone.is_none());
        assert!(request.company.is_none());
        assert!(request.purpose.is_none());
        assert!(request.host_user_id.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.device_ids.is_none());
    }

    #[test]
    fn test_visitor_create_request_clone() {
        let request = VisitorCreateRequest {
            name: "王五".to_string(),
            phone: Some("13900139000".to_string()),
            company: Some("腾讯".to_string()),
            purpose: Some("商务洽谈".to_string()),
            host_user_id: Some("host456".to_string()),
            start_time: Some(1641000000),
            end_time: Some(1641100000),
            device_ids: Some(vec!["device3".to_string()]),
        };

        let cloned = request.clone();

        assert_eq!(request.name, cloned.name);
        assert_eq!(request.phone, cloned.phone);
        assert_eq!(request.company, cloned.company);
        assert_eq!(request.purpose, cloned.purpose);
        assert_eq!(request.host_user_id, cloned.host_user_id);
        assert_eq!(request.start_time, cloned.start_time);
        assert_eq!(request.end_time, cloned.end_time);
        assert_eq!(request.device_ids, cloned.device_ids);
    }

    #[test]
    fn test_visitor_create_request_debug() {
        let request = VisitorCreateRequest {
            name: "赵六".to_string(),
            phone: Some("13700137000".to_string()),
            company: Some("百度".to_string()),
            purpose: Some("技术分享".to_string()),
            host_user_id: Some("host789".to_string()),
            start_time: Some(1641200000),
            end_time: Some(1641300000),
            device_ids: Some(vec!["device4".to_string(), "device5".to_string()]),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("VisitorCreateRequest"));
        assert!(debug_str.contains("赵六"));
        assert!(debug_str.contains("13700137000"));
        assert!(debug_str.contains("百度"));
    }

    #[test]
    fn test_visitor_create_request_serialization_full() {
        let request = VisitorCreateRequest {
            name: "测试用户".to_string(),
            phone: Some("15800158000".to_string()),
            company: Some("测试公司".to_string()),
            purpose: Some("功能测试".to_string()),
            host_user_id: Some("host_test".to_string()),
            start_time: Some(1640000000),
            end_time: Some(1640086400),
            device_ids: Some(vec!["test_device1".to_string(), "test_device2".to_string()]),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("测试用户"));
        assert!(serialized.contains("15800158000"));
        assert!(serialized.contains("测试公司"));
        assert!(serialized.contains("功能测试"));
        assert!(serialized.contains("host_test"));
        assert!(serialized.contains("1640000000"));
        assert!(serialized.contains("1640086400"));
        assert!(serialized.contains("test_device1"));
        assert!(serialized.contains("test_device2"));
    }

    #[test]
    fn test_visitor_create_request_serialization_minimal() {
        let request = VisitorCreateRequest {
            name: "最小用户".to_string(),
            phone: None,
            company: None,
            purpose: None,
            host_user_id: None,
            start_time: None,
            end_time: None,
            device_ids: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("最小用户"));
        // Optional fields should not appear in serialization when None
        assert!(!serialized.contains("phone"));
        assert!(!serialized.contains("company"));
        assert!(!serialized.contains("purpose"));
        assert!(!serialized.contains("host_user_id"));
        assert!(!serialized.contains("start_time"));
        assert!(!serialized.contains("end_time"));
        assert!(!serialized.contains("device_ids"));
    }

    #[test]
    fn test_visitor_create_request_deserialization() {
        let json = r#"{
            "name": "反序列化用户",
            "phone": "18800188000",
            "company": "反序列化公司",
            "purpose": "反序列化测试",
            "host_user_id": "host_deser",
            "start_time": 1650000000,
            "end_time": 1650086400,
            "device_ids": ["deser_device1", "deser_device2"]
        }"#;

        let request: VisitorCreateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.name, "反序列化用户");
        assert_eq!(request.phone, Some("18800188000".to_string()));
        assert_eq!(request.company, Some("反序列化公司".to_string()));
        assert_eq!(request.purpose, Some("反序列化测试".to_string()));
        assert_eq!(request.host_user_id, Some("host_deser".to_string()));
        assert_eq!(request.start_time, Some(1650000000));
        assert_eq!(request.end_time, Some(1650086400));
        assert_eq!(
            request.device_ids,
            Some(vec![
                "deser_device1".to_string(),
                "deser_device2".to_string()
            ])
        );
    }

    #[test]
    fn test_visitor_create_request_deserialization_minimal() {
        let json = r#"{"name": "仅名称用户"}"#;

        let request: VisitorCreateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.name, "仅名称用户");
        assert_eq!(request.phone, None);
        assert_eq!(request.company, None);
        assert_eq!(request.purpose, None);
        assert_eq!(request.host_user_id, None);
        assert_eq!(request.start_time, None);
        assert_eq!(request.end_time, None);
        assert_eq!(request.device_ids, None);
    }

    #[test]
    fn test_visitor_create_request_with_empty_strings() {
        let request = VisitorCreateRequest {
            name: "".to_string(),
            phone: Some("".to_string()),
            company: Some("".to_string()),
            purpose: Some("".to_string()),
            host_user_id: Some("".to_string()),
            start_time: Some(0),
            end_time: Some(0),
            device_ids: Some(vec![]),
        };

        assert_eq!(request.name, "");
        assert_eq!(request.phone, Some("".to_string()));
        assert_eq!(request.company, Some("".to_string()));
        assert_eq!(request.purpose, Some("".to_string()));
        assert_eq!(request.host_user_id, Some("".to_string()));
        assert_eq!(request.start_time, Some(0));
        assert_eq!(request.end_time, Some(0));
        assert_eq!(request.device_ids, Some(vec![]));
    }

    #[test]
    fn test_visitor_create_request_with_special_characters() {
        let request = VisitorCreateRequest {
            name: "用户@#$%^&*()".to_string(),
            phone: Some("+86-138-0013-8000".to_string()),
            company: Some("公司名称!@#$%".to_string()),
            purpose: Some("目的包含特殊字符: <>?\"'\\".to_string()),
            host_user_id: Some("host_user@domain.com".to_string()),
            start_time: Some(-1),
            end_time: Some(i64::MAX),
            device_ids: Some(vec![
                "device_id@domain".to_string(),
                "device-with-dash".to_string(),
            ]),
        };

        assert_eq!(request.name, "用户@#$%^&*()");
        assert_eq!(request.phone, Some("+86-138-0013-8000".to_string()));
        assert_eq!(request.company, Some("公司名称!@#$%".to_string()));
        assert_eq!(
            request.purpose,
            Some("目的包含特殊字符: <>?\"'\\".to_string())
        );
        assert_eq!(
            request.host_user_id,
            Some("host_user@domain.com".to_string())
        );
        assert_eq!(request.start_time, Some(-1));
        assert_eq!(request.end_time, Some(i64::MAX));
        assert_eq!(
            request.device_ids,
            Some(vec![
                "device_id@domain".to_string(),
                "device-with-dash".to_string()
            ])
        );
    }

    #[test]
    fn test_visitor_create_request_with_long_strings() {
        let long_name = "很".repeat(100);
        let long_phone = "1".repeat(50);
        let long_company = "公司".repeat(200);
        let long_purpose = "目的".repeat(300);
        let long_host_id = "host".repeat(50);
        let long_device_ids = vec!["device".repeat(100); 10];

        let request = VisitorCreateRequest {
            name: long_name.clone(),
            phone: Some(long_phone.clone()),
            company: Some(long_company.clone()),
            purpose: Some(long_purpose.clone()),
            host_user_id: Some(long_host_id.clone()),
            start_time: Some(1640995200),
            end_time: Some(1641081600),
            device_ids: Some(long_device_ids.clone()),
        };

        assert_eq!(request.name, long_name);
        assert_eq!(request.phone, Some(long_phone));
        assert_eq!(request.company, Some(long_company));
        assert_eq!(request.purpose, Some(long_purpose));
        assert_eq!(request.host_user_id, Some(long_host_id));
        assert_eq!(request.device_ids, Some(long_device_ids));
    }

    #[test]
    fn test_visitor_create_request_with_many_devices() {
        let many_devices: Vec<String> = (0..1000).map(|i| format!("device_{}", i)).collect();
        let request = VisitorCreateRequest {
            name: "多设备用户".to_string(),
            phone: Some("13000130000".to_string()),
            company: Some("测试公司".to_string()),
            purpose: Some("多设备测试".to_string()),
            host_user_id: Some("host_many".to_string()),
            start_time: Some(1640995200),
            end_time: Some(1641081600),
            device_ids: Some(many_devices.clone()),
        };

        assert_eq!(request.device_ids, Some(many_devices));
        assert_eq!(request.device_ids.as_ref().unwrap().len(), 1000);
    }

    #[test]
    fn test_visitor_create_request_time_boundaries() {
        let request = VisitorCreateRequest {
            name: "时间边界测试".to_string(),
            phone: None,
            company: None,
            purpose: None,
            host_user_id: None,
            start_time: Some(i64::MIN),
            end_time: Some(i64::MAX),
            device_ids: None,
        };

        assert_eq!(request.start_time, Some(i64::MIN));
        assert_eq!(request.end_time, Some(i64::MAX));
    }

    #[test]
    fn test_visitor_create_response_debug() {
        use crate::service::acs::models::{Visitor, VisitorStatus};

        let visitor = Visitor {
            visitor_id: "visitor_123".to_string(),
            name: "测试访客".to_string(),
            phone: Some("13800138000".to_string()),
            company: Some("测试公司".to_string()),
            purpose: Some("测试目的".to_string()),
            host_user_id: Some("host_123".to_string()),
            host_name: Some("主机名".to_string()),
            status: Some(VisitorStatus::Active),
            start_time: Some(1640995200),
            end_time: Some(1641081600),
            created_at: Some(1640000000),
        };

        let response = VisitorCreateResponse { visitor };
        let debug_str = format!("{:?}", response);

        assert!(debug_str.contains("VisitorCreateResponse"));
        assert!(debug_str.contains("visitor_123"));
        assert!(debug_str.contains("测试访客"));
    }

    #[test]
    fn test_visitor_create_response_serialization() {
        use crate::service::acs::models::{Visitor, VisitorStatus};

        let visitor = Visitor {
            visitor_id: "ser_visitor_123".to_string(),
            name: "序列化访客".to_string(),
            phone: Some("15800158000".to_string()),
            company: Some("序列化公司".to_string()),
            purpose: Some("序列化测试".to_string()),
            host_user_id: Some("host_ser".to_string()),
            host_name: Some("主机序列化".to_string()),
            status: Some(VisitorStatus::Active),
            start_time: Some(1650000000),
            end_time: Some(1650086400),
            created_at: Some(1649000000),
        };

        let response = VisitorCreateResponse { visitor };
        let serialized = serde_json::to_string(&response).unwrap();

        assert!(serialized.contains("ser_visitor_123"));
        assert!(serialized.contains("序列化访客"));
        assert!(serialized.contains("15800158000"));
        assert!(serialized.contains("序列化公司"));
    }

    #[test]
    fn test_visitor_create_response_deserialization() {
        let json = r#"{
            "visitor": {
                "visitor_id": "deser_visitor_123",
                "name": "反序列化访客",
                "phone": "18800188000",
                "company": "反序列化公司",
                "purpose": "反序列化测试",
                "host_user_id": "host_deser",
                "host_name": "主机反序列化",
                "status": "active",
                "start_time": 1660000000,
                "end_time": 1660086400,
                "created_at": 1659000000
            }
        }"#;

        let response: VisitorCreateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.visitor.visitor_id, "deser_visitor_123");
        assert_eq!(response.visitor.name, "反序列化访客");
        assert_eq!(response.visitor.phone, Some("18800188000".to_string()));
        assert_eq!(response.visitor.company, Some("反序列化公司".to_string()));
    }

    #[test]
    fn test_visitor_create_response_data_format() {
        let format = VisitorCreateResponse::data_format();
        assert!(matches!(format, ResponseFormat::Data));
    }

    #[test]
    fn test_visitor_delete_response_debug() {
        let response = VisitorDeleteResponse { success: true };
        let debug_str = format!("{:?}", response);

        assert!(debug_str.contains("VisitorDeleteResponse"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_visitor_delete_response_serialization() {
        let response = VisitorDeleteResponse { success: true };
        let serialized = serde_json::to_string(&response).unwrap();

        assert!(serialized.contains("success"));
        assert!(serialized.contains("true"));

        let response_false = VisitorDeleteResponse { success: false };
        let serialized_false = serde_json::to_string(&response_false).unwrap();

        assert!(serialized_false.contains("success"));
        assert!(serialized_false.contains("false"));
    }

    #[test]
    fn test_visitor_delete_response_deserialization() {
        let json_true = r#"{"success": true}"#;
        let response_true: VisitorDeleteResponse = serde_json::from_str(json_true).unwrap();
        assert!(response_true.success);

        let json_false = r#"{"success": false}"#;
        let response_false: VisitorDeleteResponse = serde_json::from_str(json_false).unwrap();
        assert!(!response_false.success);
    }

    #[test]
    fn test_visitor_delete_response_data_format() {
        let format = VisitorDeleteResponse::data_format();
        assert!(matches!(format, ResponseFormat::Data));
    }

    #[test]
    fn test_memory_efficiency() {
        let config = Config::builder().app_id("test").app_secret("test").build();
        let service = VisitorService::new(config);
        let size = std::mem::size_of_val(&service);

        assert!(size > 0);
        assert!(size < 2048); // Reasonable size limit

        let request = VisitorCreateRequest {
            name: "memory_test".to_string(),
            phone: None,
            company: None,
            purpose: None,
            host_user_id: None,
            start_time: None,
            end_time: None,
            device_ids: None,
        };
        let request_size = std::mem::size_of_val(&request);
        assert!(request_size > 0);
        assert!(request_size < 1024);
    }

    #[test]
    fn test_visitor_create_request_with_unicode_names() {
        let request = VisitorCreateRequest {
            name: "张三李四王五赵六钱七".to_string(),
            phone: Some("📞13800138000".to_string()),
            company: Some("🏢阿里巴巴集团控股有限公司".to_string()),
            purpose: Some("🎯技术交流与合作洽谈".to_string()),
            host_user_id: Some("👤host_张三".to_string()),
            start_time: Some(1640995200),
            end_time: Some(1641081600),
            device_ids: Some(vec![
                "🔒device_一号".to_string(),
                "🔐device_二号".to_string(),
            ]),
        };

        assert_eq!(request.name, "张三李四王五赵六钱七");
        assert_eq!(request.phone, Some("📞13800138000".to_string()));
        assert_eq!(
            request.company,
            Some("🏢阿里巴巴集团控股有限公司".to_string())
        );
        assert_eq!(request.purpose, Some("🎯技术交流与合作洽谈".to_string()));
        assert_eq!(request.host_user_id, Some("👤host_张三".to_string()));
        assert_eq!(
            request.device_ids,
            Some(vec![
                "🔒device_一号".to_string(),
                "🔐device_二号".to_string()
            ])
        );
    }

    #[test]
    fn test_visitor_service_with_different_app_types() {
        let config_self_built = Config::builder()
            .app_id("app1")
            .app_secret("secret1")
            .app_type(AppType::SelfBuild)
            .build();
        let service_self_built = VisitorService::new(config_self_built);

        let config_marketplace = Config::builder()
            .app_id("app2")
            .app_secret("secret2")
            .app_type(AppType::Marketplace)
            .build();
        let service_marketplace = VisitorService::new(config_marketplace);

        // Both services should be created successfully
        assert_eq!(service_self_built.config.app_id, "app1");
        assert_eq!(service_marketplace.config.app_id, "app2");
    }

    #[test]
    fn test_visitor_create_request_field_independence() {
        let mut request = VisitorCreateRequest {
            name: "原始名称".to_string(),
            phone: Some("原始电话".to_string()),
            company: Some("原始公司".to_string()),
            purpose: Some("原始目的".to_string()),
            host_user_id: Some("原始主机".to_string()),
            start_time: Some(1000),
            end_time: Some(2000),
            device_ids: Some(vec!["原始设备".to_string()]),
        };

        // Modify individual fields
        request.name = "新名称".to_string();
        request.phone = Some("新电话".to_string());
        request.company = Some("新公司".to_string());
        request.purpose = Some("新目的".to_string());
        request.host_user_id = Some("新主机".to_string());
        request.start_time = Some(3000);
        request.end_time = Some(4000);
        request.device_ids = Some(vec!["新设备".to_string()]);

        assert_eq!(request.name, "新名称");
        assert_eq!(request.phone, Some("新电话".to_string()));
        assert_eq!(request.company, Some("新公司".to_string()));
        assert_eq!(request.purpose, Some("新目的".to_string()));
        assert_eq!(request.host_user_id, Some("新主机".to_string()));
        assert_eq!(request.start_time, Some(3000));
        assert_eq!(request.end_time, Some(4000));
        assert_eq!(request.device_ids, Some(vec!["新设备".to_string()]));
    }

    #[test]
    fn test_visitor_create_request_option_none_to_some() {
        let mut request = VisitorCreateRequest {
            name: "测试用户".to_string(),
            phone: None,
            company: None,
            purpose: None,
            host_user_id: None,
            start_time: None,
            end_time: None,
            device_ids: None,
        };

        // Change None to Some
        request.phone = Some("新增电话".to_string());
        request.company = Some("新增公司".to_string());
        request.purpose = Some("新增目的".to_string());
        request.host_user_id = Some("新增主机".to_string());
        request.start_time = Some(5000);
        request.end_time = Some(6000);
        request.device_ids = Some(vec!["新增设备".to_string()]);

        assert_eq!(request.phone, Some("新增电话".to_string()));
        assert_eq!(request.company, Some("新增公司".to_string()));
        assert_eq!(request.purpose, Some("新增目的".to_string()));
        assert_eq!(request.host_user_id, Some("新增主机".to_string()));
        assert_eq!(request.start_time, Some(5000));
        assert_eq!(request.end_time, Some(6000));
        assert_eq!(request.device_ids, Some(vec!["新增设备".to_string()]));
    }

    #[test]
    fn test_visitor_create_request_option_some_to_none() {
        let mut request = VisitorCreateRequest {
            name: "测试用户".to_string(),
            phone: Some("原有电话".to_string()),
            company: Some("原有公司".to_string()),
            purpose: Some("原有目的".to_string()),
            host_user_id: Some("原有主机".to_string()),
            start_time: Some(7000),
            end_time: Some(8000),
            device_ids: Some(vec!["原有设备".to_string()]),
        };

        // Change Some to None
        request.phone = None;
        request.company = None;
        request.purpose = None;
        request.host_user_id = None;
        request.start_time = None;
        request.end_time = None;
        request.device_ids = None;

        assert!(request.phone.is_none());
        assert!(request.company.is_none());
        assert!(request.purpose.is_none());
        assert!(request.host_user_id.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.device_ids.is_none());
    }
}
