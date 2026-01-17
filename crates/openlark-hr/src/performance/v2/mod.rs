pub mod activity;
pub mod additional_information;
pub mod additional_informations;
pub mod indicator;
pub mod metric_detail;
pub mod metric_field;
pub mod metric_lib;
pub mod metric_tag;
pub mod metric_template;
pub mod question;
pub mod review_data;
pub mod review_template;
pub mod reviewee;
pub mod user_group_user_rel;

use crate::service::HrService;
use std::sync::Arc;

#[derive(Clone)]
pub struct PerformanceV2 {
    service: Arc<HrService>,
}

impl PerformanceV2 {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    pub fn activity(&self) -> activity::Activity {
        activity::Activity::new(self.service.clone())
    }

    pub fn additional_information(&self) -> additional_information::AdditionalInformation {
        additional_information::AdditionalInformation::new(self.service.clone())
    }

    pub fn additional_informations_batch(
        &self,
    ) -> additional_informations::batch::AdditionalInformationsBatch {
        additional_informations::batch::AdditionalInformationsBatch::new(self.service.clone())
    }

    pub fn indicator(&self) -> indicator::Indicator {
        indicator::Indicator::new(self.service.clone())
    }

    pub fn metric_detail(&self) -> metric_detail::MetricDetail {
        metric_detail::MetricDetail::new(self.service.clone())
    }

    pub fn metric_field(&self) -> metric_field::MetricField {
        metric_field::MetricField::new(self.service.clone())
    }

    pub fn metric_lib(&self) -> metric_lib::MetricLib {
        metric_lib::MetricLib::new(self.service.clone())
    }

    pub fn metric_tag(&self) -> metric_tag::MetricTag {
        metric_tag::MetricTag::new(self.service.clone())
    }

    pub fn metric_template(&self) -> metric_template::MetricTemplate {
        metric_template::MetricTemplate::new(self.service.clone())
    }

    pub fn question(&self) -> question::Question {
        question::Question::new(self.service.clone())
    }

    pub fn review_data(&self) -> review_data::ReviewData {
        review_data::ReviewData::new(self.service.clone())
    }

    pub fn review_template(&self) -> review_template::ReviewTemplate {
        review_template::ReviewTemplate::new(self.service.clone())
    }

    pub fn reviewee(&self) -> reviewee::Reviewee {
        reviewee::Reviewee::new(self.service.clone())
    }

    pub fn user_group_user_rel(&self) -> user_group_user_rel::UserGroupUserRel {
        user_group_user_rel::UserGroupUserRel::new(self.service.clone())
    }
}
