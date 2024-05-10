use std::env;

use dotenvy::dotenv;

use open_lark::client::LarkClient;
use open_lark::service::im;
use open_lark::service::im::v1::message::{CreateMessageReqBody, CreateMessageReqBuilder};

fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::new(&app_id, &app_secret);
    let req = CreateMessageReqBuilder::new()
        .receive_id_type("open_id")
        .body(CreateMessageReqBody {
            receive_id: "ou_7d8a6e6df7621556ce0d21922b676706ccs".to_string(),
            msg_type: "text".to_string(),
            content: r#"{"text":"test content"}"#.to_string(),
            uuid: Some("uuid".to_string()),
        })
        .build();

    im::v1::message::create(&client, req)
}
