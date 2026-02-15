//! Document AI V1 模块

pub mod bank_card;
pub mod business_card;
pub mod business_license;
pub mod chinese_passport;
pub mod contract;
pub mod driving_license;
pub mod food_manage_license;
pub mod food_produce_license;
pub mod health_certificate;
pub mod hkm_mainland_travel_permit;
pub mod id_card;
pub mod resume;
pub mod taxi_invoice;
pub mod train_invoice;
pub mod tw_mainland_travel_permit;
pub mod vat_invoice;
pub mod vehicle_invoice;
pub mod vehicle_license;

use openlark_core::config::Config;
use std::sync::Arc;

/// Document AI V1 API
#[derive(Clone)]
pub struct DocumentAiV1 {
    config: Arc<Config>,
}

impl DocumentAiV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
