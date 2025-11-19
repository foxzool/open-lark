use reqwest::Method;
use serde_json::json;

use crate::{
    core::{
        api::Response, config::Config, constants::AccessTokenType,
        endpoints::attendance::*, http::Transport, req_option::RequestOption,
        trait_system::Service, SDKResult,
    },
    impl_executable_builder_owned,
};

use super::models::{
    QueryStatsFieldsRequest, QueryStatsFieldsRespData, QueryStatsSettingsRequest,
    QueryStatsSettingsRespData, QueryUserStatsDataRequest, QueryUserStatsDataRespData,
    UpdateUserStatsDataRequest, UpdateUserStatsDataRespData,
};

/// 用户统计数据服务
pub struct UserStatsDataService {
    pub config: Config,
}

impl UserStatsDataService {
    /// 更新统计设置
    ///
    /// 该接口用于更新考勤统计的设置，包括统计范围、日期、用户和字段等。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query
    pub async fn update(
        &self,
        request: UpdateUserStatsDataRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UpdateUserStatsDataRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_USER_STATS_DATAS_UPDATE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "stats_setting": request.stats_setting
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询统计设置
    ///
    /// 该接口用于查询当前考勤统计的设置信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query
    pub async fn query_settings(
        &self,
        request: QueryStatsSettingsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryStatsSettingsRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_USER_STATS_DATAS_QUERY.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询统计表头
    ///
    /// 该接口用于查询考勤统计的可用字段信息，包括字段名称、类型等。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query
    pub async fn query_fields(
        &self,
        request: QueryStatsFieldsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryStatsFieldsRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_USER_STATS_DATAS_QUERY_FIELDS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(locale) = request.locale {
            api_req.query_params.insert("locale", locale);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询统计数据
    ///
    /// 该接口用于查询用户的考勤统计数据，支持按日期范围、用户和字段进行筛选。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query
    pub async fn query_data(
        &self,
        request: QueryUserStatsDataRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryUserStatsDataRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_USER_STATS_DATAS_QUERY_DATA.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(locale) = request.locale {
            api_req.query_params.insert("locale", locale);
        }

        // 构建请求体
        let body = json!({
            "start_date": request.start_date,
            "end_date": request.end_date,
            "user_ids": request.user_ids,
            "need_fields": request.need_fields
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

impl Service for UserStatsDataService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "user_stats_data"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

// Builder implementations
impl_executable_builder_owned!(
    UpdateUserStatsDataRequest,
    UserStatsDataService,
    UpdateUserStatsDataRequest,
    Response<UpdateUserStatsDataRespData>,
    update
);

impl_executable_builder_owned!(
    QueryStatsSettingsRequest,
    UserStatsDataService,
    QueryStatsSettingsRequest,
    Response<QueryStatsSettingsRespData>,
    query_settings
);

impl_executable_builder_owned!(
    QueryStatsFieldsRequest,
    UserStatsDataService,
    QueryStatsFieldsRequest,
    Response<QueryStatsFieldsRespData>,
    query_fields
);

impl_executable_builder_owned!(
    QueryUserStatsDataRequest,
    UserStatsDataService,
    QueryUserStatsDataRequest,
    Response<QueryUserStatsDataRespData>,
    query_data
);

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use openlark_core::{api::ApiRequest, config::Config};
    use crate::service::attendance::v1::models::StatsSettings;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.api.com")
            .build()
    }

    fn create_test_service() -> UserStatsDataService {
        UserStatsDataService {
            config: create_test_config(),
        }
    }

    fn create_test_stats_settings() -> StatsSettings {
        StatsSettings {
            stats_scope: 1,
            start_date: "2023-01-01".to_string(),
            end_date: "2023-12-31".to_string(),
            user_ids: vec!["user001".to_string(), "user002".to_string()],
            need_fields: vec!["field1".to_string(), "field2".to_string()],
        }
    }

    #[test]
    fn test_service_creation() {
        let service = create_test_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert_eq!(service.config.base_url, "https://test.api.com");
    }

    #[test]
    fn test_update_user_stats_data_request() {
        let request = UpdateUserStatsDataRequest {
            api_req: ApiRequest::default(),
            employee_type: "employee_id".to_string(),
            stats_setting: create_test_stats_settings(),
        };

        assert_eq!(request.employee_type, "employee_id");
        assert_eq!(request.stats_setting.stats_scope, 1);
        assert_eq!(request.stats_setting.start_date, "2023-01-01");
        assert_eq!(request.stats_setting.end_date, "2023-12-31");
        assert_eq!(request.stats_setting.user_ids.len(), 2);
        assert_eq!(request.stats_setting.need_fields.len(), 2);
    }

    #[test]
    fn test_query_stats_settings_request() {
        let request = QueryStatsSettingsRequest {
            api_req: ApiRequest::default(),
            employee_type: "employee_no".to_string(),
        };

        assert_eq!(request.employee_type, "employee_no");
    }

    #[test]
    fn test_query_stats_fields_request() {
        let request = QueryStatsFieldsRequest {
            api_req: ApiRequest::default(),
            employee_type: "open_id".to_string(),
            locale: Some("zh-CN".to_string()),
        };

        assert_eq!(request.employee_type, "open_id");
        assert_eq!(request.locale, Some("zh-CN".to_string()));
    }

    #[test]
    fn test_query_stats_fields_request_without_locale() {
        let request = QueryStatsFieldsRequest {
            api_req: ApiRequest::default(),
            employee_type: "union_id".to_string(),
            locale: None,
        };

        assert_eq!(request.employee_type, "union_id");
        assert!(request.locale.is_none());
    }

    #[test]
    fn test_query_user_stats_data_request() {
        let request = QueryUserStatsDataRequest {
            api_req: ApiRequest::default(),
            employee_type: "employee_id".to_string(),
            start_date: "2023-01-01".to_string(),
            end_date: "2023-01-31".to_string(),
            user_ids: vec![
                "user001".to_string(),
                "user002".to_string(),
                "user003".to_string(),
            ],
            need_fields: vec!["attendance_days".to_string(), "work_hours".to_string()],
            locale: Some("en-US".to_string()),
        };

        assert_eq!(request.employee_type, "employee_id");
        assert_eq!(request.start_date, "2023-01-01");
        assert_eq!(request.end_date, "2023-01-31");
        assert_eq!(request.user_ids.len(), 3);
        assert_eq!(request.need_fields.len(), 2);
        assert_eq!(request.locale, Some("en-US".to_string()));
    }

    #[test]
    fn test_stats_settings_variations() {
        // Test different scope values
        let settings1 = StatsSettings {
            stats_scope: 1, // 自定义范围
            start_date: "2023-01-01".to_string(),
            end_date: "2023-12-31".to_string(),
            user_ids: vec!["user001".to_string()],
            need_fields: vec!["field1".to_string()],
        };

        let settings2 = StatsSettings {
            stats_scope: 2, // 全部
            start_date: "2023-01-01".to_string(),
            end_date: "2023-12-31".to_string(),
            user_ids: vec![],
            need_fields: vec![],
        };

        assert_eq!(settings1.stats_scope, 1);
        assert_eq!(settings2.stats_scope, 2);
    }

    #[test]
    fn test_different_employee_types() {
        let employee_types = vec!["employee_id", "employee_no", "open_id", "union_id"];

        for emp_type in employee_types {
            let request = QueryStatsSettingsRequest {
                api_req: ApiRequest::default(),
                employee_type: emp_type.to_string(),
            };
            assert_eq!(request.employee_type, emp_type);
        }
    }

    #[test]
    fn test_different_locales() {
        let locales = vec![
            Some("zh-CN".to_string()),
            Some("en-US".to_string()),
            Some("ja-JP".to_string()),
            Some("zh-TW".to_string()),
            Some("ko-KR".to_string()),
            None,
        ];

        for locale in locales {
            let request = QueryStatsFieldsRequest {
                api_req: ApiRequest::default(),
                employee_type: "employee_id".to_string(),
                locale: locale.clone(),
            };
            assert_eq!(request.locale, locale);
        }
    }

    #[test]
    fn test_date_format_variations() {
        let dates = vec![
            ("2023-01-01", "2023-01-31"), // January
            ("2023-02-01", "2023-02-28"), // February
            ("2023-12-01", "2023-12-31"), // December
            ("2023-06-15", "2023-06-15"), // Single day
            ("2020-02-29", "2020-02-29"), // Leap year
            ("1900-01-01", "2099-12-31"), // Wide range
        ];

        for (start_date, end_date) in dates {
            let settings = StatsSettings {
                stats_scope: 1,
                start_date: start_date.to_string(),
                end_date: end_date.to_string(),
                user_ids: vec!["user001".to_string()],
                need_fields: vec!["field1".to_string()],
            };

            assert_eq!(settings.start_date, start_date);
            assert_eq!(settings.end_date, end_date);
        }
    }

    #[test]
    fn test_user_ids_variations() {
        let user_id_lists = vec![
            vec![],                                         // Empty list
            vec!["single_user".to_string()],                // Single user
            vec!["user1".to_string(), "user2".to_string()], // Two users
            vec![
                "user1".to_string(),
                "user2".to_string(),
                "user3".to_string(),
                "user4".to_string(),
                "user5".to_string(),
            ], // Multiple users
            vec![
                "user-with-dashes".to_string(),
                "user_with_underscores".to_string(),
            ], // Special characters
            vec!["用户001".to_string(), "用户002".to_string()], // Unicode
            vec!["user🚀001".to_string(), "user🔐002".to_string()], // Emoji
        ];

        for user_ids in user_id_lists {
            let settings = StatsSettings {
                stats_scope: 1,
                start_date: "2023-01-01".to_string(),
                end_date: "2023-12-31".to_string(),
                user_ids: user_ids.clone(),
                need_fields: vec!["field1".to_string()],
            };

            assert_eq!(settings.user_ids, user_ids);
        }
    }

    #[test]
    fn test_need_fields_variations() {
        let field_lists = vec![
            vec![],                                                        // Empty list
            vec!["attendance_days".to_string()],                           // Single field
            vec!["attendance_days".to_string(), "work_hours".to_string()], // Two fields
            vec![
                "field1".to_string(),
                "field2".to_string(),
                "field3".to_string(),
                "field4".to_string(),
            ], // Multiple fields
            vec![
                "field-with-dashes".to_string(),
                "field_with_underscores".to_string(),
            ], // Special characters
            vec!["考勤天数".to_string(), "工作小时".to_string()],          // Unicode
            vec!["field🎯1".to_string(), "field📊2".to_string()],          // Emoji
        ];

        for need_fields in field_lists {
            let settings = StatsSettings {
                stats_scope: 1,
                start_date: "2023-01-01".to_string(),
                end_date: "2023-12-31".to_string(),
                user_ids: vec!["user001".to_string()],
                need_fields: need_fields.clone(),
            };

            assert_eq!(settings.need_fields, need_fields);
        }
    }

    #[test]
    fn test_stats_scope_edge_cases() {
        let scopes = vec![
            0,   // Edge case: invalid scope
            1,   // 自定义范围
            2,   // 全部
            -1,  // Edge case: negative scope
            999, // Edge case: large scope
        ];

        for scope in scopes {
            let settings = StatsSettings {
                stats_scope: scope,
                start_date: "2023-01-01".to_string(),
                end_date: "2023-12-31".to_string(),
                user_ids: vec!["user001".to_string()],
                need_fields: vec!["field1".to_string()],
            };

            assert_eq!(settings.stats_scope, scope);
        }
    }

    #[test]
    fn test_stats_settings_clone() {
        let original_settings = create_test_stats_settings();
        let cloned_settings = original_settings.clone();

        assert_eq!(original_settings.stats_scope, cloned_settings.stats_scope);
        assert_eq!(original_settings.start_date, cloned_settings.start_date);
        assert_eq!(original_settings.end_date, cloned_settings.end_date);
        assert_eq!(original_settings.user_ids, cloned_settings.user_ids);
        assert_eq!(original_settings.need_fields, cloned_settings.need_fields);
    }

    #[test]
    fn test_stats_settings_debug() {
        let settings = create_test_stats_settings();
        let debug_str = format!("{:?}", settings);

        assert!(debug_str.contains("StatsSettings"));
        assert!(debug_str.contains("2023-01-01"));
        assert!(debug_str.contains("2023-12-31"));
        assert!(debug_str.contains("user001"));
        assert!(debug_str.contains("field1"));
    }

    #[test]
    fn test_request_default_trait() {
        let query_fields_request = QueryStatsFieldsRequest::default();
        assert!(query_fields_request.employee_type.is_empty());
        assert!(query_fields_request.locale.is_none());

        let update_request = UpdateUserStatsDataRequest::default();
        assert!(update_request.employee_type.is_empty());

        let query_settings_request = QueryStatsSettingsRequest::default();
        assert!(query_settings_request.employee_type.is_empty());

        let query_data_request = QueryUserStatsDataRequest::default();
        assert!(query_data_request.employee_type.is_empty());
        assert!(query_data_request.start_date.is_empty());
        assert!(query_data_request.end_date.is_empty());
        assert!(query_data_request.user_ids.is_empty());
        assert!(query_data_request.need_fields.is_empty());
        assert!(query_data_request.locale.is_none());
    }

    #[test]
    fn test_request_build_methods() {
        let update_request = UpdateUserStatsDataRequest {
            api_req: ApiRequest::default(),
            employee_type: "employee_id".to_string(),
            stats_setting: create_test_stats_settings(),
        };
        let built_update = update_request.build();
        assert_eq!(built_update.employee_type, "employee_id");

        let query_settings_request = QueryStatsSettingsRequest {
            api_req: ApiRequest::default(),
            employee_type: "employee_no".to_string(),
        };
        let built_query_settings = query_settings_request.build();
        assert_eq!(built_query_settings.employee_type, "employee_no");

        let query_fields_request = QueryStatsFieldsRequest {
            api_req: ApiRequest::default(),
            employee_type: "open_id".to_string(),
            locale: Some("zh-CN".to_string()),
        };
        let built_query_fields = query_fields_request.build();
        assert_eq!(built_query_fields.employee_type, "open_id");
        assert_eq!(built_query_fields.locale, Some("zh-CN".to_string()));
    }

    #[test]
    fn test_json_serialization() {
        let settings = create_test_stats_settings();

        // Test serialization
        let json_result = serde_json::to_string(&settings);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("2023-01-01"));
        assert!(json_str.contains("2023-12-31"));
        assert!(json_str.contains("user001"));
        assert!(json_str.contains("field1"));

        // Test deserialization
        let deserialized_result: Result<StatsSettings, _> = serde_json::from_str(&json_str);
        assert!(deserialized_result.is_ok());

        let deserialized_settings = deserialized_result.unwrap();
        assert_eq!(deserialized_settings.stats_scope, settings.stats_scope);
        assert_eq!(deserialized_settings.start_date, settings.start_date);
        assert_eq!(deserialized_settings.end_date, settings.end_date);
        assert_eq!(deserialized_settings.user_ids, settings.user_ids);
        assert_eq!(deserialized_settings.need_fields, settings.need_fields);
    }

    #[test]
    fn test_empty_collections_serialization() {
        let settings = StatsSettings {
            stats_scope: 2,
            start_date: "2023-01-01".to_string(),
            end_date: "2023-12-31".to_string(),
            user_ids: vec![],
            need_fields: vec![],
        };

        let json_result = serde_json::to_string(&settings);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("[]") || json_str.contains("user_ids"));
        assert!(json_str.contains("[]") || json_str.contains("need_fields"));
    }

    #[test]
    fn test_large_collections() {
        let large_user_ids: Vec<String> = (0..1000).map(|i| format!("user_{:04}", i)).collect();
        let large_need_fields: Vec<String> = (0..100).map(|i| format!("field_{:03}", i)).collect();

        let settings = StatsSettings {
            stats_scope: 1,
            start_date: "2023-01-01".to_string(),
            end_date: "2023-12-31".to_string(),
            user_ids: large_user_ids.clone(),
            need_fields: large_need_fields.clone(),
        };

        assert_eq!(settings.user_ids.len(), 1000);
        assert_eq!(settings.need_fields.len(), 100);
        assert_eq!(settings.user_ids, large_user_ids);
        assert_eq!(settings.need_fields, large_need_fields);
    }

    #[test]
    fn test_special_date_formats() {
        let date_pairs = vec![
            ("", ""),                         // Empty dates
            ("2023-01-01", ""),               // Empty end date
            ("", "2023-12-31"),               // Empty start date
            ("invalid-date", "also-invalid"), // Invalid format
            ("2023/01/01", "2023/12/31"),     // Different separator
            ("01-01-2023", "31-12-2023"),     // Different order
        ];

        for (start_date, end_date) in date_pairs {
            let settings = StatsSettings {
                stats_scope: 1,
                start_date: start_date.to_string(),
                end_date: end_date.to_string(),
                user_ids: vec!["user001".to_string()],
                need_fields: vec!["field1".to_string()],
            };

            assert_eq!(settings.start_date, start_date);
            assert_eq!(settings.end_date, end_date);
        }
    }

    #[test]
    fn test_memory_efficiency() {
        // Create multiple request instances to test memory usage
        let requests: Vec<UpdateUserStatsDataRequest> = (0..100)
            .map(|i| UpdateUserStatsDataRequest {
                api_req: ApiRequest::default(),
                employee_type: "employee_id".to_string(),
                stats_setting: StatsSettings {
                    stats_scope: i % 3 + 1,
                    start_date: format!("2023-{:02}-01", (i % 12) + 1),
                    end_date: format!("2023-{:02}-28", (i % 12) + 1),
                    user_ids: vec![format!("user_{:03}", i)],
                    need_fields: vec![format!("field_{:03}", i)],
                },
            })
            .collect();

        assert_eq!(requests.len(), 100);

        for (i, request) in requests.iter().enumerate() {
            assert_eq!(request.employee_type, "employee_id");
            assert_eq!(request.stats_setting.stats_scope, (i % 3 + 1) as i32);
            assert_eq!(request.stats_setting.user_ids[0], format!("user_{:03}", i));
            assert_eq!(
                request.stats_setting.need_fields[0],
                format!("field_{:03}", i)
            );
        }
    }
}
