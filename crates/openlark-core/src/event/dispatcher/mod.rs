use crate::event::{context::EventContext, EventHandler};

// TODO: Re-enable attendance event handlers once service imports are fixed
/*
#[cfg(feature = "attendance")]
use crate::service::attendance::v1::{
    p2_attendance_user_task_status_change_v1::{
        P2AttendanceUserTaskStatusChangeV1, P2AttendanceUserTaskStatusChangeV1ProcessorImpl,
    },
    p2_attendance_user_task_updated_v1::{
        P2AttendanceUserTaskUpdatedV1, P2AttendanceUserTaskUpdatedV1ProcessorImpl,
    },
};
*/

// #[cfg(feature = "im")]
// use crate::service::im::v1::{
//     p2_im_chat_created_v1::{P2ImChatCreatedV1, P2ImChatCreatedV1ProcessorImpl},
//     p2_im_chat_disbanded_v1::{P2ImChatDisbandedV1, P2ImChatDisbandedV1ProcessorImpl},
//     p2_im_chat_member_user_added_v1::{
//         P2ImChatMemberUserAddedV1, P2ImChatMemberUserAddedV1ProcessorImpl,
//     },
//     p2_im_chat_member_user_deleted_v1::{
//         P2ImChatMemberUserDeletedV1, P2ImChatMemberUserDeletedV1ProcessorImpl,
//     },
//     p2_im_chat_updated_v1::{P2ImChatUpdatedV1, P2ImChatUpdatedV1ProcessorImpl},
//     p2_im_message_read_v1::{P2ImMessageReadV1, P2ImMessageReadV1ProcessorImpl},
//     p2_im_message_recalled_v1::{P2ImMessageRecalledV1, P2ImMessageRecalledV1ProcessorImpl},
//     p2_im_message_receive_v1::{P2ImMessageReceiveV1, P2ImMessageReceiveV1ProcessorImpl},
// }; // TODO: Fix service import

// TODO: Re-enable payroll event handlers once service imports are fixed
/*
#[cfg(feature = "payroll")]
use crate::service::payroll::v1::{
    p2_payroll_payment_activity_approved_v1::{
        P2PayrollPaymentActivityApprovedV1, P2PayrollPaymentActivityApprovedV1ProcessorImpl,
    },
    p2_payroll_payment_activity_status_changed_v1::{
        P2PayrollPaymentActivityStatusChangedV1,
        P2PayrollPaymentActivityStatusChangedV1ProcessorImpl,
    },
};
*/

// #[cfg(feature = "contact")]
// use crate::service::contact::v3::{
//     p2_contact_department_created_v3::{
//         P2ContactDepartmentCreatedV3, P2ContactDepartmentCreatedV3ProcessorImpl,
//     },
//     p2_contact_department_deleted_v3::{
//         P2ContactDepartmentDeletedV3, P2ContactDepartmentDeletedV3ProcessorImpl,
//     },
//     p2_contact_department_updated_v3::{
//         P2ContactDepartmentUpdatedV3, P2ContactDepartmentUpdatedV3ProcessorImpl,
//     },
//     p2_contact_user_created_v3::{P2ContactUserCreatedV3, P2ContactUserCreatedV3ProcessorImpl},
//     p2_contact_user_deleted_v3::{P2ContactUserDeletedV3, P2ContactUserDeletedV3ProcessorImpl},
//     p2_contact_user_updated_v3::{P2ContactUserUpdatedV3, P2ContactUserUpdatedV3ProcessorImpl},
// }; // TODO: Fix service import

// TODO: Re-enable cloud-docs, calendar, and vc event handlers once service imports are fixed
/*
#[cfg(feature = "cloud-docs")]
use crate::service::cloud_docs::drive::v1::{
    p2_drive_file_created_v1::{P2DriveFileCreatedV1, P2DriveFileCreatedV1ProcessorImpl},
    p2_drive_file_deleted_v1::{P2DriveFileDeletedV1, P2DriveFileDeletedV1ProcessorImpl},
    p2_drive_file_updated_v1::{P2DriveFileUpdatedV1, P2DriveFileUpdatedV1ProcessorImpl},
};

#[cfg(feature = "calendar")]
use crate::service::calendar::v4::p2_calendar_event_created_v4::{
    P2CalendarEventCreatedV4, P2CalendarEventCreatedV4ProcessorImpl,
};

#[cfg(feature = "vc")]
use crate::service::vc::v1::{
    p2_vc_meeting_ended_v1::{P2VcMeetingEndedV1, P2VcMeetingEndedV1ProcessorImpl},
    p2_vc_meeting_participant_joined_v1::{
        P2VcMeetingParticipantJoinedV1, P2VcMeetingParticipantJoinedV1ProcessorImpl,
    },
    p2_vc_meeting_participant_left_v1::{
        P2VcMeetingParticipantLeftV1, P2VcMeetingParticipantLeftV1ProcessorImpl,
    },
    p2_vc_meeting_started_v1::{P2VcMeetingStartedV1, P2VcMeetingStartedV1ProcessorImpl},
};
*/

// TODO: Re-enable approval event handlers once service imports are fixed
/*
#[cfg(feature = "approval")]
use crate::service::approval::v4::{
    p2_approval_instance_approved_v4::{
        P2ApprovalInstanceApprovedV4, P2ApprovalInstanceApprovedV4ProcessorImpl,
    },
    p2_approval_instance_created_v4::{
        P2ApprovalInstanceCreatedV4, P2ApprovalInstanceCreatedV4ProcessorImpl,
    },
    p2_approval_instance_rejected_v4::{
        P2ApprovalInstanceRejectedV4, P2ApprovalInstanceRejectedV4ProcessorImpl,
    },
};
*/
use log::debug;
use std::collections::HashMap;

/// 事件分发处理器
pub struct EventDispatcherHandler {
    /// 事件map,key为事件类型，value为事件处理器
    processor_map: HashMap<String, Box<dyn EventHandler>>,
    // 事件回调签名token，消息解密key
    verification_token: Option<String>,
    event_encrypt_key: Option<String>,
}

impl EventDispatcherHandler {
    pub fn builder() -> EventDispatcherHandlerBuilder {
        EventDispatcherHandlerBuilder {
            processor_map: HashMap::new(),
            verification_token: None,
            event_encrypt_key: None,
        }
    }

    pub fn set_verification_token(&mut self, verification_token: String) {
        self.verification_token = Some(verification_token);
    }

    pub fn set_event_encrypt_key(&mut self, event_encrypt_key: String) {
        self.event_encrypt_key = Some(event_encrypt_key);
    }

    fn emit(&self, event: &str, payload: &[u8]) -> anyhow::Result<()> {
        if let Some(handler) = self.processor_map.get(event) {
            handler.handle(payload)
        } else {
            log::warn!("No event processor found for event: {event}");
            Err(anyhow::anyhow!("event processor {} not found", event))
        }
    }

    pub fn do_without_validation(&self, payload: Vec<u8>) -> anyhow::Result<()> {
        let mut context = serde_json::from_slice::<EventContext>(&payload)?;
        debug!("{context:?}");
        if context.schema.is_some() {
            // 解析 v2 事件
            context.schema = Some("p2".to_string());
            let header = context
                .header
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("missing event header for v2 event"))?;
            context.type_ = header.event_type.clone();
            context.token = header.token.clone();
        } else if context.uuid.is_some() {
            // 解析 v1 事件
            context.schema = Some("p1".to_string());
            context.type_ = context.event.get("type").map(|v| v.to_string());
        }

        let schema = context
            .schema
            .clone()
            .ok_or_else(|| anyhow::anyhow!("missing event schema"))?;
        let type_ = context
            .type_
            .clone()
            .ok_or_else(|| anyhow::anyhow!("missing event type"))?;
        let handler_name = format!("{}.{}", schema, type_);
        self.emit(&handler_name, &payload)
    }
}

pub struct EventDispatcherHandlerBuilder {
    /// 事件map,key为事件类型，value为事件处理器
    processor_map: HashMap<String, Box<dyn EventHandler>>,
    // 事件回调签名token，消息解密key
    verification_token: Option<String>,
    event_encrypt_key: Option<String>,
}

impl EventDispatcherHandlerBuilder {
    pub fn build(self) -> EventDispatcherHandler {
        EventDispatcherHandler {
            processor_map: self.processor_map,
            verification_token: self.verification_token,
            event_encrypt_key: self.event_encrypt_key,
        }
    }
}

// TODO: Re-enable EventDispatcherHandlerBuilder once service imports are fixed

// TODO: Re-enable EventDispatcherHandlerBuilder once service imports are fixed

// Temporary placeholder for event handler functionality
// This will be re-implemented once service imports are fixed

#[cfg(test)]
mod tests {
    #[test]
    fn test_event_dispatcher_placeholder() {
        // Placeholder test to ensure module compiles
        // TODO: Add proper event dispatcher tests
    }
}
