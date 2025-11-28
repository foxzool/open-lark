pub mod v1;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct Attendance {
    service: Arc<HrService>,
}

impl Attendance {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::AttendanceV1 {
        v1::AttendanceV1::new(self.service.clone())
    }
}