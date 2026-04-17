use openlark_hr::hire::hire::{
    v1::{
        application, attachment, evaluation, external_application, external_offer,
        interview_record as interview_record_v1, interviewer, job, location, note, offer,
        offer_application_form, offer_schema, referral, referral_account, role, subject,
        talent_object, talent_pool, todo, user_role, website,
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

#[test]
fn note_and_attachment_contracts_are_typed() {
    let created: note::create::CreateResponse = parse_contract(json!({
        "note": {
            "id": "note_1",
            "talent_id": "talent_1",
            "application_id": "app_1",
            "is_private": false,
            "content": "测试"
        }
    }));
    assert_eq!(
        created.note.as_ref().and_then(|v| v.id.as_deref()),
        Some("note_1")
    );

    let notes: note::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "note_1",
            "creator_id": "ou_creator",
            "content": "备注列表项"
        }],
        "has_more": true,
        "page_token": "cursor_note"
    }));
    assert_eq!(notes.items[0].creator_id.as_deref(), Some("ou_creator"));
    assert_eq!(notes.page_token.as_deref(), Some("cursor_note"));

    let attachment_created: attachment::create::CreateResponse = parse_contract(json!({
        "id": "att_1",
        "name": "test.rar",
        "url": "https://example.com/blob"
    }));
    assert_eq!(attachment_created.id.as_deref(), Some("att_1"));

    let attachment_get: attachment::get::GetResponse = parse_contract(json!({
        "attachment": {
            "id": "att_1",
            "name": "resume.pdf",
            "mime": "application/pdf"
        }
    }));
    assert_eq!(
        attachment_get
            .attachment
            .as_ref()
            .and_then(|v| v.name.as_deref()),
        Some("resume.pdf")
    );

    let attachment_preview: attachment::preview::PreviewResponse = parse_contract(json!({
        "url": "https://example.com/blob"
    }));
    assert_eq!(
        attachment_preview.url.as_deref(),
        Some("https://example.com/blob")
    );
}

#[test]
fn offer_contracts_are_typed() {
    let offers: offer::list::ListResponse = parse_contract(json!({
        "has_more": true,
        "page_token": "cursor_offer",
        "items": [{
            "id": "offer_1",
            "job_info": {"job_id": "job_1", "job_name": "后端工程师"},
            "offer_status": 1,
            "employee_type": {"id": "1", "zh_name": "正式", "en_name": "Regular"}
        }]
    }));
    assert_eq!(
        offers.items[0]
            .job_info
            .as_ref()
            .and_then(|v| v.job_id.as_deref()),
        Some("job_1")
    );

    let created: offer::create::CreateResponse = parse_contract(json!({
        "offer_id": "offer_1",
        "application_id": "app_1",
        "schema_id": "schema_1",
        "offer_type": 1,
        "basic_info": {"department_id": "od_1", "leader_user_id": "ou_leader"},
        "salary_info": {"currency": "CNY", "basic_salary": "1000000"},
        "customized_info_list": [{"id": "field_1", "value": "1"}]
    }));
    assert_eq!(created.offer_id.as_deref(), Some("offer_1"));
    assert_eq!(
        created
            .salary_info
            .as_ref()
            .and_then(|v| v.currency.as_deref()),
        Some("CNY")
    );

    let updated: offer::update::UpdateResponse = parse_contract(json!({
        "offer_id": "offer_1",
        "schema_id": "schema_1",
        "offer_type": 1,
        "basic_info": {"department_id": "od_1"}
    }));
    assert_eq!(updated.schema_id.as_deref(), Some("schema_1"));

    let status: offer::offer_status::OfferStatusResponse = parse_contract(json!({}));
    let intern: offer::intern_offer_status::InternOfferStatusResponse = parse_contract(json!({
        "offer_id": "offer_1",
        "operation": "confirm_onboarding",
        "onboarding_info": {"actual_onboarding_date": "2022-01-01"}
    }));
    assert_eq!(
        intern
            .onboarding_info
            .as_ref()
            .and_then(|v| v.actual_onboarding_date.as_deref()),
        Some("2022-01-01")
    );
    let _ = status;

    let forms: offer_application_form::list::ListResponse = parse_contract(json!({
        "has_more": false,
        "items": [{
            "id": "form_1",
            "name": {"zh_cn": "校招 Offer 申请表", "en_us": "campus offer application form"},
            "create_time": "1628512038000"
        }]
    }));
    assert_eq!(forms.items[0].id.as_deref(), Some("form_1"));
}

#[test]
fn offer_detail_and_schema_contracts_are_typed() {
    let detail: offer::get::GetResponse = parse_contract(json!({
        "offer": {
            "id": "offer_1",
            "application_id": "app_1",
            "basic_info": {
                "offer_type": 1,
                "employee_type": {"id": "1", "zh_name": "正式", "en_name": "Regular"},
                "onboard_address": {
                    "id": "addr_1",
                    "zh_name": "名字",
                    "en_name": "name",
                    "city": {"zh_name": "中文", "en_name": "eng", "code": "400700", "location_type": 3}
                }
            },
            "salary_plan": {"currency": "CNY", "basic_salary": "1000000"}
        }
    }));
    assert_eq!(
        detail
            .offer
            .as_ref()
            .and_then(|v| v.salary_plan.as_ref())
            .and_then(|v| v.currency.as_deref()),
        Some("CNY")
    );

    let form: offer_application_form::get::GetResponse = parse_contract(json!({
        "offer_apply_form": {
            "id": "form_1",
            "name": {"zh_cn": "校招 Offer 申请表", "en_us": "campus offer application form"},
            "schema": {
                "id": "schema_1",
                "module_list": [{
                    "id": "module_1",
                    "name": {"zh_cn": "基础信息模块", "en_us": "basic info module"},
                    "object_list": [{
                        "id": "obj_1",
                        "name": {"zh_cn": "薪资字段", "en_us": "salary field"},
                        "config": {
                            "options": [{"id": "opt_1", "name": {"zh_cn": "全年薪资", "en_us": "annual salary"}}],
                            "object_display_config": {"display_condition": 1}
                        }
                    }]
                }]
            }
        }
    }));
    assert_eq!(
        form.offer_apply_form
            .as_ref()
            .and_then(|v| v.schema.as_ref())
            .and_then(|v| v.module_list.as_ref())
            .map(|v| v.len()),
        Some(1)
    );

    let schema: offer_schema::get::GetResponse = parse_contract(json!({
        "id": "schema_1",
        "scenario": 1,
        "version": 121,
        "object_list": [{
            "id": "obj_1",
            "name": {"zh_cn": "名字", "en_us": "name"},
            "type": "number",
            "is_customized": true,
            "option_list": [{
                "name": {"zh_cn": "名字", "en_us": "name"},
                "index": 121,
                "active_status": 1
            }]
        }]
    }));
    assert_eq!(schema.object_list[0].object_type.as_deref(), Some("number"));
}

#[test]
fn application_contracts_are_typed() {
    let created: application::create::CreateResponse = parse_contract(json!({
        "application_id": "app_1",
        "talent_id": "talent_1",
        "job_id": "job_1",
        "application_status": 1,
        "stage_id": "stage_resume",
        "stage_name": "简历筛选",
        "job_info": {"job_id": "job_1", "job_name": "后端工程师"}
    }));
    assert_eq!(created.application_id.as_deref(), Some("app_1"));

    let get_resp: application::get::GetResponse = parse_contract(json!({
        "application_id": "app_1",
        "talent_id": "talent_1",
        "job_info": {"job_id": "job_1", "job_name": "后端工程师"},
        "stage_name": "初筛"
    }));
    assert_eq!(
        get_resp
            .job_info
            .as_ref()
            .and_then(|v| v.job_name.as_deref()),
        Some("后端工程师")
    );

    let detail: application::get_detail::GetDetailResponse = parse_contract(json!({
        "application_id": "app_1",
        "talent_info": {
            "talent_id": "talent_1",
            "talent_name": "张三",
            "email": "zhangsan@example.com"
        },
        "job_info": {"job_id": "job_1", "job_name": "后端工程师"},
        "offer_info": {"offer_id": "offer_1", "offer_status": 2}
    }));
    assert_eq!(
        detail
            .talent_info
            .as_ref()
            .and_then(|v| v.talent_name.as_deref()),
        Some("张三")
    );

    let list: application::list::ListResponse = parse_contract(json!({
        "items": [{
            "application_id": "app_1",
            "talent_id": "talent_1",
            "job_info": {"job_id": "job_1", "job_name": "后端工程师"},
            "application_status": 1
        }],
        "has_more": true,
        "page_token": "cursor_app"
    }));
    assert_eq!(list.items[0].application_id.as_deref(), Some("app_1"));
    assert_eq!(list.page_token.as_deref(), Some("cursor_app"));

    let interviews: application::interview::list::ListResponse = parse_contract(json!({
        "records": [{
            "interview_id": "iv_1",
            "interview_round_id": "round_1",
            "interview_round_name": "一面",
            "status": 2,
            "interviewer": {"id": "ou_1", "name": {"zh_cn": "面试官", "en_us": "Interviewer"}},
            "score": {"score": 88.0, "total_score": 100.0}
        }],
        "has_more": false
    }));
    assert_eq!(interviews.records[0].interview_id.as_deref(), Some("iv_1"));
    assert_eq!(
        interviews.records[0].score.as_ref().and_then(|v| v.score),
        Some(88.0)
    );
}

#[test]
fn job_contracts_are_typed() {
    let created: job::combined_create::CombinedCreateResponse = parse_contract(json!({
        "job_id": "job_1",
        "active_status": 1,
        "success": true
    }));
    assert_eq!(created.job_id.as_deref(), Some("job_1"));

    let get_resp: job::get::GetResponse = parse_contract(json!({
        "job_id": "job_1",
        "job_name": "后端工程师",
        "active_status": 1,
        "process_id": "process_1",
        "process_name": "标准流程",
        "department_id": "dept_1",
        "recruiters": [{
            "recruiter_id": "rec_1",
            "user_id": "ou_1",
            "name": "招聘负责人",
            "role": "owner",
            "role_type": 1
        }]
    }));
    assert_eq!(get_resp.job_name.as_deref(), Some("后端工程师"));
    assert_eq!(get_resp.recruiters.as_ref().map(|v| v.len()), Some(1));

    let detail: job::get_detail::GetDetailResponse = parse_contract(json!({
        "job_id": "job_1",
        "job_name": "后端工程师",
        "job_description": "负责核心服务开发",
        "recruiters": [{
            "recruiter_id": "rec_1",
            "name": "招聘负责人"
        }]
    }));
    assert_eq!(detail.job_description.as_deref(), Some("负责核心服务开发"));

    let listed: job::list::ListResponse = parse_contract(json!({
        "items": [{
            "job_id": "job_1",
            "job_name": "后端工程师",
            "active_status": 1,
            "process_name": "标准流程"
        }],
        "has_more": true,
        "page_token": "cursor_job"
    }));
    assert_eq!(listed.items[0].job_id.as_deref(), Some("job_1"));
    assert_eq!(listed.page_token.as_deref(), Some("cursor_job"));

    let config: job::config::ConfigResponse = parse_contract(json!({
        "job_id": "job_1",
        "process_id": "process_1",
        "process_name": "标准流程",
        "job_requirement_schema_id": "schema_job",
        "offer_application_form_id": "form_offer"
    }));
    assert_eq!(config.config.process_id.as_deref(), Some("process_1"));

    let recruiters: job::recruiter::RecruiterResponse = parse_contract(json!({
        "recruiters": [{
            "recruiter_id": "rec_1",
            "user_id": "ou_1",
            "name": "招聘负责人",
            "role_type": 1
        }],
        "has_more": false
    }));
    assert_eq!(recruiters.recruiters[0].user_id.as_deref(), Some("ou_1"));

    let opened: job::open::OpenResponse = parse_contract(json!({
        "job_id": "job_1",
        "result": true
    }));
    assert_eq!(opened.result, Some(true));

    let closed: job::close::CloseResponse = parse_contract(json!({
        "job_id": "job_1",
        "success": true
    }));
    assert_eq!(closed.success, Some(true));

    let updated: job::combined_update::CombinedUpdateResponse = parse_contract(json!({
        "job_id": "job_1",
        "result": true
    }));
    assert_eq!(updated.job_id.as_deref(), Some("job_1"));

    let update_config: job::update_config::UpdateConfigResponse = parse_contract(json!({
        "job_id": "job_1",
        "result": true
    }));
    assert_eq!(update_config.result, Some(true));
}

#[test]
fn external_application_and_offer_contracts_are_typed() {
    let ext_app_created: external_application::create::CreateResponse = parse_contract(json!({
        "external_application_id": "ext_app_1",
        "application_id": "app_1",
        "status": 1,
        "success": true
    }));
    assert_eq!(
        ext_app_created.external_application_id.as_deref(),
        Some("ext_app_1")
    );

    let ext_app_list: external_application::list::ListResponse = parse_contract(json!({
        "items": [{
            "external_application_id": "ext_app_1",
            "application_id": "app_1",
            "talent_id": "talent_1",
            "job_id": "job_1",
            "status": 1,
            "source_name": "Boss"
        }],
        "has_more": false
    }));
    assert_eq!(ext_app_list.items[0].source_name.as_deref(), Some("Boss"));

    let ext_app_updated: external_application::update::UpdateResponse = parse_contract(json!({
        "external_application_id": "ext_app_1",
        "result": true
    }));
    assert_eq!(ext_app_updated.result, Some(true));

    let ext_offer_created: external_offer::create::CreateResponse = parse_contract(json!({
        "external_offer_id": "ext_offer_1",
        "offer_id": "offer_1",
        "status": 2,
        "success": true
    }));
    assert_eq!(ext_offer_created.offer_id.as_deref(), Some("offer_1"));

    let ext_offer_list: external_offer::batch_query::BatchQueryResponse = parse_contract(json!({
        "items": [{
            "external_offer_id": "ext_offer_1",
            "offer_id": "offer_1",
            "application_id": "app_1",
            "talent_id": "talent_1",
            "status": 2
        }],
        "page_token": "cursor_ext_offer"
    }));
    assert_eq!(ext_offer_list.items[0].status, Some(2));
    assert_eq!(
        ext_offer_list.page_token.as_deref(),
        Some("cursor_ext_offer")
    );

    let ext_offer_updated: external_offer::update::UpdateResponse = parse_contract(json!({
        "external_offer_id": "ext_offer_1",
        "offer_id": "offer_1",
        "result": true
    }));
    assert_eq!(
        ext_offer_updated.external_offer_id.as_deref(),
        Some("ext_offer_1")
    );
}

#[test]
fn website_channel_delivery_and_job_post_contracts_are_typed() {
    let channel_created: website::channel::create::CreateResponse = parse_contract(json!({
        "channel": {
            "channel_id": "channel_1",
            "website_id": "site_1",
            "name": "Boss",
            "code": "boss",
            "active_status": 1
        }
    }));
    assert_eq!(
        channel_created
            .channel
            .as_ref()
            .and_then(|v| v.channel_id.as_deref()),
        Some("channel_1")
    );

    let channels: website::channel::list::ListResponse = parse_contract(json!({
        "items": [{
            "channel_id": "channel_1",
            "website_id": "site_1",
            "name": "Boss",
            "code": "boss",
            "active_status": 1
        }],
        "has_more": false
    }));
    assert_eq!(channels.items[0].code.as_deref(), Some("boss"));

    let delivery: website::delivery_task::get::GetResponse = parse_contract(json!({
        "delivery_task_id": "task_1",
        "application_id": "app_1",
        "talent_id": "talent_1",
        "status": 2,
        "error_message": "invalid resume"
    }));
    assert_eq!(delivery.delivery_task.status, Some(2));

    let post: website::job_post::get::GetResponse = parse_contract(json!({
        "job_post": {
            "job_post_id": "post_1",
            "website_id": "site_1",
            "job_id": "job_1",
            "title": "后端工程师",
            "active_status": 1,
            "job_channel_id": "channel_1"
        }
    }));
    assert_eq!(
        post.job_post
            .as_ref()
            .and_then(|v| v.job_channel_id.as_deref()),
        Some("channel_1")
    );

    let post_list: website::job_post::list::ListResponse = parse_contract(json!({
        "items": [{
            "job_post_id": "post_1",
            "website_id": "site_1",
            "job_id": "job_1",
            "title": "后端工程师"
        }],
        "page_token": "cursor_post"
    }));
    assert_eq!(post_list.page_token.as_deref(), Some("cursor_post"));

    let site_user: website::site_user::create::CreateResponse = parse_contract(json!({
        "site_user": {
            "site_user_id": "site_user_1",
            "website_id": "site_1",
            "user_id": "ou_1",
            "email": "test@example.com"
        }
    }));
    assert_eq!(
        site_user
            .site_user
            .as_ref()
            .and_then(|v| v.user_id.as_deref()),
        Some("ou_1")
    );
}
