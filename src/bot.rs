use crate::message::{ LarkMessage};

pub trait LarkBot {
    /// 发送原始消息
    fn send_raw_message(&self, body: impl serde::Serialize);

    /// 发送消息结构体
    fn send_message(&self, message: &LarkMessage);
}

pub mod custom_bot;
