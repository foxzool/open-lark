use dotenvy::dotenv;
use open_lark::client::ws::LarkWsClient;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();

    let client = LarkWsClient::new(app_id, app_secret);
    client.start().await.unwrap();
}
