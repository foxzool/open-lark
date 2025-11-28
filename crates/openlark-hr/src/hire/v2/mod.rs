pub mod interview_record;
pub mod talent;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct HireV2 {
    service: Arc<HrService>,
}

impl HireV2 {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn interview_record(&self) -> interview_record::InterviewRecord {
        interview_record::InterviewRecord::new(self.service.clone())
    }

    pub fn talent(&self) -> talent::Talent {
        talent::Talent::new(self.service.clone())
    }
}