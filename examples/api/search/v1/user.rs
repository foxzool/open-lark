use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    core::{api_resp::ApiResponse, req_option::RequestOption},
    service::search::v1::user::SearchUserRequest,
};

/// 搜索用户
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let user_access_token = env::var("USER_ACCESS_TOKEN").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = SearchUserRequest::builder().query("周亮").build();

    // 发起请求
    let resp = client
        .search
        .v1
        .user
        .search_user(
            req.clone(),
            Some(
                RequestOption::builder()
                    .user_access_token(user_access_token.clone())
                    .build(),
            ),
        )
        .unwrap();
    if let ApiResponse::Success { data, .. } = resp {
        println!("search: {:#?}", data);
    }

    // 使用迭代器
    client
        .search
        .v1
        .user
        .search_user_iter(
            req,
            Some(
                RequestOption::builder()
                    .user_access_token(user_access_token)
                    .build(),
            ),
        )
        .for_each(|users| {
            for user in users {
                println!("user {:?}", user);
            }
        })
}
