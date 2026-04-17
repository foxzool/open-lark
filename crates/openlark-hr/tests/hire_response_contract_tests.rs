use openlark_hr::hire::hire::{
    v1::{
        evaluation, interview_record as interview_record_v1, interviewer, location, referral,
        referral_account, role, subject, talent_object, talent_pool, todo, user_role, website,
    },
    v2::{interview_record as interview_record_v2, talent},
};
use serde::de::DeserializeOwned;
use serde_json::json;

fn parse_contract<T: DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value).expect("contract payload should deserialize")
}

#[test]
fn role_and_subject_list_contracts_are_typed() {
    let roles: role::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "101",
            "name": {"zh_cn": "测试", "en_us": "test"},
            "description": {"zh_cn": "描述", "en_us": "desc"},
            "role_status": 1,
            "role_type": 1
        }],
        "has_more": true,
        "page_token": "cursor_1"
    }));
    assert_eq!(roles.items[0].id.as_deref(), Some("101"));
    assert_eq!(
        roles.items[0]
            .name
            .as_ref()
            .and_then(|v| v.zh_cn.as_deref()),
        Some("测试")
    );
    assert_eq!(roles.page_token.as_deref(), Some("cursor_1"));

    let subjects: subject::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "sub_1",
            "name": {"zh_cn": "秋招", "en_us": "Campus"},
            "active_status": 1,
            "creator": {"id": "ou_creator", "name": {"zh_cn": "张三", "en_us": "Zhang"}}
        }],
        "has_more": false
    }));
    assert_eq!(
        subjects.items[0]
            .creator
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("ou_creator")
    );
}

#[test]
fn website_location_and_interviewer_contracts_are_typed() {
    let websites: website::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "site_1",
            "name": {"zh_cn": "校招官网", "en_us": "Campus Site"},
            "process_type_list": [2],
            "job_channel_id": "channel_1"
        }]
    }));
    assert_eq!(
        websites.items[0].job_channel_id.as_deref(),
        Some("channel_1")
    );

    let locations: location::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "loc_1",
            "name": {"zh_cn": "成都高新区", "en_us": "Gaoxin, Chengdu"},
            "city": {"code": "CT_22", "name": {"zh_cn": "成都", "en_us": "Chengdu"}},
            "active_status": 1
        }],
        "has_more": true,
        "page_token": "cursor_2"
    }));
    assert_eq!(
        locations.items[0]
            .city
            .as_ref()
            .and_then(|v| v.code.as_deref()),
        Some("CT_22")
    );

    let interviewers: interviewer::list::ListResponse = parse_contract(json!({
        "items": [{"user_id": "ou_1", "verify_status": 2}],
        "has_more": false
    }));
    assert_eq!(interviewers.items[0].verify_status, Some(2));
}

#[test]
fn evaluation_and_todo_contracts_are_typed() {
    let evaluations: evaluation::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "eval_1",
            "application_id": "app_1",
            "commit_status": 1,
            "conclusion": 1,
            "content": "优秀"
        }],
        "has_more": false
    }));
    assert_eq!(
        evaluations.items[0].application_id.as_deref(),
        Some("app_1")
    );

    let todos: todo::list::ListResponse = parse_contract(json!({
        "items": [{
            "evaluation": {"id": "eval_task", "talent_id": "talent_1"},
            "offer": {"id": "offer_task", "application_id": "app_1"}
        }],
        "page_token": "cursor_3"
    }));
    assert_eq!(
        todos.items[0]
            .evaluation
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("eval_task")
    );
    assert_eq!(todos.page_token.as_deref(), Some("cursor_3"));
}

#[test]
fn referral_account_and_search_contracts_are_typed() {
    let created: referral_account::create::CreateResponse = parse_contract(json!({
        "account": {
            "account_id": "acc_1",
            "assets": {"confirmed_bonus": {"point_bonus": 100}},
            "status": 1
        }
    }));
    assert_eq!(
        created
            .account
            .as_ref()
            .and_then(|v| v.account_id.as_deref()),
        Some("acc_1")
    );
    assert_eq!(
        created
            .account
            .as_ref()
            .and_then(|v| v.assets.as_ref())
            .and_then(|v| v.confirmed_bonus.as_ref())
            .and_then(|v| v.point_bonus),
        Some(100)
    );

    let reconciled: referral_account::reconciliation::ReconciliationResponse =
        parse_contract(json!({
            "check_failed_list": [{
                "account_id": "acc_1",
                "total_withdraw_reward_info": {"point_bonus": 50},
                "total_recharge_reward_info": {"point_bonus": 100}
            }]
        }));
    assert_eq!(
        reconciled.check_failed_list[0].account_id.as_deref(),
        Some("acc_1")
    );

    let referrals: referral::search::SearchResponse = parse_contract(json!({
        "items": [{
            "id": "ref_1",
            "application_ids": ["app_1"],
            "referral_user": {"id": "ou_ref", "name": {"zh_cn": "张三", "en_us": "Zhang"}}
        }]
    }));
    assert_eq!(
        referrals.items[0].application_ids.as_ref().map(|v| v.len()),
        Some(1)
    );
}

#[test]
fn interview_record_v2_contract_is_typed() {
    let records: interview_record_v2::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "record_1",
            "feedback_form_id": "form_1",
            "commit_status": 1,
            "record_score": {"score": 95.0, "total_score": 100.0},
            "interviewer": {"id": "ou_interviewer", "name": {"zh_cn": "面试官", "en_us": "Interviewer"}},
            "attachments": [{"file_id": "file_1", "file_name": "report.pdf", "content_type": "application/pdf"}],
            "module_assessments": [{
                "interview_feedback_form_module_id": "module_1",
                "module_name": {"zh_cn": "结论", "en_us": "Conclusion"},
                "dimension_assessments": [{
                    "interview_feedback_form_dimension_id": "dim_1",
                    "dimension_name": {"zh_cn": "知识", "en_us": "Knowledge"},
                    "dimension_score": 10.0
                }]
            }]
        }],
        "has_more": true,
        "page_token": "cursor_4"
    }));
    assert_eq!(
        records.items[0].attachments.as_ref().map(|v| v.len()),
        Some(1)
    );
    assert_eq!(
        records.items[0].module_assessments.as_ref().unwrap()[0]
            .dimension_assessments
            .as_ref()
            .unwrap()[0]
            .dimension_score,
        Some(10.0)
    );
}

#[test]
fn user_role_and_talent_pool_contracts_are_typed() {
    let roles: user_role::list::ListResponse = parse_contract(json!({
        "items": [{
            "user_id": "ou_user",
            "role_id": "101",
            "role_name": {"zh_cn": "招聘 HRBP", "en_us": "Recruitment HRBP"},
            "business_management_scopes": [{
                "entity": {"code": "application", "name": {"zh_cn": "测试", "en_us": "test"}},
                "scope_rule": {"rule_type": 1}
            }]
        }],
        "has_more": true
    }));
    assert_eq!(roles.items[0].role_id.as_deref(), Some("101"));
    assert_eq!(
        roles.items[0].business_management_scopes.as_ref().unwrap()[0]
            .scope_rule
            .as_ref()
            .unwrap()
            .rule_type,
        Some(1)
    );

    let pools: talent_pool::search::SearchResponse = parse_contract(json!({
        "items": [{
            "id": "pool_1",
            "i18n_name": {"zh_cn": "公共人才库", "en_us": "Common Talent Pool"},
            "is_private": 1
        }],
        "has_more": false
    }));
    assert_eq!(pools.items[0].id.as_deref(), Some("pool_1"));
}

#[test]
fn attachment_talent_object_and_talent_v2_contracts_are_typed() {
    let attachment: interview_record_v1::attachment::get::GetResponse = parse_contract(json!({
        "attachment": {
            "id": "att_1",
            "url": "https://hire.feishu.cn/blob/xx/",
            "name": "面试记录.pdf",
            "mime": "application/pdf"
        }
    }));
    assert_eq!(
        attachment.attachment.as_ref().and_then(|v| v.id.as_deref()),
        Some("att_1")
    );

    let talent_object_resp: talent_object::query::QueryResponse = parse_contract(json!({
        "items": [{
            "id": "obj_1",
            "name": {"zh_cn": "教育经历", "en_us": "Education"},
            "setting": {"object_type": 11, "config": {"options": [{"key": "1", "name": {"zh_cn": "选项1", "en_us": "Option1"}}]}},
            "children_list": [{
                "id": "child_1",
                "name": {"zh_cn": "学历", "en_us": "Degree"},
                "parent_id": "obj_1"
            }]
        }]
    }));
    assert_eq!(
        talent_object_resp.items[0].children_list.as_ref().unwrap()[0]
            .parent_id
            .as_deref(),
        Some("obj_1")
    );

    let talent_resp: talent::get::GetResponse = parse_contract(json!({
        "talent_id": "talent_1",
        "basic_info": {
            "name": "张三",
            "email": "test@example.com",
            "customized_data_list": [{
                "object_id": "obj_1",
                "name": {"zh_cn": "字段1", "en_us": "field1"},
                "value": {
                    "content": "text",
                    "option": {"key": "opt_1", "name": {"zh_cn": "选项1", "en_us": "Option1"}}
                }
            }]
        }
    }));
    assert_eq!(talent_resp.talent_id.as_deref(), Some("talent_1"));
    assert_eq!(
        talent_resp
            .basic_info
            .as_ref()
            .unwrap()
            .customized_data_list
            .as_ref()
            .unwrap()[0]
            .object_id
            .as_deref(),
        Some("obj_1")
    );
}
