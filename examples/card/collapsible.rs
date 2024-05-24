use std::env;

use dotenvy::dotenv;
use uuid::Uuid;

use open_lark::{
    card::{
        components::{
            CardElement,
            containers::collapsible_panel::{CollapsibleHeader, CollapsiblePanel},
            content_components::{
                plain_text::PlainText,
                rich_text::FeishuCardMarkdown,
                title::{FeishuCardTitle, Title},
            },
        },
        FeishuCard,
        icon::FeishuCardTextIcon,
    },
    client::LarkClientBuilder,
    service::im::v1::message::{CreateMessageRequest, CreateMessageRequestBody, SendMessageTrait},
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    let uuid = Uuid::new_v4();

    let elements = vec![
        CardElement::Markdown(FeishuCardMarkdown::new(
            "下面是一个 默认折叠 的折叠面板组件",
        )),
        CardElement::CollapsePanelContainer(
            CollapsiblePanel::new()
                .expanded(false)
                .header(
                    CollapsibleHeader::new("折叠面板标题")
                        .vertical_align("center")
                        .icon(
                            FeishuCardTextIcon::new()
                                .token("down-small-ccm_outlined")
                                .size("16px 16px"),
                        )
                        .icon_position("right")
                        .icon_expanded_angle(-180),
                )
                .border("grey", "5px")
                .vertical_spacing("8px")
                .padding("8px 8px 8px 8px")
                .elements(vec![CardElement::Markdown(FeishuCardMarkdown::new(
                    "很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本",
                ))]),
        ),

        CardElement::Markdown(FeishuCardMarkdown::new(
            "下面是一个 标题带背景色 且 默认展开 的折叠面板组件",
        )),
        CardElement::CollapsePanelContainer(
            CollapsiblePanel::new()
                .expanded(true)
                .header(
                    CollapsibleHeader::default()
                        .title(PlainText::markdown("**<font color='white'>面板标题文本</font>**"))
                        .background_color("yellow")
                        .vertical_align("center")
                        .icon(
                            FeishuCardTextIcon::new()
                                .token("down-small-ccm_outlined")
                                .color("white")
                                .size("16px 16px"),
                        )
                        .icon_position("right")
                        .icon_expanded_angle(-180),
                )
                .border("grey", "5px")
                .vertical_spacing("8px")
                .padding("8px 8px 8px 8px")
                .elements(vec![CardElement::Markdown(FeishuCardMarkdown::new(
                    "很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本",
                ))]),
        ),

        CardElement::Markdown(FeishuCardMarkdown::new(
            "下面是一个无边框折叠面板组件",
        )),
        CardElement::CollapsePanelContainer(
            CollapsiblePanel::new()
                .expanded(true)
                .header(
                    CollapsibleHeader::default()
                        .title(PlainText::markdown("**<font color='white'>面板标题文本</font>**"))
                        .vertical_align("center")
                        .padding("4px 0px 4px 8px")
                        .icon(
                            FeishuCardTextIcon::new()
                                .token("down-small-ccm_outlined")
                                .size("16px 16px"),
                        )
                        .icon_position("follow_text")
                        .icon_expanded_angle(-180),
                )
                .vertical_spacing("8px")
                .padding("8px 8px 8px 8px")
                .elements(vec![CardElement::Markdown(FeishuCardMarkdown::new(
                    "很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本很长的文本",
                ))]),
        ),
    ];
    // 飞书卡片
    let feishu_card = FeishuCard::new()
        .header(
            "zh_cn",
            FeishuCardTitle::new()
                .title(Title::new("折叠面板展示"))
                .template("yellow"),
        )
        .elements("zh_cn", elements);

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
