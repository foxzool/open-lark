pub mod acct_item;
pub mod cost_allocation_detail;
pub mod cost_allocation_plan;
pub mod cost_allocation_report;
pub mod datasource;
pub mod datasource_record;
pub mod paygroup;
pub mod payment_activity;
pub mod payment_activity_detail;
pub mod payment_detail;

use crate::service::HrService;
use std::sync::Arc;

#[derive(Clone)]
pub struct PayrollV1 {
    service: Arc<HrService>,
}

impl PayrollV1 {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    pub fn acct_item(&self) -> acct_item::AcctItem {
        acct_item::AcctItem::new(self.service.clone())
    }

    pub fn cost_allocation_detail(&self) -> cost_allocation_detail::CostAllocationDetail {
        cost_allocation_detail::CostAllocationDetail::new(self.service.clone())
    }

    pub fn cost_allocation_plan(&self) -> cost_allocation_plan::CostAllocationPlan {
        cost_allocation_plan::CostAllocationPlan::new(self.service.clone())
    }

    pub fn cost_allocation_report(&self) -> cost_allocation_report::CostAllocationReport {
        cost_allocation_report::CostAllocationReport::new(self.service.clone())
    }

    pub fn datasource(&self) -> datasource::Datasource {
        datasource::Datasource::new(self.service.clone())
    }

    pub fn datasource_record(&self) -> datasource_record::DatasourceRecord {
        datasource_record::DatasourceRecord::new(self.service.clone())
    }

    pub fn paygroup(&self) -> paygroup::Paygroup {
        paygroup::Paygroup::new(self.service.clone())
    }

    pub fn payment_activity(&self) -> payment_activity::PaymentActivity {
        payment_activity::PaymentActivity::new(self.service.clone())
    }

    pub fn payment_activity_detail(&self) -> payment_activity_detail::PaymentActivityDetail {
        payment_activity_detail::PaymentActivityDetail::new(self.service.clone())
    }

    pub fn payment_detail(&self) -> payment_detail::PaymentDetail {
        payment_detail::PaymentDetail::new(self.service.clone())
    }
}
