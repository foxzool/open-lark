use reqwest::Method;
use serde_json::json;

use crate::{
    core::{
        api::Response,
        config::Config,
        constants::AccessTokenType,
        endpoints::{attendance::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::models::{
    DelArchiveReportRequest, DelArchiveReportRespData, ListArchiveRulesRequest,
    ListArchiveRulesRespData, QueryArchiveStatsFieldsRequest, QueryArchiveStatsFieldsRespData,
    UploadArchiveReportRequest, UploadArchiveReportRespData,
};

/// 归档报表服务
pub struct ArchiveRuleService {
    pub config: Config,
}

impl ArchiveRuleService {
    /// 查询归档报表表头
    ///
    /// 该接口用于查询指定归档规则的报表字段定义信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/user_stats_fields_query
    pub async fn query_user_stats_fields(
        &self,
        request: QueryArchiveStatsFieldsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryArchiveStatsFieldsRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_ARCHIVE_RULE_USER_STATS_FIELDS,
            "archive_rule_id",
            &request.archive_rule_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 写入归档报表结果
    ///
    /// 该接口用于向指定归档规则写入报表数据，支持批量上传考勤统计结果。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/upload_report
    pub async fn upload_report(
        &self,
        request: UploadArchiveReportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UploadArchiveReportRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_ARCHIVE_RULE_UPLOAD_REPORT,
            "archive_rule_id",
            &request.archive_rule_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "report_data": request.report_data
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 删除归档报表行数据
    ///
    /// 该接口用于删除指定的归档报表数据记录。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/del_report
    pub async fn del_report(
        &self,
        request: DelArchiveReportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<DelArchiveReportRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_ARCHIVE_RULE_DEL_REPORT,
            "archive_rule_id",
            &request.archive_rule_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "record_ids": request.record_ids
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询所有归档规则
    ///
    /// 该接口用于获取企业的所有归档规则列表，包括规则名称、状态等信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/list
    pub async fn list(
        &self,
        request: ListArchiveRulesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ListArchiveRulesRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(ATTENDANCE_V1_ARCHIVE_RULES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// Builder implementations
impl_executable_builder_owned!(
    QueryArchiveStatsFieldsRequest,
    ArchiveRuleService,
    QueryArchiveStatsFieldsRequest,
    Response<QueryArchiveStatsFieldsRespData>,
    query_user_stats_fields
);

impl_executable_builder_owned!(
    UploadArchiveReportRequest,
    ArchiveRuleService,
    UploadArchiveReportRequest,
    Response<UploadArchiveReportRespData>,
    upload_report
);

impl_executable_builder_owned!(
    DelArchiveReportRequest,
    ArchiveRuleService,
    DelArchiveReportRequest,
    Response<DelArchiveReportRespData>,
    del_report
);

impl_executable_builder_owned!(
    ListArchiveRulesRequest,
    ArchiveRuleService,
    ListArchiveRulesRequest,
    Response<ListArchiveRulesRespData>,
    list
);

impl Service for ArchiveRuleService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "archive_rule"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::{api::ApiRequest, config::Config};
    use crate::service::attendance::v1::models::ArchiveReportRecord;

    #[test]
    fn test_archive_rule_service_creation() {
        let config = Config::default();
        let service = ArchiveRuleService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_archive_rule_service_with_custom_config() {
        let config = Config::builder()
            .app_id("archive_test_app")
            .app_secret("archive_test_secret")
            .build();

        let service = ArchiveRuleService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, "archive_test_app");
        assert_eq!(service.config.app_secret, "archive_test_secret");
    }

    #[test]
    fn test_query_archive_stats_fields_request_construction() {
        let request = QueryArchiveStatsFieldsRequest {
            api_req: ApiRequest::default(),
            archive_rule_id: "rule_123".to_string(),
            employee_type: "1".to_string(),
        };

        assert_eq!(request.archive_rule_id, "rule_123");
        assert_eq!(request.employee_type, "1");
    }

    #[test]
    fn test_upload_archive_report_request_construction() {
        let record1 = ArchiveReportRecord {
            record_id: Some("record1".to_string()),
            user_id: "user1".to_string(),
            archive_date: "2024-01-01".to_string(),
            field_data: std::collections::HashMap::new(),
        };
        let record2 = ArchiveReportRecord {
            record_id: Some("record2".to_string()),
            user_id: "user2".to_string(),
            archive_date: "2024-01-02".to_string(),
            field_data: std::collections::HashMap::new(),
        };

        let request = UploadArchiveReportRequest {
            api_req: ApiRequest::default(),
            archive_rule_id: "rule_456".to_string(),
            employee_type: "2".to_string(),
            report_data: vec![record1, record2],
        };

        assert_eq!(request.archive_rule_id, "rule_456");
        assert_eq!(request.employee_type, "2");
        assert_eq!(request.report_data.len(), 2);
        assert_eq!(request.report_data[0].user_id, "user1");
        assert_eq!(request.report_data[1].user_id, "user2");
    }

    #[test]
    fn test_del_archive_report_request_construction() {
        let request = DelArchiveReportRequest {
            api_req: ApiRequest::default(),
            archive_rule_id: "rule_789".to_string(),
            employee_type: "3".to_string(),
            record_ids: vec!["id1".to_string(), "id2".to_string(), "id3".to_string()],
        };

        assert_eq!(request.archive_rule_id, "rule_789");
        assert_eq!(request.employee_type, "3");
        assert_eq!(request.record_ids.len(), 3);
        assert_eq!(request.record_ids[2], "id3");
    }

    #[test]
    fn test_list_archive_rules_request_construction() {
        let request = ListArchiveRulesRequest {
            api_req: ApiRequest::default(),
            employee_type: "4".to_string(),
            page_size: Some(50),
            page_token: Some("token_123".to_string()),
        };

        assert_eq!(request.employee_type, "4");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token_123".to_string()));
    }

    #[test]
    fn test_list_archive_rules_request_with_none_values() {
        let request = ListArchiveRulesRequest {
            api_req: ApiRequest::default(),
            employee_type: "5".to_string(),
            page_size: None,
            page_token: None,
        };

        assert_eq!(request.employee_type, "5");
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_archive_rule_service_config_independence() {
        let config1 = Config::builder().app_id("archive_app_1").build();

        let config2 = Config::builder().app_id("archive_app_2").build();

        let service1 = ArchiveRuleService { config: config1 };
        let service2 = ArchiveRuleService { config: config2 };

        assert_eq!(service1.config.app_id, "archive_app_1");
        assert_eq!(service2.config.app_id, "archive_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_upload_archive_report_request_with_empty_data() {
        let request = UploadArchiveReportRequest {
            api_req: ApiRequest::default(),
            archive_rule_id: "rule_empty".to_string(),
            employee_type: "1".to_string(),
            report_data: vec![],
        };

        assert_eq!(request.archive_rule_id, "rule_empty");
        assert_eq!(request.employee_type, "1");
        assert!(request.report_data.is_empty());
    }

    #[test]
    fn test_del_archive_report_request_with_single_record() {
        let request = DelArchiveReportRequest {
            api_req: ApiRequest::default(),
            archive_rule_id: "rule_single".to_string(),
            employee_type: "2".to_string(),
            record_ids: vec!["single_id".to_string()],
        };

        assert_eq!(request.archive_rule_id, "rule_single");
        assert_eq!(request.employee_type, "2");
        assert_eq!(request.record_ids.len(), 1);
        assert_eq!(request.record_ids[0], "single_id");
    }

    #[test]
    fn test_request_structs_debug_trait() {
        let query_request = QueryArchiveStatsFieldsRequest {
            api_req: ApiRequest::default(),
            archive_rule_id: "debug_rule".to_string(),
            employee_type: "1".to_string(),
        };

        let debug_str = format!("{:?}", query_request);
        assert!(debug_str.contains("QueryArchiveStatsFieldsRequest"));
        assert!(debug_str.contains("debug_rule"));
    }

    #[test]
    fn test_list_archive_rules_request_edge_cases() {
        // Test with very large page size
        let request_large = ListArchiveRulesRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            page_size: Some(10000),
            page_token: None,
        };

        assert_eq!(request_large.page_size, Some(10000));

        // Test with zero page size
        let request_zero = ListArchiveRulesRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            page_size: Some(0),
            page_token: None,
        };

        assert_eq!(request_zero.page_size, Some(0));

        // Test with very long page token
        let long_token = "a".repeat(1000);
        let request_long_token = ListArchiveRulesRequest {
            api_req: ApiRequest::default(),
            employee_type: "3".to_string(),
            page_size: Some(20),
            page_token: Some(long_token.clone()),
        };

        assert_eq!(request_long_token.page_token, Some(long_token));
    }
}
