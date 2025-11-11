//! open-lark-extensions module for OpenLark SDK
//!
//! This crate provides open-lark-extensions functionality for the OpenLark SDK.

#![allow(missing_docs)]

pub mod board;
pub mod event;

/// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use openlark_core::*;
    // Board module exports
    pub use crate::board::v1::{
        CreateWhiteboardNodeRequest, DownloadWhiteboardAsImageRequest, GetWhiteboardThemeRequest,
        ListWhiteboardNodesRequest,
    };
    pub use crate::board::BoardService;
    // Event module exports
    pub use crate::event::v1::GetOutboundIpRequest;
    pub use crate::event::EventService;
}
