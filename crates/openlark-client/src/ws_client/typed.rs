//! typed 事件 payload（schema 2.0）与按域子模块。
//!
//! typed 事件共 159 个，含 main 原有的 `im.message.receive_v1`，其中 158 个是新增。
//! 新增分布：im 12、contact 13、approval 3、calendar 4、task 4、drive 12、mail 1、
//! hire 12、vc 24、corehr v1 28、corehr v2 45（相加为 158）。
//! 全部 159 个 raw 事件的 handler trait、闭包 blanket impl 与 `register_*` 方法由
//! `macros` 的 `typed_events!` 样板宏在各域子模块批量生成（`im.message.receive_v1`
//! 也在 `im` 子模块，#709 从本文件手写链收敛）。卡片回调 `card.action.trigger`
//! 走 callback map（可回写 ACK），机制与 raw 事件不同，保留在本文件手写实现，
//! 且不计入这 159 个。
//!
//! envelope 类型名按事件串全段 `CamelCase` 机械生成，如
//! `im.chat.disbanded_v1` → [`im::ImChatDisbandedV1`]、
//! `vc.meeting.meeting_started_v1` → [`vc::VcMeetingMeetingStartedV1`]。
//!
//! typed 注册写入 raw map，与 [`EventDispatcherHandler::register_raw`] 使用同一
//! `event_type` key，重复注册返回 Err；卡片回调写入 callback map，ACK 业务 JSON
//! 走 [`CallbackEventHandler`]。未覆盖的事件继续用
//! [`EventDispatcherHandler::register_raw`] 手动反序列化。
//!
//! 事件串与 `EventDispatcherHandler` 注册方法对照：
//!
//! <!-- typed-event-index-start -->
//!
//! | 事件串 | 注册方法 |
//! | --- | --- |
//! | `im.message.receive_v1` | `register_im_message_receive_v1` |
//! | `im.chat.disbanded_v1` | `register_im_chat_disbanded_v1` |
//! | `im.chat.updated_v1` | `register_im_chat_updated_v1` |
//! | `im.chat.access_event.bot_p2p_chat_entered_v1` | `register_im_chat_access_event_bot_p2p_chat_entered_v1` |
//! | `im.chat.member.bot.added_v1` | `register_im_chat_member_bot_added_v1` |
//! | `im.chat.member.bot.deleted_v1` | `register_im_chat_member_bot_deleted_v1` |
//! | `im.chat.member.user.added_v1` | `register_im_chat_member_user_added_v1` |
//! | `im.chat.member.user.deleted_v1` | `register_im_chat_member_user_deleted_v1` |
//! | `im.chat.member.user.withdrawn_v1` | `register_im_chat_member_user_withdrawn_v1` |
//! | `im.message.message_read_v1` | `register_im_message_message_read_v1` |
//! | `im.message.recalled_v1` | `register_im_message_recalled_v1` |
//! | `im.message.reaction.created_v1` | `register_im_message_reaction_created_v1` |
//! | `im.message.reaction.deleted_v1` | `register_im_message_reaction_deleted_v1` |
//! | `contact.user.created_v3` | `register_contact_user_created_v3` |
//! | `contact.user.updated_v3` | `register_contact_user_updated_v3` |
//! | `contact.user.deleted_v3` | `register_contact_user_deleted_v3` |
//! | `contact.department.created_v3` | `register_contact_department_created_v3` |
//! | `contact.department.updated_v3` | `register_contact_department_updated_v3` |
//! | `contact.department.deleted_v3` | `register_contact_department_deleted_v3` |
//! | `contact.custom_attr_event.updated_v3` | `register_contact_custom_attr_event_updated_v3` |
//! | `contact.scope.updated_v3` | `register_contact_scope_updated_v3` |
//! | `contact.employee_type_enum.created_v3` | `register_contact_employee_type_enum_created_v3` |
//! | `contact.employee_type_enum.updated_v3` | `register_contact_employee_type_enum_updated_v3` |
//! | `contact.employee_type_enum.deleted_v3` | `register_contact_employee_type_enum_deleted_v3` |
//! | `contact.employee_type_enum.actived_v3` | `register_contact_employee_type_enum_actived_v3` |
//! | `contact.employee_type_enum.deactivated_v3` | `register_contact_employee_type_enum_deactivated_v3` |
//! | `approval.approval.updated_v4` | `register_approval_approval_updated_v4` |
//! | `approval.instance.status_changed_v4` | `register_approval_instance_status_changed_v4` |
//! | `approval.task.status_changed_v4` | `register_approval_task_status_changed_v4` |
//! | `calendar.calendar.changed_v4` | `register_calendar_calendar_changed_v4` |
//! | `calendar.calendar.acl.created_v4` | `register_calendar_calendar_acl_created_v4` |
//! | `calendar.calendar.acl.deleted_v4` | `register_calendar_calendar_acl_deleted_v4` |
//! | `calendar.calendar.event.changed_v4` | `register_calendar_calendar_event_changed_v4` |
//! | `task.task.updated_v1` | `register_task_task_updated_v1` |
//! | `task.task.update_tenant_v1` | `register_task_task_update_tenant_v1` |
//! | `task.task.comment.updated_v1` | `register_task_task_comment_updated_v1` |
//! | `task.task.update_user_access_v2` | `register_task_task_update_user_access_v2` |
//! | `drive.file.created_in_folder_v1` | `register_drive_file_created_in_folder_v1` |
//! | `drive.file.deleted_v1` | `register_drive_file_deleted_v1` |
//! | `drive.file.edit_v1` | `register_drive_file_edit_v1` |
//! | `drive.file.read_v1` | `register_drive_file_read_v1` |
//! | `drive.file.trashed_v1` | `register_drive_file_trashed_v1` |
//! | `drive.file.title_updated_v1` | `register_drive_file_title_updated_v1` |
//! | `drive.file.bitable_field_changed_v1` | `register_drive_file_bitable_field_changed_v1` |
//! | `drive.file.bitable_record_changed_v1` | `register_drive_file_bitable_record_changed_v1` |
//! | `drive.file.permission_member_added_v1` | `register_drive_file_permission_member_added_v1` |
//! | `drive.file.permission_member_applied_v1` | `register_drive_file_permission_member_applied_v1` |
//! | `drive.file.permission_member_removed_v1` | `register_drive_file_permission_member_removed_v1` |
//! | `drive.notice.comment_add_v1` | `register_drive_notice_comment_add_v1` |
//! | `mail.user_mailbox.event.message_received_v1` | `register_mail_user_mailbox_event_message_received_v1` |
//! | `hire.application.deleted_v1` | `register_hire_application_deleted_v1` |
//! | `hire.application.stage_changed_v1` | `register_hire_application_stage_changed_v1` |
//! | `hire.eco_account.created_v1` | `register_hire_eco_account_created_v1` |
//! | `hire.eco_background_check.canceled_v1` | `register_hire_eco_background_check_canceled_v1` |
//! | `hire.eco_background_check.created_v1` | `register_hire_eco_background_check_created_v1` |
//! | `hire.eco_exam.created_v1` | `register_hire_eco_exam_created_v1` |
//! | `hire.ehr_import_task.imported_v1` | `register_hire_ehr_import_task_imported_v1` |
//! | `hire.ehr_import_task_for_internship_offer.imported_v1` | `register_hire_ehr_import_task_for_internship_offer_imported_v1` |
//! | `hire.offer.status_changed_v1` | `register_hire_offer_status_changed_v1` |
//! | `hire.referral_account.assets_update_v1` | `register_hire_referral_account_assets_update_v1` |
//! | `hire.talent.deleted_v1` | `register_hire_talent_deleted_v1` |
//! | `hire.talent.tag_subscription_v1` | `register_hire_talent_tag_subscription_v1` |
//! | `vc.bot.meeting_activity_v1` | `register_vc_bot_meeting_activity_v1` |
//! | `vc.bot.meeting_ended_v1` | `register_vc_bot_meeting_ended_v1` |
//! | `vc.bot.meeting_invited_v1` | `register_vc_bot_meeting_invited_v1` |
//! | `vc.bot.meeting_started_v1` | `register_vc_bot_meeting_started_v1` |
//! | `vc.meeting.all_meeting_ended_v1` | `register_vc_meeting_all_meeting_ended_v1` |
//! | `vc.meeting.all_meeting_started_v1` | `register_vc_meeting_all_meeting_started_v1` |
//! | `vc.meeting.join_meeting_v1` | `register_vc_meeting_join_meeting_v1` |
//! | `vc.meeting.leave_meeting_v1` | `register_vc_meeting_leave_meeting_v1` |
//! | `vc.meeting.meeting_ended_v1` | `register_vc_meeting_meeting_ended_v1` |
//! | `vc.meeting.meeting_started_v1` | `register_vc_meeting_meeting_started_v1` |
//! | `vc.meeting.participant_meeting_ended_v1` | `register_vc_meeting_participant_meeting_ended_v1` |
//! | `vc.meeting.recording_ended_v1` | `register_vc_meeting_recording_ended_v1` |
//! | `vc.meeting.recording_ready_v1` | `register_vc_meeting_recording_ready_v1` |
//! | `vc.meeting.recording_started_v1` | `register_vc_meeting_recording_started_v1` |
//! | `vc.meeting.share_ended_v1` | `register_vc_meeting_share_ended_v1` |
//! | `vc.meeting.share_started_v1` | `register_vc_meeting_share_started_v1` |
//! | `vc.note.generated_v1` | `register_vc_note_generated_v1` |
//! | `vc.reserve_config.updated_v1` | `register_vc_reserve_config_updated_v1` |
//! | `vc.room.created_v1` | `register_vc_room_created_v1` |
//! | `vc.room.deleted_v1` | `register_vc_room_deleted_v1` |
//! | `vc.room.updated_v1` | `register_vc_room_updated_v1` |
//! | `vc.room_level.created_v1` | `register_vc_room_level_created_v1` |
//! | `vc.room_level.deleted_v1` | `register_vc_room_level_deleted_v1` |
//! | `vc.room_level.updated_v1` | `register_vc_room_level_updated_v1` |
//! | `corehr.common_data.id.user_mapping_changed_v1` | `register_corehr_common_data_id_user_mapping_changed_v1` |
//! | `corehr.common_data.meta_data.updated_v1` | `register_corehr_common_data_meta_data_updated_v1` |
//! | `corehr.contract.created_v1` | `register_corehr_contract_created_v1` |
//! | `corehr.contract.deleted_v1` | `register_corehr_contract_deleted_v1` |
//! | `corehr.contract.updated_v1` | `register_corehr_contract_updated_v1` |
//! | `corehr.department.created_v1` | `register_corehr_department_created_v1` |
//! | `corehr.department.deleted_v1` | `register_corehr_department_deleted_v1` |
//! | `corehr.department.updated_v1` | `register_corehr_department_updated_v1` |
//! | `corehr.employment.converted_v1` | `register_corehr_employment_converted_v1` |
//! | `corehr.employment.created_v1` | `register_corehr_employment_created_v1` |
//! | `corehr.employment.deleted_v1` | `register_corehr_employment_deleted_v1` |
//! | `corehr.employment.resigned_v1` | `register_corehr_employment_resigned_v1` |
//! | `corehr.employment.updated_v1` | `register_corehr_employment_updated_v1` |
//! | `corehr.job.created_v1` | `register_corehr_job_created_v1` |
//! | `corehr.job.deleted_v1` | `register_corehr_job_deleted_v1` |
//! | `corehr.job.updated_v1` | `register_corehr_job_updated_v1` |
//! | `corehr.job_change.updated_v1` | `register_corehr_job_change_updated_v1` |
//! | `corehr.job_data.changed_v1` | `register_corehr_job_data_changed_v1` |
//! | `corehr.job_data.created_v1` | `register_corehr_job_data_created_v1` |
//! | `corehr.job_data.deleted_v1` | `register_corehr_job_data_deleted_v1` |
//! | `corehr.job_data.employed_v1` | `register_corehr_job_data_employed_v1` |
//! | `corehr.job_data.updated_v1` | `register_corehr_job_data_updated_v1` |
//! | `corehr.offboarding.updated_v1` | `register_corehr_offboarding_updated_v1` |
//! | `corehr.org_role_authorization.updated_v1` | `register_corehr_org_role_authorization_updated_v1` |
//! | `corehr.person.created_v1` | `register_corehr_person_created_v1` |
//! | `corehr.person.deleted_v1` | `register_corehr_person_deleted_v1` |
//! | `corehr.person.updated_v1` | `register_corehr_person_updated_v1` |
//! | `corehr.pre_hire.updated_v1` | `register_corehr_pre_hire_updated_v1` |
//! | `corehr.approval_groups.updated_v2` | `register_corehr_approval_groups_updated_v2` |
//! | `corehr.company.created_v2` | `register_corehr_company_created_v2` |
//! | `corehr.company.deleted_v2` | `register_corehr_company_deleted_v2` |
//! | `corehr.company.updated_v2` | `register_corehr_company_updated_v2` |
//! | `corehr.cost_center.created_v2` | `register_corehr_cost_center_created_v2` |
//! | `corehr.cost_center.deleted_v2` | `register_corehr_cost_center_deleted_v2` |
//! | `corehr.cost_center.updated_v2` | `register_corehr_cost_center_updated_v2` |
//! | `corehr.custom_org.created_v2` | `register_corehr_custom_org_created_v2` |
//! | `corehr.custom_org.deleted_v2` | `register_corehr_custom_org_deleted_v2` |
//! | `corehr.custom_org.updated_v2` | `register_corehr_custom_org_updated_v2` |
//! | `corehr.department.created_v2` | `register_corehr_department_created_v2` |
//! | `corehr.department.updated_v2` | `register_corehr_department_updated_v2` |
//! | `corehr.employee.domain_event_v2` | `register_corehr_employee_domain_event_v2` |
//! | `corehr.job_change.status_updated_v2` | `register_corehr_job_change_status_updated_v2` |
//! | `corehr.job_change.updated_v2` | `register_corehr_job_change_updated_v2` |
//! | `corehr.job_family.created_v2` | `register_corehr_job_family_created_v2` |
//! | `corehr.job_family.deleted_v2` | `register_corehr_job_family_deleted_v2` |
//! | `corehr.job_family.updated_v2` | `register_corehr_job_family_updated_v2` |
//! | `corehr.job_grade.created_v2` | `register_corehr_job_grade_created_v2` |
//! | `corehr.job_grade.deleted_v2` | `register_corehr_job_grade_deleted_v2` |
//! | `corehr.job_grade.updated_v2` | `register_corehr_job_grade_updated_v2` |
//! | `corehr.job_level.created_v2` | `register_corehr_job_level_created_v2` |
//! | `corehr.job_level.deleted_v2` | `register_corehr_job_level_deleted_v2` |
//! | `corehr.job_level.updated_v2` | `register_corehr_job_level_updated_v2` |
//! | `corehr.location.created_v2` | `register_corehr_location_created_v2` |
//! | `corehr.location.deleted_v2` | `register_corehr_location_deleted_v2` |
//! | `corehr.location.updated_v2` | `register_corehr_location_updated_v2` |
//! | `corehr.offboarding.checklist_updated_v2` | `register_corehr_offboarding_checklist_updated_v2` |
//! | `corehr.offboarding.status_updated_v2` | `register_corehr_offboarding_status_updated_v2` |
//! | `corehr.offboarding.updated_v2` | `register_corehr_offboarding_updated_v2` |
//! | `corehr.pathway.created_v2` | `register_corehr_pathway_created_v2` |
//! | `corehr.pathway.deleted_v2` | `register_corehr_pathway_deleted_v2` |
//! | `corehr.pathway.updated_v2` | `register_corehr_pathway_updated_v2` |
//! | `corehr.position.created_v2` | `register_corehr_position_created_v2` |
//! | `corehr.position.deleted_v2` | `register_corehr_position_deleted_v2` |
//! | `corehr.position.updated_v2` | `register_corehr_position_updated_v2` |
//! | `corehr.pre_hire.onboarding_task_changed_v2` | `register_corehr_pre_hire_onboarding_task_changed_v2` |
//! | `corehr.probation.updated_v2` | `register_corehr_probation_updated_v2` |
//! | `corehr.process.updated_v2` | `register_corehr_process_updated_v2` |
//! | `corehr.process.approver.updated_v2` | `register_corehr_process_approver_updated_v2` |
//! | `corehr.process.cc.updated_v2` | `register_corehr_process_cc_updated_v2` |
//! | `corehr.process.node.updated_v2` | `register_corehr_process_node_updated_v2` |
//! | `corehr.process.status.update_v2` | `register_corehr_process_status_update_v2` |
//! | `corehr.process_comment_info.updated_v2` | `register_corehr_process_comment_info_updated_v2` |
//! | `corehr.signature_file.status_updated_v2` | `register_corehr_signature_file_status_updated_v2` |
//!
//! <!-- typed-event-index-end -->

#[macro_use]
mod macros;

mod flex_time;

pub mod approval;
pub mod calendar;
pub mod contact;
pub mod corehr_v1;
pub mod corehr_v2;
pub mod drive;
pub mod hire;
pub mod im;
pub mod mail;
pub mod task;
pub mod vc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::serialization_error;

use super::{CallbackEventHandler, EventDispatcherHandler};

/// schema 2.0 事件头（高频字段）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct EventHeader {
    /// 事件 ID。
    #[serde(default)]
    pub event_id: String,
    /// 事件类型。
    #[serde(default)]
    pub event_type: String,
    /// 创建时间（毫秒）。JSON 字符串与整数都接受，存成十进制字符串。
    #[serde(default, deserialize_with = "flex_time::de_string")]
    pub create_time: String,
    /// verification token。
    #[serde(default)]
    pub token: String,
    /// 应用 ID。
    #[serde(default)]
    pub app_id: String,
    /// 租户 key。
    #[serde(default)]
    pub tenant_key: String,
}

/// `card.action.trigger` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardActionTrigger {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CardActionTriggerEvent,
}

/// 卡片回传事件体。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardActionTriggerEvent {
    /// 操作者。
    #[serde(default)]
    pub operator: Option<CardOperator>,
    /// 回传 token。
    #[serde(default)]
    pub token: String,
    /// 动作。
    #[serde(default)]
    pub action: Option<CardAction>,
    /// 宿主，例如 `im_message`。
    #[serde(default)]
    pub host: String,
    /// 卡片发送渠道，例如 `url_preview`。
    #[serde(default)]
    pub delivery_type: String,
    /// 上下文。
    #[serde(default)]
    pub context: Option<CardContext>,
}

/// 卡片操作者。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CardOperator {
    /// 租户 key。
    #[serde(default)]
    pub tenant_key: String,
    /// user_id。
    #[serde(default)]
    pub user_id: String,
    /// open_id。
    #[serde(default)]
    pub open_id: String,
}

/// 卡片点击动作。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardAction {
    /// 自定义 value。结构因卡片而异。
    #[serde(default)]
    pub value: Option<Value>,
    /// 组件 tag。
    #[serde(default)]
    pub tag: String,
    /// 时区。
    #[serde(default)]
    pub timezone: String,
    /// 组件 name。
    #[serde(default)]
    pub name: String,
    /// 表单值。
    #[serde(default)]
    pub form_value: Option<Value>,
    /// 输入值。
    #[serde(default)]
    pub input_value: String,
    /// 单选项。
    #[serde(default)]
    pub option: String,
    /// 多选项。
    #[serde(default)]
    pub options: Vec<String>,
    /// 是否勾选。
    #[serde(default)]
    pub checked: bool,
}

/// 卡片上下文。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct CardContext {
    /// 消息 ID。
    #[serde(default)]
    pub open_message_id: String,
    /// 会话 ID。
    #[serde(default)]
    pub open_chat_id: String,
    /// 预览 URL。
    #[serde(default)]
    pub url: String,
    /// 预览 token。
    #[serde(default)]
    pub preview_token: String,
}

/// 卡片 callback 业务响应（写入 ACK `data`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CardActionTriggerResponse {
    /// toast。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toast: Option<CardToast>,
    /// 更新后的卡片（`type` + `data`，对齐官方 Go `callback.Card`）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardActionCard>,
}

/// toast 提示。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CardToast {
    /// 类型，例如 `info` / `success`。
    #[serde(default)]
    pub r#type: String,
    /// 标题。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// ACK 中的卡片更新体。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardActionCard {
    /// `raw` / `template` / `card_json`。
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    /// 卡片数据。模板变量或卡片 JSON，形状因 `type` 而异。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// `card.action.trigger` typed 回调处理器。
///
/// 闭包只要签名匹配也会自动实现本 trait。
pub trait CardActionTriggerHandler: Send + Sync + 'static {
    /// 处理卡片回传，可选返回 toast / 新卡片。
    fn handle(
        &self,
        event: CardActionTrigger,
    ) -> Result<Option<CardActionTriggerResponse>, Box<dyn std::error::Error + Send + Sync>>;
}

impl<F> CardActionTriggerHandler for F
where
    F: Fn(
            CardActionTrigger,
        )
            -> Result<Option<CardActionTriggerResponse>, Box<dyn std::error::Error + Send + Sync>>
        + Send
        + Sync
        + 'static,
{
    fn handle(
        &self,
        event: CardActionTrigger,
    ) -> Result<Option<CardActionTriggerResponse>, Box<dyn std::error::Error + Send + Sync>> {
        self(event)
    }
}

struct CardActionTriggerAdapter<H>(H);

impl<H: CardActionTriggerHandler> CallbackEventHandler for CardActionTriggerAdapter<H> {
    fn handle(
        &self,
        payload: &[u8],
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        let event: CardActionTrigger = serde_json::from_slice(payload)
            .map_err(|e| serialization_error(format!("反序列化 card.action.trigger 失败: {e}")))?;
        match self.0.handle(event)? {
            Some(resp) => Ok(Some(serde_json::to_value(resp)?)),
            None => Ok(None),
        }
    }

    fn handle_from_value(
        &self,
        value: &Value,
        _payload: &[u8],
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        let event: CardActionTrigger = CardActionTrigger::deserialize(value)
            .map_err(|e| serialization_error(format!("反序列化 card.action.trigger 失败: {e}")))?;
        match self.0.handle(event)? {
            Some(resp) => Ok(Some(serde_json::to_value(resp)?)),
            None => Ok(None),
        }
    }
}

impl EventDispatcherHandler {
    /// 事件类型 `card.action.trigger`。
    pub const CARD_ACTION_TRIGGER: &'static str = "card.action.trigger";

    /// 注册 `card.action.trigger` typed 回调处理器（可回写 ACK）。
    ///
    /// 写入 callback map，命中后不再走 raw。与 [`Self::register_callback`] 使用同一 key。
    pub fn register_card_action_trigger<H>(self, handler: H) -> Result<Self, String>
    where
        H: CardActionTriggerHandler,
    {
        self.register_callback(Self::CARD_ACTION_TRIGGER, CardActionTriggerAdapter(handler))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    use crate::ws_client::EventHandler;

    const CARD_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"f7984f25108f8137722bb63c1d00bd823c2","event_type":"card.action.trigger","create_time":"1603977298000000","token":"tok","app_id":"cli_a511af62e2b5d07f","tenant_key":"736588c9260f175d"},
      "event":{"operator":{"tenant_key":"736588c9260f175d","user_id":"on_8f6f0d15799e5c45","open_id":"ou_4063d88c980c9f2d"},"token":"c-295eed59e6dbb014b72cba6f2ff6d48da9971e99","action":{"value":{"key":"value"},"tag":"button","timezone":"8","name":"btn","form_value":{},"input_value":"","option":"","options":[],"checked":false},"host":"im_message","context":{"open_message_id":"om_dc0d7ab6d7b734d5bff5c0556e8e7616","open_chat_id":"oc_4d83f1dc8596c773a09e86f50a931b77"}}
    }"#;

    struct ToastCard;
    impl CardActionTriggerHandler for ToastCard {
        fn handle(
            &self,
            event: CardActionTrigger,
        ) -> Result<Option<CardActionTriggerResponse>, Box<dyn std::error::Error + Send + Sync>>
        {
            assert_eq!(event.header.event_type, "card.action.trigger");
            let action = event.event.action.expect("action");
            assert_eq!(action.tag, "button");
            assert_eq!(action.name, "btn");
            Ok(Some(CardActionTriggerResponse {
                toast: Some(CardToast {
                    r#type: "info".into(),
                    title: Some("hi".into()),
                    content: None,
                }),
                card: None,
            }))
        }
    }

    #[test]
    fn typed_card_action_returns_toast_ack() {
        let handler = EventDispatcherHandler::builder()
            .register_card_action_trigger(ToastCard)
            .expect("register")
            .build();
        let ack = handler
            .dispatch_with_response(CARD_FIXTURE.as_bytes())
            .expect("dispatch")
            .expect("ack");
        let v: Value = serde_json::from_slice(&ack).expect("json");
        assert_eq!(v["toast"]["type"], "info");
        assert_eq!(v["toast"]["title"], "hi");
        assert!(v.get("card").is_none());
    }

    #[test]
    fn typed_card_short_circuits_raw_for_same_event_type() {
        struct Raw(Arc<Mutex<bool>>);
        impl EventHandler for Raw {
            fn handle(
                &self,
                _payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                *self.0.lock().expect("lock") = true;
                Ok(())
            }
        }
        let raw_seen = Arc::new(Mutex::new(false));
        let handler = EventDispatcherHandler::builder()
            .register_card_action_trigger(ToastCard)
            .expect("callback")
            .register_raw("card.action.trigger", Raw(Arc::clone(&raw_seen)))
            .expect("raw")
            .build();
        let ack = handler
            .dispatch_with_response(CARD_FIXTURE.as_bytes())
            .expect("dispatch")
            .expect("ack");
        let v: Value = serde_json::from_slice(&ack).expect("json");
        assert_eq!(v["toast"]["type"], "info");
        assert!(!*raw_seen.lock().expect("lock"));
    }

    #[test]
    fn typed_card_bad_json_errors() {
        let handler = EventDispatcherHandler::builder()
            .register_card_action_trigger(|_event: CardActionTrigger| Ok(None))
            .expect("register")
            .build();
        let err = handler
            .dispatch_with_response(br#"{"header":{"event_type":"card.action.trigger"}}"#)
            .expect_err("bad json");
        assert!(
            err.contains("card.action.trigger"),
            "error should name the event type, got {err}"
        );
    }
}

#[cfg(test)]
mod followup_tests {

    const COUNT_SENTENCE: &str =
        "共 159 个，含 main 原有的 `im.message.receive_v1`，其中 158 个是新增";

    fn doc_table_rows(src: &str) -> Vec<(String, String)> {
        let start = src
            .find("<!-- typed-event-index-start -->")
            .expect("table start");
        let end = src
            .find("<!-- typed-event-index-end -->")
            .expect("table end");
        let mut rows = Vec::new();
        for line in src[start..end].lines() {
            let line = line.trim().trim_start_matches("//!").trim();
            if !line.starts_with("| `") {
                continue;
            }
            let parts: Vec<&str> = line.split('`').collect();
            assert!(parts.len() >= 4, "对照表行无法解析: {line}");
            rows.push((parts[1].to_string(), parts[3].to_string()));
        }
        rows
    }

    fn source_registration_rows() -> Vec<(String, String)> {
        let mut rows = Vec::new();
        let sources = [
            include_str!("typed/im.rs"),
            include_str!("typed/contact.rs"),
            include_str!("typed/approval.rs"),
            include_str!("typed/calendar.rs"),
            include_str!("typed/task.rs"),
            include_str!("typed/drive.rs"),
            include_str!("typed/mail.rs"),
            include_str!("typed/hire.rs"),
            include_str!("typed/vc.rs"),
            include_str!("typed/corehr_v1.rs"),
            include_str!("typed/corehr_v2.rs"),
        ];
        for src in sources {
            for line in src.lines() {
                let parts: Vec<&str> = line.split('|').map(str::trim).collect();
                if parts.len() != 5 || !parts[4].starts_with("register_") {
                    continue;
                }
                let Some(event) = parts[0].strip_prefix('"').and_then(|s| s.strip_suffix('"'))
                else {
                    continue;
                };
                let method = parts[4].trim_end_matches(';');
                rows.push((event.to_string(), method.to_string()));
            }
        }
        rows
    }

    #[test]
    fn event_index_matches_all_159_registrations() {
        let documented = doc_table_rows(include_str!("typed.rs"));
        let implemented = source_registration_rows();
        assert_eq!(documented.len(), 159, "对照表行数");
        assert_eq!(implemented.len(), 159, "源码注册数");
        assert_eq!(documented, implemented);
        assert_eq!(
            documented
                .iter()
                .filter(|(event, _)| event == "im.message.receive_v1")
                .count(),
            1
        );
        assert!(
            documented
                .iter()
                .all(|(event, _)| event != "card.action.trigger")
        );
        let events: std::collections::BTreeSet<_> =
            documented.iter().map(|(event, _)| event.as_str()).collect();
        assert_eq!(events.len(), 159);
    }

    #[test]
    fn event_count_wording_matches_in_typed_and_lib() {
        let typed = include_str!("typed.rs");
        let lib = include_str!("../lib.rs");
        assert!(typed.contains(COUNT_SENTENCE), "typed.rs 计数口径");
        assert!(lib.contains(COUNT_SENTENCE), "lib.rs 计数口径");
        let added = [12, 13, 3, 4, 4, 12, 1, 12, 24, 28, 45];
        assert_eq!(added.iter().sum::<i32>(), 158);
        assert!(typed.contains("相加为 158"));
        assert!(lib.contains("相加为 158"));
    }

    #[test]
    fn not_recommended_events_document_official_replacements() {
        let v1 = include_str!("typed/corehr_v1.rs");
        assert!(v1.contains("corehr.department.created_v2"));
        assert!(v1.contains("register_corehr_department_created_v1"));
        assert!(v1.contains("corehr.department.updated_v2"));
        assert!(v1.contains("register_corehr_department_updated_v1"));
        assert!(v1.contains("corehr.job_change.status_updated_v2"));
        assert!(v1.contains("register_corehr_job_change_updated_v1"));
        assert!(v1.contains("corehr.offboarding.status_updated_v2"));
        assert!(v1.contains("register_corehr_offboarding_updated_v1"));

        let v2 = include_str!("typed/corehr_v2.rs");
        assert!(v2.contains("官方没有替代事件"));
        assert!(v2.contains("approval_group_status_v2"));
        assert!(v2.contains("register_corehr_approval_groups_updated_v2"));
    }
}
