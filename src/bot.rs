pub trait LarkBot {
    /// 发送文本消息
    fn send_text_message(&self, content: &str);
}

pub mod custom_bot;
