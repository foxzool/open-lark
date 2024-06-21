use dotenvy::dotenv;
use open_lark::client::ws::LarkWsClient;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();

    let mut client = LarkWsClient::open(&app_id, &app_secret).await.unwrap();


    while let Some(ws_event) = client.event_rx.recv().await {
        println!("{:?}", ws_event)
    }

}
