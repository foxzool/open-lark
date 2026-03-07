#[derive(Debug, Clone, PartialEq)]
pub enum HelpdeskApiV1 {
    // Ticket APIs
    TicketCreate,
    TicketGet(String),
    TicketUpdate(String),
    TicketList,
    // Agent APIs
    AgentEmail,
    AgentPatch(String),
    AgentScheduleList,
    AgentScheduleCreate,
    AgentScheduleGet(String),
    AgentSchedulePatch(String),
    AgentScheduleDelete(String),
    AgentSkillList,
    AgentSkillGet(String),
    AgentSkillCreate,
    AgentSkillPatch(String),
    AgentSkillDelete(String),
    AgentSkillRuleList,
    // Category APIs
    CategoryList,
    CategoryGet(String),
    CategoryCreate,
    CategoryPatch(String),
    CategoryDelete(String),
    // FAQ APIs
    FaqList,
    FaqGet(String),
    FaqCreate,
    FaqPatch(String),
    FaqDelete(String),
    FaqSearch,
    FaqImage(String, String),
    // Notification APIs
    NotificationList,
    NotificationGet(String),
    NotificationCreate,
    NotificationPatch(String),
    NotificationSubmitApprove(String),
    NotificationCancelApprove(String),
    NotificationExecuteSend(String),
    NotificationPreview(String),
    NotificationCancelSend(String),
    // Ticket Message APIs
    TicketMessageList(String),
    TicketMessageCreate(String),
    // Ticket Customized Field APIs
    TicketCustomizedFieldList,
    TicketCustomizedFieldGet(String),
    TicketCustomizedFieldCreate,
    TicketCustomizedFieldPatch(String),
    TicketCustomizedFieldDelete(String),
    // Event APIs
    EventSubscribe,
    EventUnsubscribe,
    // Bot Message
    BotMessageCreate,
    // Other Ticket APIs
    TicketAnswerUserQuery(String),
    TicketCustomizedFields,
    TicketStartService,
    TicketImage,
}

impl HelpdeskApiV1 {
    pub fn to_url(&self) -> String {
        match self {
            // Ticket APIs
            HelpdeskApiV1::TicketCreate => "/open-apis/helpdesk/v1/tickets".to_string(),
            HelpdeskApiV1::TicketGet(id) => format!("/open-apis/helpdesk/v1/tickets/{}", id),
            HelpdeskApiV1::TicketUpdate(id) => format!("/open-apis/helpdesk/v1/tickets/{}", id),
            HelpdeskApiV1::TicketList => "/open-apis/helpdesk/v1/tickets".to_string(),
            // Agent APIs
            HelpdeskApiV1::AgentEmail => "/open-apis/helpdesk/v1/agent_emails".to_string(),
            HelpdeskApiV1::AgentPatch(id) => format!("/open-apis/helpdesk/v1/agents/{}", id),
            HelpdeskApiV1::AgentScheduleList => {
                "/open-apis/helpdesk/v1/agent_schedules".to_string()
            }
            HelpdeskApiV1::AgentScheduleCreate => {
                "/open-apis/helpdesk/v1/agent_schedules".to_string()
            }
            HelpdeskApiV1::AgentScheduleGet(id) => {
                format!("/open-apis/helpdesk/v1/agents/{}/schedules", id)
            }
            HelpdeskApiV1::AgentSchedulePatch(id) => {
                format!("/open-apis/helpdesk/v1/agents/{}/schedules", id)
            }
            HelpdeskApiV1::AgentScheduleDelete(id) => {
                format!("/open-apis/helpdesk/v1/agents/{}/schedules", id)
            }
            HelpdeskApiV1::AgentSkillList => "/open-apis/helpdesk/v1/agent_skills".to_string(),
            HelpdeskApiV1::AgentSkillGet(id) => {
                format!("/open-apis/helpdesk/v1/agent_skills/{}", id)
            }
            HelpdeskApiV1::AgentSkillCreate => "/open-apis/helpdesk/v1/agent_skills".to_string(),
            HelpdeskApiV1::AgentSkillPatch(id) => {
                format!("/open-apis/helpdesk/v1/agent_skills/{}", id)
            }
            HelpdeskApiV1::AgentSkillDelete(id) => {
                format!("/open-apis/helpdesk/v1/agent_skills/{}", id)
            }
            HelpdeskApiV1::AgentSkillRuleList => {
                "/open-apis/helpdesk/v1/agent_skill_rules".to_string()
            }
            // Category APIs
            HelpdeskApiV1::CategoryList => "/open-apis/helpdesk/v1/categories".to_string(),
            HelpdeskApiV1::CategoryGet(id) => format!("/open-apis/helpdesk/v1/categories/{}", id),
            HelpdeskApiV1::CategoryCreate => "/open-apis/helpdesk/v1/categories".to_string(),
            HelpdeskApiV1::CategoryPatch(id) => format!("/open-apis/helpdesk/v1/categories/{}", id),
            HelpdeskApiV1::CategoryDelete(id) => {
                format!("/open-apis/helpdesk/v1/categories/{}", id)
            }
            // FAQ APIs
            HelpdeskApiV1::FaqList => "/open-apis/helpdesk/v1/faqs".to_string(),
            HelpdeskApiV1::FaqGet(id) => format!("/open-apis/helpdesk/v1/faqs/{}", id),
            HelpdeskApiV1::FaqCreate => "/open-apis/helpdesk/v1/faqs".to_string(),
            HelpdeskApiV1::FaqPatch(id) => format!("/open-apis/helpdesk/v1/faqs/{}", id),
            HelpdeskApiV1::FaqDelete(id) => format!("/open-apis/helpdesk/v1/faqs/{}", id),
            HelpdeskApiV1::FaqSearch => "/open-apis/helpdesk/v1/faqs/search".to_string(),
            HelpdeskApiV1::FaqImage(id, _) => format!("/open-apis/helpdesk/v1/faqs/{}/image", id),
            // Notification APIs
            HelpdeskApiV1::NotificationList => "/open-apis/helpdesk/v1/notifications".to_string(),
            HelpdeskApiV1::NotificationGet(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{}", id)
            }
            HelpdeskApiV1::NotificationCreate => "/open-apis/helpdesk/v1/notifications".to_string(),
            HelpdeskApiV1::NotificationPatch(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{}", id)
            }
            HelpdeskApiV1::NotificationSubmitApprove(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{}/submit_approve", id)
            }
            HelpdeskApiV1::NotificationCancelApprove(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{}/cancel_approve", id)
            }
            HelpdeskApiV1::NotificationExecuteSend(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{}/execute_send", id)
            }
            HelpdeskApiV1::NotificationPreview(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{}/preview", id)
            }
            HelpdeskApiV1::NotificationCancelSend(id) => {
                format!("/open-apis/helpdesk/v1/notifications/{}/cancel_send", id)
            }
            // Ticket Message APIs
            HelpdeskApiV1::TicketMessageList(ticket_id) => {
                format!("/open-apis/helpdesk/v1/tickets/{}/messages", ticket_id)
            }
            HelpdeskApiV1::TicketMessageCreate(ticket_id) => {
                format!("/open-apis/helpdesk/v1/tickets/{}/messages", ticket_id)
            }
            // Ticket Customized Field APIs
            HelpdeskApiV1::TicketCustomizedFieldList => {
                "/open-apis/helpdesk/v1/ticket_customized_fields".to_string()
            }
            HelpdeskApiV1::TicketCustomizedFieldGet(id) => {
                format!("/open-apis/helpdesk/v1/ticket_customized_fields/{}", id)
            }
            HelpdeskApiV1::TicketCustomizedFieldCreate => {
                "/open-apis/helpdesk/v1/ticket_customized_fields".to_string()
            }
            HelpdeskApiV1::TicketCustomizedFieldPatch(id) => {
                format!("/open-apis/helpdesk/v1/ticket_customized_fields/{}", id)
            }
            HelpdeskApiV1::TicketCustomizedFieldDelete(id) => {
                format!("/open-apis/helpdesk/v1/ticket_customized_fields/{}", id)
            }
            // Event APIs
            HelpdeskApiV1::EventSubscribe => "/open-apis/helpdesk/v1/events/subscribe".to_string(),
            HelpdeskApiV1::EventUnsubscribe => {
                "/open-apis/helpdesk/v1/events/unsubscribe".to_string()
            }
            // Bot Message
            HelpdeskApiV1::BotMessageCreate => "/open-apis/helpdesk/v1/message".to_string(),
            // Other Ticket APIs
            HelpdeskApiV1::TicketAnswerUserQuery(id) => {
                format!("/open-apis/helpdesk/v1/tickets/{}/answer_user_query", id)
            }
            HelpdeskApiV1::TicketCustomizedFields => {
                "/open-apis/helpdesk/v1/customized_fields".to_string()
            }
            HelpdeskApiV1::TicketStartService => "/open-apis/helpdesk/v1/start_service".to_string(),
            HelpdeskApiV1::TicketImage => "/open-apis/helpdesk/v1/ticket_images".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helpdeskapiv1_to_url_coverage() {
        let urls = vec![
            HelpdeskApiV1::TicketCreate.to_url(),
            HelpdeskApiV1::TicketGet("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketUpdate("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketList.to_url(),
            HelpdeskApiV1::AgentEmail.to_url(),
            HelpdeskApiV1::AgentPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentScheduleList.to_url(),
            HelpdeskApiV1::AgentScheduleCreate.to_url(),
            HelpdeskApiV1::AgentScheduleGet("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSchedulePatch("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentScheduleDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSkillList.to_url(),
            HelpdeskApiV1::AgentSkillGet("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSkillCreate.to_url(),
            HelpdeskApiV1::AgentSkillPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSkillDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::AgentSkillRuleList.to_url(),
            HelpdeskApiV1::CategoryList.to_url(),
            HelpdeskApiV1::CategoryGet("id1".to_string()).to_url(),
            HelpdeskApiV1::CategoryCreate.to_url(),
            HelpdeskApiV1::CategoryPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::CategoryDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::FaqList.to_url(),
            HelpdeskApiV1::FaqGet("id1".to_string()).to_url(),
            HelpdeskApiV1::FaqCreate.to_url(),
            HelpdeskApiV1::FaqPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::FaqDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::FaqSearch.to_url(),
            HelpdeskApiV1::FaqImage("id1".to_string(), "id2".to_string()).to_url(),
            HelpdeskApiV1::NotificationList.to_url(),
            HelpdeskApiV1::NotificationGet("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationCreate.to_url(),
            HelpdeskApiV1::NotificationPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationSubmitApprove("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationCancelApprove("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationExecuteSend("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationPreview("id1".to_string()).to_url(),
            HelpdeskApiV1::NotificationCancelSend("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketMessageList("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketMessageCreate("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketCustomizedFieldList.to_url(),
            HelpdeskApiV1::TicketCustomizedFieldGet("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketCustomizedFieldCreate.to_url(),
            HelpdeskApiV1::TicketCustomizedFieldPatch("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketCustomizedFieldDelete("id1".to_string()).to_url(),
            HelpdeskApiV1::EventSubscribe.to_url(),
            HelpdeskApiV1::EventUnsubscribe.to_url(),
            HelpdeskApiV1::BotMessageCreate.to_url(),
            HelpdeskApiV1::TicketAnswerUserQuery("id1".to_string()).to_url(),
            HelpdeskApiV1::TicketCustomizedFields.to_url(),
            HelpdeskApiV1::TicketStartService.to_url(),
            HelpdeskApiV1::TicketImage.to_url(),
        ];
        assert!(urls.iter().all(|url| url.starts_with("/open-apis/")));
    }
}
