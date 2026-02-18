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
            .and(body_json(json!({"name":"张三","email":"zhangsan@example.com"})))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"success","data":{"talent_id":"talent_001"}})),
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
            data: json!({"records":[]})
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
            data: json!({"items":[]})
        }
    );

    roundtrip_eq!(
        test_offer_create_response_serialization,
        offer::create::CreateResponse,
        offer::create::CreateResponse {
            data: json!({"offer_id":"of_001"})
        }
    );
    roundtrip_eq!(
        test_offer_get_response_serialization,
        offer::get::GetResponse,
        offer::get::GetResponse {
            data: json!({"offer_id":"of_001","status":"pending"})
        }
    );
    roundtrip_eq!(
        test_offer_update_response_serialization,
        offer::update::UpdateResponse,
        offer::update::UpdateResponse {
            data: json!({"result":true})
        }
    );
    roundtrip_eq!(
        test_offer_list_response_serialization,
        offer::list::ListResponse,
        offer::list::ListResponse {
            data: json!({"items":[]})
        }
    );
    roundtrip_eq!(
        test_offer_status_response_serialization,
        offer::offer_status::OfferStatusResponse,
        offer::offer_status::OfferStatusResponse {
            data: json!({"status":"accepted"})
        }
    );

    roundtrip_eq!(
        test_application_create_response_serialization,
        application::create::CreateResponse,
        application::create::CreateResponse {
            data: json!({"application_id":"ap_001"})
        }
    );
    roundtrip_eq!(
        test_application_get_response_serialization,
        application::get::GetResponse,
        application::get::GetResponse {
            data: json!({"application_id":"ap_001"})
        }
    );
    roundtrip_eq!(
        test_application_get_detail_response_serialization,
        application::get_detail::GetDetailResponse,
        application::get_detail::GetDetailResponse {
            data: json!({"application_id":"ap_001","detail":"x"})
        }
    );
    roundtrip_eq!(
        test_application_list_response_serialization,
        application::list::ListResponse,
        application::list::ListResponse {
            data: json!({"items":[]})
        }
    );

    roundtrip_eq!(
        test_job_combined_create_response_serialization,
        job::combined_create::CombinedCreateResponse,
        job::combined_create::CombinedCreateResponse {
            data: json!({"job_id":"job_001"})
        }
    );
    roundtrip_eq!(
        test_job_get_response_serialization,
        job::get::GetResponse,
        job::get::GetResponse {
            data: json!({"job_id":"job_001"})
        }
    );
    roundtrip_eq!(
        test_job_combined_update_response_serialization,
        job::combined_update::CombinedUpdateResponse,
        job::combined_update::CombinedUpdateResponse {
            data: json!({"result":true})
        }
    );
    roundtrip_eq!(
        test_job_list_response_serialization,
        job::list::ListResponse,
        job::list::ListResponse {
            data: json!({"items":[]})
        }
    );
    roundtrip_eq!(
        test_job_open_response_serialization,
        job::open::OpenResponse,
        job::open::OpenResponse {
            data: json!({"result":true})
        }
    );

    roundtrip_eq!(
        test_evaluation_list_response_serialization,
        evaluation::list::ListResponse,
        evaluation::list::ListResponse {
            data: json!({"items":[]})
        }
    );
    roundtrip_eq!(
        test_evaluation_task_list_response_serialization,
        evaluation_task::list::ListResponse,
        evaluation_task::list::ListResponse {
            data: json!({"items":[]})
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
        test_advertisement_publish_response_serialization,
        advertisement::publish::PublishResponse,
        advertisement::publish::PublishResponse {
            data: json!({"publish_id":"pub_001"})
        }
    );
    roundtrip_eq!(
        test_attachment_get_response_serialization,
        attachment::get::GetResponse,
        attachment::get::GetResponse {
            data: json!({"attachment_id":"att_001"})
        }
    );
    roundtrip_eq!(
        test_referral_search_response_serialization,
        referral::search::SearchResponse,
        referral::search::SearchResponse {
            data: json!({"items":[]})
        }
    );
}
