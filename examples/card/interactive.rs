use std::env;

use dotenvy::dotenv;
use serde_json::json;
use uuid::Uuid;

use open_lark::{
    card::{
        components::{
            containers::{
                column_set::{Column, ColumnSetContainer},
                interactive::InteractiveContainer,
            },
            interactive_components::{
                button::FeishuCardButton,
                input::{FeishuCardInput, InputConfirm},
                select_static::{SelectStatic, SelectStaticOption},
            },
            CardElement,
        },
        icon::FeishuCardTextIcon,
        interactions::{Behaviors, CallbackBehavior},
        FeishuCard, FeishuCardConfig,
    },
    client::LarkClientBuilder,
    service::im::v1::message::{CreateMessageReqBody, CreateMessageReqBuilder, SendMessageTrait},
};
use open_lark::card::components::content_components::rich_text::FeishuCardMarkdown;
use open_lark::card::components::content_components::title::{FeishuCardTitle, FeishuCardUdIcon, Title};

fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    let uuid = Uuid::new_v4();

    let elements = vec![
        CardElement::Markdown(FeishuCardMarkdown::new(
            "在「内容创作」话题下，我可以帮助你进行产品方案、营销文案、工作报告等内容的创作。",
        )),
        CardElement::ColumnSet(
            ColumnSetContainer::new()
                .flex_mode("none")
                .background_style("default")
                .columns(vec![Column::new()
                    .width("weighted")
                    .weight(1)
                    .vertical_align("top")
                    .vertical_spacing("8px")
                    .elements(vec![
                        CardElement::Markdown(
                            FeishuCardMarkdown::new("<font color='grey'>你可以对我说：</font>")
                                .text_size("notation"),
                        ),
                        CardElement::InteractiveSet(
                            InteractiveContainer::new()
                                .width("fill")
                                .height("auto")
                                .background_style("default")
                                .has_border(true)
                                .border_color("grey")
                                .corner_radius("8px")
                                .padding("4px 12px 4px 12px")
                                .behaviors(vec![Behaviors::Callback(CallbackBehavior::new(
                                    json!({"key": "value"}),
                                ))])
                                .elements(vec![CardElement::Markdown(
                                    FeishuCardMarkdown::new("帮我生成一篇产品方案的框架").icon(
                                        FeishuCardTextIcon::new()
                                            .token("frame-selection_outlined")
                                            .color("orange"),
                                    ),
                                )]),
                        ),
                        CardElement::InteractiveSet(
                            InteractiveContainer::new()
                                .width("fill")
                                .height("auto")
                                .background_style("default")
                                .has_border(true)
                                .border_color("grey")
                                .corner_radius("8px")
                                .padding("4px 12px 4px 12px")
                                .behaviors(vec![Behaviors::Callback(CallbackBehavior::new(
                                    json!({"key": "value"}),
                                ))])
                                .elements(vec![CardElement::Markdown(
                                    FeishuCardMarkdown::new("帮我生成一篇产品文案").icon(
                                        FeishuCardTextIcon::new()
                                            .token("file-link-docx_outlined")
                                            .color("orange"),
                                    ),
                                )]),
                        ),
                        CardElement::InteractiveSet(
                            InteractiveContainer::new()
                                .width("fill")
                                .height("auto")
                                .background_style("default")
                                .has_border(true)
                                .border_color("grey")
                                .corner_radius("8px")
                                .padding("4px 12px 4px 12px")
                                .behaviors(vec![Behaviors::Callback(CallbackBehavior::new(
                                    json!({"key": "value"}),
                                ))])
                                .elements(vec![CardElement::Markdown(
                                    FeishuCardMarkdown::new("帮我写一篇周报").icon(
                                        FeishuCardTextIcon::new()
                                            .token("pa-calibration-report_outlined")
                                            .color("orange"),
                                    ),
                                )]),
                        ),
                    ])]),
        ),
        CardElement::ColumnSet(
            ColumnSetContainer::new()
                .flex_mode("none")
                .background_style("default")
                .columns(vec![Column::new()
                    .width("weighted")
                    .weight(1)
                    .vertical_align("top")
                    .vertical_spacing("8px")
                    .elements(vec![
                        CardElement::Markdown(
                            FeishuCardMarkdown::new("<font color='grey'>或者继续之前的话题</font>")
                                .text_size("notation"),
                        ),
                        CardElement::InteractiveSet(
                            InteractiveContainer::new()
                                .width("fill")
                                .height("auto")
                                .background_style("default")
                                .has_border(true)
                                .border_color("grey")
                                .corner_radius("8px")
                                .padding("4px 12px 4px 12px")
                                .behaviors(vec![Behaviors::Callback(CallbackBehavior::new(
                                    json!({"key": "value"}),
                                ))])
                                .disabled(false)
                                .elements(vec![CardElement::ColumnSet(
                                    ColumnSetContainer::new()
                                        .flex_mode("none")
                                        .background_style("default")
                                        .columns(vec![
                                            Column::new()
                                                .width("weighted")
                                                .weight(1)
                                                .vertical_align("center")
                                                .elements(vec![CardElement::Markdown(
                                                    FeishuCardMarkdown::new(
                                                        "内容创作:创作暑假营销活动文案",
                                                    )
                                                    .icon(
                                                        FeishuCardTextIcon::new()
                                                            .token("chat-history_outlined"),
                                                    ),
                                                )]),
                                            Column::new()
                                                .weight(1)
                                                .vertical_align("center")
                                                .elements(vec![CardElement::Markdown(
                                                    FeishuCardMarkdown::new("产品文案")
                                                        .text_size("notation"),
                                                )]),
                                        ]),
                                )]),
                        ),
                    ])]),
        ),
    ];
    // 飞书卡片
    let feishu_card = FeishuCard::new()
        .header(
            "zh_cn",
            FeishuCardTitle::new()
                .title(Title::new("交互容器示例（依赖端版本 7.4+)"))
                .ud_icon(FeishuCardUdIcon::default()),
        )
        .elements("zh_cn", elements);

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
