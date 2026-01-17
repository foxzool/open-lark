pub mod whiteboard;

use std::sync::Arc;
use crate::service::WorkflowService;

#[derive(Clone)]
pub struct BoardV1 {
    service: Arc<WorkflowService>,
}

impl BoardV1 {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    pub fn whiteboard(&self) -> whiteboard::Whiteboard {
        whiteboard::Whiteboard::new(self.service.clone())
    }

    pub fn whiteboard_node(&self) -> whiteboard::node::WhiteboardNode {
        whiteboard::node::WhiteboardNode::new(self.service.clone())
    }
}