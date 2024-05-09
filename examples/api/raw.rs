use std::collections::HashSet;
use std::env;

use dotenvy::dotenv;
use reqwest::Method;
use serde_json::json;

use open_lark::client::LarkClient;
use open_lark::core::enum_type::AccessTokenType;
use open_lark::core::model::BaseRequest;


fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let client = LarkClient::builder()
        .app_id(app_id)
        .app_secret(app_secret)
        .build();

    let request = BaseRequest::builder()
        .http_method(Method::POST)
        .uri("/open-apis/contact/v3/users/batch_get_id")
        .token_types(HashSet::from([AccessTokenType::TENANT]))
        .queries(vec![("user_id_type".to_string(), "open_id".to_string())])
        .body(json!({"emails": ["xxxx@bytedance.com"], "mobiles": ["15000000000"]}))
        .build();



    let response = client.request(request , None).unwrap();

    println!("{:?}", response);
}
