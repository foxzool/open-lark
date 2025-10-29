//! 事件处理模块
//!
//! 提供飞书开放平台事件的接收、处理和分发功能。
//! 支持WebSocket长连接和HTTP回调两种事件接收方式。
//!
//! # 主要组件
//!
//! - **dispatcher**: 事件分发器，用于注册和处理各种类型的事件
//! - **context**: 事件上下文，提供事件处理时的环境信息
//!
//! # 支持的事件类型
//!
//! - 📨 消息接收事件
//! - 👀 消息已读事件
//! - 👥 群聊成员变更事件
//! - 📞 视频会议事件
//! - 🗓️ 日历事件
//! - 📋 审批事件
//! - 更多事件类型持续添加中
//!
//! # 快速开始
//!
//! ```no_run
//! use open_lark::event::dispatcher::EventDispatcherHandler;
//!
//! // 创建事件处理器
//! let handler = EventDispatcherHandler::builder()
//!     .register_p2_im_message_receive_v1(|event| {
//!         println!("收到消息: {:?}", event.event.message);
//!     })?
//!     .build();
//!
//! // 处理接收到的事件
//! // handler.handle_event(event_data).await?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

/// 事件处理器trait
///
/// 所有事件处理器都需要实现此trait，用于处理接收到的事件数据
pub trait EventHandler: Send + Sync {
    /// 处理事件数据
    ///
    /// # 参数
    ///
    /// * `payload` - 事件的原始字节数据
    ///
    /// # 返回值
    ///
    /// 返回处理结果的Ok(())或错误信息
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()>;
}

/// 事件上下文相关
pub mod context;
/// 事件分发器
pub mod dispatcher;
