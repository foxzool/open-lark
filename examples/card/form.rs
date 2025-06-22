use std::env;

use dotenvy::dotenv;
use serde_json::json;
use uuid::Uuid;

use open_lark::{
    card::{
        components::{
            containers::{
                column_set::{Column, ColumnSetContainer},
                form::FormContainer,
            },
            content_components::{
                plain_text::PlainText,
                rich_text::FeishuCardMarkdown,
                title::{FeishuCardTitle, Title},
            },
            interactive_components::{
                button::FeishuCardButton,
                input::{FeishuCardInput, InputConfirm},
                select_static::{SelectStatic, SelectStaticOption},
            },
            CardElement,
        },
        FeishuCard,
    },
    prelude::LarkClient,
    service::im::v1::message::{CreateMessageRequest, CreateMessageRequestBody, SendMessageTrait},
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();
    let uuid = Uuid::new_v4();

    let form = FormContainer::new().name("Form_lvxmxsxf").elements(vec![
        CardElement::ColumnSet(
            ColumnSetContainer::new()
                .flex_mode("stretch")
                .background_style("default")
                .columns(vec![
                    Column::new()
                        .width("weighted")
                        .weight(1)
                        .vertical_align("top")
                        .elements(vec![CardElement::Markdown(FeishuCardMarkdown::new(
                            "请选择：",
                        ))]),
                    Column::new()
                        .width("weighted")
                        .weight(1)
                        .vertical_align("top")
                        .elements(vec![CardElement::SelectStatic(
                            SelectStatic::new()
                                .name("Select_pj6kw7cxyl")
                                .placeholder(PlainText::text("这是一个选择菜单"))
                                .options(vec![
                                    SelectStaticOption::new("选项1", "1"),
                                    SelectStaticOption::new("选项2", "2"),
                                    SelectStaticOption::new("选项3", "3"),
                                    SelectStaticOption::new("选项4", "4"),
                                ]),
                        )]),
                ]),
        ),
        CardElement::InputForm(
            FeishuCardInput::new()
                .name("Input_fhaty9jktke")
                .placeholder(PlainText::text("请输入"))
                .max_length(5)
                .label(PlainText::text("请输入文本："))
                .label_position("left")
                .value(json!({"k":"v"})),
        ),
        CardElement::Button(
            FeishuCardButton::new()
                .action_type("form_submit")
                .name("Button_e4d9u982x5k")
                .r#type("primary")
                .text(PlainText::text("提交").tag("lark_md"))
                .confirm(InputConfirm::new("title", "确认提交吗")),
        ),
    ]);
    // 飞书卡片
    let feishu_card = FeishuCard::new()
        .header(
            "zh_cn",
            FeishuCardTitle::new()
                .title(Title::new("选择菜单"))
                .template("green"),
        )
        .expect("Failed to set header")
        .elements("zh_cn", vec![CardElement::FormSet(form)])
        .expect("Failed to set elements");

    let req = CreateMessageRequest::builder()
        .receive_id_type("chat_id")
        .request_body(
            CreateMessageRequestBody::builder()
                .receive_id("oc_84d53efe245072c16ba4b4ff597f52f3")
                .msg_type(feishu_card.msg_type())
                .content(feishu_card.content())
                .uuid(uuid)
                .build(),
        )
        .build();

    // 发起请求
    let resp = client.im.v1.message.create(req, None).await.unwrap();
    println!("response: {:?}", resp);
}
