use std::sync::Arc;

use crate::core::config::Config;

/// Message service
///
/// Provides core message functionality including creating, sending, and retrieving messages.
/// Supports multiple message types: text, post, image, file, audio, media, sticker,
/// interactive, share_chat, share_user.
#[derive(Debug, Clone)]
pub struct MessageService {
    /// Service configuration
    pub config: Arc<Config>,
}

impl MessageService {
    /// Create a new message service instance
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

// Import all functionality from the message module
pub use crate::service::im::v1::message::*;
