pub mod custom_attr;
pub mod department;
pub mod employee_type_enum;
pub mod functional_role;
pub mod group;
pub mod job_family;
pub mod job_level;
pub mod job_title;
pub mod scope;
pub mod unit;
pub mod user;
pub mod work_city;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct ContactV3 {
    service: Arc<CommunicationService>,
}

impl ContactV3 {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn custom_attr(&self) -> custom_attr::CustomAttr {
        custom_attr::CustomAttr::new(self.service.clone())
    }

    pub fn department(&self) -> department::Department {
        department::Department::new(self.service.clone())
    }

    pub fn employee_type_enum(&self) -> employee_type_enum::EmployeeTypeEnum {
        employee_type_enum::EmployeeTypeEnum::new(self.service.clone())
    }

    pub fn functional_role(&self) -> functional_role::FunctionalRole {
        functional_role::FunctionalRole::new(self.service.clone())
    }

    pub fn functional_role_member(&self) -> functional_role::member::FunctionalRoleMember {
        functional_role::member::FunctionalRoleMember::new(self.service.clone())
    }

    pub fn group(&self) -> group::Group {
        group::Group::new(self.service.clone())
    }

    pub fn group_member(&self) -> group::member::GroupMember {
        group::member::GroupMember::new(self.service.clone())
    }

    pub fn job_family(&self) -> job_family::JobFamily {
        job_family::JobFamily::new(self.service.clone())
    }

    pub fn job_level(&self) -> job_level::JobLevel {
        job_level::JobLevel::new(self.service.clone())
    }

    pub fn job_title(&self) -> job_title::JobTitle {
        job_title::JobTitle::new(self.service.clone())
    }

    pub fn scope(&self) -> scope::Scope {
        scope::Scope::new(self.service.clone())
    }

    pub fn unit(&self) -> unit::Unit {
        unit::Unit::new(self.service.clone())
    }

    pub fn user(&self) -> user::User {
        user::User::new(self.service.clone())
    }

    pub fn work_city(&self) -> work_city::WorkCity {
        work_city::WorkCity::new(self.service.clone())
    }
}