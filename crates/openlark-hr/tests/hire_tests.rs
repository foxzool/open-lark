use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::hire::hire::v1::*;
use rstest::rstest;
use serde_json::json;
use wiremock::{
    matchers::{body_json, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

fn test_config(base_url: &str) -> Config {
    Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url(base_url)
        .enable_token_cache(false)
        .build()
}

fn auth_option() -> RequestOption {
    RequestOption::builder()
        .user_access_token("test_token")
        .build()
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    macro_rules! smoke_builder {
        ($name:ident, $expr:expr) => {
            #[test]
            fn $name() {
                let _request = $expr;
            }
        };
    }

    smoke_builder!(
        test_talent_combined_create_builder,
        talent::CombinedCreateRequest::new(test_config("https://open.feishu.cn"))
            .name("张三".to_string())
            .email("zhangsan@example.com".to_string())
    );
    smoke_builder!(
        test_talent_get_builder,
        talent::GetRequest::new(test_config("https://open.feishu.cn"))
            .talent_id("talent_001".to_string())
            .need_detail(true)
    );
    smoke_builder!(
        test_talent_combined_update_builder,
        talent::CombinedUpdateRequest::new(test_config("https://open.feishu.cn"))
            .talent_id("talent_001".to_string())
            .name("李四".to_string())
    );
    smoke_builder!(
        test_talent_list_builder,
        talent::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .query("后端".to_string())
    );
    smoke_builder!(
        test_talent_batch_get_id_builder,
        talent::BatchGetIdRequest::new(test_config("https://open.feishu.cn"))
            .add_talent_id("talent_001".to_string())
            .add_talent_id("talent_002".to_string())
    );

    smoke_builder!(
        test_interview_list_builder,
        interview::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_interview_get_by_talent_builder,
        interview::get_by_talent::GetByTalentRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_application_interview_list_builder,
        application::interview::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_interview_record_get_builder,
        interview_record::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_interview_record_list_builder,
        interview_record::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_offer_create_builder,
        offer::create::CreateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_offer_get_builder,
        offer::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_offer_update_builder,
        offer::update::UpdateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_offer_list_builder,
        offer::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_offer_status_builder,
        offer::offer_status::OfferStatusRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_application_create_builder,
        application::create::CreateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_application_get_builder,
        application::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_application_get_detail_builder,
        application::get_detail::GetDetailRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_application_list_builder,
        application::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_job_combined_create_builder,
        job::combined_create::CombinedCreateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_job_get_builder,
        job::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_job_combined_update_builder,
        job::combined_update::CombinedUpdateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_job_list_builder,
        job::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_job_open_builder,
        job::open::OpenRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_evaluation_list_builder,
        evaluation::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_evaluation_task_list_builder,
        evaluation_task::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_interview_feedback_form_list_builder,
        interview_feedback_form::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_advertisement_publish_builder,
        advertisement::publish::PublishRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_attachment_get_builder,
        attachment::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_referral_search_builder,
        referral::search::SearchRequest::new(test_config("https://open.feishu.cn"))
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_talent_combined_create_name_validation(#[case] name: &str) {
        let result = talent::CombinedCreateRequest::new(test_config("https://127.0.0.1:9"))
            .name(name.to_string())
            .email("test@example.com".to_string())
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("候选人姓名不能为空"));
    }

    #[tokio::test]
    async fn test_talent_combined_create_contact_validation() {
        let result = talent::CombinedCreateRequest::new(test_config("https://127.0.0.1:9"))
            .name("张三".to_string())
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("至少需要提供邮箱或手机号"));
    }

    #[rstest]
    #[case(0)]
    #[case(101)]
    #[tokio::test]
    async fn test_talent_list_page_size_boundary(#[case] page_size: i32) {
        let result = talent::ListRequest::new(test_config("https://127.0.0.1:9"))
            .page_size(page_size)
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("page_size 必须在 1-100 之间"));
    }

    #[tokio::test]
    async fn test_talent_batch_get_id_empty_validation() {
        let result = talent::BatchGetIdRequest::new(test_config("https://127.0.0.1:9"))
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("不能为空"));
    }

    #[tokio::test]
    async fn test_talent_combined_update_requires_fields() {
        let result = talent::CombinedUpdateRequest::new(test_config("https://127.0.0.1:9"))
            .talent_id("talent_001".to_string())
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("至少需要提供一个更新字段"));
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    #[tokio::test]
    async fn test_talent_combined_create_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/talents/combined_create"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(
                json!({"name":"张三","email":"zhangsan@example.com"}),
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    json!({"code":0,"msg":"success","data":{"talent_id":"talent_001"}}),
                ),
            )
            .mount(&mock_server)
            .await;

        let resp = talent::CombinedCreateRequest::new(test_config(&mock_server.uri()))
            .name("张三".to_string())
            .email("zhangsan@example.com".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.talent_id, "talent_001");
    }

    #[tokio::test]
    async fn test_talent_get_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/talents/talent_001"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{
                    "talent":{"talent_id":"talent_001","name":"张三"}
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = talent::GetRequest::new(test_config(&mock_server.uri()))
            .talent_id("talent_001".to_string())
            .need_detail(true)
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.talent.talent_id, "talent_001");
    }

    #[tokio::test]
    async fn test_talent_combined_update_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/talents/combined_update"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({"talent_id":"talent_001","name":"李四"})))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"success","data":{"result":true}})),
            )
            .mount(&mock_server)
            .await;

        let resp = talent::CombinedUpdateRequest::new(test_config(&mock_server.uri()))
            .talent_id("talent_001".to_string())
            .name("李四".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_talent_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/talents"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{
                    "talent_list":[{"talent_id":"talent_001","name":"张三"}],
                    "has_more":false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = talent::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.talent_list.len(), 1);
    }

    #[tokio::test]
    async fn test_talent_batch_get_id_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/talents/batch_get_id"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({"talent_ids":["talent_001","talent_002"]})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{
                    "talent_id_list":[
                        {"talent_id":"talent_001","is_valid":true},
                        {"talent_id":"talent_002","is_valid":true}
                    ]
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = talent::BatchGetIdRequest::new(test_config(&mock_server.uri()))
            .talent_ids(vec!["talent_001".to_string(), "talent_002".to_string()])
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.talent_id_list.len(), 2);
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    macro_rules! roundtrip_eq {
        ($name:ident, $ty:ty, $value:expr) => {
            #[test]
            fn $name() {
                let origin: $ty = $value;
                let raw = serde_json::to_string(&origin).unwrap();
                let decoded: $ty = serde_json::from_str(&raw).unwrap();
                assert_eq!(origin, decoded);
            }
        };
    }

    roundtrip_eq!(
        test_talent_combined_create_response_serialization,
        talent::CombinedCreateResponse,
        talent::CombinedCreateResponse {
            talent_id: "talent_001".to_string()
        }
    );
    roundtrip_eq!(
        test_talent_get_response_serialization,
        talent::GetResponse,
        talent::GetResponse {
            talent: talent::Talent {
                talent_id: "talent_001".to_string(),
                name: Some("张三".to_string()),
                resume: None,
                email: Some("zhangsan@example.com".to_string()),
                phone: None,
                status: Some(1),
                create_time: None,
                update_time: None,
            },
            education_list: None,
            work_experience_list: None,
        }
    );
    roundtrip_eq!(
        test_talent_combined_update_response_serialization,
        talent::CombinedUpdateResponse,
        talent::CombinedUpdateResponse { result: true }
    );
    roundtrip_eq!(
        test_talent_list_response_serialization,
        talent::ListResponse,
        talent::ListResponse {
            talent_list: vec![talent::Talent {
                talent_id: "talent_001".to_string(),
                name: Some("张三".to_string()),
                resume: None,
                email: None,
                phone: None,
                status: Some(1),
                create_time: None,
                update_time: None,
            }],
            page_token: Some("next_1".to_string()),
            has_more: Some(false),
        }
    );
    roundtrip_eq!(
        test_talent_batch_get_id_response_serialization,
        talent::BatchGetIdResponse,
        talent::BatchGetIdResponse {
            talent_id_list: vec![talent::TalentIdInfo {
                talent_id: "talent_001".to_string(),
                is_valid: true,
            }],
        }
    );

    roundtrip_eq!(
        test_interview_list_response_serialization,
        interview::list::ListResponse,
        interview::list::ListResponse {
            data: json!({"items":[]})
        }
    );
    roundtrip_eq!(
        test_interview_get_by_talent_response_serialization,
        interview::get_by_talent::GetByTalentResponse,
        interview::get_by_talent::GetByTalentResponse {
            data: json!({"interview_id":"iv_001"})
        }
    );
    roundtrip_eq!(
        test_application_interview_list_response_serialization,
        application::interview::list::ListResponse,
        application::interview::list::ListResponse {
            records: vec![],
            page_token: None,
            has_more: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_interview_record_get_response_serialization,
        interview_record::get::GetResponse,
        interview_record::get::GetResponse {
            data: json!({"record_id":"ir_001"})
        }
    );
    roundtrip_eq!(
        test_interview_record_list_response_serialization,
        interview_record::list::ListResponse,
        interview_record::list::ListResponse {
            items: vec![],
            page_token: None,
            has_more: None,
            extra: Default::default(),
        }
    );

    roundtrip_eq!(
        test_offer_create_response_serialization,
        offer::create::CreateResponse,
        offer::create::CreateResponse {
            offer_id: Some("of_001".to_string()),
            application_id: Some("ap_001".to_string()),
            schema_id: Some("schema_1".to_string()),
            offer_type: Some(1),
            basic_info: None,
            salary_info: None,
            customized_info_list: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_get_response_serialization,
        offer::get::GetResponse,
        offer::get::GetResponse {
            offer: Some(offer::get::OfferDetail {
                id: Some("of_001".to_string()),
                application_id: Some("ap_001".to_string()),
                basic_info: None,
                salary_plan: None,
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_update_response_serialization,
        offer::update::UpdateResponse,
        offer::update::UpdateResponse {
            offer_id: Some("of_001".to_string()),
            application_id: None,
            schema_id: Some("schema_1".to_string()),
            offer_type: Some(1),
            basic_info: None,
            salary_info: None,
            customized_info_list: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_list_response_serialization,
        offer::list::ListResponse,
        offer::list::ListResponse {
            has_more: Some(false),
            page_token: None,
            items: vec![],
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_intern_status_response_serialization,
        offer::intern_offer_status::InternOfferStatusResponse,
        offer::intern_offer_status::InternOfferStatusResponse {
            offer_id: Some("of_001".to_string()),
            operation: Some("confirm_onboarding".to_string()),
            onboarding_info: Some(offer::intern_offer_status::InternOnboardingInfo {
                actual_onboarding_date: Some("2022-01-01".to_string()),
                extra: Default::default(),
            }),
            offboarding_info: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_status_response_serialization,
        offer::offer_status::OfferStatusResponse,
        offer::offer_status::OfferStatusResponse {}
    );

    roundtrip_eq!(
        test_application_create_response_serialization,
        application::create::CreateResponse,
        application::create::CreateResponse {
            application_id: Some("ap_001".to_string()),
            talent_id: Some("talent_001".to_string()),
            job_id: Some("job_001".to_string()),
            application_status: Some(1),
            stage_id: Some("stage_1".to_string()),
            stage_name: Some("初筛".to_string()),
            job_info: Some(openlark_hr::hire::hire::common_models::ApplicationJobInfo {
                job_id: Some("job_001".to_string()),
                job_name: Some("后端工程师".to_string()),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_application_get_response_serialization,
        application::get::GetResponse,
        application::get::GetResponse {
            application_id: Some("ap_001".to_string()),
            talent_id: Some("talent_001".to_string()),
            job_id: Some("job_001".to_string()),
            application_status: Some(1),
            stage_id: Some("stage_1".to_string()),
            stage_name: Some("初筛".to_string()),
            job_info: Some(openlark_hr::hire::hire::common_models::ApplicationJobInfo {
                job_id: Some("job_001".to_string()),
                job_name: Some("后端工程师".to_string()),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_application_get_detail_response_serialization,
        application::get_detail::GetDetailResponse,
        application::get_detail::GetDetailResponse {
            application_id: Some("ap_001".to_string()),
            talent_id: Some("talent_001".to_string()),
            job_id: Some("job_001".to_string()),
            application_status: Some(2),
            stage_id: Some("stage_offer".to_string()),
            stage_name: Some("Offer".to_string()),
            job_info: Some(openlark_hr::hire::hire::common_models::ApplicationJobInfo {
                job_id: Some("job_001".to_string()),
                job_name: Some("后端工程师".to_string()),
                extra: Default::default(),
            }),
            talent_info: Some(
                openlark_hr::hire::hire::common_models::ApplicationTalentInfo {
                    talent_id: Some("talent_001".to_string()),
                    talent_name: Some("张三".to_string()),
                    mobile: None,
                    email: Some("zhangsan@example.com".to_string()),
                    extra: Default::default(),
                }
            ),
            offer_info: Some(
                openlark_hr::hire::hire::common_models::ApplicationOfferInfo {
                    offer_id: Some("offer_001".to_string()),
                    offer_status: Some(1),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_application_list_response_serialization,
        application::list::ListResponse,
        application::list::ListResponse {
            items: vec![openlark_hr::hire::hire::common_models::ApplicationSummary {
                application_id: Some("ap_001".to_string()),
                talent_id: Some("talent_001".to_string()),
                job_id: Some("job_001".to_string()),
                application_status: Some(1),
                stage_id: Some("stage_1".to_string()),
                stage_name: Some("初筛".to_string()),
                job_info: Some(openlark_hr::hire::hire::common_models::ApplicationJobInfo {
                    job_id: Some("job_001".to_string()),
                    job_name: Some("后端工程师".to_string()),
                    extra: Default::default(),
                }),
                extra: Default::default(),
            }],
            page_token: Some("cursor_1".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );

    roundtrip_eq!(
        test_job_combined_create_response_serialization,
        job::combined_create::CombinedCreateResponse,
        job::combined_create::CombinedCreateResponse {
            job_id: Some("job_001".to_string()),
            active_status: Some(1),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_get_response_serialization,
        job::get::GetResponse,
        job::get::GetResponse {
            job_id: Some("job_001".to_string()),
            job_name: Some("后端工程师".to_string()),
            active_status: Some(1),
            process_id: Some("process_1".to_string()),
            process_name: Some("标准流程".to_string()),
            department_id: Some("dept_1".to_string()),
            recruiters: Some(vec![
                openlark_hr::hire::hire::common_models::JobRecruiterRecord {
                    recruiter_id: Some("rec_1".to_string()),
                    manager_id: None,
                    user_id: Some("ou_1".to_string()),
                    name: Some("招聘负责人".to_string()),
                    role: Some("owner".to_string()),
                    role_type: Some(1),
                    extra: Default::default(),
                }
            ]),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_get_detail_response_serialization,
        job::get_detail::GetDetailResponse,
        job::get_detail::GetDetailResponse {
            job_id: Some("job_001".to_string()),
            job_name: Some("后端工程师".to_string()),
            active_status: Some(1),
            process_id: Some("process_1".to_string()),
            process_name: Some("标准流程".to_string()),
            department_id: Some("dept_1".to_string()),
            job_description: Some("负责核心服务开发".to_string()),
            recruiters: Some(vec![
                openlark_hr::hire::hire::common_models::JobRecruiterRecord {
                    recruiter_id: Some("rec_1".to_string()),
                    manager_id: None,
                    user_id: Some("ou_1".to_string()),
                    name: Some("招聘负责人".to_string()),
                    role: Some("owner".to_string()),
                    role_type: Some(1),
                    extra: Default::default(),
                }
            ]),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_combined_update_response_serialization,
        job::combined_update::CombinedUpdateResponse,
        job::combined_update::CombinedUpdateResponse {
            job_id: Some("job_001".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_list_response_serialization,
        job::list::ListResponse,
        job::list::ListResponse {
            items: vec![openlark_hr::hire::hire::common_models::JobSummary {
                job_id: Some("job_001".to_string()),
                job_name: Some("后端工程师".to_string()),
                active_status: Some(1),
                process_id: Some("process_1".to_string()),
                process_name: Some("标准流程".to_string()),
                department_id: Some("dept_1".to_string()),
                job_description: None,
                recruiters: None,
                extra: Default::default(),
            }],
            page_token: Some("cursor_job".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_open_response_serialization,
        job::open::OpenResponse,
        job::open::OpenResponse {
            job_id: Some("job_001".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_close_response_serialization,
        job::close::CloseResponse,
        job::close::CloseResponse {
            job_id: Some("job_001".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_config_response_serialization,
        job::config::ConfigResponse,
        job::config::ConfigResponse {
            config: openlark_hr::hire::hire::common_models::JobConfigInfo {
                job_id: Some("job_001".to_string()),
                process_id: Some("process_1".to_string()),
                process_name: Some("标准流程".to_string()),
                job_requirement_schema_id: Some("schema_job".to_string()),
                interview_registration_schema_id: Some("schema_interview".to_string()),
                offer_application_form_id: Some("form_offer".to_string()),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_job_recruiter_response_serialization,
        job::recruiter::RecruiterResponse,
        job::recruiter::RecruiterResponse {
            recruiters: vec![openlark_hr::hire::hire::common_models::JobRecruiterRecord {
                recruiter_id: Some("rec_1".to_string()),
                manager_id: None,
                user_id: Some("ou_1".to_string()),
                name: Some("招聘负责人".to_string()),
                role: Some("owner".to_string()),
                role_type: Some(1),
                extra: Default::default(),
            }],
            page_token: None,
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_update_config_response_serialization,
        job::update_config::UpdateConfigResponse,
        job::update_config::UpdateConfigResponse {
            job_id: Some("job_001".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_create_response_serialization,
        external_application::create::CreateResponse,
        external_application::create::CreateResponse {
            external_application_id: Some("ext_app_1".to_string()),
            application_id: Some("app_1".to_string()),
            status: Some(1),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_delete_response_serialization,
        external_application::delete::DeleteResponse,
        external_application::delete::DeleteResponse {
            external_application_id: Some("ext_app_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_list_response_serialization,
        external_application::list::ListResponse,
        external_application::list::ListResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::ExternalApplicationSummary {
                    external_application_id: Some("ext_app_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    job_id: Some("job_1".to_string()),
                    status: Some(1),
                    source_name: Some("Boss".to_string()),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_ext_app".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_update_response_serialization,
        external_application::update::UpdateResponse,
        external_application::update::UpdateResponse {
            external_application_id: Some("ext_app_1".to_string()),
            application_id: Some("app_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_create_response_serialization,
        external_offer::create::CreateResponse,
        external_offer::create::CreateResponse {
            external_offer_id: Some("ext_offer_1".to_string()),
            offer_id: Some("offer_1".to_string()),
            status: Some(1),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_delete_response_serialization,
        external_offer::delete::DeleteResponse,
        external_offer::delete::DeleteResponse {
            external_offer_id: Some("ext_offer_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_batch_query_response_serialization,
        external_offer::batch_query::BatchQueryResponse,
        external_offer::batch_query::BatchQueryResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::ExternalOfferSummary {
                    external_offer_id: Some("ext_offer_1".to_string()),
                    offer_id: Some("offer_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    status: Some(2),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_ext_offer".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_update_response_serialization,
        external_offer::update::UpdateResponse,
        external_offer::update::UpdateResponse {
            external_offer_id: Some("ext_offer_1".to_string()),
            offer_id: Some("offer_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );

    roundtrip_eq!(
        test_evaluation_list_response_serialization,
        evaluation::list::ListResponse,
        evaluation::list::ListResponse {
            items: vec![],
            page_token: None,
            has_more: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_evaluation_task_list_response_serialization,
        evaluation_task::list::ListResponse,
        evaluation_task::list::ListResponse {
            items: vec![],
            page_token: None,
            has_more: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_interview_feedback_form_list_response_serialization,
        interview_feedback_form::list::ListResponse,
        interview_feedback_form::list::ListResponse {
            data: json!({"items":[]})
        }
    );

    roundtrip_eq!(
        test_note_create_response_serialization,
        note::create::CreateResponse,
        note::create::CreateResponse {
            note: Some(openlark_hr::hire::hire::common_models::NoteRecord {
                id: Some("note_1".to_string()),
                content: Some("备注".to_string()),
                ..Default::default()
            })
        }
    );
    roundtrip_eq!(
        test_note_get_response_serialization,
        note::get::GetResponse,
        note::get::GetResponse {
            note: Some(openlark_hr::hire::hire::common_models::NoteRecord {
                id: Some("note_1".to_string()),
                content: Some("备注".to_string()),
                ..Default::default()
            })
        }
    );
    roundtrip_eq!(
        test_note_list_response_serialization,
        note::list::ListResponse,
        note::list::ListResponse {
            items: vec![openlark_hr::hire::hire::common_models::NoteRecord {
                id: Some("note_1".to_string()),
                content: Some("备注".to_string()),
                ..Default::default()
            }],
            has_more: Some(false),
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_note_patch_response_serialization,
        note::patch::PatchResponse,
        note::patch::PatchResponse {
            note: Some(openlark_hr::hire::hire::common_models::NoteRecord {
                id: Some("note_1".to_string()),
                content: Some("更新备注".to_string()),
                ..Default::default()
            })
        }
    );
    roundtrip_eq!(
        test_note_delete_response_serialization,
        note::delete::DeleteResponse,
        note::delete::DeleteResponse {}
    );

    roundtrip_eq!(
        test_offer_application_form_get_response_serialization,
        offer_application_form::get::GetResponse,
        offer_application_form::get::GetResponse {
            offer_apply_form: Some(offer_application_form::get::OfferApplyForm {
                id: Some("form_1".to_string()),
                name: None,
                schema: None,
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_application_form_list_response_serialization,
        offer_application_form::list::ListResponse,
        offer_application_form::list::ListResponse {
            has_more: Some(false),
            page_token: None,
            items: vec![],
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_schema_get_response_serialization,
        offer_schema::get::GetResponse,
        offer_schema::get::GetResponse {
            id: Some("schema_1".to_string()),
            scenario: Some(1),
            version: Some(121),
            object_list: vec![],
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_advertisement_publish_response_serialization,
        advertisement::publish::PublishResponse,
        advertisement::publish::PublishResponse {
            data: json!({"publish_id":"pub_001"})
        }
    );
    roundtrip_eq!(
        test_attachment_create_response_serialization,
        attachment::create::CreateResponse,
        attachment::create::CreateResponse {
            id: Some("att_001".to_string()),
            name: Some("resume.pdf".to_string()),
            url: Some("https://example.com/file".to_string()),
        }
    );
    roundtrip_eq!(
        test_attachment_get_response_serialization,
        attachment::get::GetResponse,
        attachment::get::GetResponse {
            attachment: Some(openlark_hr::hire::hire::common_models::HireAttachment {
                id: Some("att_001".to_string()),
                name: Some("resume.pdf".to_string()),
                ..Default::default()
            })
        }
    );
    roundtrip_eq!(
        test_attachment_preview_response_serialization,
        attachment::preview::PreviewResponse,
        attachment::preview::PreviewResponse {
            url: Some("https://example.com/file".to_string()),
        }
    );
    roundtrip_eq!(
        test_referral_search_response_serialization,
        referral::search::SearchResponse,
        referral::search::SearchResponse {
            items: vec![],
            extra: Default::default(),
        }
    );
}
