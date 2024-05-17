use std::env;

use dotenvy::dotenv;
use uuid::Uuid;

use open_lark::{
    client::LarkClientBuilder,
    feishu_card::{
        card_components::{
            CardElement,
            containers::column_set::{Column, ColumnAction, ColumnSetContainer},
            content_components::{
                divider::FeishuCardDivider,
                image::FeishuCardImage,
                plain_text::PlainText,
                rich_text::FeishuCardMarkdown,
                title::{FeishuCardTitle, Title},
            },
        },
        FeishuCard,
        href::FeishuCardHrefVal,
    },
    service::im::v1::message::{CreateMessageReqBody, CreateMessageReqBuilder, SendMessageTrait},
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
    let feishu_card = FeishuCard::new()
        .header(
            "zh_cn",
            FeishuCardTitle::new()
                .title(Title::new("🏨 酒店申请已通过，请选择房型"))
                .template("green"),
        )
        .elements(
            "zh_cn",
            vec![
                CardElement::Markdown(FeishuCardMarkdown::new(
                    "入住酒店：杭州xxxx酒店\n<font color='grey'>📍 浙江省杭州市西湖区</font>",
                )),
                CardElement::Divider(FeishuCardDivider::default()),
                CardElement::ColumnSet(
                    ColumnSetContainer::new()
                        .flex_mode("none")
                        .background_style("default")
                        .horizontal_spacing("default")
                        .action(
                            ColumnAction::new().multi_url(
                                FeishuCardHrefVal::new()
                                    .url("https://open.feishu.cn")
                                    .android_url("https://developer.android.com/")
                                    .ios_url("https://developer.apple.com/")
                                    .pc_url("https://www.windows.com"),
                            ),
                        )
                        .columns(vec![
                            Column::new()
                                .width("weighted")
                                .weight(1)
                                .vertical_align("center")
                                .elements(vec![CardElement::Image(
                                    FeishuCardImage::new()
                                        .img_key("img_v2_120b03c8-27e3-456f-89c0-90ede1aa59ag").scale_type("fit_horizontal").alt(PlainText::default()),
                                )]),
                            Column::new()
                                .width("weighted")
                                .weight(3)
                                .elements(vec![CardElement::Markdown(
                                    FeishuCardMarkdown::new("**高级双床房**\n<font color='grey'>双早|40-47㎡|有窗户|双床</font>\n<font color='red'>¥699</font> 起")
                                        .text_align("left"),
                                )]),
                        ]),
                ),
                CardElement::Divider(FeishuCardDivider::default()),
                CardElement::ColumnSet(
                    ColumnSetContainer::new()
                        .flex_mode("none")
                        .background_style("default")
                        .horizontal_spacing("default")
                        .action(
                            ColumnAction::new().multi_url(
                                FeishuCardHrefVal::new()
                                    .url("https://open.feishu.cn")
                                    .android_url("https://developer.android.com/")
                                    .ios_url("https://developer.apple.com/")
                                    .pc_url("https://www.windows.com"),
                            ),
                        )
                        .columns(vec![
                            Column::new()
                                .width("weighted")
                                .weight(1)
                                .vertical_align("center")
                                .elements(vec![CardElement::Image(
                                    FeishuCardImage::new()
                                        .img_key("img_v2_120b03c8-27e3-456f-89c0-90ede1aa59ag").scale_type("fit_horizontal").alt(PlainText::default()),
                                )]),
                            Column::new()
                                .width("weighted")
                                .weight(3)
                                .vertical_align("top")
                                .elements(vec![CardElement::Markdown(
                                    FeishuCardMarkdown::new("**精品大床房**\n<font color='grey'>双早|40-47㎡|有窗户|大床</font>\n<font color='red'>¥666</font> 起")
                                        .text_align("left"),
                                )]),
                        ]),
                ),
            ],
        );

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
