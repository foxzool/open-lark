pub mod review_data;
pub mod semester;
pub mod stage_task;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct PerformanceV1 {
    service: Arc<HrService>,
}

impl PerformanceV1 {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn review_data(&self) -> review_data::ReviewData {
        review_data::ReviewData::new(self.service.clone())
    }

    pub fn semester(&self) -> semester::Semester {
        semester::Semester::new(self.service.clone())
    }

    pub fn stage_task(&self) -> stage_task::StageTask {
        stage_task::StageTask::new(self.service.clone())
    }
}