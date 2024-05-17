use std::env;

use dotenvy::dotenv;
use serde_json::json;
use uuid::Uuid;

use open_lark::client::LarkClientBuilder;
use open_lark::feishu_card::card_components::containers::column_set::{
    Column, ColumnAction, ColumnSetContainer,
};
use open_lark::feishu_card::card_components::containers::form::FormContainer;
use open_lark::feishu_card::card_components::content_components::divider::FeishuCardDivider;
use open_lark::feishu_card::card_components::content_components::image::FeishuCardImage;
use open_lark::feishu_card::card_components::content_components::plain_text::PlainText;
use open_lark::feishu_card::card_components::content_components::rich_text::FeishuCardMarkdown;
use open_lark::feishu_card::card_components::content_components::title::{FeishuCardTitle, Title};
use open_lark::feishu_card::card_components::CardElement;
use open_lark::feishu_card::card_components::interactive_components::button::FeishuCardButton;
use open_lark::feishu_card::card_components::interactive_components::input::{
    FeishuCardInput, InputConfirm,
};
use open_lark::feishu_card::card_components::interactive_components::select_static::{
    SelectStatic, SelectStaticOption,
};
use open_lark::feishu_card::FeishuCard;
use open_lark::feishu_card::href::FeishuCardHrefVal;
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
                        .elements(vec![CardElement::Markdown(
                            FeishuCardMarkdown::new().content("请选择："),
                        )]),
                    Column::new()
                        .width("weighted")
                        .weight(1)
                        .vertical_align("top")
                        .elements(vec![CardElement::SelectStatic(
                            SelectStatic::new()
                                .name("Select_pj6kw7cxyl")
                                .placeholder(PlainText::new("这是一个选择菜单"))
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
                .placeholder(PlainText::new("请输入"))
                .max_length(5)
                .label(PlainText::new("请输入文本："))
                .label_position("left")
                .value(json!({"k":"v"})),
        ),
        CardElement::Button(
            FeishuCardButton::new()
                .action_type("form_submit")
                .name("Button_e4d9u982x5k")
                .r#type("primary")
                .text(PlainText::new("提交").tag("lark_md"))
                .confirm(InputConfirm::new("title", "确认提交吗")),
        ),
    ]);
    // 飞书卡片
    let feishu_card = FeishuCard::new()
        .header(
            "zh_cn",
            FeishuCardTitle::new()
                .title(Title::new().content("选择菜单"))
                .template("green"),
        )
        .elements("zh_cn", vec![CardElement::FormSet(form)]);

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
