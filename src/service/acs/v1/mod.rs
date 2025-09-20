//! # ACS v1 事件
//!
//! 智能门禁系统的事件定义，包括用户信息变更和门禁访问记录事件。

use crate::service::acs::models::{AccessRecord, AcsUser};
use serde::{Deserialize, Serialize};

/// 用户信息变更事件 (p2_acs_user_updated_v1)
///
/// 当门禁用户信息发生变更时触发此事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2AcsUserUpdatedV1 {
    /// 事件ID
    pub event_id: String,
    /// 事件类型
    pub event_type: String,
    /// 事件时间戳
    pub created_time: String,
    /// 事件数据
    pub event: AcsUserUpdatedEvent,
}

/// 用户信息变更事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcsUserUpdatedEvent {
    /// 变更类型 (created, updated, deleted)
    pub change_type: String,
    /// 变更前的用户信息（更新/删除时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_user: Option<AcsUser>,
    /// 变更后的用户信息（创建/更新时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_user: Option<AcsUser>,
}

/// 新增门禁访问记录事件 (p2_acs_access_record_created_v1)
///
/// 当产生新的门禁访问记录时触发此事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2AcsAccessRecordCreatedV1 {
    /// 事件ID
    pub event_id: String,
    /// 事件类型
    pub event_type: String,
    /// 事件时间戳
    pub created_time: String,
    /// 事件数据
    pub event: AccessRecordCreatedEvent,
}

/// 门禁访问记录创建事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRecordCreatedEvent {
    /// 新增的访问记录
    pub access_record: AccessRecord,
    /// 是否为异常访问
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_abnormal: Option<bool>,
    /// 异常原因（如果是异常访问）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abnormal_reason: Option<String>,
}

// 为事件实现 Display trait，便于日志输出
impl std::fmt::Display for P2AcsUserUpdatedV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACS User Updated Event [{}]: {} at {}",
            self.event_id, self.event.change_type, self.created_time
        )
    }
}

impl std::fmt::Display for P2AcsAccessRecordCreatedV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACS Access Record Created Event [{}]: user {} accessed device {} at {}",
            self.event_id,
            self.event
                .access_record
                .user_id
                .as_deref()
                .unwrap_or("unknown"),
            self.event.access_record.device_id,
            self.created_time
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::acs::models::{
        AccessMethod, AccessResult, AccessType, UserStatus, UserType,
    };

    #[test]
    fn test_p2_acs_user_updated_v1_creation() {
        let user = AcsUser {
            user_id: "user_123".to_string(),
            employee_id: Some("emp_123".to_string()),
            name: "张三".to_string(),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            department: Some("技术部".to_string()),
            phone: Some("13800138000".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            has_face_image: Some(true),
            created_at: Some(1640995200),
            updated_at: Some(1640995260),
        };

        let event_data = AcsUserUpdatedEvent {
            change_type: "created".to_string(),
            old_user: None,
            new_user: Some(user),
        };

        let event = P2AcsUserUpdatedV1 {
            event_id: "event_123".to_string(),
            event_type: "p2_acs_user_updated_v1".to_string(),
            created_time: "2021-12-31T16:00:00Z".to_string(),
            event: event_data,
        };

        assert_eq!(event.event_id, "event_123");
        assert_eq!(event.event_type, "p2_acs_user_updated_v1");
        assert_eq!(event.created_time, "2021-12-31T16:00:00Z");
        assert_eq!(event.event.change_type, "created");
        assert!(event.event.old_user.is_none());
        assert!(event.event.new_user.is_some());
    }

    #[test]
    fn test_p2_acs_access_record_created_v1_creation() {
        let access_record = AccessRecord {
            record_id: "record_123".to_string(),
            user_id: Some("user_456".to_string()),
            user_name: Some("测试用户".to_string()),
            device_id: "device_789".to_string(),
            device_name: Some("主入口设备".to_string()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::Card),
            result: AccessResult::Success,
            has_face_image: Some(false),
            access_time: 1640995200,
            created_at: Some(1640995200),
        };

        let event_data = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: Some(false),
            abnormal_reason: None,
        };

        let event = P2AcsAccessRecordCreatedV1 {
            event_id: "event_456".to_string(),
            event_type: "p2_acs_access_record_created_v1".to_string(),
            created_time: "2021-12-31T16:00:00Z".to_string(),
            event: event_data,
        };

        assert_eq!(event.event_id, "event_456");
        assert_eq!(event.event_type, "p2_acs_access_record_created_v1");
        assert_eq!(event.created_time, "2021-12-31T16:00:00Z");
        assert_eq!(event.event.access_record.record_id, "record_123");
        assert_eq!(event.event.is_abnormal, Some(false));
        assert!(event.event.abnormal_reason.is_none());
    }

    #[test]
    fn test_acs_user_updated_event_creation() {
        let old_user = AcsUser {
            user_id: "user_123".to_string(),
            employee_id: Some("emp_old".to_string()),
            name: "旧姓名".to_string(),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            department: Some("旧部门".to_string()),
            phone: Some("13800138000".to_string()),
            email: Some("old@example.com".to_string()),
            has_face_image: Some(false),
            created_at: Some(1640995200),
            updated_at: Some(1640995200),
        };

        let new_user = AcsUser {
            user_id: "user_123".to_string(),
            employee_id: Some("emp_new".to_string()),
            name: "新姓名".to_string(),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            department: Some("新部门".to_string()),
            phone: Some("13800138001".to_string()),
            email: Some("new@example.com".to_string()),
            has_face_image: Some(false),
            created_at: Some(1640995200),
            updated_at: Some(1640995260),
        };

        let event = AcsUserUpdatedEvent {
            change_type: "updated".to_string(),
            old_user: Some(old_user),
            new_user: Some(new_user),
        };

        assert_eq!(event.change_type, "updated");
        assert!(event.old_user.is_some());
        assert!(event.new_user.is_some());

        let old = event.old_user.as_ref().unwrap();
        let new = event.new_user.as_ref().unwrap();
        assert_eq!(old.name, "旧姓名");
        assert_eq!(new.name, "新姓名");
    }

    #[test]
    fn test_access_record_created_event_creation() {
        let access_record = AccessRecord {
            record_id: "record_789".to_string(),
            user_id: Some("user_999".to_string()),
            user_name: Some("失败用户".to_string()),
            device_id: "device_888".to_string(),
            device_name: Some("后门设备".to_string()),
            access_type: Some(AccessType::Exit),
            access_method: Some(AccessMethod::Fingerprint),
            result: AccessResult::Failed,
            has_face_image: Some(true),
            access_time: 1640995200,
            created_at: Some(1640995200),
        };

        let event = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: Some(true),
            abnormal_reason: Some("多次尝试失败".to_string()),
        };

        assert_eq!(event.access_record.record_id, "record_789");
        assert_eq!(event.is_abnormal, Some(true));
        assert_eq!(event.abnormal_reason.as_ref().unwrap(), "多次尝试失败");
    }

    #[test]
    fn test_p2_acs_user_updated_v1_display() {
        let event_data = AcsUserUpdatedEvent {
            change_type: "deleted".to_string(),
            old_user: None,
            new_user: None,
        };

        let event = P2AcsUserUpdatedV1 {
            event_id: "evt_display_test".to_string(),
            event_type: "p2_acs_user_updated_v1".to_string(),
            created_time: "2023-01-01T10:30:00Z".to_string(),
            event: event_data,
        };

        let display_str = format!("{}", event);
        assert!(display_str.contains("ACS User Updated Event"));
        assert!(display_str.contains("evt_display_test"));
        assert!(display_str.contains("deleted"));
        assert!(display_str.contains("2023-01-01T10:30:00Z"));
    }

    #[test]
    fn test_p2_acs_access_record_created_v1_display() {
        let access_record = AccessRecord {
            record_id: "record_display".to_string(),
            user_id: Some("user_display".to_string()),
            user_name: Some("显示用户".to_string()),
            device_id: "device_display".to_string(),
            device_name: Some("显示设备".to_string()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::Card),
            result: AccessResult::Success,
            has_face_image: None,
            access_time: 1672574600,
            created_at: None,
        };

        let event_data = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: None,
            abnormal_reason: None,
        };

        let event = P2AcsAccessRecordCreatedV1 {
            event_id: "evt_access_display".to_string(),
            event_type: "p2_acs_access_record_created_v1".to_string(),
            created_time: "2023-01-01T10:30:00Z".to_string(),
            event: event_data,
        };

        let display_str = format!("{}", event);
        assert!(display_str.contains("ACS Access Record Created Event"));
        assert!(display_str.contains("evt_access_display"));
        assert!(display_str.contains("user_display"));
        assert!(display_str.contains("device_display"));
        assert!(display_str.contains("2023-01-01T10:30:00Z"));
    }

    #[test]
    fn test_p2_acs_access_record_created_v1_display_with_unknown_user() {
        let access_record = AccessRecord {
            record_id: "record_no_user".to_string(),
            user_id: None,
            user_name: None,
            device_id: "device_123".to_string(),
            device_name: Some("测试设备".to_string()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::Card),
            result: AccessResult::Failed,
            has_face_image: None,
            access_time: 1672574600,
            created_at: None,
        };

        let event_data = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: Some(true),
            abnormal_reason: Some("无效卡片".to_string()),
        };

        let event = P2AcsAccessRecordCreatedV1 {
            event_id: "evt_no_user".to_string(),
            event_type: "p2_acs_access_record_created_v1".to_string(),
            created_time: "2023-01-01T11:00:00Z".to_string(),
            event: event_data,
        };

        let display_str = format!("{}", event);
        assert!(display_str.contains("unknown"));
        assert!(display_str.contains("device_123"));
    }

    #[test]
    fn test_p2_acs_user_updated_v1_debug() {
        let event_data = AcsUserUpdatedEvent {
            change_type: "created".to_string(),
            old_user: None,
            new_user: None,
        };

        let event = P2AcsUserUpdatedV1 {
            event_id: "debug_test".to_string(),
            event_type: "p2_acs_user_updated_v1".to_string(),
            created_time: "2023-01-01T00:00:00Z".to_string(),
            event: event_data,
        };

        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("P2AcsUserUpdatedV1"));
        assert!(debug_str.contains("debug_test"));
        assert!(debug_str.contains("created"));
    }

    #[test]
    fn test_p2_acs_access_record_created_v1_debug() {
        let access_record = AccessRecord {
            record_id: "debug_record".to_string(),
            user_id: None,
            user_name: None,
            device_id: "debug_device".to_string(),
            device_name: None,
            access_type: None,
            access_method: None,
            result: AccessResult::Failed,
            has_face_image: None,
            access_time: 1672574600,
            created_at: None,
        };

        let event_data = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: None,
            abnormal_reason: None,
        };

        let event = P2AcsAccessRecordCreatedV1 {
            event_id: "debug_event".to_string(),
            event_type: "p2_acs_access_record_created_v1".to_string(),
            created_time: "2023-01-01T12:00:00Z".to_string(),
            event: event_data,
        };

        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("P2AcsAccessRecordCreatedV1"));
        assert!(debug_str.contains("debug_event"));
        assert!(debug_str.contains("debug_device"));
    }

    #[test]
    fn test_p2_acs_user_updated_v1_clone() {
        let event_data = AcsUserUpdatedEvent {
            change_type: "updated".to_string(),
            old_user: None,
            new_user: None,
        };

        let event = P2AcsUserUpdatedV1 {
            event_id: "clone_test".to_string(),
            event_type: "p2_acs_user_updated_v1".to_string(),
            created_time: "2023-01-01T00:00:00Z".to_string(),
            event: event_data,
        };

        let cloned = event.clone();
        assert_eq!(event.event_id, cloned.event_id);
        assert_eq!(event.event_type, cloned.event_type);
        assert_eq!(event.created_time, cloned.created_time);
        assert_eq!(event.event.change_type, cloned.event.change_type);
    }

    #[test]
    fn test_acs_user_updated_event_clone() {
        let user = AcsUser {
            user_id: "clone_user".to_string(),
            employee_id: None,
            name: "克隆测试".to_string(),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            department: None,
            phone: None,
            email: None,
            has_face_image: None,
            created_at: None,
            updated_at: None,
        };

        let event = AcsUserUpdatedEvent {
            change_type: "created".to_string(),
            old_user: None,
            new_user: Some(user),
        };

        let cloned = event.clone();
        assert_eq!(event.change_type, cloned.change_type);
        assert_eq!(event.old_user.is_none(), cloned.old_user.is_none());
        assert_eq!(event.new_user.is_some(), cloned.new_user.is_some());

        if let (Some(original), Some(cloned_user)) = (&event.new_user, &cloned.new_user) {
            assert_eq!(original.user_id, cloned_user.user_id);
            assert_eq!(original.name, cloned_user.name);
        }
    }

    #[test]
    fn test_access_record_created_event_clone() {
        let access_record = AccessRecord {
            record_id: "clone_record".to_string(),
            user_id: Some("clone_user".to_string()),
            user_name: Some("克隆用户".to_string()),
            device_id: "clone_device".to_string(),
            device_name: Some("克隆设备".to_string()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::Card),
            result: AccessResult::Success,
            has_face_image: Some(false),
            access_time: 1672574600,
            created_at: Some(1672574600),
        };

        let event = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: Some(false),
            abnormal_reason: None,
        };

        let cloned = event.clone();
        assert_eq!(
            event.access_record.record_id,
            cloned.access_record.record_id
        );
        assert_eq!(
            event.access_record.device_id,
            cloned.access_record.device_id
        );
        assert_eq!(event.is_abnormal, cloned.is_abnormal);
        assert_eq!(event.abnormal_reason, cloned.abnormal_reason);
    }

    #[test]
    fn test_p2_acs_user_updated_v1_serialization() {
        let user = AcsUser {
            user_id: "ser_user".to_string(),
            employee_id: Some("ser_emp".to_string()),
            name: "序列化测试".to_string(),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            department: Some("测试部".to_string()),
            phone: Some("13800000000".to_string()),
            email: Some("serialize@test.com".to_string()),
            has_face_image: Some(false),
            created_at: Some(1640995200),
            updated_at: Some(1640995260),
        };

        let event_data = AcsUserUpdatedEvent {
            change_type: "created".to_string(),
            old_user: None,
            new_user: Some(user),
        };

        let event = P2AcsUserUpdatedV1 {
            event_id: "ser_event".to_string(),
            event_type: "p2_acs_user_updated_v1".to_string(),
            created_time: "2021-12-31T16:00:00Z".to_string(),
            event: event_data,
        };

        let json_str = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json_str.contains("ser_event"));
        assert!(json_str.contains("p2_acs_user_updated_v1"));
        assert!(json_str.contains("序列化测试"));
        assert!(json_str.contains("created"));
    }

    #[test]
    fn test_p2_acs_user_updated_v1_deserialization() {
        let json_str = r#"{
            "event_id": "deser_event",
            "event_type": "p2_acs_user_updated_v1",
            "created_time": "2023-01-01T10:00:00Z",
            "event": {
                "change_type": "updated",
                "old_user": null,
                "new_user": {
                    "user_id": "deser_user",
                    "employee_id": "deser_emp",
                    "name": "反序列化测试",
                    "user_type": "employee",
                    "status": "active",
                    "department": "测试部门",
                    "phone": "13900000000",
                    "email": "deserialize@test.com",
                    "has_face_image": false,
                    "created_at": 1640995200,
                    "updated_at": 1640995260
                }
            }
        }"#;

        let event: P2AcsUserUpdatedV1 =
            serde_json::from_str(json_str).expect("Failed to deserialize");
        assert_eq!(event.event_id, "deser_event");
        assert_eq!(event.event_type, "p2_acs_user_updated_v1");
        assert_eq!(event.event.change_type, "updated");

        let user = event.event.new_user.expect("new_user should exist");
        assert_eq!(user.user_id, "deser_user");
        assert_eq!(user.name, "反序列化测试");
        assert_eq!(user.email.as_ref().unwrap(), "deserialize@test.com");
    }

    #[test]
    fn test_p2_acs_access_record_created_v1_serialization() {
        let access_record = AccessRecord {
            record_id: "ser_record".to_string(),
            user_id: Some("ser_user".to_string()),
            user_name: Some("序列化用户".to_string()),
            device_id: "ser_device".to_string(),
            device_name: Some("序列化设备".to_string()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::Card),
            result: AccessResult::Success,
            has_face_image: Some(true),
            access_time: 1672574600,
            created_at: Some(1672574600),
        };

        let event_data = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: Some(false),
            abnormal_reason: None,
        };

        let event = P2AcsAccessRecordCreatedV1 {
            event_id: "ser_access_event".to_string(),
            event_type: "p2_acs_access_record_created_v1".to_string(),
            created_time: "2023-01-01T10:30:00Z".to_string(),
            event: event_data,
        };

        let json_str = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json_str.contains("ser_access_event"));
        assert!(json_str.contains("p2_acs_access_record_created_v1"));
        assert!(json_str.contains("ser_record"));
        assert!(json_str.contains("序列化用户"));
    }

    #[test]
    fn test_p2_acs_access_record_created_v1_deserialization() {
        let json_str = r#"{
            "event_id": "deser_access_event",
            "event_type": "p2_acs_access_record_created_v1",
            "created_time": "2023-01-01T11:00:00Z",
            "event": {
                "access_record": {
                    "record_id": "deser_record",
                    "user_id": "deser_user",
                    "user_name": "反序列化用户",
                    "device_id": "deser_device",
                    "device_name": "反序列化设备",
                    "access_type": "entry",
                    "access_method": "fingerprint",
                    "result": "success",
                    "has_face_image": true,
                    "access_time": 1672574600,
                    "created_at": 1672574600
                },
                "is_abnormal": true,
                "abnormal_reason": "异常访问测试"
            }
        }"#;

        let event: P2AcsAccessRecordCreatedV1 =
            serde_json::from_str(json_str).expect("Failed to deserialize");
        assert_eq!(event.event_id, "deser_access_event");
        assert_eq!(event.event_type, "p2_acs_access_record_created_v1");

        let record = &event.event.access_record;
        assert_eq!(record.record_id, "deser_record");
        assert_eq!(record.user_id.as_ref().unwrap(), "deser_user");
        assert_eq!(record.device_id, "deser_device");

        assert_eq!(event.event.is_abnormal, Some(true));
        assert_eq!(
            event.event.abnormal_reason.as_ref().unwrap(),
            "异常访问测试"
        );
    }

    #[test]
    fn test_acs_user_updated_event_serialization_skip_none() {
        let event = AcsUserUpdatedEvent {
            change_type: "deleted".to_string(),
            old_user: None,
            new_user: None,
        };

        let json_str = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json_str.contains("deleted"));
        // 验证 None 值被跳过序列化
        assert!(!json_str.contains("old_user"));
        assert!(!json_str.contains("new_user"));
    }

    #[test]
    fn test_access_record_created_event_serialization_skip_none() {
        let access_record = AccessRecord {
            record_id: "skip_test".to_string(),
            user_id: None,
            user_name: None,
            device_id: "device_skip".to_string(),
            device_name: None,
            access_type: None,
            access_method: None,
            result: AccessResult::Failed,
            has_face_image: None,
            access_time: 1672574600,
            created_at: None,
        };

        let event = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: None,
            abnormal_reason: None,
        };

        let json_str = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json_str.contains("skip_test"));
        assert!(json_str.contains("device_skip"));
        // 验证 None 值被跳过序列化
        assert!(!json_str.contains("is_abnormal"));
        assert!(!json_str.contains("abnormal_reason"));
    }

    #[test]
    fn test_event_with_unicode_content() {
        let user = AcsUser {
            user_id: "unicode_user_测试".to_string(),
            employee_id: Some("员工_ID".to_string()),
            name: "用户姓名_🔐".to_string(),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            department: Some("部门_测试".to_string()),
            phone: Some("电话号码".to_string()),
            email: Some("测试@example.com".to_string()),
            has_face_image: Some(true),
            created_at: Some(1640995200),
            updated_at: Some(1640995260),
        };

        let event_data = AcsUserUpdatedEvent {
            change_type: "创建".to_string(),
            old_user: None,
            new_user: Some(user),
        };

        let event = P2AcsUserUpdatedV1 {
            event_id: "事件_ID_🎯".to_string(),
            event_type: "p2_acs_user_updated_v1".to_string(),
            created_time: "2021-12-31T16:00:00Z".to_string(),
            event: event_data,
        };

        // 验证 Unicode 字符处理
        assert_eq!(event.event_id, "事件_ID_🎯");
        assert_eq!(event.event.change_type, "创建");

        let user = event.event.new_user.as_ref().unwrap();
        assert_eq!(user.name, "用户姓名_🔐");
        assert_eq!(user.email.as_ref().unwrap(), "测试@example.com");

        // 测试序列化和反序列化
        let json_str = serde_json::to_string(&event).expect("Failed to serialize Unicode");
        let deserialized: P2AcsUserUpdatedV1 =
            serde_json::from_str(&json_str).expect("Failed to deserialize Unicode");
        assert_eq!(deserialized.event_id, "事件_ID_🎯");
        assert_eq!(
            deserialized.event.new_user.as_ref().unwrap().name,
            "用户姓名_🔐"
        );
    }

    #[test]
    fn test_event_with_special_characters() {
        let access_record = AccessRecord {
            record_id: "record_!@#$%^&*()".to_string(),
            user_id: Some("user_<>?:\"{}|".to_string()),
            user_name: Some("特殊用户_!@#".to_string()),
            device_id: "device_[]\\;',./:_+=".to_string(),
            device_name: Some("设备_特殊字符".to_string()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::Password),
            result: AccessResult::Failed,
            has_face_image: Some(false),
            access_time: 1672574600,
            created_at: Some(1672574600),
        };

        let event_data = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: Some(true),
            abnormal_reason: Some("原因_特殊字符!@#$%^&*()".to_string()),
        };

        let event = P2AcsAccessRecordCreatedV1 {
            event_id: "event_!@#$%^&*()".to_string(),
            event_type: "p2_acs_access_record_created_v1".to_string(),
            created_time: "2023-01-01T10:30:00Z".to_string(),
            event: event_data,
        };

        // 验证特殊字符处理
        assert_eq!(event.event_id, "event_!@#$%^&*()");
        assert_eq!(event.event.access_record.record_id, "record_!@#$%^&*()");
        assert_eq!(
            event.event.abnormal_reason.as_ref().unwrap(),
            "原因_特殊字符!@#$%^&*()"
        );

        // 测试序列化和反序列化
        let json_str = serde_json::to_string(&event).expect("Failed to serialize special chars");
        let deserialized: P2AcsAccessRecordCreatedV1 =
            serde_json::from_str(&json_str).expect("Failed to deserialize special chars");
        assert_eq!(deserialized.event_id, "event_!@#$%^&*()");
        assert_eq!(
            deserialized.event.access_record.record_id,
            "record_!@#$%^&*()"
        );
    }

    #[test]
    fn test_event_with_empty_strings() {
        let user = AcsUser {
            user_id: "".to_string(),
            employee_id: Some("".to_string()),
            name: "".to_string(),
            user_type: Some(UserType::Visitor),
            status: Some(UserStatus::Disabled),
            department: Some("".to_string()),
            phone: Some("".to_string()),
            email: Some("".to_string()),
            has_face_image: Some(false),
            created_at: Some(0),
            updated_at: Some(0),
        };

        let event_data = AcsUserUpdatedEvent {
            change_type: "".to_string(),
            old_user: Some(user),
            new_user: None,
        };

        let event = P2AcsUserUpdatedV1 {
            event_id: "".to_string(),
            event_type: "".to_string(),
            created_time: "".to_string(),
            event: event_data,
        };

        // 验证空字符串处理
        assert_eq!(event.event_id, "");
        assert_eq!(event.event_type, "");
        assert_eq!(event.created_time, "");
        assert_eq!(event.event.change_type, "");

        let user = event.event.old_user.as_ref().unwrap();
        assert_eq!(user.user_id, "");
        assert_eq!(user.name, "");

        // 测试序列化和反序列化
        let json_str = serde_json::to_string(&event).expect("Failed to serialize empty strings");
        let deserialized: P2AcsUserUpdatedV1 =
            serde_json::from_str(&json_str).expect("Failed to deserialize empty strings");
        assert_eq!(deserialized.event_id, "");
        assert_eq!(deserialized.event.old_user.as_ref().unwrap().user_id, "");
    }

    #[test]
    fn test_event_with_very_long_strings() {
        let long_string = "a".repeat(1000);

        let access_record = AccessRecord {
            record_id: long_string.clone(),
            user_id: Some(long_string.clone()),
            user_name: Some(long_string.clone()),
            device_id: long_string.clone(),
            device_name: Some(long_string.clone()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::Card),
            result: AccessResult::Success,
            has_face_image: Some(false),
            access_time: 1672574600,
            created_at: Some(1672574600),
        };

        let event_data = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: Some(false),
            abnormal_reason: Some(long_string.clone()),
        };

        let event = P2AcsAccessRecordCreatedV1 {
            event_id: long_string.clone(),
            event_type: "p2_acs_access_record_created_v1".to_string(),
            created_time: "2023-01-01T10:30:00Z".to_string(),
            event: event_data,
        };

        // 验证长字符串处理
        assert_eq!(event.event_id.len(), 1000);
        assert_eq!(event.event.access_record.record_id.len(), 1000);
        assert_eq!(event.event.abnormal_reason.as_ref().unwrap().len(), 1000);

        // 测试序列化和反序列化
        let json_str = serde_json::to_string(&event).expect("Failed to serialize long strings");
        let deserialized: P2AcsAccessRecordCreatedV1 =
            serde_json::from_str(&json_str).expect("Failed to deserialize long strings");
        assert_eq!(deserialized.event_id.len(), 1000);
        assert_eq!(deserialized.event.access_record.record_id.len(), 1000);
    }

    #[test]
    fn test_edge_case_boundary_timestamps() {
        let access_record = AccessRecord {
            record_id: "boundary_test".to_string(),
            user_id: Some("boundary_user".to_string()),
            user_name: Some("边界用户".to_string()),
            device_id: "boundary_device".to_string(),
            device_name: Some("边界设备".to_string()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::Card),
            result: AccessResult::Success,
            has_face_image: None,
            access_time: 0, // Unix epoch start
            created_at: None,
        };

        let event_data = AccessRecordCreatedEvent {
            access_record,
            is_abnormal: None,
            abnormal_reason: None,
        };

        let event = P2AcsAccessRecordCreatedV1 {
            event_id: "boundary_event".to_string(),
            event_type: "p2_acs_access_record_created_v1".to_string(),
            created_time: "1970-01-01T00:00:00Z".to_string(),
            event: event_data,
        };

        // 验证边界时间戳处理
        assert_eq!(event.event.access_record.access_time, 0);
        assert_eq!(event.created_time, "1970-01-01T00:00:00Z");

        // 测试大时间戳
        let large_timestamp_record = AccessRecord {
            record_id: "large_timestamp".to_string(),
            user_id: None,
            user_name: None,
            device_id: "future_device".to_string(),
            device_name: Some("未来设备".to_string()),
            access_type: Some(AccessType::Exit),
            access_method: Some(AccessMethod::Manual),
            result: AccessResult::Failed,
            has_face_image: None,
            access_time: 2147483647, // 最大32位整数时间戳
            created_at: None,
        };

        assert_eq!(large_timestamp_record.access_time, 2147483647);
    }
}
