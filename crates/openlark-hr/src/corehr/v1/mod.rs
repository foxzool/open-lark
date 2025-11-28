pub mod assigned_user;
pub mod authorization;
pub mod common_data;
pub mod company;
pub mod compensation_standard;
pub mod contract;
pub mod country_region;
pub mod currency;
pub mod custom_field;
pub mod department;
pub mod employee_type;
pub mod employment;
pub mod file;
pub mod job;
pub mod job_change;
pub mod job_data;
pub mod job_family;
pub mod job_level;
pub mod leave;
pub mod leave_granting_record;
pub mod location;
pub mod national_id_type;
pub mod offboarding;
pub mod person;
pub mod pre_hire;
pub mod process;
pub mod security_group;
pub mod subdivision;
pub mod subregion;
pub mod transfer_reason;
pub mod transfer_type;
pub mod working_hours_type;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct CorehrV1 {
    service: Arc<HrService>,
}

impl CorehrV1 {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn assigned_user(&self) -> assigned_user::AssignedUser {
        assigned_user::AssignedUser::new(self.service.clone())
    }

    pub fn authorization(&self) -> authorization::Authorization {
        authorization::Authorization::new(self.service.clone())
    }

    pub fn common_data_id(&self) -> common_data::id::CommonDataId {
        common_data::id::CommonDataId::new(self.service.clone())
    }

    pub fn common_data_meta_data(&self) -> common_data::meta_data::CommonDataMetaData {
        common_data::meta_data::CommonDataMetaData::new(self.service.clone())
    }

    pub fn company(&self) -> company::Company {
        company::Company::new(self.service.clone())
    }

    pub fn compensation_standard(&self) -> compensation_standard::CompensationStandard {
        compensation_standard::CompensationStandard::new(self.service.clone())
    }

    pub fn contract(&self) -> contract::Contract {
        contract::Contract::new(self.service.clone())
    }

    pub fn country_region(&self) -> country_region::CountryRegion {
        country_region::CountryRegion::new(self.service.clone())
    }

    pub fn currency(&self) -> currency::Currency {
        currency::Currency::new(self.service.clone())
    }

    pub fn custom_field(&self) -> custom_field::CustomField {
        custom_field::CustomField::new(self.service.clone())
    }

    pub fn department(&self) -> department::Department {
        department::Department::new(self.service.clone())
    }

    pub fn employee_type(&self) -> employee_type::EmployeeType {
        employee_type::EmployeeType::new(self.service.clone())
    }

    pub fn employment(&self) -> employment::Employment {
        employment::Employment::new(self.service.clone())
    }

    pub fn file(&self) -> file::File {
        file::File::new(self.service.clone())
    }

    pub fn job(&self) -> job::Job {
        job::Job::new(self.service.clone())
    }

    pub fn job_change(&self) -> job_change::JobChange {
        job_change::JobChange::new(self.service.clone())
    }

    pub fn job_data(&self) -> job_data::JobData {
        job_data::JobData::new(self.service.clone())
    }

    pub fn job_family(&self) -> job_family::JobFamily {
        job_family::JobFamily::new(self.service.clone())
    }

    pub fn job_level(&self) -> job_level::JobLevel {
        job_level::JobLevel::new(self.service.clone())
    }

    pub fn leave(&self) -> leave::Leave {
        leave::Leave::new(self.service.clone())
    }

    pub fn leave_granting_record(&self) -> leave_granting_record::LeaveGrantingRecord {
        leave_granting_record::LeaveGrantingRecord::new(self.service.clone())
    }

    pub fn location(&self) -> location::Location {
        location::Location::new(self.service.clone())
    }

    pub fn national_id_type(&self) -> national_id_type::NationalIdType {
        national_id_type::NationalIdType::new(self.service.clone())
    }

    pub fn offboarding(&self) -> offboarding::Offboarding {
        offboarding::Offboarding::new(self.service.clone())
    }

    pub fn person(&self) -> person::Person {
        person::Person::new(self.service.clone())
    }

    pub fn pre_hire(&self) -> pre_hire::PreHire {
        pre_hire::PreHire::new(self.service.clone())
    }

    pub fn process_form_variable_data(&self) -> process::form_variable_data::ProcessFormVariableData {
        process::form_variable_data::ProcessFormVariableData::new(self.service.clone())
    }

    pub fn security_group(&self) -> security_group::SecurityGroup {
        security_group::SecurityGroup::new(self.service.clone())
    }

    pub fn subdivision(&self) -> subdivision::Subdivision {
        subdivision::Subdivision::new(self.service.clone())
    }

    pub fn subregion(&self) -> subregion::Subregion {
        subregion::Subregion::new(self.service.clone())
    }

    pub fn transfer_reason(&self) -> transfer_reason::TransferReason {
        transfer_reason::TransferReason::new(self.service.clone())
    }

    pub fn transfer_type(&self) -> transfer_type::TransferType {
        transfer_type::TransferType::new(self.service.clone())
    }

    pub fn working_hours_type(&self) -> working_hours_type::WorkingHoursType {
        working_hours_type::WorkingHoursType::new(self.service.clone())
    }
}