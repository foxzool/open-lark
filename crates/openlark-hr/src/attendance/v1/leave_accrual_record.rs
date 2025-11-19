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

use super::models::{PatchLeaveAccrualRecordRequest, PatchLeaveAccrualRecordRespData};

/// 休假发放记录服务
pub struct LeaveAccrualRecordService {
    pub config: Config,
}

impl LeaveAccrualRecordService {
    /// 修改发放记录
    ///
    /// 该接口用于修改指定的休假发放记录信息，包括发放数量、过期时间、发放原因等。
    /// 支持部分字段更新，只需要传入需要修改的字段即可。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/leave_accrual_record/patch
    pub async fn patch(
        &self,
        request: PatchLeaveAccrualRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<PatchLeaveAccrualRecordRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::PATCH);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_LEAVE_ACCRUAL_RECORD_GET,
            "leave_accrual_record_id",
            &request.leave_accrual_record_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "leave_accrual_record": request.leave_accrual_record
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// Builder implementations
impl_executable_builder_owned!(
    PatchLeaveAccrualRecordRequest,
    LeaveAccrualRecordService,
    PatchLeaveAccrualRecordRequest,
    Response<PatchLeaveAccrualRecordRespData>,
    patch
);

impl Service for LeaveAccrualRecordService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "leave_accrual_record"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use openlark_core::{api::ApiRequest, config::Config};
    use crate::service::attendance::v1::models::LeaveAccrualRecordPatch;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.api.com")
            .build()
    }

    fn create_test_service() -> LeaveAccrualRecordService {
        LeaveAccrualRecordService {
            config: create_test_config(),
        }
    }

    fn create_test_request() -> PatchLeaveAccrualRecordRequest {
        PatchLeaveAccrualRecordRequest {
            api_req: ApiRequest::default(),
            employee_type: "employee_id".to_string(),
            leave_accrual_record_id: "record123".to_string(),
            leave_accrual_record: LeaveAccrualRecordPatch {
                employee_id: Some("emp001".to_string()),
                leave_type_id: Some("leave_type_001".to_string()),
                granted_amount: Some(8.0),
                expire_time: Some(1640995200000),
                granted_time: Some(1609459200000),
                granted_reason: Some("Annual leave allocation".to_string()),
                validity_type: Some(2),
                granted_type: Some(1),
                granted_description: Some("Test description".to_string()),
            },
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
    fn test_request_structure() {
        let request = create_test_request();

        assert_eq!(request.employee_type, "employee_id");
        assert_eq!(request.leave_accrual_record_id, "record123");
        assert_eq!(
            request.leave_accrual_record.employee_id,
            Some("emp001".to_string())
        );
        assert_eq!(
            request.leave_accrual_record.leave_type_id,
            Some("leave_type_001".to_string())
        );
        assert_eq!(request.leave_accrual_record.granted_amount, Some(8.0));
        assert_eq!(
            request.leave_accrual_record.expire_time,
            Some(1640995200000)
        );
        assert_eq!(
            request.leave_accrual_record.granted_time,
            Some(1609459200000)
        );
        assert_eq!(
            request.leave_accrual_record.granted_reason,
            Some("Annual leave allocation".to_string())
        );
        assert_eq!(request.leave_accrual_record.validity_type, Some(2));
        assert_eq!(request.leave_accrual_record.granted_type, Some(1));
    }

    #[test]
    fn test_empty_leave_accrual_record_patch() {
        let empty_patch = LeaveAccrualRecordPatch::default();

        assert!(empty_patch.employee_id.is_none());
        assert!(empty_patch.leave_type_id.is_none());
        assert!(empty_patch.granted_amount.is_none());
        assert!(empty_patch.expire_time.is_none());
        assert!(empty_patch.granted_time.is_none());
        assert!(empty_patch.granted_reason.is_none());
        assert!(empty_patch.validity_type.is_none());
        assert!(empty_patch.granted_type.is_none());
    }

    #[test]
    fn test_partial_leave_accrual_record_patch() {
        let partial_patch = LeaveAccrualRecordPatch {
            granted_amount: Some(16.0),
            granted_reason: Some("Updated allocation".to_string()),
            ..Default::default()
        };

        assert!(partial_patch.employee_id.is_none());
        assert!(partial_patch.leave_type_id.is_none());
        assert_eq!(partial_patch.granted_amount, Some(16.0));
        assert!(partial_patch.expire_time.is_none());
        assert!(partial_patch.granted_time.is_none());
        assert_eq!(
            partial_patch.granted_reason,
            Some("Updated allocation".to_string())
        );
        assert!(partial_patch.validity_type.is_none());
        assert!(partial_patch.granted_type.is_none());
    }

    #[test]
    fn test_different_employee_types() {
        let employee_types = vec!["employee_id", "employee_no", "open_id", "union_id"];

        for emp_type in employee_types {
            let request = PatchLeaveAccrualRecordRequest {
                api_req: ApiRequest::default(),
                employee_type: emp_type.to_string(),
                leave_accrual_record_id: "record123".to_string(),
                leave_accrual_record: LeaveAccrualRecordPatch::default(),
            };

            assert_eq!(request.employee_type, emp_type);
        }
    }

    #[test]
    fn test_different_leave_accrual_record_ids() {
        let record_ids = vec![
            "simple_id",
            "complex-record-id_123",
            "record@with#special$chars",
            "很长的中文记录标识符",
            "record🚀with🔐emoji",
            "",
        ];

        for record_id in record_ids {
            let request = PatchLeaveAccrualRecordRequest {
                api_req: ApiRequest::default(),
                employee_type: "employee_id".to_string(),
                leave_accrual_record_id: record_id.to_string(),
                leave_accrual_record: LeaveAccrualRecordPatch::default(),
            };

            assert_eq!(request.leave_accrual_record_id, record_id);
        }
    }

    #[test]
    fn test_granted_amount_variations() {
        let amounts = vec![
            Some(0.0),
            Some(0.5),
            Some(8.0),
            Some(24.0),
            Some(999.99),
            Some(-1.0), // Edge case: negative amount
            None,
        ];

        for amount in amounts {
            let patch = LeaveAccrualRecordPatch {
                granted_amount: amount,
                ..Default::default()
            };

            assert_eq!(patch.granted_amount, amount);
        }
    }

    #[test]
    fn test_timestamp_variations() {
        let timestamps = vec![
            Some(0i64),
            Some(1609459200000i64), // 2021-01-01
            Some(1640995200000i64), // 2022-01-01
            Some(2556144000000i64), // 2051-01-01
            Some(-1i64),            // Edge case: negative timestamp
            None,
        ];

        for timestamp in timestamps.clone() {
            let patch = LeaveAccrualRecordPatch {
                expire_time: timestamp,
                ..Default::default()
            };
            assert_eq!(patch.expire_time, timestamp);
        }

        for timestamp in timestamps {
            let patch = LeaveAccrualRecordPatch {
                granted_time: timestamp,
                ..Default::default()
            };
            assert_eq!(patch.granted_time, timestamp);
        }
    }

    #[test]
    fn test_validity_type_variations() {
        let validity_types = vec![
            Some(1),   // 永久有效
            Some(2),   // 指定过期时间
            Some(0),   // Edge case: invalid type
            Some(-1),  // Edge case: negative type
            Some(999), // Edge case: large number
            None,
        ];

        for validity_type in validity_types {
            let patch = LeaveAccrualRecordPatch {
                validity_type,
                ..Default::default()
            };

            assert_eq!(patch.validity_type, validity_type);
        }
    }

    #[test]
    fn test_granted_type_variations() {
        let granted_types = vec![
            Some(1),   // 系统自动发放
            Some(2),   // 管理员手动发放
            Some(3),   // 员工申请发放
            Some(0),   // Edge case: invalid type
            Some(-1),  // Edge case: negative type
            Some(999), // Edge case: large number
            None,
        ];

        for granted_type in granted_types {
            let patch = LeaveAccrualRecordPatch {
                granted_type,
                ..Default::default()
            };

            assert_eq!(patch.granted_type, granted_type);
        }
    }

    #[test]
    fn test_granted_reason_variations() {
        let reasons = vec![
            Some("Annual leave allocation".to_string()),
            Some("Manual adjustment".to_string()),
            Some("System error correction".to_string()),
            Some("中文原因说明".to_string()),
            Some("Reason with special chars: @#$%^&*()".to_string()),
            Some("🎯 Emoji in reason".to_string()),
            Some("".to_string()), // Empty string
            Some("Very long reason that might exceed normal limits and should still be handled properly by the system without causing any issues or truncation problems".to_string()),
            None,
        ];

        for reason in reasons {
            let patch = LeaveAccrualRecordPatch {
                granted_reason: reason.clone(),
                ..Default::default()
            };

            assert_eq!(patch.granted_reason, reason);
        }
    }

    #[test]
    fn test_employee_id_variations() {
        let employee_ids = vec![
            Some("emp001".to_string()),
            Some("employee-with-dashes".to_string()),
            Some("employee_with_underscores".to_string()),
            Some("employee.with.dots".to_string()),
            Some("员工001".to_string()),
            Some("emp🚀001".to_string()),
            Some("".to_string()),
            None,
        ];

        for employee_id in employee_ids {
            let patch = LeaveAccrualRecordPatch {
                employee_id: employee_id.clone(),
                ..Default::default()
            };

            assert_eq!(patch.employee_id, employee_id);
        }
    }

    #[test]
    fn test_leave_type_id_variations() {
        let leave_type_ids = vec![
            Some("annual_leave".to_string()),
            Some("sick_leave".to_string()),
            Some("personal_leave".to_string()),
            Some("maternity_leave".to_string()),
            Some("leave-type-001".to_string()),
            Some("请假类型_001".to_string()),
            Some("leave🏖️type".to_string()),
            Some("".to_string()),
            None,
        ];

        for leave_type_id in leave_type_ids {
            let patch = LeaveAccrualRecordPatch {
                leave_type_id: leave_type_id.clone(),
                ..Default::default()
            };

            assert_eq!(patch.leave_type_id, leave_type_id);
        }
    }

    #[test]
    fn test_leave_accrual_record_patch_clone() {
        let original_patch = LeaveAccrualRecordPatch {
            employee_id: Some("emp001".to_string()),
            leave_type_id: Some("annual_leave".to_string()),
            granted_amount: Some(8.0),
            expire_time: Some(1640995200000),
            granted_time: Some(1609459200000),
            granted_reason: Some("Test allocation".to_string()),
            validity_type: Some(2),
            granted_type: Some(1),
            granted_description: Some("Test clone".to_string()),
        };

        let cloned_patch = original_patch.clone();

        assert_eq!(original_patch.employee_id, cloned_patch.employee_id);
        assert_eq!(original_patch.leave_type_id, cloned_patch.leave_type_id);
        assert_eq!(original_patch.granted_amount, cloned_patch.granted_amount);
        assert_eq!(original_patch.expire_time, cloned_patch.expire_time);
        assert_eq!(original_patch.granted_time, cloned_patch.granted_time);
        assert_eq!(original_patch.granted_reason, cloned_patch.granted_reason);
        assert_eq!(original_patch.validity_type, cloned_patch.validity_type);
        assert_eq!(original_patch.granted_type, cloned_patch.granted_type);
    }

    #[test]
    fn test_leave_accrual_record_patch_debug() {
        let patch = LeaveAccrualRecordPatch {
            employee_id: Some("emp001".to_string()),
            granted_amount: Some(8.0),
            ..Default::default()
        };

        let debug_str = format!("{:?}", patch);
        assert!(debug_str.contains("LeaveAccrualRecordPatch"));
        assert!(debug_str.contains("emp001"));
        assert!(debug_str.contains("8.0"));
    }

    #[test]
    fn test_request_build_method() {
        let request = create_test_request();
        let built_request = request.build();

        assert_eq!(built_request.employee_type, "employee_id");
        assert_eq!(built_request.leave_accrual_record_id, "record123");
    }

    #[test]
    fn test_json_serialization() {
        let patch = LeaveAccrualRecordPatch {
            employee_id: Some("emp001".to_string()),
            leave_type_id: Some("annual_leave".to_string()),
            granted_amount: Some(8.0),
            expire_time: Some(1640995200000),
            granted_time: Some(1609459200000),
            granted_reason: Some("Test allocation".to_string()),
            validity_type: Some(2),
            granted_type: Some(1),
            granted_description: Some("Test serialization".to_string()),
        };

        // Test serialization
        let json_result = serde_json::to_string(&patch);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("emp001"));
        assert!(json_str.contains("annual_leave"));
        assert!(json_str.contains("8.0") || json_str.contains("8"));
        assert!(json_str.contains("Test allocation"));

        // Test deserialization
        let deserialized_result: Result<LeaveAccrualRecordPatch, _> =
            serde_json::from_str(&json_str);
        assert!(deserialized_result.is_ok());

        let deserialized_patch = deserialized_result.unwrap();
        assert_eq!(deserialized_patch.employee_id, patch.employee_id);
        assert_eq!(deserialized_patch.leave_type_id, patch.leave_type_id);
        assert_eq!(deserialized_patch.granted_amount, patch.granted_amount);
        assert_eq!(deserialized_patch.expire_time, patch.expire_time);
        assert_eq!(deserialized_patch.granted_time, patch.granted_time);
        assert_eq!(deserialized_patch.granted_reason, patch.granted_reason);
        assert_eq!(deserialized_patch.validity_type, patch.validity_type);
        assert_eq!(deserialized_patch.granted_type, patch.granted_type);
    }

    #[test]
    fn test_json_serialization_with_nulls() {
        let patch = LeaveAccrualRecordPatch {
            employee_id: Some("emp001".to_string()),
            leave_type_id: None,
            granted_amount: Some(8.0),
            expire_time: None,
            granted_time: None,
            granted_reason: None,
            validity_type: None,
            granted_type: Some(1),
            granted_description: None,
        };

        let json_result = serde_json::to_string(&patch);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("emp001"));
        assert!(json_str.contains("null") || !json_str.contains("leave_type_id"));
    }

    #[test]
    fn test_memory_efficiency() {
        // Create multiple request instances to test memory usage
        let requests: Vec<PatchLeaveAccrualRecordRequest> = (0..100)
            .map(|i| PatchLeaveAccrualRecordRequest {
                api_req: ApiRequest::default(),
                employee_type: "employee_id".to_string(),
                leave_accrual_record_id: format!("record_{}", i),
                leave_accrual_record: LeaveAccrualRecordPatch {
                    employee_id: Some(format!("emp_{}", i)),
                    granted_amount: Some(8.0 + i as f64),
                    ..Default::default()
                },
            })
            .collect();

        assert_eq!(requests.len(), 100);

        for (i, request) in requests.iter().enumerate() {
            assert_eq!(request.leave_accrual_record_id, format!("record_{}", i));
            assert_eq!(
                request.leave_accrual_record.employee_id,
                Some(format!("emp_{}", i))
            );
            assert_eq!(
                request.leave_accrual_record.granted_amount,
                Some(8.0 + i as f64)
            );
        }
    }
}
