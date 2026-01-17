pub mod archive;
pub mod change_reason;
pub mod indicator;
pub mod item;
pub mod item_category;
pub mod lump_sum_payment;
pub mod plan;
pub mod recurring_payment;
pub mod social_archive;
pub mod social_archive_adjust_record;
pub mod social_insurance;
pub mod social_plan;

use crate::service::HrService;
use std::sync::Arc;

#[derive(Clone)]
pub struct CompensationV1 {
    service: Arc<HrService>,
}

impl CompensationV1 {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    pub fn archive(&self) -> archive::Archive {
        archive::Archive::new(self.service.clone())
    }

    pub fn change_reason(&self) -> change_reason::ChangeReason {
        change_reason::ChangeReason::new(self.service.clone())
    }

    pub fn indicator(&self) -> indicator::Indicator {
        indicator::Indicator::new(self.service.clone())
    }

    pub fn item(&self) -> item::Item {
        item::Item::new(self.service.clone())
    }

    pub fn item_category(&self) -> item_category::ItemCategory {
        item_category::ItemCategory::new(self.service.clone())
    }

    pub fn lump_sum_payment(&self) -> lump_sum_payment::LumpSumPayment {
        lump_sum_payment::LumpSumPayment::new(self.service.clone())
    }

    pub fn plan(&self) -> plan::Plan {
        plan::Plan::new(self.service.clone())
    }

    pub fn recurring_payment(&self) -> recurring_payment::RecurringPayment {
        recurring_payment::RecurringPayment::new(self.service.clone())
    }

    pub fn social_archive(&self) -> social_archive::SocialArchive {
        social_archive::SocialArchive::new(self.service.clone())
    }

    pub fn social_archive_adjust_record(
        &self,
    ) -> social_archive_adjust_record::SocialArchiveAdjustRecord {
        social_archive_adjust_record::SocialArchiveAdjustRecord::new(self.service.clone())
    }

    pub fn social_insurance(&self) -> social_insurance::SocialInsurance {
        social_insurance::SocialInsurance::new(self.service.clone())
    }

    pub fn social_plan(&self) -> social_plan::SocialPlan {
        social_plan::SocialPlan::new(self.service.clone())
    }
}
