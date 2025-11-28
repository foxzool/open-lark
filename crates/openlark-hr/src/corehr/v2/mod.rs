pub mod approval_groups;
pub mod approver;
pub mod basic_info;
pub mod bp;
pub mod company;
pub mod contract;
pub mod cost_allocation;
pub mod cost_center;
pub mod custom_org;
pub mod default_cost_center;
pub mod department;
pub mod draft;
pub mod employee;
pub mod employees;
pub mod r#enum;
pub mod job;
pub mod job_change;
pub mod job_family;
pub mod job_grade;
pub mod job_level;
pub mod location;
pub mod offboarding;
pub mod pathway;
pub mod person;
pub mod position;
pub mod pre_hire;
pub mod probation;
pub mod process;
pub mod process_revoke;
pub mod process_withdraw;
pub mod report_detail_row;
pub mod workforce_plan;
pub mod workforce_plan_detail;
pub mod workforce_plan_detail_row;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct CorehrV2 {
    service: Arc<HrService>,
}

impl CorehrV2 {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn approval_groups(&self) -> approval_groups::ApprovalGroups {
        approval_groups::ApprovalGroups::new(self.service.clone())
    }

    pub fn approver(&self) -> approver::Approver {
        approver::Approver::new(self.service.clone())
    }

    pub fn basic_info_bank(&self) -> basic_info::bank::BasicInfoBank {
        basic_info::bank::BasicInfoBank::new(self.service.clone())
    }

    pub fn basic_info_bank_branch(&self) -> basic_info::bank_branch::BasicInfoBankBranch {
        basic_info::bank_branch::BasicInfoBankBranch::new(self.service.clone())
    }

    pub fn basic_info_city(&self) -> basic_info::city::BasicInfoCity {
        basic_info::city::BasicInfoCity::new(self.service.clone())
    }

    pub fn basic_info_country_region(&self) -> basic_info::country_region::BasicInfoCountryRegion {
        basic_info::country_region::BasicInfoCountryRegion::new(self.service.clone())
    }

    pub fn basic_info_country_region_subdivision(&self) -> basic_info::country_region_subdivision::BasicInfoCountryRegionSubdivision {
        basic_info::country_region_subdivision::BasicInfoCountryRegionSubdivision::new(self.service.clone())
    }

    pub fn basic_info_currency(&self) -> basic_info::currency::BasicInfoCurrency {
        basic_info::currency::BasicInfoCurrency::new(self.service.clone())
    }

    pub fn basic_info_district(&self) -> basic_info::district::BasicInfoDistrict {
        basic_info::district::BasicInfoDistrict::new(self.service.clone())
    }

    pub fn basic_info_language(&self) -> basic_info::language::BasicInfoLanguage {
        basic_info::language::BasicInfoLanguage::new(self.service.clone())
    }

    pub fn basic_info_nationality(&self) -> basic_info::nationality::BasicInfoNationality {
        basic_info::nationality::BasicInfoNationality::new(self.service.clone())
    }

    pub fn basic_info_time_zone(&self) -> basic_info::time_zone::BasicInfoTimeZone {
        basic_info::time_zone::BasicInfoTimeZone::new(self.service.clone())
    }

    pub fn bp(&self) -> bp::Bp {
        bp::Bp::new(self.service.clone())
    }

    pub fn company(&self) -> company::Company {
        company::Company::new(self.service.clone())
    }

    pub fn contract(&self) -> contract::Contract {
        contract::Contract::new(self.service.clone())
    }

    pub fn cost_allocation(&self) -> cost_allocation::CostAllocation {
        cost_allocation::CostAllocation::new(self.service.clone())
    }

    pub fn cost_center(&self) -> cost_center::CostCenter {
        cost_center::CostCenter::new(self.service.clone())
    }

    pub fn cost_center_version(&self) -> cost_center::version::CostCenterVersion {
        cost_center::version::CostCenterVersion::new(self.service.clone())
    }

    pub fn custom_org(&self) -> custom_org::CustomOrg {
        custom_org::CustomOrg::new(self.service.clone())
    }

    pub fn default_cost_center(&self) -> default_cost_center::DefaultCostCenter {
        default_cost_center::DefaultCostCenter::new(self.service.clone())
    }

    pub fn department(&self) -> department::Department {
        department::Department::new(self.service.clone())
    }

    pub fn draft(&self) -> draft::Draft {
        draft::Draft::new(self.service.clone())
    }

    pub fn employee(&self) -> employee::Employee {
        employee::Employee::new(self.service.clone())
    }

    pub fn employees_additional_job(&self) -> employees::additional_job::EmployeesAdditionalJob {
        employees::additional_job::EmployeesAdditionalJob::new(self.service.clone())
    }

    pub fn employees_bp(&self) -> employees::bp::EmployeesBp {
        employees::bp::EmployeesBp::new(self.service.clone())
    }

    pub fn employees_international_assignment(&self) -> employees::international_assignment::EmployeesInternationalAssignment {
        employees::international_assignment::EmployeesInternationalAssignment::new(self.service.clone())
    }

    pub fn employees_job_data(&self) -> employees::job_data::EmployeesJobData {
        employees::job_data::EmployeesJobData::new(self.service.clone())
    }

    pub fn r#enum(&self) -> r#enum::Enum {
        r#enum::Enum::new(self.service.clone())
    }

    pub fn job(&self) -> job::Job {
        job::Job::new(self.service.clone())
    }

    pub fn job_change(&self) -> job_change::JobChange {
        job_change::JobChange::new(self.service.clone())
    }

    pub fn job_family(&self) -> job_family::JobFamily {
        job_family::JobFamily::new(self.service.clone())
    }

    pub fn job_grade(&self) -> job_grade::JobGrade {
        job_grade::JobGrade::new(self.service.clone())
    }

    pub fn job_level(&self) -> job_level::JobLevel {
        job_level::JobLevel::new(self.service.clone())
    }

    pub fn location(&self) -> location::Location {
        location::Location::new(self.service.clone())
    }

    pub fn location_address(&self) -> location::address::LocationAddress {
        location::address::LocationAddress::new(self.service.clone())
    }

    pub fn offboarding(&self) -> offboarding::Offboarding {
        offboarding::Offboarding::new(self.service.clone())
    }

    pub fn pathway(&self) -> pathway::Pathway {
        pathway::Pathway::new(self.service.clone())
    }

    pub fn person(&self) -> person::Person {
        person::Person::new(self.service.clone())
    }

    pub fn position(&self) -> position::Position {
        position::Position::new(self.service.clone())
    }

    pub fn pre_hire(&self) -> pre_hire::PreHire {
        pre_hire::PreHire::new(self.service.clone())
    }

    pub fn probation(&self) -> probation::Probation {
        probation::Probation::new(self.service.clone())
    }

    pub fn probation_assessment(&self) -> probation::assessment::ProbationAssessment {
        probation::assessment::ProbationAssessment::new(self.service.clone())
    }

    pub fn process(&self) -> process::Process {
        process::Process::new(self.service.clone())
    }

    pub fn process_approver(&self) -> process::approver::ProcessApprover {
        process::approver::ProcessApprover::new(self.service.clone())
    }

    pub fn process_extra(&self) -> process::extra::ProcessExtra {
        process::extra::ProcessExtra::new(self.service.clone())
    }

    pub fn process_form_variable_data(&self) -> process::form_variable_data::ProcessFormVariableData {
        process::form_variable_data::ProcessFormVariableData::new(self.service.clone())
    }

    pub fn process_transfer(&self) -> process::transfer::ProcessTransfer {
        process::transfer::ProcessTransfer::new(self.service.clone())
    }

    pub fn process_revoke(&self) -> process_revoke::ProcessRevoke {
        process_revoke::ProcessRevoke::new(self.service.clone())
    }

    pub fn process_withdraw(&self) -> process_withdraw::ProcessWithdraw {
        process_withdraw::ProcessWithdraw::new(self.service.clone())
    }

    pub fn report_detail_row(&self) -> report_detail_row::ReportDetailRow {
        report_detail_row::ReportDetailRow::new(self.service.clone())
    }

    pub fn workforce_plan(&self) -> workforce_plan::WorkforcePlan {
        workforce_plan::WorkforcePlan::new(self.service.clone())
    }

    pub fn workforce_plan_detail(&self) -> workforce_plan_detail::WorkforcePlanDetail {
        workforce_plan_detail::WorkforcePlanDetail::new(self.service.clone())
    }

    pub fn workforce_plan_detail_row(&self) -> workforce_plan_detail_row::WorkforcePlanDetailRow {
        workforce_plan_detail_row::WorkforcePlanDetailRow::new(self.service.clone())
    }
}
