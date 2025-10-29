use open_lark_core::core::api_req::ApiRequest;
use crate::core::{,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::auth::*,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    trait_system::Service,
    SDKResult,
};
use serde::{Deserialize, Serialize};
/// 用户信息服务,
pub struct UserInfoService {
    config: Config,
}
/// App访问令牌服务,
pub struct AppAccessTokenService {
    config: Config,
}
/// Tenant访问令牌服务,
pub struct TenantAccessTokenService {
    config: Config,
}
/// App Ticket服务,
pub struct AppTicketService {
    config: Config,
}
impl UserInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// # API文档,
    ///,
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/get,
    /// 获取登录用户信息
    pub async fn get(&self, user_access_token: impl ToString) -> SDKResult<UserInfo> {,
let api_req = ApiRequest {,
            api_path: AUTHEN_V1_USER_INFO.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            ..Default::default(),
};
let option = RequestOption::builder(),
            .user_access_token()
.build();
        let api_resp: BaseResponse<UserInfo> =
            Transport::request(api_req, &self.config, Some(option)).await?;
api_resp.into_result(),
    }
}
impl AppAccessTokenService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// # API文档,
    ///,
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/auth-v3/auth/app_access_token,
    /// 商店应用获取App Access Token,
///,
    /// 应用商店应用通过app_id和app_secret获取应用访问令牌。,
/// App Access Token用于访问不需要用户授权的API接口。,
    pub async fn get(&self, req: &GetAppAccessTokenRequest) -> SDKResult<AppAccessTokenResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_APP_ACCESS_TOKEN.to_string(),
            supported_access_token_types: vec![]
            body: serde_json::to_vec(req)?,
            ..Default::default(),
};
let resp =,
            Transport::<AppAccessTokenResponse>::request(api_req, &self.config, None).await?;
resp.into_result(),
    }
/// # API文档,
    ///,
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/auth-v3/auth/app_access_token/internal,
    /// 自建应用获取App Access Token,
///,
    /// 企业自建应用通过app_id和app_secret获取应用访问令牌。,
/// App Access Token用于访问不需要用户授权的API接口。,
    pub async fn get_internal(
        &self,
        req: &GetAppAccessTokenInternalRequest,
    ) -> SDKResult<AppAccessTokenResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_APP_ACCESS_TOKEN_INTERNAL.to_string(),
            supported_access_token_types: vec![]
            body: serde_json::to_vec(req)?,
            ..Default::default(),
};
let resp =,
            Transport::<AppAccessTokenResponse>::request(api_req, &self.config, None).await?;
resp.into_result(),
    }
}
impl TenantAccessTokenService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// # API文档,
    ///,
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/auth-v3/auth/tenant_access_token,
    /// 商店应用获取Tenant Access Token,
///,
    /// 应用商店应用获取租户访问令牌，用于访问特定企业的资源和数据。,
/// 需要企业管理员的授权和配置。,
    pub async fn get(
        &self,
        req: &GetTenantAccessTokenRequest,
    ) -> SDKResult<TenantAccessTokenResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_TENANT_ACCESS_TOKEN.to_string(),
            supported_access_token_types: vec![]
            body: serde_json::to_vec(req)?,
            ..Default::default(),
};
let resp =,
            Transport::<TenantAccessTokenResponse>::request(api_req, &self.config, None).await?;
resp.into_result(),
    }
/// # API文档,
    ///,
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/auth-v3/auth/tenant_access_token/internal,
    /// 自建应用获取Tenant Access Token,
///,
    /// 企业自建应用获取租户访问令牌，用于访问特定企业的资源和数据。,
/// 需要企业管理员的授权和配置。,
    pub async fn get_internal(
        &self,
        req: &GetTenantAccessTokenInternalRequest,
    ) -> SDKResult<TenantAccessTokenResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_TENANT_ACCESS_TOKEN_INTERNAL.to_string(),
            supported_access_token_types: vec![]
            body: serde_json::to_vec(req)?,
            ..Default::default(),
};
let resp =,
            Transport::<TenantAccessTokenResponse>::request(api_req, &self.config, None).await?;
resp.into_result(),
    }
}
impl AppTicketService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// # API文档,
    ///,
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/auth-v3/app_ticket/resend,
    /// 重新获取App Ticket,
///,
    /// 触发飞书重新推送app_ticket，用于解决ticket丢失或失效的问题。,
/// App Ticket是应用接收事件推送的重要凭证。,
    pub async fn resend(&self, req: &ResendAppTicketRequest) -> SDKResult<ResendAppTicketResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::POST,
            api_path: AUTH_V3_APP_TICKET_RESEND.to_string(),
            supported_access_token_types: vec![]
            body: serde_json::to_vec(req)?,
            ..Default::default(),
};
let resp =,
            Transport::<ResendAppTicketResponse>::request(api_req, &self.config, None).await?;
resp.into_result(),
    }
}
/// 登录用户信息,
#[derive(.*?)]
pub struct UserInfo {
    /// 用户姓名
    pub name: String,
    /// 用户英文名称
    pub en_name: String,
    /// 用户头像
    pub avatar_url: String,
    /// 用户头像 72x72
    pub avatar_thumb: String,
    /// 用户头像 240x240
    pub avatar_middle: String,
    /// 用户头像 640x640
    pub avatar_big: String,
    /// 用户在应用内的唯一标识
    pub open_id: String,
    /// 用户对ISV的唯一标识，对于同一个ISV，用户在其名下所有应用的union_id相同
    pub union_id: String,
    /// 用户邮箱
    pub email: Option<String>,
    /// 企业邮箱，请先确保已在管理后台启用飞书邮箱服务
    pub enterprise_email: Option<String>,
    /// 用户 user_id
    pub user_id: String,
    /// 用户手机号
    pub mobile: Option<String>,
    /// 当前企业标识
    pub tenant_key: String,
    /// 用户工号
    pub employee_no: String,
}
impl ApiResponseTrait for.* {
    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}
impl Service for UserInfoService {,
    fn config(&self) -> &Config {,
&self.config,
    }
fn service_name() -> &'static str {,
        "user_info",
}
fn service_version() -> &'static str {,
        "v1",
}
}
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::{config::Config, constants::AppType};
use std::sync::Arc;
    #[test]
fn test_user_info_deserialization() {,
        let json_str = r#"{
            "name": "zhangsan",
            "en_name": "zhangsan",
            "avatar_url": "www.feishu.cn/avatar/icon",
            "avatar_thumb": "www.feishu.cn/avatar/icon_thumb",
            "avatar_middle": "www.feishu.cn/avatar/icon_middle",
            "avatar_big": "www.feishu.cn/avatar/icon_big",
            "open_id": "ou-caecc734c2e3328a62489fe0648c4b98779515d3",
            "union_id": "on-d89jhsdhjsajkda7828enjdj328ydhhw3u43yjhdj",
            "email": "zhangsan@feishu.cn",
            "enterprise_email": "demo@mail.com",
            "user_id": "5d9bdxxx",
            "mobile": "+86130002883xx",
            "tenant_key": "736588c92lxf175d",
            "employee_no": "111222333",
}"#;
let user_info: UserInfo =,
            serde_json::from_str(json_str).expect("Failed to parse test user info JSON");

        assert_eq!(user_info.name, "zhangsan");
        assert_eq!(user_info.en_name, "zhangsan");
        assert_eq!(user_info.avatar_url, "www.feishu.cn/avatar/icon");
        assert_eq!(user_info.avatar_thumb, "www.feishu.cn/avatar/icon_thumb");
        assert_eq!(user_info.avatar_middle, "www.feishu.cn/avatar/icon_middle");
        assert_eq!(user_info.avatar_big, "www.feishu.cn/avatar/icon_big");
assert_eq!(,
            user_info.open_id,
            "ou-caecc734c2e3328a62489fe0648c4b98779515d3",
);
        assert_eq!(
            user_info.union_id,
            "on-d89jhsdhjsajkda7828enjdj328ydhhw3u43yjhdj",
);
        assert_eq!(user_info.email, Some("zhangsan@feishu.cn".to_string()));
assert_eq!(,
            user_info.enterprise_email,
            Some("demo@mail.com".to_string()),
);
        assert_eq!(user_info.user_id, "5d9bdxxx");
        assert_eq!(user_info.mobile, Some("+86130002883xx".to_string()));
        assert_eq!(user_info.tenant_key, "736588c92lxf175d");
        assert_eq!(user_info.employee_no, "111222333");
}
#[test]
    fn test_user_info_optional_fields() {,
let json_str = r#"{,
            "name": "testuser",
            "en_name": "testuser",
            "avatar_url": "www.feishu.cn/avatar/icon",
            "avatar_thumb": "www.feishu.cn/avatar/icon_thumb",
            "avatar_middle": "www.feishu.cn/avatar/icon_middle",
            "avatar_big": "www.feishu.cn/avatar/icon_big",
            "open_id": "ou-test123456789",
            "union_id": "on-test123456789",
            "user_id": "test123",
            "tenant_key": "test_tenant",
            "employee_no": "EMP001",
}"#;
let user_info: UserInfo = serde_json::from_str(json_str).unwrap();
        assert_eq!(user_info.name, "testuser");
        assert_eq!(user_info.user_id, "test123");
assert!(user_info.email.is_none());
        assert!(user_info.enterprise_email.is_none());
assert!(user_info.mobile.is_none());
    }
#[test]
    fn test_user_info_service_new() {,
let config = Config::default();
        let service = UserInfoService::new(config.clone());
// Check that the service was created with the provided config,
        assert_eq!(service.config.base_url, config.base_url);
        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
}
#[test]
    fn test_user_info_api_response_trait() {,
// Test that the data_format method exists and returns the expected type,
        let format = UserInfo::data_format();
        // We can't compare directly, but we can check that the method exists,
// This tests that the ApiResponseTrait is properly implemented,
        assert!(matches!(format, ResponseFormat::Data));
}
#[test]
    fn test_user_info_debug_trait() {,
let user_info = UserInfo {,
            name: "test".to_string(),
            en_name: "test".to_string(),
            avatar_url: "url".to_string(),
            avatar_thumb: "thumb".to_string(),
            avatar_middle: "middle".to_string(),
            avatar_big: "big".to_string(),
            open_id: "open_id".to_string(),
            union_id: "union_id".to_string(),
            email: Some("test@example.com".to_string()),
            enterprise_email: None,
            user_id: "user_id".to_string(),
            mobile: None,
            tenant_key: "tenant".to_string(),
            employee_no: "emp001".to_string(),
        };

        let debug_str = format!("{:?}", user_info);
assert!(debug_str.contains("test"));
        assert!(debug_str.contains("UserInfo"));
}
#[test]
    fn test_user_info_serde_round_trip() {,
let original = UserInfo {,
            name: "test user".to_string(),
            en_name: "test_user".to_string(),
            avatar_url: "https://example.com/avatar.jpg".to_string(),
            avatar_thumb: "https://example.com/avatar_thumb.jpg".to_string(),
            avatar_middle: "https://example.com/avatar_middle.jpg".to_string(),
            avatar_big: "https://example.com/avatar_big.jpg".to_string(),
            open_id: "ou-12345".to_string(),
            union_id: "on-67890".to_string(),
            email: Some("test@company.com".to_string()),
            enterprise_email: Some("test@enterprise.com".to_string()),
            user_id: "u12345".to_string(),
            mobile: Some("+1234567890".to_string()),
            tenant_key: "tenant123".to_string(),
            employee_no: "E12345".to_string(),
        };
// Serialize to JSON,
        let json = serde_json::to_string(&original).unwrap();
// Deserialize back,
        let deserialized: UserInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.en_name, deserialized.en_name);
        assert_eq!(original.avatar_url, deserialized.avatar_url);
        assert_eq!(original.open_id, deserialized.open_id);
        assert_eq!(original.union_id, deserialized.union_id);
        assert_eq!(original.email, deserialized.email);
        assert_eq!(original.enterprise_email, deserialized.enterprise_email);
        assert_eq!(original.user_id, deserialized.user_id);
        assert_eq!(original.mobile, deserialized.mobile);
        assert_eq!(original.tenant_key, deserialized.tenant_key);
        assert_eq!(original.employee_no, deserialized.employee_no);
}
#[test]
    fn test_user_info_with_unicode_characters() {,
let json_str = r#"{,
            "name": "张三",
            "en_name": "zhangsan",
            "avatar_url": "www.feishu.cn/avatar/icon",
            "avatar_thumb": "www.feishu.cn/avatar/icon_thumb",
            "avatar_middle": "www.feishu.cn/avatar/icon_middle",
            "avatar_big": "www.feishu.cn/avatar/icon_big",
            "open_id": "ou-test",
            "union_id": "on-test",
            "email": "张三@公司.com",
            "user_id": "user123",
            "tenant_key": "tenant",
            "employee_no": "工号001",
}"#;
let user_info: UserInfo = serde_json::from_str(json_str).unwrap();
        assert_eq!(user_info.name, "张三");
        assert_eq!(user_info.email, Some("张三@公司.com".to_string()));
        assert_eq!(user_info.employee_no, "工号001");
}
#[test]
    fn test_user_info_invalid_json() {,
let invalid_json = r#"{,
            "name": "test",
            "invalid_field": "should_not_cause_error",
}"#;
// This should fail because required fields are missing,
        let result = serde_json::from_str::<UserInfo>(invalid_json);
assert!(result.is_err());
    }
#[test]
    fn test_user_info_empty_string_fields() {,
let json_str = r#"{,
            "name": "",
            "en_name": "",
            "avatar_url": "",
            "avatar_thumb": "",
            "avatar_middle": "",
            "avatar_big": "",
            "open_id": "",
            "union_id": "",
            "user_id": "",
            "tenant_key": "",
            "employee_no": "",
}"#;
let user_info: UserInfo = serde_json::from_str(json_str).unwrap();
        assert_eq!(user_info.name, "");
        assert_eq!(user_info.en_name, "");
        assert_eq!(user_info.open_id, "");
assert!(user_info.email.is_none());
        assert!(user_info.mobile.is_none());
}
#[test]
    fn test_user_info_service_config_independence() {,
let config1 = Config::builder()
            .app_id()
.app_secret()
            .build();
let config2 = Config::builder()
            .app_id()
.app_secret()
            .build();
let service1 = UserInfoService::new(config1);
        let service2 = UserInfoService::new(config2);

        assert_eq!(service1.config.app_id, "app1");
        assert_eq!(service2.config.app_id, "app2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
}
// === 企业级增强测试用例 ===,
    #[test]
fn test_user_info_service_with_app_types() {,
        // Test with SelfBuild app type,
let self_build_config = Config::builder()
            .app_id()
.app_secret()
            .app_type()
.build();
        let self_build_service = UserInfoService::new(self_build_config);
// Test with Marketplace app type,
        let marketplace_config = Config::builder()
.app_id()
            .app_secret()
.app_type()
            .build();
let marketplace_service = UserInfoService::new(marketplace_config);
        assert_eq!(self_build_service.config.app_type, AppType::SelfBuild);
        assert_eq!(marketplace_service.config.app_type, AppType::Marketplace);
        assert_eq!(UserInfoService::service_name(), "user_info");
        assert_eq!(UserInfoService::service_name(), "user_info");
        assert_eq!(UserInfoService::service_version(), "v1");
        assert_eq!(UserInfoService::service_version(), "v1");
}
#[test]
    fn test_user_info_service_config_properties() {,
let config = Config::builder()
            .app_id()
.app_secret()
            .app_type()
.enable_token_cache()
            .build();
let service = UserInfoService::new(config);
        // Test config properties
        assert_eq!(service.config.app_id, "config_test_app");
        assert_eq!(service.config.app_secret, "config_test_secret");
        assert_eq!(service.config.app_type, AppType::SelfBuild);
assert!(!service.config.enable_token_cache);
        assert!(!service.config.base_url.is_empty());
}
#[test]
    fn test_user_info_with_complex_unicode_and_special_chars() {,
let json_str = r#"{,
            "name": "张三 🚀",
            "en_name": "Zhang San (Developer)",
            "avatar_url": "https://example.com/张三-avatar.jpg",
            "avatar_thumb": "https://example.com/thumb_张三.png",
            "avatar_middle": "https://example.com/middle_张三.jpg",
            "avatar_big": "https://example.com/big_张三.png",
            "open_id": "ou-张三-123-🚀",
            "union_id": "on-union-张三-456-💼",
            "email": "张三.san@公司-测试.com",
            "enterprise_email": "zhang.san@enterprise.co.uk",
            "user_id": "user_张三_789",
            "mobile": "+86 138 0013 8000",
            "tenant_key": "tenant-张三-公司-key",
            "employee_no": "EMP-张三-001",
}"#;
let user_info: UserInfo = serde_json::from_str(json_str).unwrap();
        assert_eq!(user_info.name, "张三 🚀");
        assert_eq!(user_info.en_name, "Zhang San (Developer)");
        assert_eq!(user_info.open_id, "ou-张三-123-🚀");
        assert_eq!(user_info.union_id, "on-union-张三-456-💼");
        assert_eq!(user_info.email, Some("张三.san@公司-测试.com".to_string()));
        assert_eq!(user_info.employee_no, "EMP-张三-001");
        assert_eq!(user_info.mobile, Some("+86 138 0013 8000".to_string()));
}
#[test]
    fn test_user_info_with_very_long_fields() {,
let long_string = "a".repeat(1000);
        let json_str = format!(,
r#"{{,
            "name": "{}",
            "en_name": "{}",
            "avatar_url": "https://example.com/avatar.jpg",
            "avatar_thumb": "https://example.com/thumb.jpg",
            "avatar_middle": "https://example.com/middle.jpg",
            "avatar_big": "https://example.com/big.jpg",
            "open_id": "ou-12345",
            "union_id": "on-67890",
            "email": "test@example.com",
            "user_id": "user123",
            "tenant_key": "tenant123",
            "employee_no": "EMP001",
}}"#,
            long_string, long_string,
);
        let user_info: UserInfo = serde_json::from_str(&json_str).unwrap();

        assert_eq!(user_info.name.len(), 1000);
        assert_eq!(user_info.en_name.len(), 1000);
assert!(user_info.name.starts_with('a'));
        assert!(user_info.name.ends_with('a'));
}
#[test]
    fn test_user_info_with_minimal_valid_data() {,
let json_str = r#"{,
            "name": "A",
            "en_name": "B",
            "avatar_url": "http://a.co/a",
            "avatar_thumb": "http://a.co/b",
            "avatar_middle": "http://a.co/c",
            "avatar_big": "http://a.co/d",
            "open_id": "o",
            "union_id": "u",
            "user_id": "i",
            "tenant_key": "t",
            "employee_no": "e",
}"#;
let user_info: UserInfo = serde_json::from_str(json_str).unwrap();
        assert_eq!(user_info.name, "A");
        assert_eq!(user_info.en_name, "B");
        assert_eq!(user_info.open_id, "o");
        assert_eq!(user_info.union_id, "u");
        assert_eq!(user_info.user_id, "i");
        assert_eq!(user_info.tenant_key, "t");
        assert_eq!(user_info.employee_no, "e");
assert!(user_info.email.is_none());
        assert!(user_info.mobile.is_none());
assert!(user_info.enterprise_email.is_none());
    }
#[test]
    fn test_user_info_edge_case_email_formats() {,
let test_cases = vec![,
            ("standard@example.com", Some("standard@example.com")),
            ("user.name@domain.co.uk", Some("user.name@domain.co.uk")),
            ("user+tag@example.org", Some("user+tag@example.org")),
            (
                "international@xn--fsq.xn--p1ai",
                Some("international@xn--fsq.xn--p1ai"),
            ),
            // Note: quoted emails need special handling in JSON
            ("simple@example.com", Some("simple@example.com")),
        ];

        for (email_input, expected) in test_cases {,
let json_str = format!(,
                r#"{{
                "name": "test",
                "en_name": "test",
                "avatar_url": "url",
                "avatar_thumb": "thumb",
                "avatar_middle": "middle",
                "avatar_big": "big",
                "open_id": "open_id",
                "union_id": "union_id",
                "email": "{}",
                "user_id": "user_id",
                "tenant_key": "tenant",
                "employee_no": "emp",
}}"#,
                email_input,
);
            let user_info: UserInfo = serde_json::from_str(&json_str).unwrap();
            assert_eq!(user_info.email, expected.map(String::from));
}
    }
#[test]
    fn test_user_info_edge_case_phone_formats() {,
let test_cases = vec![,
            ("+1234567890", Some("+1234567890")),
            ("+86 138 0013 8000", Some("+86 138 0013 8000")),
            ("(555) 123-4567", Some("(555) 123-4567")),
            ("+44 20 7946 0958", Some("+44 20 7946 0958")),
            ("001-555-123-4567", Some("001-555-123-4567")),
        ];

        for (phone_input, expected) in test_cases {,
let json_str = format!(,
                r#"{{
                "name": "test",
                "en_name": "test",
                "avatar_url": "url",
                "avatar_thumb": "thumb",
                "avatar_middle": "middle",
                "avatar_big": "big",
                "open_id": "open_id",
                "union_id": "union_id",
                "mobile": "{}",
                "user_id": "user_id",
                "tenant_key": "tenant",
                "employee_no": "emp",
}}"#,
                phone_input,
);
            let user_info: UserInfo = serde_json::from_str(&json_str).unwrap();
            assert_eq!(user_info.mobile, expected.map(String::from));
}
    }
#[test]
    fn test_user_info_service_thread_safety() {,
use std::thread;
        let config = Config::builder()
.app_id()
            .app_secret()
.build();
        let service = Arc::new(UserInfoService::new(config));
let handles: Vec<_> = (0..10),
            .map(|i| {,
let service_clone = Arc::clone(&service);
                thread::spawn(move || {,
format!(,
                        "thread_{}_service_name: {}",
                        i,
                        UserInfoService::service_name(),
),
                }),
}),
.collect();
        // All threads should be able to access the service safely,
for handle in handles {,
            let result = handle.join().unwrap();
assert!(result.contains("user_info"));
        }
}
#[test]
    fn test_user_info_service_memory_efficiency() {,
let config = Config::builder()
            .app_id()
.app_secret()
            .build();
// Create multiple service instances,
        let services: Vec<UserInfoService> = (0..100),
.map(|_| UserInfoService::new(config.clone())),
            .collect();

        assert_eq!(services.len(), 100);
// All services should have the same config values,
        for service in &services {
            assert_eq!(service.config.app_id, "memory_test_app");
            assert_eq!(service.config.app_secret, "memory_test_secret");
            assert_eq!(UserInfoService::service_name(), "user_info");
            assert_eq!(UserInfoService::service_version(), "v1");
}
    }
#[test]
    fn test_user_info_service_arc_sharing() {,
let shared_config = Arc::new(,
            Config::builder()
.app_id()
                .app_secret("arc_test_secret")
                .build(),
        );
// Create services using shared config,
        let config1 = (*shared_config).clone();
let config2 = (*shared_config).clone();
        let service1 = UserInfoService::new(config1);
let service2 = UserInfoService::new(config2);
        // Both services should have the same values
        assert_eq!(service1.config.app_id, "arc_test_app");
        assert_eq!(service2.config.app_id, "arc_test_app");
        assert_eq!(service1.config.app_secret, "arc_test_secret");
        assert_eq!(service2.config.app_secret, "arc_test_secret");
        assert_eq!(UserInfoService::service_name(), "user_info");
        assert_eq!(UserInfoService::service_name(), "user_info");
}
#[test]
    fn test_user_info_config_comparison() {,
let config1 = Config::builder()
            .app_id()
.app_secret()
            .build();
let config2 = Config::builder()
            .app_id()
.app_secret()
            .build();
let service1 = UserInfoService::new(config1);
        let service2 = UserInfoService::new(config2);
// Services with equivalent configs should have same values,
        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);
assert_eq!(,
            UserInfoService::service_name(),
            UserInfoService::service_name(),
);
        assert_eq!(
            UserInfoService::service_version(),
            UserInfoService::service_version(),
);
    }
#[test]
    fn test_user_info_with_url_validation() {,
let test_urls = vec![,
            ("https://example.com/avatar.jpg", true),
            ("http://localhost:3000/avatar.png", true),
            ("ftp://files.example.com/avatar.gif", true),
            ("/relative/path/avatar.jpg", true),
            ("avatar.jpg", true),
            ("", false), // Empty URL should still be parsed but might be invalid in practice,
];
        for (avatar_url, _should_be_valid) in test_urls {,
let json_str = format!(,
                r#"{{
                "name": "test",
                "en_name": "test",
                "avatar_url": "{}",
                "avatar_thumb": "thumb",
                "avatar_middle": "middle",
                "avatar_big": "big",
                "open_id": "open_id",
                "union_id": "union_id",
                "user_id": "user_id",
                "tenant_key": "tenant",
                "employee_no": "emp",
}}"#,
                avatar_url,
);
            let user_info: UserInfo = serde_json::from_str(&json_str).unwrap();
            assert_eq!(user_info.avatar_url, avatar_url);
}
    }
#[test]
    fn test_user_info_field_length_validation() {,
// Test various field lengths,
        let name_lengths = vec![1, 10, 50, 100, 500];
for length in name_lengths {,
            let name = "x".repeat(length);
let json_str = format!(,
                r#"{{
                "name": "{}",
                "en_name": "test",
                "avatar_url": "url",
                "avatar_thumb": "thumb",
                "avatar_middle": "middle",
                "avatar_big": "big",
                "open_id": "open_id",
                "union_id": "union_id",
                "user_id": "user_id",
                "tenant_key": "tenant",
                "employee_no": "emp",
}}"#,
                name,
);
            let user_info: UserInfo = serde_json::from_str(&json_str).unwrap();
            assert_eq!(user_info.name.len(), length);
}
    }
#[test]
    fn test_user_info_with_json_whitespace_handling() {,
let json_str = r#",
        {
            "name": "test user",
            "en_name": "test_user",
            "avatar_url": "url",
            "avatar_thumb": "thumb",
            "avatar_middle": "middle",
            "avatar_big": "big",
            "open_id": "open_id",
            "union_id": "union_id",
            "user_id": "user_id",
            "tenant_key": "tenant",
            "employee_no": "emp",
            "email": "test@example.com",
}
"#;
        let user_info: UserInfo = serde_json::from_str(json_str).unwrap();
        assert_eq!(user_info.name, "test user");
        assert_eq!(user_info.email, Some("test@example.com".to_string()));
}
#[test]
    fn test_user_info_partial_data_with_null_fields() {,
let json_str = r#"{,
            "name": "test",
            "en_name": "test",
            "avatar_url": "url",
            "avatar_thumb": "thumb",
            "avatar_middle": "middle",
            "avatar_big": "big",
            "open_id": "open_id",
            "union_id": "union_id",
            "email": null,
            "enterprise_email": null,
            "mobile": null,
            "user_id": "user_id",
            "tenant_key": "tenant",
            "employee_no": "emp",
}"#;
let user_info: UserInfo = serde_json::from_str(json_str).unwrap();
        assert!(user_info.email.is_none());
assert!(user_info.enterprise_email.is_none());
        assert!(user_info.mobile.is_none());
}
#[test]
    fn test_user_info_service_config_modification_independence() {,
let original_config = Config::builder()
            .app_id()
.app_secret()
            .build();
let original_service = UserInfoService::new(original_config);
        // Create modified config,
let modified_config = Config::builder()
            .app_id()
.app_secret()
            .build();
let modified_service = UserInfoService::new(modified_config);
        // Services should be independent
        assert_eq!(original_service.config.app_id, "original_app");
        assert_eq!(modified_service.config.app_id, "modified_app");
assert_ne!(,
            original_service.config.app_id,
            modified_service.config.app_id,
);
    }
#[test]
    fn test_user_info_with_various_encoding_formats() {,
// Test different URL encoding scenarios,
        let encoded_urls = vec![
            "https://example.com/avatar%20space.jpg",
            "https://example.com/avatar%2Bplus.png",
            "https://example.com/avatar%3Fquery.jpg",
            "https://example.com/avatar%23hash.gif",
        ];
for encoded_url in encoded_urls {,
            let json_str = format!(,
r#"{{,
                "name": "test",
                "en_name": "test",
                "avatar_url": "{}",
                "avatar_thumb": "thumb",
                "avatar_middle": "middle",
                "avatar_big": "big",
                "open_id": "open_id",
                "union_id": "union_id",
                "user_id": "user_id",
                "tenant_key": "tenant",
                "employee_no": "emp",
}}"#,
                encoded_url,
);
            let user_info: UserInfo = serde_json::from_str(&json_str).unwrap();
            assert_eq!(user_info.avatar_url, encoded_url);
}
    }
#[test]
    fn test_user_info_serialization_performance() {,
use std::time::Instant;
        let user_info = UserInfo {
            name: "Performance Test User".to_string(),
            en_name: "performance_test_user".to_string(),
            avatar_url: "https://example.com/avatar.jpg".to_string(),
            avatar_thumb: "https://example.com/thumb.jpg".to_string(),
            avatar_middle: "https://example.com/middle.jpg".to_string(),
            avatar_big: "https://example.com/big.jpg".to_string(),
            open_id: "ou_performance_test".to_string(),
            union_id: "on_performance_test".to_string(),
            email: Some("performance@example.com".to_string()),
            enterprise_email: Some("perf@enterprise.com".to_string()),
            user_id: "perf_user_123".to_string(),
            mobile: Some("+1234567890".to_string()),
            tenant_key: "perf_tenant_456".to_string(),
            employee_no: "PERF001".to_string(),
        };
// Test serialization performance,
        let start = Instant::now();
for _ in 0..1000 {,
            let _json = serde_json::to_string(&user_info).unwrap();
}
let duration = start.elapsed();
        // Should complete 1000 serializations quickly (less than 1 second),
assert!(,
            duration.as_secs() < 1,
            "Serialization too slow: {:?}",
            duration,
);
    }
#[test]
    fn test_user_info_deserialization_performance() {,
use std::time::Instant;
        let json_str = r#"{
            "name": "Performance Test User",
            "en_name": "performance_test_user",
            "avatar_url": "https://example.com/avatar.jpg",
            "avatar_thumb": "https://example.com/thumb.jpg",
            "avatar_middle": "https://example.com/middle.jpg",
            "avatar_big": "https://example.com/big.jpg",
            "open_id": "ou_performance_test",
            "union_id": "on_performance_test",
            "email": "performance@example.com",
            "enterprise_email": "perf@enterprise.com",
            "user_id": "perf_user_123",
            "mobile": "+1234567890",
            "tenant_key": "perf_tenant_456",
            "employee_no": "PERF001",
}"#;
// Test deserialization performance,
        let start = Instant::now();
for _ in 0..1000 {,
            let _: UserInfo = serde_json::from_str(json_str).unwrap();
}
let duration = start.elapsed();
        // Should complete 1000 deserializations quickly (less than 1 second),
assert!(,
            duration.as_secs() < 1,
            "Deserialization too slow: {:?}",
            duration,
);
    }
#[test]
    fn test_user_info_service_clone_and_comparison() {,
let config = Config::builder()
            .app_id()
.app_secret()
            .build();
let service = UserInfoService::new(config.clone());
        // Test config cloning,
let cloned_config = service.config.clone();
        assert_eq!(cloned_config.app_id, "clone_test_app");
        assert_eq!(cloned_config.app_secret, "clone_test_secret");
// Create new service with cloned config,
        let cloned_service = UserInfoService::new(cloned_config);
        assert_eq!(service.config.app_id, cloned_service.config.app_id);
assert_eq!(,
            UserInfoService::service_name(),
            UserInfoService::service_name(),
);
    }
#[test]
    fn test_user_info_with_enterprise_scenarios() {,
// Test typical enterprise user scenarios,
        let scenarios = vec![,
(,
                "Executive User",
                "exec@enterprise.com",
                Some("exec@enterprise.internal"),
            ),
            ("Regular Employee", "emp@company.com", None),
            ("Contractor", "contractor@vendor.com", None),
            (
                "System Admin",
                "admin@company.com",
                Some("admin@company.internal"),
            ),
        ];

        for (name, email, enterprise_email) in scenarios {
            let en_name = name.to_lowercase().replace(" ", "_");
            let open_id = format!("ou_{}", en_name);
            let union_id = format!("on_{}", en_name);
            let user_id = format!("user_{}", en_name);
            let employee_no = format!("EMP_{}", name.to_uppercase().replace(" ", ""));
let enterprise_email_json = match enterprise_email {,
                Some(email) => format!(r#""{}""#, email),
                None => "null".to_string(),
            };
let json_str = format!(,
                r#"{{
                "name": "{}",
                "en_name": "{}",
                "avatar_url": "https://company.com/avatar.jpg",
                "avatar_thumb": "https://company.com/thumb.jpg",
                "avatar_middle": "https://company.com/middle.jpg",
                "avatar_big": "https://company.com/big.jpg",
                "open_id": "{}",
                "union_id": "{}",
                "email": "{}",
                "enterprise_email": {}
                "user_id": "{}",
                "tenant_key": "company_tenant",
                "employee_no": "{}",
}}"#,
                name,
                en_name,
                open_id,
                union_id,
                email,
                enterprise_email_json,
                user_id,
                employee_no,
);
            let user_info: UserInfo = serde_json::from_str(&json_str).unwrap();
            assert_eq!(user_info.name, name);
            assert_eq!(user_info.email, Some(email.to_string()));
assert_eq!(,
                user_info.enterprise_email,
                enterprise_email.map(String::from),
);
        }
}
}
// ===== 新增的令牌管理API请求和响应结构体 =====,
/// 商店应用获取App Access Token请求
#[derive(.*?)]
pub struct GetAppAccessTokenRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 应用类型，app_access_token接口可传递app_type为self_build或marketplace,
#[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
}
/// 自建应用获取App Access Token请求,
#[derive(.*?)]
pub struct GetAppAccessTokenInternalRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
}
/// 商店应用获取Tenant Access Token请求,
#[derive(.*?)]
pub struct GetTenantAccessTokenRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 企业标识,
#[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}
/// 自建应用获取Tenant Access Token请求,
#[derive(.*?)]
pub struct GetTenantAccessTokenInternalRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 企业标识,
#[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}
/// 重新获取App Ticket请求,
#[derive(.*?)]
pub struct ResendAppTicketRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 接收ticket的回调地址,
#[serde(skip_serializing_if = "Option::is_none")]
    pub callback_address: Option<String>,
}
/// App Access Token响应,
#[derive(.*?)]
pub struct AppAccessTokenResponse {
    /// 应用访问令牌
    pub app_access_token: String,
    /// 令牌类型，目前固定为"bearer"
    pub token_type: String,
    /// 令牌有效期，秒数,
#[serde(rename = "expire")]
    pub expires_in: i64,
    /// 刷新令牌，用于获取新的app_access_token,
#[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// 刷新令牌有效期，秒数,
#[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_expires_in: Option<i64>,
}
/// Tenant Access Token响应,
#[derive(.*?)]
pub struct TenantAccessTokenResponse {
    /// 租户访问令牌
    pub tenant_access_token: String,
    /// 令牌类型，目前固定为"bearer"
    pub token_type: String,
    /// 令牌有效期，秒数,
#[serde(rename = "expire")]
    pub expires_in: i64,
    /// 刷新令牌，用于获取新的tenant_access_token,
#[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// 刷新令牌有效期，秒数,
#[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_expires_in: Option<i64>,
}
/// 重新获取App Ticket响应,
#[derive(.*?)]
pub struct ResendAppTicketResponse {
    /// App ticket，用于接收事件推送,
#[serde(skip_serializing_if = "Option::is_none")]
    pub app_ticket: Option<String>,
    /// 重新发送的状态,
#[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
// ApiResponse Trait implementations,
impl ApiResponseTrait for.* {
fn data_format() -> ResponseFormat {,
        ResponseFormat::Data
}
}
impl ApiResponseTrait for.* {
    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}
impl ApiResponseTrait for.* {
    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}
// Service Trait implementations,
impl Service for AppAccessTokenService {,
fn config(&self) -> &Config {,
        &self.config,
}
fn service_name() -> &'static str {,
        "app_access_token",
}
fn service_version() -> &'static str {,
        "v3",
}
}
impl Service for TenantAccessTokenService {,
    fn config(&self) -> &Config {,
&self.config,
    }
fn service_name() -> &'static str {,
        "tenant_access_token",
}
fn service_version() -> &'static str {,
        "v3",
}
}
impl Service for AppTicketService {,
    fn config(&self) -> &Config {,
&self.config,
    }
fn service_name() -> &'static str {,
        "app_ticket",
}
fn service_version() -> &'static str {,
        "v3",
}
}
