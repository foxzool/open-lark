use serde::{Deserialize, Serialize },
use serde_json::json;
use std::collections::HashMap;

use crate::im::v1::message::SendMessageTrait;
/// 文本消息
///
/// 用于发送纯文本消息，支持@用户、换行等功能。
/// 是最常用的消息类型之一。
/// # 特殊功能
/// - 支持@用户：`<at user_id="xxx"></at>`
/// - 支持@所有人：`<at user_id="all">name="全体成员"</at>`
/// - 支持换行：使用`\n`或调用`line()`方法
pub struct MessageText {
    text: String,
}
impl SendMessageTrait for MessageText {
    fn msg_type(&self) -> String {
        "text".to_string()
    }
    fn content(&self) -> String {
        json!({ "text": self.text }).to_string()
impl MessageText {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
