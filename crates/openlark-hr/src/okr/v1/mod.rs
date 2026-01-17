pub mod image;
pub mod okr;
pub mod period;
pub mod period_rule;
pub mod progress_record;
pub mod review;
pub mod user;

use crate::service::HrService;
use std::sync::Arc;

#[derive(Clone)]
pub struct OkrV1 {
    service: Arc<HrService>,
}

impl OkrV1 {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    pub fn image(&self) -> image::Image {
        image::Image::new(self.service.clone())
    }

    pub fn okr(&self) -> okr::Okr {
        okr::Okr::new(self.service.clone())
    }

    pub fn period(&self) -> period::Period {
        period::Period::new(self.service.clone())
    }

    pub fn period_rule(&self) -> period_rule::PeriodRule {
        period_rule::PeriodRule::new(self.service.clone())
    }

    pub fn progress_record(&self) -> progress_record::ProgressRecord {
        progress_record::ProgressRecord::new(self.service.clone())
    }

    pub fn review(&self) -> review::Review {
        review::Review::new(self.service.clone())
    }

    pub fn user_okr(&self) -> user::okr::UserOkr {
        user::okr::UserOkr::new(self.service.clone())
    }
}
