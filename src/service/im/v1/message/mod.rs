//! Message service module for IM v1 API
//!
//! This module provides functionality for creating, sending, and managing messages
//! in the Lark/Feishu IM system.

pub mod builders;
pub mod content_types;
pub mod list;
pub mod send;
pub mod types;

// Re-export main types and services for easier imports
pub use builders::{
    CreateMessageRequest, CreateMessageRequestBody, CreateMessageRequestBodyBuilder,
    CreateMessageRequestBuilder, UpdateMessageRequest, UpdateMessageRequestBuilder,
};
pub use content_types::*;
pub use list::{ListMessageRequest, ListMessageRequestBuilder, ListMessageRespData};
pub use types::{CreateMessageResp, ListMessageIterator, Message, SendMessageTrait};

use crate::core::config::Config;

/// Message service
///
/// Provides core message functionality including creating, sending, and retrieving messages.
/// Supports multiple message types: text, post, image, file, audio, media, sticker,
/// interactive, share_chat, share_user.
#[derive(Debug, Clone)]
pub struct MessageService {
    /// Service configuration
    pub config: Config,
}

impl MessageService {
    /// Create a new message service instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
