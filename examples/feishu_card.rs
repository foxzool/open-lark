use std::env;

use dotenvy::dotenv;
use uuid::Uuid;

use open_lark::client::LarkClientBuilder;
use open_lark::feishu_card::{FeishuCardBuilder, FeishuCardElement};
use open_lark::feishu_card::card_components::content_components::rich_text::FeishuCardMarkdown;
use open_lark::feishu_card::card_components::content_components::title::{
    FeishuCardTitleBuilder, TitleBuilder,
};
use open_lark::service::im::v1::message::{
    CreateMessageReqBody, CreateMessageReqBuilder, SendMessageTrait,
};

fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    let uuid = Uuid::new_v4();
    // 飞书卡片
    let feishu_card =FeishuCardBuilder::new()
        .push_header(FeishuCardTitleBuilder::new().title(TitleBuilder::new().content("这里是卡片标题").build()).template("blue").build())
        // .push_element(FeishuCardElement::Hr)
        .push_element(FeishuCardElement::Markdown(
            FeishuCardMarkdown::new()
                .content("这里是卡片文本，支持使用markdown标签设置文本格式。例如：\n*斜体* 、**粗体**、~~删除线~~、[文字链接](https://www.feishu.cn)、<at id=all></at>、<font color='red'> 彩色文本 </font>")
                ,
        ))
        .build();

    let req = CreateMessageReqBuilder::new()
        .receive_id_type("chat_id")
        .body(CreateMessageReqBody {
            receive_id: "oc_84d53efe245072c16ba4b4ff597f52f3".to_string(),
            msg_type: feishu_card.msg_type(),
            content: feishu_card.content(),
            uuid: Some(uuid.to_string()),
        })
        .build();

    // 发起请求
    let resp = client.im.v1.message.create(req, None).unwrap();
    if resp.success() {
        // 业务处理
        println!("response: {:?}", resp.data);
    } else {
        println!("send message failed: {} ", resp.error_msg());
    }
}
