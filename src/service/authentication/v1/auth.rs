use crate::core::{
    api_req::ApiRequest,
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

pub struct UserInfoService {
    config: Config,
}

impl UserInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取登录用户信息
    ///
    /// <https://open.feishu.cn/document/server-docs/authentication-v1/user/get>
    pub async fn get(&self, user_access_token: impl ToString) -> SDKResult<UserInfo> {
        let api_req = ApiRequest {
            api_path: AUTHEN_V1_USER_INFO.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        let option = RequestOption::builder()
            .user_access_token(user_access_token)
            .build();
        let api_resp: BaseResponse<UserInfo> =
            Transport::request(api_req, &self.config, Some(option)).await?;
        api_resp.into_result()
    }
}

/// 登录用户信息
#[derive(Debug, Deserialize, Serialize)]
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

impl ApiResponseTrait for UserInfo {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl Service for UserInfoService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "user_info"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    #[test]
    fn test_user_info_deserialization() {
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
            "employee_no": "111222333"
        }"#;

        let user_info: UserInfo =
            serde_json::from_str(json_str).expect("Failed to parse test user info JSON");

        assert_eq!(user_info.name, "zhangsan");
        assert_eq!(user_info.en_name, "zhangsan");
        assert_eq!(user_info.avatar_url, "www.feishu.cn/avatar/icon");
        assert_eq!(user_info.avatar_thumb, "www.feishu.cn/avatar/icon_thumb");
        assert_eq!(user_info.avatar_middle, "www.feishu.cn/avatar/icon_middle");
        assert_eq!(user_info.avatar_big, "www.feishu.cn/avatar/icon_big");
        assert_eq!(
            user_info.open_id,
            "ou-caecc734c2e3328a62489fe0648c4b98779515d3"
        );
        assert_eq!(
            user_info.union_id,
            "on-d89jhsdhjsajkda7828enjdj328ydhhw3u43yjhdj"
        );
        assert_eq!(user_info.email, Some("zhangsan@feishu.cn".to_string()));
        assert_eq!(
            user_info.enterprise_email,
            Some("demo@mail.com".to_string())
        );
        assert_eq!(user_info.user_id, "5d9bdxxx");
        assert_eq!(user_info.mobile, Some("+86130002883xx".to_string()));
        assert_eq!(user_info.tenant_key, "736588c92lxf175d");
        assert_eq!(user_info.employee_no, "111222333");
    }

    #[test]
    fn test_user_info_optional_fields() {
        let json_str = r#"{
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
            "employee_no": "EMP001"
        }"#;

        let user_info: UserInfo = serde_json::from_str(json_str).unwrap();

        assert_eq!(user_info.name, "testuser");
        assert_eq!(user_info.user_id, "test123");
        assert!(user_info.email.is_none());
        assert!(user_info.enterprise_email.is_none());
        assert!(user_info.mobile.is_none());
    }

    #[test]
    fn test_user_info_service_new() {
        let config = Config::default();
        let service = UserInfoService::new(config.clone());

        // Check that the service was created with the provided config
        assert_eq!(service.config.base_url, config.base_url);
        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_user_info_api_response_trait() {
        // Test that the data_format method exists and returns the expected type
        let format = UserInfo::data_format();
        // We can't compare directly, but we can check that the method exists
        // This tests that the ApiResponseTrait is properly implemented
        assert!(matches!(format, ResponseFormat::Data));
    }

    #[test]
    fn test_user_info_debug_trait() {
        let user_info = UserInfo {
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
    fn test_user_info_serde_round_trip() {
        let original = UserInfo {
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

        // Serialize to JSON
        let json = serde_json::to_string(&original).unwrap();

        // Deserialize back
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
    fn test_user_info_with_unicode_characters() {
        let json_str = r#"{
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
            "employee_no": "工号001"
        }"#;

        let user_info: UserInfo = serde_json::from_str(json_str).unwrap();

        assert_eq!(user_info.name, "张三");
        assert_eq!(user_info.email, Some("张三@公司.com".to_string()));
        assert_eq!(user_info.employee_no, "工号001");
    }

    #[test]
    fn test_user_info_invalid_json() {
        let invalid_json = r#"{
            "name": "test",
            "invalid_field": "should_not_cause_error"
        }"#;

        // This should fail because required fields are missing
        let result = serde_json::from_str::<UserInfo>(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_user_info_empty_string_fields() {
        let json_str = r#"{
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
            "employee_no": ""
        }"#;

        let user_info: UserInfo = serde_json::from_str(json_str).unwrap();

        assert_eq!(user_info.name, "");
        assert_eq!(user_info.en_name, "");
        assert_eq!(user_info.open_id, "");
        assert!(user_info.email.is_none());
        assert!(user_info.mobile.is_none());
    }

    #[test]
    fn test_user_info_service_config_independence() {
        let config1 = Config::builder()
            .app_id("app1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("app2")
            .app_secret("secret2")
            .build();

        let service1 = UserInfoService::new(config1);
        let service2 = UserInfoService::new(config2);

        assert_eq!(service1.config.app_id, "app1");
        assert_eq!(service2.config.app_id, "app2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }
}
