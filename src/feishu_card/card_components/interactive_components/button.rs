use serde::{Deserialize, Serialize};

use crate::feishu_card::card_components::content_components::plain_text::PlainText;
use crate::feishu_card::card_components::interactive_components::input::InputConfirm;
use crate::feishu_card::icon::FeishuCardTextIcon;
use crate::feishu_card::interactions::Behaviors;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardButton {
    tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    width: Option<String>,
    text: Option<PlainText>,
    icon: Option<FeishuCardTextIcon>,
    hover_tips: Option<PlainText>,
    disabled: Option<bool>,
    disabled_tips: Option<PlainText>,
    confirm: Option<InputConfirm>,
    behaviors: Option<Behaviors>,
}
