//! Representative contract tests for openlark-security request/response models.
//!
//! Covers ACS (访问控制) and Security & Compliance (安全合规) data models,
//! verifying bidirectional serde consistency via `parse_contract` and `assert_json_contract`.

use openlark_security::models::{
    AccessRecord, AccessResult, ApplyStatus, ComplianceCheckResult, ComplianceResult,
    ComplianceRuleType, ComplianceStatus, DeviceApplyRecord, DeviceApplyRecordApproveRequest,
    DeviceComplianceRule, DeviceInfo, DeviceRecord, DeviceRecordRequest, DeviceRecordStatus,
    DeviceRecordUpdateRequest, DeviceStatus, DeviceType, ExtensionMap, FaceImageInfo, GeoLocation,
    KeyValue, OpenApiLog, OpenApiLogQueryRequest, OperationResponse, PageRequest, PageResponse,
    PermissionRuleInfo, SortDirection, Status, TimeRange, UserInfo, UserListResponse, UserRequest,
    VerificationMethod, VisitorInfo, VisitorStatus,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_value, json, to_value, Value};

fn assert_json_contract<T>(value: &T, expected: Value)
where
    T: Serialize,
{
    assert_eq!(to_value(value).unwrap(), expected);
}

fn parse_contract<T>(payload: Value) -> T
where
    T: DeserializeOwned,
{
    from_value(payload).unwrap()
}

// ---------------------------------------------------------------------------
// ACS 模块：核心模型
// ---------------------------------------------------------------------------

#[test]
fn user_info_contract() {
    let user: UserInfo = parse_contract(json!({
        "user_id": "ou_security_001",
        "name": "李安全",
        "email": "li.secure@example.com",
        "mobile": "13900139000",
        "department_ids": ["od-100", "od-200"],
        "status": "active",
        "face_enrolled": true,
        "rule_ids": ["rule_01", "rule_02"],
        "create_time": 1700000000,
        "update_time": 1700001000,
        "extension": {}
    }));
    assert_eq!(user.user_id, "ou_security_001");
    assert_eq!(user.name, "李安全");
    assert!(user.face_enrolled);
    assert_eq!(user.status, Status::Active);
    assert_eq!(user.rule_ids.len(), 2);

    assert_json_contract(
        &user,
        json!({
            "user_id": "ou_security_001",
            "name": "李安全",
            "email": "li.secure@example.com",
            "mobile": "13900139000",
            "department_ids": ["od-100", "od-200"],
            "status": "active",
            "face_enrolled": true,
            "rule_ids": ["rule_01", "rule_02"],
            "create_time": 1700000000,
            "update_time": 1700001000,
            "extension": {}
        }),
    );
}

#[test]
fn user_request_contract() {
    let req: UserRequest = parse_contract(json!({
        "name": "王新员工",
        "mobile": "13800138001",
        "department_ids": ["od-300"],
        "status": "active",
        "rule_ids": ["rule_03"],
        "extension": {"note": "新建用户"}
    }));
    assert_eq!(req.name, "王新员工");
    assert!(req.email.is_none());
    assert_eq!(req.department_ids.as_ref().unwrap().len(), 1);

    assert_json_contract(
        &req,
        json!({
            "name": "王新员工",
            "email": null,
            "mobile": "13800138001",
            "department_ids": ["od-300"],
            "status": "active",
            "rule_ids": ["rule_03"],
            "extension": {"note": "新建用户"}
        }),
    );
}

#[test]
fn user_list_response_contract() {
    let resp: UserListResponse = parse_contract(json!({
        "users": [
            {
                "user_id": "u_001",
                "name": "张门禁",
                "email": null,
                "mobile": null,
                "department_ids": [],
                "status": "active",
                "face_enrolled": false,
                "rule_ids": [],
                "create_time": 1700000000,
                "update_time": 1700000000,
                "extension": {}
            }
        ],
        "has_more": true,
        "page_token": "next_page_token_abc"
    }));
    assert_eq!(resp.users.len(), 1);
    assert!(resp.has_more);
    assert_eq!(resp.page_token.as_deref(), Some("next_page_token_abc"));
}

#[test]
fn device_info_contract() {
    let device: DeviceInfo = parse_contract(json!({
        "device_id": "dev_001",
        "device_name": "1号楼大门",
        "device_type": "face_recognition",
        "status": "normal",
        "location": "总部1号楼大堂",
        "geo_location": {
            "latitude": 39.9042,
            "longitude": 116.4074,
            "address": "北京市东城区"
        },
        "online": true,
        "last_online_time": 1700002000,
        "rule_ids": ["rule_01"],
        "create_time": 1700000000,
        "update_time": 1700001000,
        "extension": {}
    }));
    assert_eq!(device.device_id, "dev_001");
    assert_eq!(device.device_type, DeviceType::FaceRecognition);
    assert_eq!(device.status, DeviceStatus::Normal);
    assert!(device.online);
    assert!(device.geo_location.is_some());
    assert_eq!(device.geo_location.as_ref().unwrap().latitude, 39.9042);

    assert_json_contract(
        &device,
        json!({
            "device_id": "dev_001",
            "device_name": "1号楼大门",
            "device_type": "face_recognition",
            "status": "normal",
            "location": "总部1号楼大堂",
            "geo_location": {
                "latitude": 39.9042,
                "longitude": 116.4074,
                "address": "北京市东城区"
            },
            "online": true,
            "last_online_time": 1700002000,
            "rule_ids": ["rule_01"],
            "create_time": 1700000000,
            "update_time": 1700001000,
            "extension": {}
        }),
    );
}

#[test]
fn device_type_variants() {
    let cases = vec![
        (json!("card_reader"), "CardReader"),
        (json!("face_recognition"), "FaceRecognition"),
        (json!("fingerprint_reader"), "FingerprintReader"),
        (json!("door_lock"), "DoorLock"),
        (json!("turnstile"), "Turnstile"),
    ];
    for (json_val, expected_name) in cases {
        let dt: DeviceType = from_value(json_val).unwrap();
        assert!(format!("{:?}", dt).contains(expected_name));
    }
}

#[test]
fn device_status_variants() {
    assert_eq!(
        from_value::<DeviceStatus>(json!("normal")).unwrap(),
        DeviceStatus::Normal
    );
    assert_eq!(
        from_value::<DeviceStatus>(json!("offline")).unwrap(),
        DeviceStatus::Offline
    );
    assert_eq!(
        from_value::<DeviceStatus>(json!("fault")).unwrap(),
        DeviceStatus::Fault
    );
    assert_eq!(
        from_value::<DeviceStatus>(json!("maintenance")).unwrap(),
        DeviceStatus::Maintenance
    );
    assert_eq!(
        from_value::<DeviceStatus>(json!("disabled")).unwrap(),
        DeviceStatus::Disabled
    );
}

#[test]
fn permission_rule_info_contract() {
    let rule: PermissionRuleInfo = parse_contract(json!({
        "rule_id": "rule_001",
        "name": "工作日通行",
        "description": "周一到周五全天通行",
        "status": "active",
        "device_ids": ["dev_001", "dev_002"],
        "user_ids": ["u_001", "u_002"],
        "valid_from": 1700000000,
        "valid_until": 1735689600,
        "create_time": 1700000000,
        "update_time": 1700000000,
        "extension": {}
    }));
    assert_eq!(rule.rule_id, "rule_001");
    assert_eq!(rule.name, "工作日通行");
    assert_eq!(rule.device_ids.len(), 2);
    assert!(rule.valid_from.is_some());
}

#[test]
fn visitor_info_contract() {
    let visitor: VisitorInfo = parse_contract(json!({
        "visitor_id": "vis_001",
        "name": "赵来访",
        "mobile": "13700137000",
        "email": "zhao@guest.com",
        "visit_reason": "商务洽谈",
        "host_info": {
            "user_id": "u_host_001",
            "name": "钱接待",
            "mobile": "13600136000"
        },
        "valid_from": 1700000000,
        "valid_until": 1700086400,
        "status": "approved",
        "rule_ids": ["rule_05"],
        "create_time": 1700000000,
        "update_time": 1700000000,
        "extension": {}
    }));
    assert_eq!(visitor.visitor_id, "vis_001");
    assert_eq!(visitor.status, VisitorStatus::Approved);
    assert!(visitor.host_info.is_some());
    assert_eq!(visitor.host_info.as_ref().unwrap().name, "钱接待");
}

#[test]
fn visitor_status_roundtrip() {
    assert_eq!(to_value(&VisitorStatus::Pending).unwrap(), json!("pending"));
    assert_eq!(
        to_value(&VisitorStatus::Approved).unwrap(),
        json!("approved")
    );
    assert_eq!(
        to_value(&VisitorStatus::Rejected).unwrap(),
        json!("rejected")
    );
    assert_eq!(to_value(&VisitorStatus::Expired).unwrap(), json!("expired"));
    assert_eq!(to_value(&VisitorStatus::Revoked).unwrap(), json!("revoked"));
}

#[test]
fn access_record_contract() {
    let record: AccessRecord = parse_contract(json!({
        "record_id": "rec_001",
        "user_id": "u_001",
        "visitor_id": null,
        "device_id": "dev_001",
        "access_time": 1700003000,
        "access_result": "success",
        "verification_method": "face",
        "reject_reason": null,
        "photo_url": "https://cdn.example.com/photo_001.jpg",
        "extension": {}
    }));
    assert_eq!(record.record_id, "rec_001");
    assert_eq!(record.access_result, AccessResult::Success);
    assert_eq!(record.verification_method, VerificationMethod::Face);
    assert!(record.user_id.is_some());
    assert!(record.visitor_id.is_none());
}

#[test]
fn verification_method_variants() {
    let cases = vec![
        (json!("face"), "Face"),
        (json!("card"), "Card"),
        (json!("fingerprint"), "Fingerprint"),
        (json!("password"), "Password"),
        (json!("qr_code"), "QrCode"),
    ];
    for (json_val, expected_name) in cases {
        let vm: VerificationMethod = from_value(json_val).unwrap();
        assert!(format!("{:?}", vm).contains(expected_name));
    }
}

#[test]
fn face_image_info_contract() {
    let face: FaceImageInfo = parse_contract(json!({
        "face_url": "https://cdn.example.com/face_001.jpg",
        "face_feature": "feature_base64_data",
        "quality_score": 0.95,
        "upload_time": 1700000000
    }));
    assert_eq!(face.face_url, "https://cdn.example.com/face_001.jpg");
    assert_eq!(face.quality_score.unwrap(), 0.95);

    assert_json_contract(
        &face,
        json!({
            "face_url": "https://cdn.example.com/face_001.jpg",
            "face_feature": "feature_base64_data",
            "quality_score": 0.95,
            "upload_time": 1700000000
        }),
    );
}

// ---------------------------------------------------------------------------
// Security & Compliance 模块
// ---------------------------------------------------------------------------

#[test]
fn device_record_contract() {
    let record: DeviceRecord = parse_contract(json!({
        "device_record_id": "drec_001",
        "device_name": "员工iPhone 15",
        "device_type": "smartphone",
        "device_brand": "Apple",
        "device_model": "iPhone 15",
        "os_type": "iOS",
        "os_version": "17.0",
        "serial_number": "SN123456",
        "mac_address": "AA:BB:CC:DD:EE:FF",
        "status": "approved",
        "user_id": "ou_dev_user",
        "user_name": "孙设备",
        "department_name": "技术部",
        "personal_device": true,
        "compliance_status": "compliant",
        "last_check_time": 1700004000,
        "create_time": 1700000000,
        "update_time": 1700001000,
        "extension": {}
    }));
    assert_eq!(record.device_record_id, "drec_001");
    assert_eq!(record.status, DeviceRecordStatus::Approved);
    assert_eq!(record.compliance_status, ComplianceStatus::Compliant);
    assert!(record.personal_device);
}

#[test]
fn device_record_request_contract() {
    let req: DeviceRecordRequest = parse_contract(json!({
        "device_name": "测试设备",
        "device_type": "laptop",
        "device_brand": "Lenovo",
        "device_model": "ThinkPad X1",
        "os_type": "Windows",
        "os_version": "11",
        "serial_number": "SN789",
        "mac_address": "11:22:33:44:55:66",
        "personal_device": false,
        "extension": null
    }));
    assert_eq!(req.device_name, "测试设备");
    assert_eq!(req.device_brand.as_deref(), Some("Lenovo"));
    assert!(req.extension.is_none());

    assert_json_contract(
        &req,
        json!({
            "device_name": "测试设备",
            "device_type": "laptop",
            "device_brand": "Lenovo",
            "device_model": "ThinkPad X1",
            "os_type": "Windows",
            "os_version": "11",
            "serial_number": "SN789",
            "mac_address": "11:22:33:44:55:66",
            "personal_device": false,
            "extension": null
        }),
    );
}

#[test]
fn device_record_update_request_contract() {
    let req: DeviceRecordUpdateRequest = parse_contract(json!({
        "device_name": "重命名设备",
        "device_brand": null,
        "device_model": null,
        "os_type": null,
        "os_version": "12.1",
        "serial_number": null,
        "mac_address": null,
        "compliance_status": "non_compliant",
        "extension": {"reason": "系统版本过低"}
    }));
    assert_eq!(req.device_name.as_deref(), Some("重命名设备"));
    assert_eq!(req.compliance_status, Some(ComplianceStatus::NonCompliant));
}

#[test]
fn device_record_status_variants() {
    assert_eq!(
        from_value::<DeviceRecordStatus>(json!("pending")).unwrap(),
        DeviceRecordStatus::Pending
    );
    assert_eq!(
        from_value::<DeviceRecordStatus>(json!("approved")).unwrap(),
        DeviceRecordStatus::Approved
    );
    assert_eq!(
        from_value::<DeviceRecordStatus>(json!("rejected")).unwrap(),
        DeviceRecordStatus::Rejected
    );
    assert_eq!(
        from_value::<DeviceRecordStatus>(json!("expired")).unwrap(),
        DeviceRecordStatus::Expired
    );
    assert_eq!(
        from_value::<DeviceRecordStatus>(json!("revoked")).unwrap(),
        DeviceRecordStatus::Revoked
    );
    assert_eq!(
        from_value::<DeviceRecordStatus>(json!("non_compliant")).unwrap(),
        DeviceRecordStatus::NonCompliant
    );
}

#[test]
fn compliance_status_variants() {
    assert_eq!(
        from_value::<ComplianceStatus>(json!("compliant")).unwrap(),
        ComplianceStatus::Compliant
    );
    assert_eq!(
        from_value::<ComplianceStatus>(json!("non_compliant")).unwrap(),
        ComplianceStatus::NonCompliant
    );
    assert_eq!(
        from_value::<ComplianceStatus>(json!("pending")).unwrap(),
        ComplianceStatus::Pending
    );
    assert_eq!(
        from_value::<ComplianceStatus>(json!("unknown")).unwrap(),
        ComplianceStatus::Unknown
    );
}

#[test]
fn open_api_log_contract() {
    let log: OpenApiLog = parse_contract(json!({
        "log_id": "log_001",
        "user_id": "ou_api_user",
        "user_name": "周API",
        "app_id": "cli_001",
        "api_path": "/open-apis/security/v1/users",
        "method": "GET",
        "status_code": 200,
        "cost_time": 150,
        "client_ip": "10.0.0.1",
        "user_agent": "OpenLark-SDK/0.15",
        "request_time": 1700005000,
        "response_time": 1700005150,
        "error_msg": null,
        "request_params": {"page_size": 20},
        "response_summary": "success",
        "extension": {}
    }));
    assert_eq!(log.log_id, "log_001");
    assert_eq!(log.status_code, 200);
    assert_eq!(log.method, "GET");
    assert!(log.cost_time.is_some());
    assert!(log.request_params.is_some());
}

#[test]
fn open_api_log_query_request_contract() {
    let req: OpenApiLogQueryRequest = parse_contract(json!({
        "start_time": 1700000000,
        "end_time": 1700086400,
        "user_id_filter": "ou_001",
        "api_path_filter": "/open-apis/security/",
        "status_code_filter": 200,
        "app_id_filter": "cli_001",
        "page_size": 50,
        "page_token": null,
        "sort_field": "request_time",
        "sort_direction": "desc"
    }));
    assert!(req.start_time.is_some());
    assert_eq!(req.status_code_filter, Some(200));
    assert_eq!(req.sort_field.as_deref(), Some("request_time"));
}

#[test]
fn device_apply_record_contract() {
    let record: DeviceApplyRecord = parse_contract(json!({
        "device_apply_record_id": "dapply_001",
        "device_record_id": "drec_001",
        "device_info": {
            "device_name": "新平板",
            "device_type": "tablet",
            "device_brand": "Samsung",
            "device_model": "Galaxy Tab S9",
            "os_type": "Android",
            "os_version": "14",
            "serial_number": "SN_TAB_001",
            "mac_address": null,
            "personal_device": true,
            "extension": null
        },
        "applicant_id": "ou_apply_001",
        "applicant_name": "吴申请",
        "apply_reason": "工作需要平板设备",
        "apply_status": "pending",
        "approver_id": null,
        "approver_name": null,
        "approve_comment": null,
        "approve_time": null,
        "apply_time": 1700006000,
        "update_time": 1700006000,
        "extension": {}
    }));
    assert_eq!(record.device_apply_record_id, "dapply_001");
    assert_eq!(record.apply_status, ApplyStatus::Pending);
    assert!(record.device_info.is_some());
    assert!(record.approver_id.is_none());
}

#[test]
fn device_apply_record_approve_request_contract() {
    let req: DeviceApplyRecordApproveRequest = parse_contract(json!({
        "approved": true,
        "comment": "设备符合安全策略",
        "remark": "已验证设备序列号"
    }));
    assert!(req.approved);
    assert_eq!(req.comment.as_deref(), Some("设备符合安全策略"));
}

#[test]
fn device_compliance_rule_contract() {
    let rule: DeviceComplianceRule = parse_contract(json!({
        "rule_id": "cr_001",
        "rule_name": "操作系统版本检查",
        "rule_description": "操作系统版本必须大于等于指定版本",
        "rule_type": "os_check",
        "rule_content": {"min_version": "12.0"},
        "status": "active",
        "priority": 10,
        "create_time": 1700000000,
        "update_time": 1700000000,
        "extension": {}
    }));
    assert_eq!(rule.rule_id, "cr_001");
    assert_eq!(rule.rule_type, ComplianceRuleType::OsCheck);
    assert_eq!(rule.priority, 10);
}

#[test]
fn compliance_rule_type_variants() {
    assert_eq!(
        from_value::<ComplianceRuleType>(json!("device_type_check")).unwrap(),
        ComplianceRuleType::DeviceTypeCheck
    );
    assert_eq!(
        from_value::<ComplianceRuleType>(json!("os_check")).unwrap(),
        ComplianceRuleType::OsCheck
    );
    assert_eq!(
        from_value::<ComplianceRuleType>(json!("security_software_check")).unwrap(),
        ComplianceRuleType::SecuritySoftwareCheck
    );
    assert_eq!(
        from_value::<ComplianceRuleType>(json!("encryption_check")).unwrap(),
        ComplianceRuleType::EncryptionCheck
    );
    assert_eq!(
        from_value::<ComplianceRuleType>(json!("network_access_check")).unwrap(),
        ComplianceRuleType::NetworkAccessCheck
    );
    assert_eq!(
        from_value::<ComplianceRuleType>(json!("custom")).unwrap(),
        ComplianceRuleType::Custom
    );
}

#[test]
fn compliance_check_result_contract() {
    let result: ComplianceCheckResult = parse_contract(json!({
        "result_id": "cres_001",
        "device_record_id": "drec_001",
        "rule_id": "cr_001",
        "result": "pass",
        "check_time": 1700007000,
        "message": "操作系统版本合规",
        "details": {"actual_version": "14.0", "required_version": "12.0"},
        "extension": {}
    }));
    assert_eq!(result.result_id, "cres_001");
    assert_eq!(result.result, ComplianceResult::Pass);
    assert!(result.details.is_some());
}

#[test]
fn compliance_result_roundtrip() {
    assert_eq!(to_value(&ComplianceResult::Pass).unwrap(), json!("pass"));
    assert_eq!(to_value(&ComplianceResult::Fail).unwrap(), json!("fail"));
    assert_eq!(
        to_value(&ComplianceResult::Warning).unwrap(),
        json!("warning")
    );
    assert_eq!(to_value(&ComplianceResult::Skip).unwrap(), json!("skip"));
}

// ---------------------------------------------------------------------------
// 通用模型
// ---------------------------------------------------------------------------

#[test]
fn status_enum_roundtrip() {
    let variants = vec![
        (json!("active"), Status::Active),
        (json!("disabled"), Status::Disabled),
        (json!("pending"), Status::Pending),
        (json!("deleted"), Status::Deleted),
        (json!("expired"), Status::Expired),
    ];
    for (json_val, expected) in &variants {
        let parsed: Status = from_value(json_val.clone()).unwrap();
        assert_eq!(parsed, *expected);
        assert_eq!(to_value(&parsed).unwrap(), *json_val);
    }
}

#[test]
fn page_request_contract() {
    let req: PageRequest = parse_contract(json!({
        "page_size": 50,
        "page_token": "token_abc"
    }));
    assert_eq!(req.page_size, Some(50));
    assert_eq!(req.page_token.as_deref(), Some("token_abc"));

    assert_json_contract(
        &req,
        json!({
            "page_size": 50,
            "page_token": "token_abc"
        }),
    );
}

#[test]
fn page_response_contract() {
    let resp: PageResponse<String> = parse_contract(json!({
        "has_more": false,
        "page_token": null,
        "data": ["item1", "item2", "item3"]
    }));
    assert!(!resp.has_more);
    assert!(resp.page_token.is_none());
    assert_eq!(resp.data.len(), 3);
}

#[test]
fn operation_response_contract() {
    let resp: OperationResponse = parse_contract(json!({
        "success": true,
        "operation_id": "op_001",
        "message": "操作成功"
    }));
    assert!(resp.success);
    assert_eq!(resp.operation_id.as_deref(), Some("op_001"));
}

#[test]
fn geo_location_contract() {
    let geo: GeoLocation = parse_contract(json!({
        "latitude": 31.2304,
        "longitude": 121.4737,
        "address": "上海市黄浦区"
    }));
    assert_eq!(geo.latitude, 31.2304);
    assert_eq!(geo.longitude, 121.4737);
    assert_eq!(geo.address.as_deref(), Some("上海市黄浦区"));

    assert_json_contract(
        &geo,
        json!({
            "latitude": 31.2304,
            "longitude": 121.4737,
            "address": "上海市黄浦区"
        }),
    );
}

#[test]
fn sort_direction_roundtrip() {
    assert_eq!(to_value(&SortDirection::Asc).unwrap(), json!("asc"));
    assert_eq!(to_value(&SortDirection::Desc).unwrap(), json!("desc"));
    let asc: SortDirection = from_value(json!("asc")).unwrap();
    let desc: SortDirection = from_value(json!("desc")).unwrap();
    assert_eq!(to_value(&asc).unwrap(), json!("asc"));
    assert_eq!(to_value(&desc).unwrap(), json!("desc"));
}

#[test]
fn time_range_contract() {
    let tr: TimeRange = parse_contract(json!({
        "start_time": 1700000000,
        "end_time": 1700086400
    }));
    assert_eq!(tr.start_time, 1700000000);
    assert_eq!(tr.end_time, 1700086400);
}

#[test]
fn key_value_contract() {
    let kv: KeyValue = parse_contract(json!({
        "key": "department",
        "value": "技术部"
    }));
    assert_eq!(kv.key, "department");
    assert_eq!(kv.value, "技术部");

    assert_json_contract(&kv, json!({"key": "department", "value": "技术部"}));
}

#[test]
fn access_result_roundtrip() {
    assert_eq!(to_value(&AccessResult::Success).unwrap(), json!("success"));
    assert_eq!(to_value(&AccessResult::Failed).unwrap(), json!("failed"));
    assert_eq!(to_value(&AccessResult::Timeout).unwrap(), json!("timeout"));
}

#[test]
fn apply_status_variants() {
    assert_eq!(
        from_value::<ApplyStatus>(json!("pending")).unwrap(),
        ApplyStatus::Pending
    );
    assert_eq!(
        from_value::<ApplyStatus>(json!("approved")).unwrap(),
        ApplyStatus::Approved
    );
    assert_eq!(
        from_value::<ApplyStatus>(json!("rejected")).unwrap(),
        ApplyStatus::Rejected
    );
    assert_eq!(
        from_value::<ApplyStatus>(json!("revoked")).unwrap(),
        ApplyStatus::Revoked
    );
}

#[test]
fn extension_map_handles_arbitrary_json() {
    let mut ext: ExtensionMap = ExtensionMap::new();
    ext.insert("key1".to_string(), json!("string_value"));
    ext.insert("key2".to_string(), json!({"nested": true}));
    ext.insert("key3".to_string(), json!([1, 2, 3]));

    let serialized = to_value(&ext).unwrap();
    let deserialized: ExtensionMap = from_value(serialized.clone()).unwrap();
    assert_eq!(deserialized.len(), 3);
    assert_eq!(deserialized["key1"], json!("string_value"));
    assert_eq!(deserialized["key2"]["nested"], json!(true));
    assert_eq!(deserialized["key3"][1], json!(2));
}
