//! Representative contract tests for openlark-user request/response models.
//!
//! Tests cover:
//! - `common::UserSetting` / `UserPreference` — core data models
//! - `personal_settings::system_status::*` — response & body structs
//! - Endpoint constants verification

use openlark_user::common::{UserPreference, UserSetting};
use openlark_user::common::constants::endpoints;
use openlark_user::common::constants::setting_type;
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
// common models
// ---------------------------------------------------------------------------

#[test]
fn user_setting_roundtrip() {
    let setting = UserSetting {
        key: "language".to_string(),
        value: "zh-CN".to_string(),
        setting_type: "ui".to_string(),
    };
    let expected = json!({
        "key": "language",
        "value": "zh-CN",
        "setting_type": "ui"
    });
    assert_json_contract(&setting, expected.clone());

    let back: UserSetting = parse_contract(expected);
    assert_eq!(back.key, "language");
    assert_eq!(back.value, "zh-CN");
    assert_eq!(back.setting_type, "ui");
}

#[test]
fn user_preference_with_category_roundtrip() {
    let pref = UserPreference {
        key: "default_calendar_view".to_string(),
        value: "week".to_string(),
        category: Some("calendar".to_string()),
    };
    assert_json_contract(
        &pref,
        json!({
            "key": "default_calendar_view",
            "value": "week",
            "category": "calendar"
        }),
    );

    let parsed: UserPreference = parse_contract(json!({
        "key": "time_format",
        "value": "24h",
        "category": null
    }));
    assert_eq!(parsed.key, "time_format");
    assert!(parsed.category.is_none());
    assert_json_contract(
        &parsed,
        json!({
            "key": "time_format",
            "value": "24h"
        }),
    );
}

// ---------------------------------------------------------------------------
// personal_settings system_status response models
// ---------------------------------------------------------------------------

#[test]
fn system_status_list_response_roundtrip() {
    use openlark_user::personal_settings::personal_settings::v1::system_status::list::SystemStatusListResponse;

    let resp: SystemStatusListResponse = parse_contract(json!({
        "data": {
            "items": [
                {"status_id": "s1", "title": "会议中"},
                {"status_id": "s2", "title": "外出"}
            ]
        }
    }));
    assert!(resp.data.is_some());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["status_id"], "s1");
    assert_json_contract(
        &resp,
        json!({
            "data": {
                "items": [
                    {"status_id": "s1", "title": "会议中"},
                    {"status_id": "s2", "title": "外出"}
                ]
            }
        }),
    );
}

#[test]
fn system_status_create_response_roundtrip() {
    use openlark_user::personal_settings::personal_settings::v1::system_status::create::SystemStatusCreateResponse;

    let resp: SystemStatusCreateResponse = parse_contract(json!({
        "data": {"status_id": "new_status_001", "title": "出差"}
    }));
    assert!(resp.data.is_some());
    assert_json_contract(
        &resp,
        json!({"data": {"status_id": "new_status_001", "title": "出差"}}),
    );
}

#[test]
fn system_status_get_response_roundtrip() {
    use openlark_user::personal_settings::personal_settings::v1::system_status::get::SystemStatusGetResponse;

    let resp: SystemStatusGetResponse = parse_contract(json!({
        "data": {"status_id": "s1", "title": "会议中", "is_active": true}
    }));
    assert!(resp.data.is_some());
    assert_json_contract(
        &resp,
        json!({"data": {"status_id": "s1", "title": "会议中", "is_active": true}}),
    );
}

#[test]
fn system_status_patch_response_roundtrip() {
    use openlark_user::personal_settings::personal_settings::v1::system_status::patch::SystemStatusPatchResponse;

    let resp: SystemStatusPatchResponse = parse_contract(json!({
        "data": {"status_id": "s1", "title": "忙碌"}
    }));
    assert!(resp.data.is_some());
    assert_json_contract(
        &resp,
        json!({"data": {"status_id": "s1", "title": "忙碌"}}),
    );
}

#[test]
fn system_status_delete_response_roundtrip() {
    use openlark_user::personal_settings::personal_settings::v1::system_status::delete::SystemStatusDeleteResponse;

    let resp: SystemStatusDeleteResponse = parse_contract(json!({
        "data": null
    }));
    assert!(resp.data.is_none());
    assert_json_contract(&resp, json!({"data": null}));
}

#[test]
fn system_status_batch_open_response_roundtrip() {
    use openlark_user::personal_settings::personal_settings::v1::system_status::batch_open::SystemStatusBatch_openResponse;

    let resp: SystemStatusBatch_openResponse = parse_contract(json!({
        "data": {"success_count": 5}
    }));
    assert!(resp.data.is_some());
    assert_json_contract(&resp, json!({"data": {"success_count": 5}}));
}

#[test]
fn batch_close_body_and_response_roundtrip() {
    use openlark_user::personal_settings::personal_settings::v1::system_status::batch_close::{
        BatchCloseSystemStatusBody, BatchCloseSystemStatusResponse,
    };

    let body = BatchCloseSystemStatusBody {
        user_ids: vec!["ou_001".to_string(), "ou_002".to_string()],
    };
    assert_json_contract(
        &body,
        json!({"user_ids": ["ou_001", "ou_002"]}),
    );

    let parsed: BatchCloseSystemStatusBody = parse_contract(json!({
        "user_ids": ["ou_003"]
    }));
    assert_eq!(parsed.user_ids.len(), 1);
    assert_eq!(parsed.user_ids[0], "ou_003");

    let resp: BatchCloseSystemStatusResponse = parse_contract(json!({
        "data": {"closed_count": 3}
    }));
    assert!(resp.data.is_some());
    assert_json_contract(&resp, json!({"data": {"closed_count": 3}}));
}

// ---------------------------------------------------------------------------
// endpoint & constant contracts
// ---------------------------------------------------------------------------

#[test]
fn endpoint_constants_are_stable() {
    assert_eq!(
        endpoints::SETTINGS_BASE,
        "/open-apis/user/v1/settings",
    );
    assert_eq!(
        endpoints::PREFERENCES_BASE,
        "/open-apis/user/v1/preferences",
    );
}

#[test]
fn setting_type_constants_are_stable() {
    assert_eq!(setting_type::NOTIFICATION, "notification");
    assert_eq!(setting_type::PRIVACY, "privacy");
    assert_eq!(setting_type::UI, "ui");
}
