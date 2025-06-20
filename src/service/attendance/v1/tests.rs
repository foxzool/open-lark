//! 考勤模块集成测试

#[cfg(test)]
mod integration_tests {
    use super::super::models::*;
    use chrono::{NaiveDate, Utc};

    #[test]
    fn test_user_task_query_request_creation() {
        let req = UserTaskQueryRequest::builder()
            .user_ids(vec!["user_1".to_string(), "user_2".to_string()])
            .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
            .need_absent_info(true)
            .need_supplement_info(true)
            .page_size(50)
            .build();

        assert_eq!(req.user_ids.len(), 2);
        assert_eq!(req.user_ids[0], "user_1");
        assert_eq!(
            req.check_date_from,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
        );
        assert_eq!(
            req.check_date_to,
            NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()
        );
        assert_eq!(req.need_absent_info, Some(true));
        assert_eq!(req.need_supplement_info, Some(true));
        assert_eq!(req.page_size, Some(50));
    }

    #[test]
    fn test_user_flow_query_request_creation() {
        let from_time = Utc::now() - chrono::Duration::days(7);
        let to_time = Utc::now();

        let req = UserFlowQueryRequest::builder()
            .user_ids(vec!["user_1".to_string()])
            .check_time_from(from_time)
            .check_time_to(to_time)
            .page_size(100)
            .build();

        assert_eq!(req.user_ids.len(), 1);
        assert_eq!(req.check_time_from, from_time);
        assert_eq!(req.check_time_to, to_time);
        assert_eq!(req.page_size, Some(100));
    }

    #[test]
    fn test_enum_serialization() {
        // 测试 CheckInStatus 序列化
        let status = CheckInStatus::Normal;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Normal\"");

        let deserialized: CheckInStatus = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, CheckInStatus::Normal));

        // 测试 CheckMethod 序列化
        let method = CheckMethod::GPS;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"GPS\"");

        let deserialized: CheckMethod = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, CheckMethod::GPS));

        // 测试 CheckType 序列化
        let check_type = CheckType::CheckIn;
        let json = serde_json::to_string(&check_type).unwrap();
        assert_eq!(json, "\"CheckIn\"");

        let deserialized: CheckType = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, CheckType::CheckIn));
    }

    #[test]
    fn test_user_task_record_structure() {
        let record = UserTaskRecord {
            user_id: "test_user".to_string(),
            task_id: "task_123".to_string(),
            date: NaiveDate::from_ymd_opt(2025, 1, 20).unwrap(),
            shift_id: "shift_456".to_string(),
            shift_name: Some("标准班次".to_string()),
            check_in_result: CheckInResult {
                result: CheckInStatus::Normal,
                time: Some(Utc::now()),
                location_info: None,
                exception_type: Some(ExceptionType::NoException),
            },
            check_out_result: CheckOutResult {
                result: CheckInStatus::Normal,
                time: Some(Utc::now()),
                location_info: None,
                exception_type: Some(ExceptionType::NoException),
            },
            absent_info: None,
            supplement_info: None,
        };

        assert_eq!(record.user_id, "test_user");
        assert_eq!(record.task_id, "task_123");
        assert_eq!(record.shift_name, Some("标准班次".to_string()));
        assert!(matches!(
            record.check_in_result.result,
            CheckInStatus::Normal
        ));
    }

    #[test]
    fn test_location_info_structure() {
        let location = LocationInfo {
            name: Some("公司总部".to_string()),
            longitude: Some(116.407526),
            latitude: Some(39.904030),
            address: Some("北京市朝阳区".to_string()),
        };

        assert_eq!(location.name, Some("公司总部".to_string()));
        assert!(location.longitude.is_some());
        assert!(location.latitude.is_some());
        assert_eq!(location.address, Some("北京市朝阳区".to_string()));
    }

    #[test]
    fn test_user_flow_record_structure() {
        let record = UserFlowRecord {
            user_id: "test_user".to_string(),
            check_time: Utc::now(),
            check_type: CheckType::CheckIn,
            check_method: CheckMethod::Mobile,
            location_info: Some(LocationInfo {
                name: Some("办公室".to_string()),
                longitude: Some(116.407526),
                latitude: Some(39.904030),
                address: Some("北京市朝阳区".to_string()),
            }),
            device_info: Some(DeviceInfo {
                device_id: Some("device_123".to_string()),
                device_name: Some("iPhone 15".to_string()),
                device_type: Some("mobile".to_string()),
            }),
            photo_info: None,
        };

        assert_eq!(record.user_id, "test_user");
        assert!(matches!(record.check_type, CheckType::CheckIn));
        assert!(matches!(record.check_method, CheckMethod::Mobile));
        assert!(record.location_info.is_some());
        assert!(record.device_info.is_some());
    }

    #[test]
    fn test_request_builders_with_minimal_fields() {
        // 测试仅包含必需字段的请求构建
        let task_req = UserTaskQueryRequest::builder()
            .user_ids(vec!["user1".to_string()])
            .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
            .build();

        assert_eq!(task_req.user_ids.len(), 1);
        assert!(task_req.need_absent_info.is_none());
        assert!(task_req.need_supplement_info.is_none());
        assert!(task_req.page_token.is_none());
        assert!(task_req.page_size.is_none());

        let flow_req = UserFlowQueryRequest::builder()
            .user_ids(vec!["user1".to_string()])
            .check_time_from(Utc::now() - chrono::Duration::days(1))
            .check_time_to(Utc::now())
            .build();

        assert_eq!(flow_req.user_ids.len(), 1);
        assert!(flow_req.page_token.is_none());
        assert!(flow_req.page_size.is_none());
    }
}
