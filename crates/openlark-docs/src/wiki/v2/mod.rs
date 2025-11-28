pub mod space;
pub mod task;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct WikiV2 {
    service: Arc<DocsService>,
}

impl WikiV2 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn space(&self) -> space::Space {
        space::Space::new(self.service.clone())
    }

    pub fn space_member(&self) -> space::member::SpaceMember {
        space::member::SpaceMember::new(self.service.clone())
    }

    pub fn space_node(&self) -> space::node::SpaceNode {
        space::node::SpaceNode::new(self.service.clone())
    }

    pub fn space_setting(&self) -> space::setting::SpaceSetting {
        space::setting::SpaceSetting::new(self.service.clone())
    }

    pub fn task(&self) -> task::Task {
        task::Task::new(self.service.clone())
    }
}