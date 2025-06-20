use std::{env, sync::Arc};

use dotenvy::dotenv;
use log::info;
use tokio::sync::Mutex;

use open_lark::{
    client::{ws_client::LarkWsClient, LarkClient},
    event::dispatcher::EventDispatcherHandler,
    service::im::v1::{
        message::{CreateMessageRequest, CreateMessageRequestBody},
        p2_im_message_read_v1::P2ImMessageReadV1,
        p2_im_message_receive_v1::P2ImMessageReceiveV1,
    },
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let client = LarkClient::builder(&app_id, &app_secret).build();
    let client = Arc::new(Mutex::new(client));
    let rt = tokio::runtime::Runtime::new().unwrap();
    let event_handler = EventDispatcherHandler::builder()
        .register_p2_im_message_receive_v1(move |data: P2ImMessageReceiveV1| {
            let client = client.clone();
            rt.spawn(async move {
                handle_p2_im_message_receive_v1(client, data).await;
            });
        })
        .register_p2_im_message_read_v1(handle_p2_im_message_read_v1)
        .build();

    LarkWsClient::open(&app_id, &app_secret, event_handler)
        .await
        .unwrap();
}


async fn handle_p2_im_message_receive_v1(
    api_client: Arc<Mutex<LarkClient>>,
    data: P2ImMessageReceiveV1,
) {
    info!("p2.im.message.receive_v1: {:?}", data);
    let chat_id = data.event.message.chat_id.clone();
    let req = CreateMessageRequest::builder()
        .receive_id_type("chat_id")
        .request_body(
            CreateMessageRequestBody::builder()
                .receive_id(chat_id)
                .msg_type("text")
                .content("{\"text\":\"test content\"}")
                .build(),
        )
        .build();

    // 发起请求
    match api_client
        .lock()
        .await
        .im
        .v1
        .message
        .create(req, None)
        .await
    {
        Ok(response) => info!("Message sent successfully: {:?}", response),
        Err(e) => info!("Failed to send message: {:?}", e),
    }
}

fn handle_p2_im_message_read_v1(data: P2ImMessageReadV1) {
    info!("p2.im.message.message_read_v1: {:?}", data);
}
