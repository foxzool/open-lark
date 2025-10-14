use crate::event::context::EventContext;

#[cfg(feature = "attendance")]
use crate::service::attendance::v1::{
    p2_attendance_user_task_status_change_v1::{
        P2AttendanceUserTaskStatusChangeV1, P2AttendanceUserTaskStatusChangeV1ProcessorImpl,
    },
    p2_attendance_user_task_updated_v1::{
        P2AttendanceUserTaskUpdatedV1, P2AttendanceUserTaskUpdatedV1ProcessorImpl,
    },
};

#[cfg(feature = "im")]
use crate::service::im::v1::{
    p2_im_chat_created_v1::{P2ImChatCreatedV1, P2ImChatCreatedV1ProcessorImpl},
    p2_im_chat_disbanded_v1::{P2ImChatDisbandedV1, P2ImChatDisbandedV1ProcessorImpl},
    p2_im_chat_member_user_added_v1::{
        P2ImChatMemberUserAddedV1, P2ImChatMemberUserAddedV1ProcessorImpl,
    },
    p2_im_chat_member_user_deleted_v1::{
        P2ImChatMemberUserDeletedV1, P2ImChatMemberUserDeletedV1ProcessorImpl,
    },
    p2_im_chat_updated_v1::{P2ImChatUpdatedV1, P2ImChatUpdatedV1ProcessorImpl},
    p2_im_message_read_v1::{P2ImMessageReadV1, P2ImMessageReadV1ProcessorImpl},
    p2_im_message_recalled_v1::{P2ImMessageRecalledV1, P2ImMessageRecalledV1ProcessorImpl},
    p2_im_message_receive_v1::{P2ImMessageReceiveV1, P2ImMessageReceiveV1ProcessorImpl},
};

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

#[cfg(feature = "contact")]
use crate::service::contact::v3::{
    p2_contact_department_created_v3::{
        P2ContactDepartmentCreatedV3, P2ContactDepartmentCreatedV3ProcessorImpl,
    },
    p2_contact_department_deleted_v3::{
        P2ContactDepartmentDeletedV3, P2ContactDepartmentDeletedV3ProcessorImpl,
    },
    p2_contact_department_updated_v3::{
        P2ContactDepartmentUpdatedV3, P2ContactDepartmentUpdatedV3ProcessorImpl,
    },
    p2_contact_user_created_v3::{P2ContactUserCreatedV3, P2ContactUserCreatedV3ProcessorImpl},
    p2_contact_user_deleted_v3::{P2ContactUserDeletedV3, P2ContactUserDeletedV3ProcessorImpl},
    p2_contact_user_updated_v3::{P2ContactUserUpdatedV3, P2ContactUserUpdatedV3ProcessorImpl},
};

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
            context
                .type_
                .clone_from(&context.header.as_ref().unwrap().event_type);
            context
                .token
                .clone_from(&context.header.as_ref().unwrap().token)
        } else if context.uuid.is_some() {
            // 解析 v1 事件
            context.schema = Some("p1".to_string());
            context.type_ = context.event.get("type").map(|v| v.to_string());
        }

        let handler_name = format!("{}.{}", context.schema.unwrap(), context.type_.unwrap());
        self.emit(&handler_name, &payload)
    }
}

pub trait EventHandler: Send + Sync {
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()>;
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

impl EventDispatcherHandlerBuilder {
    #[cfg(feature = "im")]
    pub fn register_p2_im_message_receive_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImMessageReceiveV1) + 'static + Sync + Send,
    {
        let key = "p2.im.message.receive_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ImMessageReceiveV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "im")]
    pub fn register_p2_im_message_read_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImMessageReadV1) + 'static + Sync + Send,
    {
        let key = "p2.im.message.message_read_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ImMessageReadV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "im")]
    pub fn register_p2_im_message_recalled_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImMessageRecalledV1) + 'static + Sync + Send,
    {
        let key = "p2.im.message.recalled_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ImMessageRecalledV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "im")]
    pub fn register_p2_im_chat_created_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImChatCreatedV1) + 'static + Sync + Send,
    {
        let key = "p2.im.chat.created_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ImChatCreatedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "im")]
    pub fn register_p2_im_chat_disbanded_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImChatDisbandedV1) + 'static + Sync + Send,
    {
        let key = "p2.im.chat.disbanded_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ImChatDisbandedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "im")]
    pub fn register_p2_im_chat_member_user_added_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImChatMemberUserAddedV1) + 'static + Sync + Send,
    {
        let key = "p2.im.chat.member.user.added_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ImChatMemberUserAddedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    /// 注册考勤打卡流水事件处理器
    #[cfg(feature = "attendance")]
    pub fn register_p2_attendance_user_task_updated_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2AttendanceUserTaskUpdatedV1) + 'static + Sync + Send,
    {
        let key = "p2.attendance.user_task.updated_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2AttendanceUserTaskUpdatedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    /// 注册考勤用户任务状态变更事件处理器
    #[cfg(feature = "attendance")]
    pub fn register_p2_attendance_user_task_status_change_v1<F>(
        mut self,
        f: F,
    ) -> Result<Self, String>
    where
        F: Fn(P2AttendanceUserTaskStatusChangeV1) + 'static + Sync + Send,
    {
        let key = "p2.attendance.user_task.status_change_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2AttendanceUserTaskStatusChangeV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    /// 注册发薪活动状态变更事件处理器
    #[cfg(feature = "payroll")]
    pub fn register_p2_payroll_payment_activity_status_changed_v1<F>(
        mut self,
        f: F,
    ) -> Result<Self, String>
    where
        F: Fn(P2PayrollPaymentActivityStatusChangedV1) -> anyhow::Result<()>
            + 'static
            + Sync
            + Send,
    {
        let key = "p2.payroll.payment_activity.status_changed_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2PayrollPaymentActivityStatusChangedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    /// 注册发薪活动封存事件处理器
    #[cfg(feature = "payroll")]
    pub fn register_p2_payroll_payment_activity_approved_v1<F>(
        mut self,
        f: F,
    ) -> Result<Self, String>
    where
        F: Fn(P2PayrollPaymentActivityApprovedV1) -> anyhow::Result<()> + 'static + Sync + Send,
    {
        let key = "p2.payroll.payment_activity.approved_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2PayrollPaymentActivityApprovedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    // IM Events - Additional
    #[cfg(feature = "im")]
    pub fn register_p2_im_chat_updated_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImChatUpdatedV1) + 'static + Sync + Send,
    {
        let key = "p2.im.chat.updated_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ImChatUpdatedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "im")]
    pub fn register_p2_im_chat_member_user_deleted_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImChatMemberUserDeletedV1) + 'static + Sync + Send,
    {
        let key = "p2.im.chat.member.user.deleted_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ImChatMemberUserDeletedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    // Contact Events
    #[cfg(feature = "contact")]
    pub fn register_p2_contact_user_created_v3<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ContactUserCreatedV3) + 'static + Sync + Send,
    {
        let key = "p2.contact.user.created_v3".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ContactUserCreatedV3ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "contact")]
    pub fn register_p2_contact_user_updated_v3<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ContactUserUpdatedV3) + 'static + Sync + Send,
    {
        let key = "p2.contact.user.updated_v3".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ContactUserUpdatedV3ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "contact")]
    pub fn register_p2_contact_user_deleted_v3<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ContactUserDeletedV3) + 'static + Sync + Send,
    {
        let key = "p2.contact.user.deleted_v3".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ContactUserDeletedV3ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "contact")]
    pub fn register_p2_contact_department_created_v3<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ContactDepartmentCreatedV3) + 'static + Sync + Send,
    {
        let key = "p2.contact.department.created_v3".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ContactDepartmentCreatedV3ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "contact")]
    pub fn register_p2_contact_department_updated_v3<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ContactDepartmentUpdatedV3) + 'static + Sync + Send,
    {
        let key = "p2.contact.department.updated_v3".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ContactDepartmentUpdatedV3ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "contact")]
    pub fn register_p2_contact_department_deleted_v3<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ContactDepartmentDeletedV3) + 'static + Sync + Send,
    {
        let key = "p2.contact.department.deleted_v3".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ContactDepartmentDeletedV3ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    // Drive Events
    #[cfg(feature = "cloud-docs")]
    pub fn register_p2_drive_file_created_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2DriveFileCreatedV1) + 'static + Sync + Send,
    {
        let key = "p2.drive.file.created_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2DriveFileCreatedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "cloud-docs")]
    pub fn register_p2_drive_file_updated_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2DriveFileUpdatedV1) + 'static + Sync + Send,
    {
        let key = "p2.drive.file.updated_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2DriveFileUpdatedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "cloud-docs")]
    pub fn register_p2_drive_file_deleted_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2DriveFileDeletedV1) + 'static + Sync + Send,
    {
        let key = "p2.drive.file.deleted_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2DriveFileDeletedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    // Calendar Events
    #[cfg(feature = "calendar")]
    pub fn register_p2_calendar_event_created_v4<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2CalendarEventCreatedV4) + 'static + Sync + Send,
    {
        let key = "p2.calendar.event.created_v4".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2CalendarEventCreatedV4ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    // VC Events
    #[cfg(feature = "vc")]
    pub fn register_p2_vc_meeting_started_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2VcMeetingStartedV1) + 'static + Sync + Send,
    {
        let key = "p2.vc.meeting.started_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2VcMeetingStartedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "vc")]
    pub fn register_p2_vc_meeting_ended_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2VcMeetingEndedV1) + 'static + Sync + Send,
    {
        let key = "p2.vc.meeting.ended_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2VcMeetingEndedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "vc")]
    pub fn register_p2_vc_meeting_participant_joined_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2VcMeetingParticipantJoinedV1) + 'static + Sync + Send,
    {
        let key = "p2.vc.meeting.participant.joined_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2VcMeetingParticipantJoinedV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "vc")]
    pub fn register_p2_vc_meeting_participant_left_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2VcMeetingParticipantLeftV1) + 'static + Sync + Send,
    {
        let key = "p2.vc.meeting.participant.left_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2VcMeetingParticipantLeftV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    // Approval Events
    #[cfg(feature = "approval")]
    pub fn register_p2_approval_instance_created_v4<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ApprovalInstanceCreatedV4) + 'static + Sync + Send,
    {
        let key = "p2.approval.instance.created_v4".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ApprovalInstanceCreatedV4ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "approval")]
    pub fn register_p2_approval_instance_approved_v4<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ApprovalInstanceApprovedV4) + 'static + Sync + Send,
    {
        let key = "p2.approval.instance.approved_v4".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ApprovalInstanceApprovedV4ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    #[cfg(feature = "approval")]
    pub fn register_p2_approval_instance_rejected_v4<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ApprovalInstanceRejectedV4) + 'static + Sync + Send,
    {
        let key = "p2.approval.instance.rejected_v4".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        let processor = P2ApprovalInstanceRejectedV4ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::context::{EventContext, EventHeader};
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    struct TestEventHandler {
        calls: Arc<Mutex<Vec<String>>>,
    }

    impl TestEventHandler {
        fn new(calls: Arc<Mutex<Vec<String>>>) -> Self {
            Self { calls }
        }
    }

    impl EventHandler for TestEventHandler {
        fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
            let payload_str = String::from_utf8_lossy(payload);
            self.calls.lock().unwrap().push(payload_str.to_string());
            Ok(())
        }
    }

    struct FailingEventHandler;

    impl EventHandler for FailingEventHandler {
        fn handle(&self, _payload: &[u8]) -> anyhow::Result<()> {
            Err(anyhow::anyhow!("Intentional test failure"))
        }
    }

    #[test]
    fn test_event_dispatcher_handler_builder_creation() {
        let builder = EventDispatcherHandler::builder();
        assert!(builder.processor_map.is_empty());
        assert!(builder.verification_token.is_none());
        assert!(builder.event_encrypt_key.is_none());
    }

    #[test]
    fn test_event_dispatcher_handler_build() {
        let handler = EventDispatcherHandler::builder().build();
        assert!(handler.processor_map.is_empty());
        assert!(handler.verification_token.is_none());
        assert!(handler.event_encrypt_key.is_none());
    }

    #[test]
    fn test_set_verification_token() {
        let mut handler = EventDispatcherHandler::builder().build();
        handler.set_verification_token("test_token".to_string());
        assert_eq!(handler.verification_token, Some("test_token".to_string()));
    }

    #[test]
    fn test_set_event_encrypt_key() {
        let mut handler = EventDispatcherHandler::builder().build();
        handler.set_event_encrypt_key("test_encrypt_key".to_string());
        assert_eq!(
            handler.event_encrypt_key,
            Some("test_encrypt_key".to_string())
        );
    }

    #[test]
    fn test_emit_with_registered_handler() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("test.event".to_string(), Box::new(test_handler));

        let test_payload = b"test payload";
        let result = handler.emit("test.event", test_payload);

        assert!(result.is_ok());
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 1);
        assert_eq!(calls_vec[0], "test payload");
    }

    #[test]
    fn test_emit_with_unregistered_handler() {
        let handler = EventDispatcherHandler::builder().build();
        let test_payload = b"test payload";

        let result = handler.emit("unregistered.event", test_payload);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("event processor unregistered.event not found"));
    }

    #[test]
    fn test_emit_with_failing_handler() {
        let failing_handler = FailingEventHandler;

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("failing.event".to_string(), Box::new(failing_handler));

        let test_payload = b"test payload";
        let result = handler.emit("failing.event", test_payload);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Intentional test failure"));
    }

    #[test]
    fn test_do_without_validation_v2_event() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("p2.test.event.type".to_string(), Box::new(test_handler));

        // Create a v2 event context
        let event_context = EventContext {
            ts: Some("1234567890".to_string()),
            uuid: None,
            token: Some("test_token".to_string()),
            type_: Some("original.type".to_string()),
            schema: Some("2.0".to_string()), // v2 event has schema
            header: Some(EventHeader {
                event_id: Some("event_123".to_string()),
                event_type: Some("test.event.type".to_string()),
                create_time: Some("1234567890".to_string()),
                token: Some("header_token".to_string()),
                app_id: Some("app_123".to_string()),
                tenant_key: Some("tenant_123".to_string()),
            }),
            event: HashMap::new(),
        };

        let payload = serde_json::to_vec(&event_context).unwrap();
        let result = handler.do_without_validation(payload.clone());

        assert!(result.is_ok());
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 1);
        assert_eq!(calls_vec[0], String::from_utf8_lossy(&payload));
    }

    #[test]
    fn test_do_without_validation_v1_event() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        // Note: JSON string values get quotes when using to_string(), so we register with quotes
        handler
            .processor_map
            .insert("p1.\"test.event.type\"".to_string(), Box::new(test_handler));

        // Create a v1 event context
        let mut event = HashMap::new();
        event.insert("type".to_string(), json!("test.event.type"));

        let event_context = EventContext {
            ts: Some("1234567890".to_string()),
            uuid: Some("uuid_123".to_string()), // v1 event has uuid
            token: Some("test_token".to_string()),
            type_: None,
            schema: None, // v1 event has no schema
            header: None,
            event,
        };

        let payload = serde_json::to_vec(&event_context).unwrap();
        let result = handler.do_without_validation(payload.clone());

        assert!(result.is_ok());
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 1);
        assert_eq!(calls_vec[0], String::from_utf8_lossy(&payload));
    }

    #[test]
    fn test_do_without_validation_invalid_json() {
        let handler = EventDispatcherHandler::builder().build();
        let invalid_payload = b"invalid json";

        let result = handler.do_without_validation(invalid_payload.to_vec());

        assert!(result.is_err());
    }

    #[test]
    fn test_do_without_validation_missing_handler() {
        let handler = EventDispatcherHandler::builder().build();

        let event_context = EventContext {
            ts: Some("1234567890".to_string()),
            uuid: Some("uuid_123".to_string()),
            token: Some("test_token".to_string()),
            type_: None,
            schema: None,
            header: None,
            event: {
                let mut event = HashMap::new();
                event.insert("type".to_string(), json!("unregistered.event"));
                event
            },
        };

        let payload = serde_json::to_vec(&event_context).unwrap();
        let result = handler.do_without_validation(payload);

        assert!(result.is_err());
        // JSON string values include quotes when converted to string
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("event processor p1.\"unregistered.event\" not found"));
    }

    #[test]
    fn test_multiple_handlers_registration() {
        let calls1 = Arc::new(Mutex::new(Vec::new()));
        let calls2 = Arc::new(Mutex::new(Vec::new()));

        let handler1 = TestEventHandler::new(calls1.clone());
        let handler2 = TestEventHandler::new(calls2.clone());

        let mut dispatcher = EventDispatcherHandler::builder().build();
        dispatcher
            .processor_map
            .insert("event.1".to_string(), Box::new(handler1));
        dispatcher
            .processor_map
            .insert("event.2".to_string(), Box::new(handler2));

        let test_payload = b"test payload";

        let result1 = dispatcher.emit("event.1", test_payload);
        let result2 = dispatcher.emit("event.2", test_payload);

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        assert_eq!(calls1.lock().unwrap().len(), 1);
        assert_eq!(calls2.lock().unwrap().len(), 1);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_event_context_parsing_edge_cases() {
        let handler = EventDispatcherHandler::builder().build();

        // Test event with neither schema nor uuid - this will cause panic in the actual code
        // because line 85 calls unwrap() on None values
        let event_context = EventContext {
            ts: Some("1234567890".to_string()),
            uuid: None,
            token: Some("test_token".to_string()),
            type_: Some("some.type".to_string()),
            schema: None,
            header: None,
            event: HashMap::new(),
        };

        let payload = serde_json::to_vec(&event_context).unwrap();

        // This test documents the current behavior - it panics due to unwrap() on None
        // In a real implementation, this should be handled more gracefully
        handler.do_without_validation(payload).unwrap();
    }

    #[test]
    fn test_handler_name_generation() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler.processor_map.insert(
            "p2.complex.event.name.with.dots".to_string(),
            Box::new(test_handler),
        );

        let event_context = EventContext {
            ts: Some("1234567890".to_string()),
            uuid: None,
            token: Some("test_token".to_string()),
            type_: Some("original.type".to_string()),
            schema: Some("2.0".to_string()),
            header: Some(EventHeader {
                event_id: Some("event_123".to_string()),
                event_type: Some("complex.event.name.with.dots".to_string()),
                create_time: Some("1234567890".to_string()),
                token: Some("header_token".to_string()),
                app_id: Some("app_123".to_string()),
                tenant_key: Some("tenant_123".to_string()),
            }),
            event: HashMap::new(),
        };

        let payload = serde_json::to_vec(&event_context).unwrap();
        let result = handler.do_without_validation(payload.clone());

        assert!(result.is_ok());
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 1);
    }

    #[test]
    fn test_empty_processor_map() {
        let handler = EventDispatcherHandler::builder().build();
        assert!(handler.processor_map.is_empty());

        let result = handler.emit("any.event", b"payload");
        assert!(result.is_err());
    }

    // ==================== Enhanced Coverage Tests ====================

    // Thread safety and concurrent access tests
    #[test]
    fn test_concurrent_handler_access() {
        use std::sync::Arc;
        use std::thread;

        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("concurrent.test".to_string(), Box::new(test_handler));

        let handler = Arc::new(handler);
        let mut handles = vec![];

        // Spawn multiple threads that call the handler concurrently
        for i in 0..10 {
            let handler_clone = handler.clone();
            let handle = thread::spawn(move || {
                let payload = format!("concurrent payload {}", i);
                let result = handler_clone.emit("concurrent.test", payload.as_bytes());
                assert!(result.is_ok());
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all calls were processed
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 10);
    }

    // Memory management and lifecycle tests
    #[test]
    fn test_handler_lifecycle() {
        let calls = Arc::new(Mutex::new(Vec::new()));

        // Create handler and let it go out of scope
        {
            let test_handler = TestEventHandler::new(calls.clone());
            let mut handler = EventDispatcherHandler::builder().build();
            handler
                .processor_map
                .insert("lifecycle.test".to_string(), Box::new(test_handler));

            let result = handler.emit("lifecycle.test", b"lifecycle payload");
            assert!(result.is_ok());
        } // handler and test_handler go out of scope here

        // Verify the call was still processed correctly
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 1);
        assert_eq!(calls_vec[0], "lifecycle payload");
    }

    // Error handling and edge case tests
    #[test]
    fn test_malformed_payload_handling() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("malformed.test".to_string(), Box::new(test_handler));

        // Test with various malformed payloads
        let test_cases = vec![
            Vec::new(), // Empty payload
            vec![0x00, 0x01, 0x02], // Binary data
            b"{".to_vec(), // Incomplete JSON
        ];

        for payload in &test_cases {
            let result = handler.emit("malformed.test", payload);
            // Handler should still succeed as it just processes raw bytes
            assert!(result.is_ok());
        }

        // Verify all calls were processed
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), test_cases.len());
    }

    // Complex event context tests
    #[test]
    fn test_complex_event_context_parsing() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("p2.complex.event.type".to_string(), Box::new(test_handler));

        // Create a complex v2 event with all fields populated
        let event_context = EventContext {
            ts: Some("1234567890".to_string()),
            uuid: None,
            token: Some("original_token".to_string()),
            type_: Some("original.type".to_string()),
            schema: Some("2.0".to_string()),
            header: Some(EventHeader {
                event_id: Some("complex_event_123".to_string()),
                event_type: Some("complex.event.type".to_string()),
                create_time: Some("1234567890".to_string()),
                token: Some("header_token".to_string()),
                app_id: Some("complex_app_456".to_string()),
                tenant_key: Some("complex_tenant_789".to_string()),
            }),
            event: {
                let mut event = HashMap::new();
                event.insert("custom_field".to_string(), json!("custom_value"));
                event.insert("nested".to_string(), json!({"key": "value"}));
                event
            },
        };

        let payload = serde_json::to_vec(&event_context).unwrap();
        let result = handler.do_without_validation(payload.clone());

        assert!(result.is_ok());
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 1);
        assert_eq!(calls_vec[0], String::from_utf8_lossy(&payload));
    }

    // Builder pattern comprehensive tests
    #[test]
    fn test_builder_pattern_comprehensive() {
        let builder = EventDispatcherHandler::builder();

        // Test builder is initially empty
        assert!(builder.processor_map.is_empty());
        assert!(builder.verification_token.is_none());
        assert!(builder.event_encrypt_key.is_none());

        // Simulate adding processors (we can't actually add them without the specific feature flags)
        // But we can test the builder structure
        let handler = builder.build();

        // Built handler should maintain initial state
        assert!(handler.processor_map.is_empty());
        assert!(handler.verification_token.is_none());
        assert!(handler.event_encrypt_key.is_none());
    }

    // Handler registration edge cases
    #[test]
    fn test_processor_map_edge_cases() {
        let mut handler = EventDispatcherHandler::builder().build();

        // Test adding handler with empty string key
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());
        handler
            .processor_map
            .insert("".to_string(), Box::new(test_handler));

        let result = handler.emit("", b"empty key test");
        assert!(result.is_ok());

        // Test adding handler with very long key
        let long_key = "a".repeat(1000);
        let test_handler2 = TestEventHandler::new(calls.clone());
        handler
            .processor_map
            .insert(long_key.clone(), Box::new(test_handler2));

        let result = handler.emit(&long_key, b"long key test");
        assert!(result.is_ok());

        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 2);
    }

    // Unicode and special character handling
    #[test]
    fn test_unicode_payload_handling() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("unicode.test".to_string(), Box::new(test_handler));

        // Test with Unicode content
        let unicode_payloads = vec![
            "测试中文内容",
            "🚀 Rocket emoji test",
            "Mixed English and 中文 content",
            "Café résumé naïve façade",
            "العربية العربية",
            "עברית עברית",
        ];

        for payload in &unicode_payloads {
            let result = handler.emit("unicode.test", payload.as_bytes());
            assert!(result.is_ok());
        }

        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), unicode_payloads.len());

        // Verify Unicode content was preserved correctly
        for (i, expected) in unicode_payloads.iter().enumerate() {
            assert_eq!(calls_vec[i], *expected);
        }
    }

    // Performance and stress tests
    #[test]
    fn test_high_frequency_events() {
        use std::time::Instant;

        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("performance.test".to_string(), Box::new(test_handler));

        let start = Instant::now();
        let num_events = 1000;

        for i in 0..num_events {
            let payload = format!("performance test payload {}", i);
            let result = handler.emit("performance.test", payload.as_bytes());
            assert!(result.is_ok());
        }

        let duration = start.elapsed();
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), num_events);

        // Performance should be reasonable (less than 1 second for 1000 events)
        assert!(duration.as_secs() < 1, "Performance test took too long: {:?}", duration);
    }

    // Event validation edge cases
    #[test]
    fn test_event_context_validation_edge_cases() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler1 = TestEventHandler::new(calls.clone());
        let test_handler2 = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("p1.\"\"".to_string(), Box::new(test_handler1));
        handler
            .processor_map
            .insert("p1.\"complex.type.with.dots\"".to_string(), Box::new(test_handler2));

        // Test v1 event with various edge cases
        let test_cases = vec![
            // Event with empty type
            {
                let mut event = HashMap::new();
                event.insert("type".to_string(), json!(""));
                event
            },
            // Event with complex nested type
            {
                let mut event = HashMap::new();
                event.insert("type".to_string(), json!("complex.type.with.dots"));
                event
            },
        ];

        for (i, event) in test_cases.into_iter().enumerate() {
            let event_context = EventContext {
                ts: Some("1234567890".to_string()),
                uuid: Some(format!("uuid_{}", i)),
                token: Some("test_token".to_string()),
                type_: None,
                schema: None,
                header: None,
                event,
            };

            let payload = serde_json::to_vec(&event_context).unwrap();
            let result = handler.do_without_validation(payload);

            // Should succeed for valid structures
            assert!(result.is_ok(), "Test case {} failed", i);
        }

        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 2);
    }

    // Resource cleanup tests
    #[test]
    fn test_resource_cleanup() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

        struct CountingDropHandler {
            calls: Arc<Mutex<Vec<String>>>,
        }

        impl Drop for CountingDropHandler {
            fn drop(&mut self) {
                DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        impl EventHandler for CountingDropHandler {
            fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
                let payload_str = String::from_utf8_lossy(payload);
                self.calls.lock().unwrap().push(payload_str.to_string());
                Ok(())
            }
        }

        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = CountingDropHandler { calls: calls.clone() };

        {
            let mut handler = EventDispatcherHandler::builder().build();
            handler
                .processor_map
                .insert("cleanup.test".to_string(), Box::new(test_handler));

            let result = handler.emit("cleanup.test", b"cleanup payload");
            assert!(result.is_ok());

            // Handler goes out of scope here, should be dropped
        }

        // Verify handler was called before cleanup
        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), 1);
        assert_eq!(calls_vec[0], "cleanup payload");

        // Verify handler was dropped (this might take a moment due to Arc)
        // Note: This test might be flaky due to async drop behavior
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(DROP_COUNT.load(Ordering::SeqCst) >= 1);
    }

    // Feature flag compilation tests
    #[cfg(feature = "im")]
    #[test]
    fn test_im_feature_flag_compilation() {
        // This test verifies that IM-specific event handlers compile correctly
        // when the "im" feature flag is enabled
        let builder = EventDispatcherHandler::builder();

        // The fact that this compiles proves the feature flag works
        assert!(builder.processor_map.is_empty());
    }

    #[cfg(feature = "attendance")]
    #[test]
    fn test_attendance_feature_flag_compilation() {
        // This test verifies that attendance-specific event handlers compile correctly
        // when the "attendance" feature flag is enabled
        let builder = EventDispatcherHandler::builder();

        assert!(builder.processor_map.is_empty());
    }

    #[cfg(feature = "contact")]
    #[test]
    fn test_contact_feature_flag_compilation() {
        // This test verifies that contact-specific event handlers compile correctly
        // when the "contact" feature flag is enabled
        let builder = EventDispatcherHandler::builder();

        assert!(builder.processor_map.is_empty());
    }

    // Error propagation tests
    #[test]
    fn test_error_propagation_chain() {
        struct ChainErrorHandler {
            should_fail: bool,
        }

        impl EventHandler for ChainErrorHandler {
            fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
                if self.should_fail {
                    Err(anyhow::anyhow!("Chain error: {}", String::from_utf8_lossy(payload)))
                } else {
                    Ok(())
                }
            }
        }

        // Test successful handler
        let success_handler = ChainErrorHandler { should_fail: false };
        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("success.chain".to_string(), Box::new(success_handler));

        let result = handler.emit("success.chain", b"success payload");
        assert!(result.is_ok());

        // Test failing handler
        let fail_handler = ChainErrorHandler { should_fail: true };
        handler
            .processor_map
            .insert("fail.chain".to_string(), Box::new(fail_handler));

        let result = handler.emit("fail.chain", b"failure payload");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Chain error: failure payload"));
    }

    // Large payload handling tests
    #[test]
    fn test_large_payload_handling() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let test_handler = TestEventHandler::new(calls.clone());

        let mut handler = EventDispatcherHandler::builder().build();
        handler
            .processor_map
            .insert("large_payload.test".to_string(), Box::new(test_handler));

        // Test with increasingly large payloads
        let sizes = vec![1024, 10240, 102400, 1024000]; // 1KB, 10KB, 100KB, 1MB

        for size in &sizes {
            let large_payload = vec![b'X'; *size];
            let result = handler.emit("large_payload.test", &large_payload);
            assert!(result.is_ok(), "Failed for payload size: {}", size);
        }

        let calls_vec = calls.lock().unwrap();
        assert_eq!(calls_vec.len(), sizes.len());

        // Verify payload sizes
        for (i, size) in sizes.iter().enumerate() {
            assert_eq!(calls_vec[i].len(), *size);
        }
    }
}
