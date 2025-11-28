pub mod advertisement;
pub mod agency;
pub mod application;
pub mod attachment;
pub mod background_check_order;
pub mod diversity_inclusion;
pub mod eco_account_custom_field;
pub mod eco_background_check;
pub mod eco_background_check_custom_field;
pub mod eco_background_check_package;
pub mod eco_exam;
pub mod eco_exam_paper;
pub mod ehr_import_task;
pub mod employee;
pub mod evaluation;
pub mod evaluation_task;
pub mod exam;
pub mod exam_marking_task;
pub mod external_application;
pub mod external_background_check;
pub mod external_interview;
pub mod external_interview_assessment;
pub mod external_offer;
pub mod external_referral_reward;
pub mod interview;
pub mod interview_feedback_form;
pub mod interview_record;
pub mod interview_registration_schema;
pub mod interview_round_type;
pub mod interview_task;
pub mod interviewer;
pub mod job;
pub mod job_function;
pub mod job_process;
pub mod job_publish_record;
pub mod job_requirement;
pub mod job_requirement_schema;
pub mod job_schema;
pub mod job_type;
pub mod location;
pub mod minutes;
pub mod note;
pub mod offer;
pub mod offer_application_form;
pub mod offer_approval_template;
pub mod offer_custom_field;
pub mod offer_schema;
pub mod portal_apply_schema;
pub mod questionnaire;
pub mod referral;
pub mod referral_account;
pub mod referral_website;
pub mod registration_schema;
pub mod resume_source;
pub mod role;
pub mod subject;
pub mod talent;
pub mod talent_blocklist;
pub mod talent_folder;
pub mod talent_object;
pub mod talent_operation_log;
pub mod talent_pool;
pub mod talent_tag;
pub mod termination_reason;
pub mod test;
pub mod todo;
pub mod tripartite_agreement;
pub mod user_role;
pub mod website;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct HireV1 {
    service: Arc<HrService>,
}

impl HireV1 {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn advertisement(&self) -> advertisement::Advertisement {
        advertisement::Advertisement::new(self.service.clone())
    }

    pub fn agency(&self) -> agency::Agency {
        agency::Agency::new(self.service.clone())
    }

    pub fn application(&self) -> application::Application {
        application::Application::new(self.service.clone())
    }

    pub fn application_interview(&self) -> application::interview::ApplicationInterview {
        application::interview::ApplicationInterview::new(self.service.clone())
    }

    pub fn attachment(&self) -> attachment::Attachment {
        attachment::Attachment::new(self.service.clone())
    }

    pub fn background_check_order(&self) -> background_check_order::BackgroundCheckOrder {
        background_check_order::BackgroundCheckOrder::new(self.service.clone())
    }

    pub fn diversity_inclusion(&self) -> diversity_inclusion::DiversityInclusion {
        diversity_inclusion::DiversityInclusion::new(self.service.clone())
    }

    pub fn eco_account_custom_field(&self) -> eco_account_custom_field::EcoAccountCustomField {
        eco_account_custom_field::EcoAccountCustomField::new(self.service.clone())
    }

    pub fn eco_background_check(&self) -> eco_background_check::EcoBackgroundCheck {
        eco_background_check::EcoBackgroundCheck::new(self.service.clone())
    }

    pub fn eco_background_check_custom_field(&self) -> eco_background_check_custom_field::EcoBackgroundCheckCustomField {
        eco_background_check_custom_field::EcoBackgroundCheckCustomField::new(self.service.clone())
    }

    pub fn eco_background_check_package(&self) -> eco_background_check_package::EcoBackgroundCheckPackage {
        eco_background_check_package::EcoBackgroundCheckPackage::new(self.service.clone())
    }

    pub fn eco_exam(&self) -> eco_exam::EcoExam {
        eco_exam::EcoExam::new(self.service.clone())
    }

    pub fn eco_exam_paper(&self) -> eco_exam_paper::EcoExamPaper {
        eco_exam_paper::EcoExamPaper::new(self.service.clone())
    }

    pub fn ehr_import_task(&self) -> ehr_import_task::EhrImportTask {
        ehr_import_task::EhrImportTask::new(self.service.clone())
    }

    pub fn employee(&self) -> employee::Employee {
        employee::Employee::new(self.service.clone())
    }

    pub fn evaluation(&self) -> evaluation::Evaluation {
        evaluation::Evaluation::new(self.service.clone())
    }

    pub fn evaluation_task(&self) -> evaluation_task::EvaluationTask {
        evaluation_task::EvaluationTask::new(self.service.clone())
    }

    pub fn exam(&self) -> exam::Exam {
        exam::Exam::new(self.service.clone())
    }

    pub fn exam_marking_task(&self) -> exam_marking_task::ExamMarkingTask {
        exam_marking_task::ExamMarkingTask::new(self.service.clone())
    }

    pub fn external_application(&self) -> external_application::ExternalApplication {
        external_application::ExternalApplication::new(self.service.clone())
    }

    pub fn external_background_check(&self) -> external_background_check::ExternalBackgroundCheck {
        external_background_check::ExternalBackgroundCheck::new(self.service.clone())
    }

    pub fn external_interview(&self) -> external_interview::ExternalInterview {
        external_interview::ExternalInterview::new(self.service.clone())
    }

    pub fn external_interview_assessment(&self) -> external_interview_assessment::ExternalInterviewAssessment {
        external_interview_assessment::ExternalInterviewAssessment::new(self.service.clone())
    }

    pub fn external_offer(&self) -> external_offer::ExternalOffer {
        external_offer::ExternalOffer::new(self.service.clone())
    }

    pub fn external_referral_reward(&self) -> external_referral_reward::ExternalReferralReward {
        external_referral_reward::ExternalReferralReward::new(self.service.clone())
    }

    pub fn interview(&self) -> interview::Interview {
        interview::Interview::new(self.service.clone())
    }

    pub fn interview_feedback_form(&self) -> interview_feedback_form::InterviewFeedbackForm {
        interview_feedback_form::InterviewFeedbackForm::new(self.service.clone())
    }

    pub fn interview_record(&self) -> interview_record::InterviewRecord {
        interview_record::InterviewRecord::new(self.service.clone())
    }

    pub fn interview_record_attachment(&self) -> interview_record::attachment::InterviewRecordAttachment {
        interview_record::attachment::InterviewRecordAttachment::new(self.service.clone())
    }

    pub fn interview_registration_schema(&self) -> interview_registration_schema::InterviewRegistrationSchema {
        interview_registration_schema::InterviewRegistrationSchema::new(self.service.clone())
    }

    pub fn interview_round_type(&self) -> interview_round_type::InterviewRoundType {
        interview_round_type::InterviewRoundType::new(self.service.clone())
    }

    pub fn interview_task(&self) -> interview_task::InterviewTask {
        interview_task::InterviewTask::new(self.service.clone())
    }

    pub fn interviewer(&self) -> interviewer::Interviewer {
        interviewer::Interviewer::new(self.service.clone())
    }

    pub fn job(&self) -> job::Job {
        job::Job::new(self.service.clone())
    }

    pub fn job_manager(&self) -> job::manager::JobManager {
        job::manager::JobManager::new(self.service.clone())
    }

    pub fn job_function(&self) -> job_function::JobFunction {
        job_function::JobFunction::new(self.service.clone())
    }

    pub fn job_process(&self) -> job_process::JobProcess {
        job_process::JobProcess::new(self.service.clone())
    }

    pub fn job_publish_record(&self) -> job_publish_record::JobPublishRecord {
        job_publish_record::JobPublishRecord::new(self.service.clone())
    }

    pub fn job_requirement(&self) -> job_requirement::JobRequirement {
        job_requirement::JobRequirement::new(self.service.clone())
    }

    pub fn job_requirement_schema(&self) -> job_requirement_schema::JobRequirementSchema {
        job_requirement_schema::JobRequirementSchema::new(self.service.clone())
    }

    pub fn job_schema(&self) -> job_schema::JobSchema {
        job_schema::JobSchema::new(self.service.clone())
    }

    pub fn job_type(&self) -> job_type::JobType {
        job_type::JobType::new(self.service.clone())
    }

    pub fn location(&self) -> location::Location {
        location::Location::new(self.service.clone())
    }

    pub fn minutes(&self) -> minutes::Minutes {
        minutes::Minutes::new(self.service.clone())
    }

    pub fn note(&self) -> note::Note {
        note::Note::new(self.service.clone())
    }

    pub fn offer(&self) -> offer::Offer {
        offer::Offer::new(self.service.clone())
    }

    pub fn offer_application_form(&self) -> offer_application_form::OfferApplicationForm {
        offer_application_form::OfferApplicationForm::new(self.service.clone())
    }

    pub fn offer_approval_template(&self) -> offer_approval_template::OfferApprovalTemplate {
        offer_approval_template::OfferApprovalTemplate::new(self.service.clone())
    }

    pub fn offer_custom_field(&self) -> offer_custom_field::OfferCustomField {
        offer_custom_field::OfferCustomField::new(self.service.clone())
    }

    pub fn offer_schema(&self) -> offer_schema::OfferSchema {
        offer_schema::OfferSchema::new(self.service.clone())
    }

    pub fn portal_apply_schema(&self) -> portal_apply_schema::PortalApplySchema {
        portal_apply_schema::PortalApplySchema::new(self.service.clone())
    }

    pub fn questionnaire(&self) -> questionnaire::Questionnaire {
        questionnaire::Questionnaire::new(self.service.clone())
    }

    pub fn referral(&self) -> referral::Referral {
        referral::Referral::new(self.service.clone())
    }

    pub fn referral_account(&self) -> referral_account::ReferralAccount {
        referral_account::ReferralAccount::new(self.service.clone())
    }

    pub fn referral_website_job_post(&self) -> referral_website::job_post::ReferralWebsiteJobPost {
        referral_website::job_post::ReferralWebsiteJobPost::new(self.service.clone())
    }

    pub fn registration_schema(&self) -> registration_schema::RegistrationSchema {
        registration_schema::RegistrationSchema::new(self.service.clone())
    }

    pub fn resume_source(&self) -> resume_source::ResumeSource {
        resume_source::ResumeSource::new(self.service.clone())
    }

    pub fn role(&self) -> role::Role {
        role::Role::new(self.service.clone())
    }

    pub fn subject(&self) -> subject::Subject {
        subject::Subject::new(self.service.clone())
    }

    pub fn talent(&self) -> talent::Talent {
        talent::Talent::new(self.service.clone())
    }

    pub fn talent_external_info(&self) -> talent::external_info::TalentExternalInfo {
        talent::external_info::TalentExternalInfo::new(self.service.clone())
    }

    pub fn talent_blocklist(&self) -> talent_blocklist::TalentBlocklist {
        talent_blocklist::TalentBlocklist::new(self.service.clone())
    }

    pub fn talent_folder(&self) -> talent_folder::TalentFolder {
        talent_folder::TalentFolder::new(self.service.clone())
    }

    pub fn talent_object(&self) -> talent_object::TalentObject {
        talent_object::TalentObject::new(self.service.clone())
    }

    pub fn talent_operation_log(&self) -> talent_operation_log::TalentOperationLog {
        talent_operation_log::TalentOperationLog::new(self.service.clone())
    }

    pub fn talent_pool(&self) -> talent_pool::TalentPool {
        talent_pool::TalentPool::new(self.service.clone())
    }

    pub fn talent_tag(&self) -> talent_tag::TalentTag {
        talent_tag::TalentTag::new(self.service.clone())
    }

    pub fn termination_reason(&self) -> termination_reason::TerminationReason {
        termination_reason::TerminationReason::new(self.service.clone())
    }

    pub fn test(&self) -> test::Test {
        test::Test::new(self.service.clone())
    }

    pub fn todo(&self) -> todo::Todo {
        todo::Todo::new(self.service.clone())
    }

    pub fn tripartite_agreement(&self) -> tripartite_agreement::TripartiteAgreement {
        tripartite_agreement::TripartiteAgreement::new(self.service.clone())
    }

    pub fn user_role(&self) -> user_role::UserRole {
        user_role::UserRole::new(self.service.clone())
    }

    pub fn website(&self) -> website::Website {
        website::Website::new(self.service.clone())
    }

    pub fn website_channel(&self) -> website::channel::WebsiteChannel {
        website::channel::WebsiteChannel::new(self.service.clone())
    }

    pub fn website_delivery(&self) -> website::delivery::WebsiteDelivery {
        website::delivery::WebsiteDelivery::new(self.service.clone())
    }

    pub fn website_delivery_task(&self) -> website::delivery_task::WebsiteDeliveryTask {
        website::delivery_task::WebsiteDeliveryTask::new(self.service.clone())
    }

    pub fn website_job_post(&self) -> website::job_post::WebsiteJobPost {
        website::job_post::WebsiteJobPost::new(self.service.clone())
    }

    pub fn website_site_user(&self) -> website::site_user::WebsiteSiteUser {
        website::site_user::WebsiteSiteUser::new(self.service.clone())
    }
}