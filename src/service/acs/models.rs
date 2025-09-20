use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 用户类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    /// 员工
    Employee,
    /// 访客
    Visitor,
    /// 承包商
    Contractor,
    /// 临时人员
    Temporary,
}

/// 用户状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    /// 活跃
    Active,
    /// 禁用
    Disabled,
    /// 过期
    Expired,
    /// 待审核
    Pending,
}

/// 门禁用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcsUser {
    /// 用户ID
    pub user_id: String,
    /// 员工工号/员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 用户姓名
    pub name: String,
    /// 用户类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 电话号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 是否有人脸图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_face_image: Option<bool>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 权限组状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleStatus {
    /// 活跃
    Active,
    /// 禁用
    Disabled,
}

/// 权限组信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExternal {
    /// 权限组ID
    pub rule_id: String,
    /// 权限组名称
    pub name: String,
    /// 权限组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限组状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleStatus>,
    /// 关联的设备ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
    /// 关联的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 生效开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 生效结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 访客状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisitorStatus {
    /// 活跃
    Active,
    /// 过期
    Expired,
    /// 已签出
    CheckedOut,
}

/// 访客信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visitor {
    /// 访客ID
    pub visitor_id: String,
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
    /// 接待人员姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    /// 访客状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VisitorStatus>,
    /// 访问开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 访问结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 设备类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    /// 门禁设备
    AccessControl,
    /// 人脸识别设备
    FaceRecognition,
    /// 闸机
    Turnstile,
    /// 电梯控制器
    ElevatorController,
}

/// 设备状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceStatus {
    /// 在线
    Online,
    /// 离线
    Offline,
    /// 故障
    Error,
    /// 维护中
    Maintenance,
}

/// 门禁设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// 设备ID
    pub device_id: String,
    /// 设备名称
    pub name: String,
    /// 设备类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<DeviceType>,
    /// 设备状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeviceStatus>,
    /// 设备位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// 设备描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 设备IP地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// 设备MAC地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 访问类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessType {
    /// 进入
    Entry,
    /// 离开
    Exit,
}

/// 访问方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMethod {
    /// 人脸识别
    FaceRecognition,
    /// 刷卡
    Card,
    /// 密码
    Password,
    /// 指纹
    Fingerprint,
    /// 手动开门
    Manual,
}

/// 访问结果
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessResult {
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 拒绝
    Denied,
    /// 超时
    Timeout,
}

/// 门禁访问记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRecord {
    /// 记录ID
    pub record_id: String,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 设备ID
    pub device_id: String,
    /// 设备名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// 访问类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<AccessType>,
    /// 访问方式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_method: Option<AccessMethod>,
    /// 访问结果
    pub result: AccessResult,
    /// 是否有人脸识别图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_face_image: Option<bool>,
    /// 访问时间戳
    pub access_time: i64,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 人脸图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceImage {
    /// 图片ID
    pub image_id: String,
    /// 图片内容 (base64编码)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_content: Option<String>,
    /// 图片格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_format: Option<String>,
    /// 图片大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 上传时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<i64>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_page_response_serialization() {
        let page_response = PageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            page_token: Some("next_page".to_string()),
            has_more: Some(true),
        };

        let serialized = serde_json::to_string(&page_response).unwrap();
        let deserialized: PageResponse<String> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(page_response.items.len(), deserialized.items.len());
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.has_more, deserialized.has_more);
    }

    #[test]
    fn test_user_type_serialization() {
        let types = vec![
            UserType::Employee,
            UserType::Visitor,
            UserType::Contractor,
            UserType::Temporary,
        ];

        for user_type in types {
            let serialized = serde_json::to_string(&user_type).unwrap();
            let deserialized: UserType = serde_json::from_str(&serialized).unwrap();

            match (user_type, deserialized) {
                (UserType::Employee, UserType::Employee) => {}
                (UserType::Visitor, UserType::Visitor) => {}
                (UserType::Contractor, UserType::Contractor) => {}
                (UserType::Temporary, UserType::Temporary) => {}
                _ => panic!("Serialization/deserialization failed"),
            }
        }
    }

    #[test]
    fn test_user_status_serialization() {
        let statuses = vec![
            UserStatus::Active,
            UserStatus::Disabled,
            UserStatus::Expired,
            UserStatus::Pending,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: UserStatus = serde_json::from_str(&serialized).unwrap();

            match (status, deserialized) {
                (UserStatus::Active, UserStatus::Active) => {}
                (UserStatus::Disabled, UserStatus::Disabled) => {}
                (UserStatus::Expired, UserStatus::Expired) => {}
                (UserStatus::Pending, UserStatus::Pending) => {}
                _ => panic!("Serialization/deserialization failed"),
            }
        }
    }

    #[test]
    fn test_acs_user_serialization() {
        let user = AcsUser {
            user_id: "user_123".to_string(),
            employee_id: Some("emp_456".to_string()),
            name: "张三".to_string(),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            department: Some("技术部".to_string()),
            phone: Some("13800138000".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            has_face_image: Some(true),
            created_at: Some(1640995200),
            updated_at: Some(1672531200),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: AcsUser = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.user_id, deserialized.user_id);
        assert_eq!(user.employee_id, deserialized.employee_id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.email, deserialized.email);
        assert_eq!(user.phone, deserialized.phone);
        assert_eq!(user.department, deserialized.department);
    }

    #[test]
    fn test_rule_external_serialization() {
        let rule = RuleExternal {
            rule_id: "rule_123".to_string(),
            name: "技术部门禁".to_string(),
            description: Some("技术部门员工门禁权限".to_string()),
            status: Some(RuleStatus::Active),
            device_ids: Some(vec!["device1".to_string(), "device2".to_string()]),
            user_ids: Some(vec!["user1".to_string(), "user2".to_string()]),
            start_time: Some(1640995200),
            end_time: Some(1672531200),
            created_at: Some(1640995200),
            updated_at: Some(1672531200),
        };

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: RuleExternal = serde_json::from_str(&serialized).unwrap();

        assert_eq!(rule.rule_id, deserialized.rule_id);
        assert_eq!(rule.name, deserialized.name);
        assert_eq!(rule.description, deserialized.description);
        assert_eq!(rule.device_ids, deserialized.device_ids);
        assert_eq!(rule.user_ids, deserialized.user_ids);
    }

    #[test]
    fn test_visitor_serialization() {
        let visitor = Visitor {
            visitor_id: "visitor_123".to_string(),
            name: "李四".to_string(),
            phone: Some("13900139000".to_string()),
            company: Some("访客公司".to_string()),
            purpose: Some("业务洽谈".to_string()),
            host_user_id: Some("host_456".to_string()),
            host_name: Some("王五".to_string()),
            status: Some(VisitorStatus::Active),
            start_time: Some(1640995200),
            end_time: Some(1672531200),
            created_at: Some(1640995200),
        };

        let serialized = serde_json::to_string(&visitor).unwrap();
        let deserialized: Visitor = serde_json::from_str(&serialized).unwrap();

        assert_eq!(visitor.visitor_id, deserialized.visitor_id);
        assert_eq!(visitor.name, deserialized.name);
        assert_eq!(visitor.phone, deserialized.phone);
        assert_eq!(visitor.company, deserialized.company);
        assert_eq!(visitor.purpose, deserialized.purpose);
        assert_eq!(visitor.host_user_id, deserialized.host_user_id);
    }

    #[test]
    fn test_device_serialization() {
        let device = Device {
            device_id: "device_123".to_string(),
            name: "前台门禁".to_string(),
            device_type: Some(DeviceType::FaceRecognition),
            status: Some(DeviceStatus::Online),
            location: Some("大厅前台".to_string()),
            description: Some("前台入口门禁设备".to_string()),
            ip_address: Some("192.168.1.100".to_string()),
            mac_address: Some("00:11:22:33:44:55".to_string()),
            created_at: Some(1640995200),
            updated_at: Some(1672531200),
        };

        let serialized = serde_json::to_string(&device).unwrap();
        let deserialized: Device = serde_json::from_str(&serialized).unwrap();

        assert_eq!(device.device_id, deserialized.device_id);
        assert_eq!(device.name, deserialized.name);
        assert_eq!(device.location, deserialized.location);
        assert_eq!(device.description, deserialized.description);
        assert_eq!(device.ip_address, deserialized.ip_address);
        assert_eq!(device.mac_address, deserialized.mac_address);
    }

    #[test]
    fn test_access_record_serialization() {
        let record = AccessRecord {
            record_id: "record_123".to_string(),
            user_id: Some("user_456".to_string()),
            user_name: Some("张三".to_string()),
            device_id: "device_789".to_string(),
            device_name: Some("前台门禁".to_string()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::FaceRecognition),
            result: AccessResult::Success,
            has_face_image: Some(true),
            access_time: 1672531200,
            created_at: Some(1672531200),
        };

        let serialized = serde_json::to_string(&record).unwrap();
        let deserialized: AccessRecord = serde_json::from_str(&serialized).unwrap();

        assert_eq!(record.record_id, deserialized.record_id);
        assert_eq!(record.user_id, deserialized.user_id);
        assert_eq!(record.user_name, deserialized.user_name);
        assert_eq!(record.device_id, deserialized.device_id);
        assert_eq!(record.device_name, deserialized.device_name);
        assert_eq!(record.access_time, deserialized.access_time);
    }

    #[test]
    fn test_face_image_serialization() {
        let face_image = FaceImage {
            image_id: "img_123".to_string(),
            image_content: Some("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==".to_string()),
            image_format: Some("PNG".to_string()),
            file_size: Some(68),
            uploaded_at: Some(1672531200),
        };

        let serialized = serde_json::to_string(&face_image).unwrap();
        let deserialized: FaceImage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(face_image.image_id, deserialized.image_id);
        assert_eq!(face_image.image_content, deserialized.image_content);
        assert_eq!(face_image.image_format, deserialized.image_format);
        assert_eq!(face_image.file_size, deserialized.file_size);
        assert_eq!(face_image.uploaded_at, deserialized.uploaded_at);
    }

    #[test]
    fn test_enum_serialization_formats() {
        // Test AccessType
        assert_eq!(
            serde_json::to_string(&AccessType::Entry).unwrap(),
            "\"entry\""
        );
        assert_eq!(
            serde_json::to_string(&AccessType::Exit).unwrap(),
            "\"exit\""
        );

        // Test AccessMethod
        assert_eq!(
            serde_json::to_string(&AccessMethod::FaceRecognition).unwrap(),
            "\"face_recognition\""
        );
        assert_eq!(
            serde_json::to_string(&AccessMethod::Card).unwrap(),
            "\"card\""
        );

        // Test AccessResult
        assert_eq!(
            serde_json::to_string(&AccessResult::Success).unwrap(),
            "\"success\""
        );
        assert_eq!(
            serde_json::to_string(&AccessResult::Failed).unwrap(),
            "\"failed\""
        );
    }

    #[test]
    fn test_models_with_none_values() {
        let user = AcsUser {
            user_id: "user_123".to_string(),
            employee_id: None,
            name: "Required Name".to_string(),
            email: None,
            phone: None,
            department: None,
            user_type: None,
            status: None,
            has_face_image: None,
            created_at: None,
            updated_at: None,
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: AcsUser = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.user_id, deserialized.user_id);
        assert!(deserialized.employee_id.is_none());
        assert_eq!(user.name, deserialized.name);
        assert!(deserialized.email.is_none());
    }

    #[test]
    fn test_debug_trait_for_models() {
        let user = AcsUser {
            user_id: "test".to_string(),
            employee_id: None,
            name: "Test User".to_string(),
            email: None,
            phone: None,
            department: None,
            user_type: None,
            status: None,
            has_face_image: None,
            created_at: None,
            updated_at: None,
        };

        let debug_string = format!("{:?}", user);
        assert!(debug_string.contains("AcsUser"));
        assert!(debug_string.contains("test"));
    }

    #[test]
    fn test_clone_trait_for_models() {
        let original_user = AcsUser {
            user_id: "test".to_string(),
            employee_id: Some("emp123".to_string()),
            name: "Test User".to_string(),
            email: None,
            phone: None,
            department: None,
            user_type: None,
            status: None,
            has_face_image: None,
            created_at: None,
            updated_at: None,
        };

        let cloned_user = original_user.clone();
        assert_eq!(original_user.user_id, cloned_user.user_id);
        assert_eq!(original_user.employee_id, cloned_user.employee_id);
        assert_eq!(original_user.name, cloned_user.name);
    }
}
