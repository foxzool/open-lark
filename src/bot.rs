pub trait LarkBot {
    /// 发送原始消息
    fn send_raw_message(&self, body: impl serde::Serialize);
    /// 发送文本消息
    fn send_text_message(&self, content: &str);
}

pub mod custom_bot;
