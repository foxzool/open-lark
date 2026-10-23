//! drive 域 typed 事件（`drive.*`）。
//!
//! 字段权威：Go SDK v3.12.2 `service/drive/v1/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/drive/v1/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::im::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `drive.file.created_in_folder_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileCreatedInFolderV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFileCreatedInFolderV1Event,
}

/// `drive.file.created_in_folder_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileCreatedInFolderV1Event {
    /// 文件类型，与文件的 file_token 相匹配（如 `docx` / `sheet` / `bitable`）。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文件的 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 文件夹 token。
    #[serde(default)]
    pub folder_token: Option<String>,
    /// 操作者的用户 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 订阅者的用户 ID 列表。
    #[serde(default)]
    pub subscriber_ids: Vec<UserId>,
}

/// `drive.file.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFileDeletedV1Event,
}

/// `drive.file.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileDeletedV1Event {
    /// 文档类型。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文档 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 操作者 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

/// `drive.file.edit_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileEditV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFileEditV1Event,
}

/// `drive.file.edit_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileEditV1Event {
    /// 文档类型。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文档 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 操作者 ID 列表。
    #[serde(default)]
    pub operator_id_list: Vec<UserId>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
    /// 表格的子 sheet ID，当表格发生编辑时将会包含此字段。
    #[serde(default)]
    pub sheet_id: Option<String>,
}

/// `drive.file.read_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileReadV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFileReadV1Event,
}

/// `drive.file.read_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileReadV1Event {
    /// 文档类型。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文档 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 操作者 ID 列表。官方字段表亦写作 `operator_id`。
    #[serde(default, alias = "operator_id")]
    pub operator_id_list: Vec<UserId>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

/// `drive.file.trashed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileTrashedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFileTrashedV1Event,
}

/// `drive.file.trashed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileTrashedV1Event {
    /// 文档类型。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文档 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 操作者 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

/// `drive.file.title_updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileTitleUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFileTitleUpdatedV1Event,
}

/// `drive.file.title_updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileTitleUpdatedV1Event {
    /// 文档类型。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文档 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 操作者 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

/// `drive.file.bitable_field_changed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileBitableFieldChangedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFileBitableFieldChangedV1Event,
}

/// `drive.file.bitable_field_changed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileBitableFieldChangedV1Event {
    /// 云文档类型。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 多维表格 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 多维表格数据表 ID。
    #[serde(default)]
    pub table_id: Option<String>,
    /// 操作者 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 字段变更操作类型列表。
    #[serde(default)]
    pub action_list: Vec<BitableTableFieldAction>,
    /// 多维表格数据表的版本号。
    #[serde(default)]
    pub revision: Option<i64>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
    /// 字段变更时间。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_i64")]
    pub update_time: Option<i64>,
}

/// `drive.file.bitable_record_changed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileBitableRecordChangedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFileBitableRecordChangedV1Event,
}

/// `drive.file.bitable_record_changed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFileBitableRecordChangedV1Event {
    /// 文件类型，即 `bitable`。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 多维表格 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 发生变更的数据表 ID。
    #[serde(default)]
    pub table_id: Option<String>,
    /// 多维表格数据表的版本号。
    #[serde(default)]
    pub revision: Option<i64>,
    /// 操作人 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 行变更操作列表。
    #[serde(default)]
    pub action_list: Vec<BitableTableRecordAction>,
    /// 订阅的用户列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
    /// 编辑时间（时间戳，单位：秒）。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_i64")]
    pub update_time: Option<i64>,
}

/// `drive.file.permission_member_added_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFilePermissionMemberAddedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFilePermissionMemberAddedV1Event,
}

/// `drive.file.permission_member_added_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFilePermissionMemberAddedV1Event {
    /// 文档类型。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文档 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 操作者 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 添加的用户列表。
    #[serde(default)]
    pub user_list: Vec<UserId>,
    /// 群聊列表。
    #[serde(default)]
    pub chat_list: Vec<String>,
    /// 部门列表。
    #[serde(default)]
    pub open_department_id_list: Vec<String>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

/// `drive.file.permission_member_applied_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFilePermissionMemberAppliedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFilePermissionMemberAppliedV1Event,
}

/// `drive.file.permission_member_applied_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFilePermissionMemberAppliedV1Event {
    /// 文件对应的类型，与文件的 file_token 相匹配。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文件的 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 发起权限申请的操作人的 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 审批人 ID，一般是文件的所有者。
    #[serde(default)]
    pub approver_id: Option<UserId>,
    /// 申请授权的用户 ID 列表。
    #[serde(default)]
    pub application_user_list: Vec<UserId>,
    /// 申请授权的群 open_chat_id 列表。
    #[serde(default)]
    pub application_chat_list: Vec<String>,
    /// 申请授权的组织架构 open_department_id 列表。
    #[serde(default)]
    pub application_department_list: Vec<String>,
    /// 权限申请备注。
    #[serde(default)]
    pub application_remark: Option<String>,
    /// 申请的协作者权限。
    #[serde(default)]
    pub permission: Option<String>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_ids: Vec<UserId>,
}

/// `drive.file.permission_member_removed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFilePermissionMemberRemovedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveFilePermissionMemberRemovedV1Event,
}

/// `drive.file.permission_member_removed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveFilePermissionMemberRemovedV1Event {
    /// 文档类型。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文档 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 操作者 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 移除的用户列表。
    #[serde(default)]
    pub user_list: Vec<UserId>,
    /// 群聊列表。
    #[serde(default)]
    pub chat_list: Vec<String>,
    /// 部门列表。
    #[serde(default)]
    pub open_department_id_list: Vec<String>,
    /// 订阅用户 ID 列表。
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

/// `drive.notice.comment_add_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveNoticeCommentAddV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DriveNoticeCommentAddV1Event,
}

/// `drive.notice.comment_add_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriveNoticeCommentAddV1Event {
    /// 通知元信息。
    #[serde(default)]
    pub notice_meta: Option<Notice>,
    /// 评论 ID。
    #[serde(default)]
    pub comment_id: Option<String>,
    /// 回复 ID。
    #[serde(default)]
    pub reply_id: Option<String>,
    /// 接收者是否被 mention。
    #[serde(default)]
    pub is_mentioned: Option<bool>,
    /// 评论可见性。
    #[serde(default)]
    pub visibility: Option<String>,
}

/// 事件中的用户 ID 集合（Go `UserId`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserId {
    /// user_id。
    #[serde(default)]
    pub user_id: Option<String>,
    /// open_id。
    #[serde(default)]
    pub open_id: Option<String>,
    /// union_id。
    #[serde(default)]
    pub union_id: Option<String>,
}

/// 多维表格字段变更操作（Go `BitableTableFieldAction`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableFieldAction {
    /// 字段变更类型：`field_added` / `field_edited` / `field_deleted`。
    #[serde(default)]
    pub action: Option<String>,
    /// 字段 ID。
    #[serde(default)]
    pub field_id: Option<String>,
    /// 操作前的字段值。
    #[serde(default)]
    pub before_value: Option<BitableTableFieldActionValue>,
    /// 操作后的字段值。
    #[serde(default)]
    pub after_value: Option<BitableTableFieldActionValue>,
}

/// 多维表格字段值（Go `BitableTableFieldActionValue`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableFieldActionValue {
    /// 字段 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 字段名字。
    #[serde(default)]
    pub name: Option<String>,
    /// 字段类型。
    #[serde(default)]
    pub r#type: Option<i64>,
    /// 字段描述。
    #[serde(default)]
    pub description: Option<String>,
    /// 字段属性。
    #[serde(default)]
    pub property: Option<BitableTableFieldActionValueProperty>,
}

/// 多维表格字段属性（Go `BitableTableFieldActionValueProperty`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableFieldActionValueProperty {
    /// 数字、公式字段的显示格式。
    #[serde(default)]
    pub formatter: Option<String>,
    /// 日期、创建时间、最后更新时间字段的显示格式。
    #[serde(default)]
    pub date_formatter: Option<String>,
    /// 日期字段中新纪录自动填写创建时间。
    #[serde(default)]
    pub auto_fill: Option<bool>,
    /// 人员字段中允许添加多个成员。
    #[serde(default)]
    pub multiple: Option<bool>,
    /// 单向、双向关联字段中关联的数据表 ID。
    #[serde(default)]
    pub table_id: Option<String>,
    /// 单向、双向关联字段中关联的数据表名字。
    #[serde(default)]
    pub table_name: Option<String>,
    /// 双向关联字段中对应的双向关联字段名字。
    #[serde(default)]
    pub back_field_name: Option<String>,
    /// 地理位置输入限制。
    #[serde(default)]
    pub input_type: Option<String>,
    /// 双向关联字段中对应的双向关联字段 ID。
    #[serde(default)]
    pub back_field_id: Option<String>,
    /// 自动编号类型。
    #[serde(default)]
    pub auto_serial: Option<BitableTableFieldActionValuePropertyAutoSerial>,
    /// 单选、多选字段的选项信息。
    #[serde(default)]
    pub options: Vec<BitableTableFieldActionValuePropertyOption>,
    /// 公式字段的公式表达式。
    #[serde(default)]
    pub formula_expression: Option<String>,
}

/// 自动编号（Go `BitableTableFieldActionValuePropertyAutoSerial`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableFieldActionValuePropertyAutoSerial {
    /// 自动编号类型。
    #[serde(default)]
    pub r#type: Option<String>,
    /// 自动编号规则列表。
    #[serde(default)]
    pub options: Vec<BitableTableFieldActionValuePropertyAutoSerialOptions>,
}

/// 自动编号规则项（Go `BitableTableFieldActionValuePropertyAutoSerialOptions`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableFieldActionValuePropertyAutoSerialOptions {
    /// 自动编号的可选规则项类型。
    #[serde(default)]
    pub r#type: Option<String>,
    /// 与类型相对应的取值。
    #[serde(default)]
    pub value: Option<String>,
}

/// 单选、多选选项（Go `BitableTableFieldActionValuePropertyOption`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableFieldActionValuePropertyOption {
    /// 选项名。
    #[serde(default)]
    pub name: Option<String>,
    /// 选项 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 选项颜色。
    #[serde(default)]
    pub color: Option<i64>,
}

/// 多维表格行变更操作（Go `BitableTableRecordAction`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableRecordAction {
    /// 发生变更的记录 ID。
    #[serde(default)]
    pub record_id: Option<String>,
    /// 记录变更类型：`record_added` / `record_deleted` / `record_edited`。
    #[serde(default)]
    pub action: Option<String>,
    /// 发生变更前的记录值。
    #[serde(default)]
    pub before_value: Vec<BitableTableRecordActionField>,
    /// 发生变更后的字段。
    #[serde(default)]
    pub after_value: Vec<BitableTableRecordActionField>,
}

/// 多维表格行记录字段（Go `BitableTableRecordActionField`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableRecordActionField {
    /// 发生变更的字段 ID。
    #[serde(default)]
    pub field_id: Option<String>,
    /// 字段值。该字段为 JSON 序列化后的字符串。
    #[serde(default)]
    pub field_value: Option<String>,
    /// 人员字段补充信息。
    #[serde(default)]
    pub field_identity_value: Option<BitableTableRecordActionFieldIdentity>,
}

/// 人员字段补充信息（Go `BitableTableRecordActionFieldIdentity`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableRecordActionFieldIdentity {
    /// 用户列表。
    #[serde(default)]
    pub users: Vec<BitableTableRecordActionFieldIdentityUser>,
}

/// 人员字段用户信息（Go `BitableTableRecordActionFieldIdentityUser`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BitableTableRecordActionFieldIdentityUser {
    /// 用户的 ID 集合。
    #[serde(default)]
    pub user_id: Option<UserId>,
    /// 用户名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 用户英文名称。
    #[serde(default)]
    pub en_name: Option<String>,
    /// 用户头像 URL。
    #[serde(default)]
    pub avatar_url: Option<String>,
}

/// 通知元信息（Go `Notice`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Notice {
    /// 文档类型。
    #[serde(default)]
    pub file_type: Option<String>,
    /// 文档 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 发送者 ID。
    #[serde(default)]
    pub from_user_id: Option<UserId>,
    /// 接收者 ID。
    #[serde(default)]
    pub to_user_id: Option<UserId>,
    /// 发送者类型。
    #[serde(default)]
    pub from_user_type: Option<String>,
    /// 评论操作类型：`add_comment` / `add_reply`。
    #[serde(default)]
    pub notice_type: Option<String>,
}

typed_events! {
    /// `drive.file.created_in_folder_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.created_in_folder_v1" | DRIVE_FILE_CREATED_IN_FOLDER_V1 | DriveFileCreatedInFolderV1 | DriveFileCreatedInFolderV1Handler | register_drive_file_created_in_folder_v1;

    /// `drive.file.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.deleted_v1" | DRIVE_FILE_DELETED_V1 | DriveFileDeletedV1 | DriveFileDeletedV1Handler | register_drive_file_deleted_v1;

    /// `drive.file.edit_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.edit_v1" | DRIVE_FILE_EDIT_V1 | DriveFileEditV1 | DriveFileEditV1Handler | register_drive_file_edit_v1;

    /// `drive.file.read_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.read_v1" | DRIVE_FILE_READ_V1 | DriveFileReadV1 | DriveFileReadV1Handler | register_drive_file_read_v1;

    /// `drive.file.trashed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.trashed_v1" | DRIVE_FILE_TRASHED_V1 | DriveFileTrashedV1 | DriveFileTrashedV1Handler | register_drive_file_trashed_v1;

    /// `drive.file.title_updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.title_updated_v1" | DRIVE_FILE_TITLE_UPDATED_V1 | DriveFileTitleUpdatedV1 | DriveFileTitleUpdatedV1Handler | register_drive_file_title_updated_v1;

    /// `drive.file.bitable_field_changed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.bitable_field_changed_v1" | DRIVE_FILE_BITABLE_FIELD_CHANGED_V1 | DriveFileBitableFieldChangedV1 | DriveFileBitableFieldChangedV1Handler | register_drive_file_bitable_field_changed_v1;

    /// `drive.file.bitable_record_changed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.bitable_record_changed_v1" | DRIVE_FILE_BITABLE_RECORD_CHANGED_V1 | DriveFileBitableRecordChangedV1 | DriveFileBitableRecordChangedV1Handler | register_drive_file_bitable_record_changed_v1;

    /// `drive.file.permission_member_added_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.permission_member_added_v1" | DRIVE_FILE_PERMISSION_MEMBER_ADDED_V1 | DriveFilePermissionMemberAddedV1 | DriveFilePermissionMemberAddedV1Handler | register_drive_file_permission_member_added_v1;

    /// `drive.file.permission_member_applied_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.permission_member_applied_v1" | DRIVE_FILE_PERMISSION_MEMBER_APPLIED_V1 | DriveFilePermissionMemberAppliedV1 | DriveFilePermissionMemberAppliedV1Handler | register_drive_file_permission_member_applied_v1;

    /// `drive.file.permission_member_removed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.file.permission_member_removed_v1" | DRIVE_FILE_PERMISSION_MEMBER_REMOVED_V1 | DriveFilePermissionMemberRemovedV1 | DriveFilePermissionMemberRemovedV1Handler | register_drive_file_permission_member_removed_v1;

    /// `drive.notice.comment_add_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "drive.notice.comment_add_v1" | DRIVE_NOTICE_COMMENT_ADD_V1 | DriveNoticeCommentAddV1 | DriveNoticeCommentAddV1Handler | register_drive_notice_comment_add_v1;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const FILE_CREATED_IN_FOLDER_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d1","event_type":"drive.file.created_in_folder_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"docx",
        "file_token":"docxnBKgoMyY5OMbUG6FioTXuBe",
        "folder_token":"fldcnUUUUUUUUUUUUUU",
        "operator_id":{"open_id":"on_op"},
        "subscriber_ids":[{"open_id":"on_sub"}]
      }
    }"#;

    const FILE_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d2","event_type":"drive.file.deleted_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"docx",
        "file_token":"docxnDeleted",
        "operator_id":{"open_id":"on_op"},
        "subscriber_id_list":[{"open_id":"on_sub"}]
      }
    }"#;

    const FILE_EDIT_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d3","event_type":"drive.file.edit_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"sheet",
        "file_token":"shtcnEdited",
        "operator_id_list":[{"open_id":"on_e1"},{"open_id":"on_e2"}],
        "subscriber_id_list":[{"open_id":"on_sub"}],
        "sheet_id":"sheet_id_1"
      }
    }"#;

    const FILE_READ_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d4","event_type":"drive.file.read_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"docx",
        "file_token":"docxnRead",
        "operator_id_list":[{"open_id":"on_r1"},{"open_id":"on_r2"}],
        "subscriber_id_list":[{"open_id":"on_sub"}]
      }
    }"#;

    const FILE_TRASHED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d5","event_type":"drive.file.trashed_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"file",
        "file_token":"boxcnTrashed",
        "operator_id":{"open_id":"on_op"},
        "subscriber_id_list":[{"open_id":"on_sub"}]
      }
    }"#;

    const FILE_TITLE_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d6","event_type":"drive.file.title_updated_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"docx",
        "file_token":"docxnRenamed",
        "operator_id":{"open_id":"on_op"},
        "subscriber_id_list":[{"open_id":"on_sub"}]
      }
    }"#;

    const FILE_BITABLE_FIELD_CHANGED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d7","event_type":"drive.file.bitable_field_changed_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"bitable",
        "file_token":"bascnField",
        "table_id":"tbl_1",
        "operator_id":{"open_id":"on_op"},
        "action_list":[{
          "action":"field_edited",
          "field_id":"fld_1",
          "before_value":{"id":"fld_1","name":"旧名","type":1},
          "after_value":{
            "id":"fld_1","name":"新名","type":1,"description":"描述",
            "property":{"formatter":"0","auto_fill":true,"multiple":false,"table_id":"tbl_2","table_name":"关联表","back_field_name":"反向","input_type":"1","back_field_id":"fld_b","formula_expression":"1+1","auto_serial":{"type":"custom","options":[{"type":"seed","value":"1"}]},"options":[{"name":"选项A","id":"opt_1","color":1}]}
          }
        }],
        "revision":3,
        "subscriber_id_list":[{"open_id":"on_sub"}],
        "update_time":1700000000
      }
    }"#;

    const FILE_BITABLE_RECORD_CHANGED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d8","event_type":"drive.file.bitable_record_changed_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"bitable",
        "file_token":"bascnRecord",
        "table_id":"tbl_1",
        "revision":7,
        "operator_id":{"open_id":"on_op"},
        "action_list":[{
          "record_id":"rec_1",
          "action":"record_edited",
          "before_value":[{"field_id":"fld_1","field_value":"{}"}],
          "after_value":[{"field_id":"fld_1","field_value":"{\"text\":\"hi\"}","field_identity_value":{"users":[{"user_id":{"open_id":"on_fu"},"name":"张三","en_name":"San","avatar_url":"https://a.example/1.png"}]}}]
        }],
        "subscriber_id_list":[{"open_id":"on_sub"}],
        "update_time":1700000001
      }
    }"#;

    const FILE_PERMISSION_MEMBER_ADDED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d9","event_type":"drive.file.permission_member_added_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"docx",
        "file_token":"docxnPerm",
        "operator_id":{"open_id":"on_op"},
        "user_list":[{"open_id":"on_new1"},{"open_id":"on_new2"}],
        "chat_list":["oc_1"],
        "open_department_id_list":["od_1"],
        "subscriber_id_list":[{"open_id":"on_sub"}]
      }
    }"#;

    const FILE_PERMISSION_MEMBER_APPLIED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d10","event_type":"drive.file.permission_member_applied_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"docx",
        "file_token":"docxnApply",
        "operator_id":{"open_id":"on_applicant"},
        "approver_id":{"open_id":"on_owner"},
        "application_user_list":[{"open_id":"on_applicant"}],
        "application_chat_list":["oc_1"],
        "application_department_list":["od_1"],
        "application_remark":"申请理由",
        "permission":"view",
        "subscriber_ids":[{"open_id":"on_sub"}]
      }
    }"#;

    const FILE_PERMISSION_MEMBER_REMOVED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d11","event_type":"drive.file.permission_member_removed_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "file_type":"docx",
        "file_token":"docxnPermRm",
        "operator_id":{"open_id":"on_op"},
        "user_list":[{"open_id":"on_rm"}],
        "chat_list":["oc_2"],
        "open_department_id_list":["od_2"],
        "subscriber_id_list":[{"open_id":"on_sub"}]
      }
    }"#;

    const NOTICE_COMMENT_ADD_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"d12","event_type":"drive.notice.comment_add_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "notice_meta":{"file_type":"docx","file_token":"docxnCm","from_user_id":{"open_id":"on_author"},"to_user_id":{"open_id":"on_receiver"},"from_user_type":"user","notice_type":"add_comment"},
        "comment_id":"cm_1",
        "reply_id":"rp_1",
        "is_mentioned":true,
        "visibility":"only_me"
      }
    }"#;

    #[test]
    fn typed_drive_file_created_in_folder_deserializes_folder() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_created_in_folder_v1(move |event: DriveFileCreatedInFolderV1| {
                assert_eq!(event.header.event_type, "drive.file.created_in_folder_v1");
                assert_eq!(event.event.file_type.as_deref(), Some("docx"));
                assert_eq!(
                    event
                        .event
                        .operator_id
                        .as_ref()
                        .and_then(|u| u.open_id.as_deref()),
                    Some("on_op")
                );
                assert_eq!(
                    event
                        .event
                        .subscriber_ids
                        .first()
                        .and_then(|u| u.open_id.as_deref()),
                    Some("on_sub")
                );
                *seen_h.lock().expect("lock") = event.event.folder_token.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_CREATED_IN_FOLDER_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(
            seen.lock().expect("lock").as_deref(),
            Some("fldcnUUUUUUUUUUUUUU")
        );
    }

    #[test]
    fn typed_drive_file_deleted_deserializes_token() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_deleted_v1(move |event: DriveFileDeletedV1| {
                assert_eq!(event.event.file_type.as_deref(), Some("docx"));
                assert_eq!(
                    event
                        .event
                        .operator_id
                        .as_ref()
                        .and_then(|u| u.open_id.as_deref()),
                    Some("on_op")
                );
                *seen_h.lock().expect("lock") = event.event.file_token.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_DELETED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("docxnDeleted"));
    }

    #[test]
    fn typed_drive_file_edit_deserializes_sheet_id() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_edit_v1(move |event: DriveFileEditV1| {
                assert_eq!(event.event.operator_id_list.len(), 2);
                assert_eq!(
                    event.event.operator_id_list[1].open_id.as_deref(),
                    Some("on_e2")
                );
                *seen_h.lock().expect("lock") = event.event.sheet_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_EDIT_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("sheet_id_1"));
    }

    #[test]
    fn typed_drive_file_read_deserializes_operators() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_read_v1(move |event: DriveFileReadV1| {
                assert_eq!(event.event.file_token.as_deref(), Some("docxnRead"));
                assert_eq!(event.event.operator_id_list.len(), 2);
                *seen_h.lock().expect("lock") = event.event.operator_id_list[0].open_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_READ_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("on_r1"));
    }

    #[test]
    fn typed_drive_file_trashed_deserializes_token() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_trashed_v1(move |event: DriveFileTrashedV1| {
                assert_eq!(event.event.file_type.as_deref(), Some("file"));
                *seen_h.lock().expect("lock") = event.event.file_token.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_TRASHED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("boxcnTrashed"));
    }

    #[test]
    fn typed_drive_file_title_updated_deserializes_token() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_title_updated_v1(move |event: DriveFileTitleUpdatedV1| {
                assert_eq!(event.event.file_type.as_deref(), Some("docx"));
                *seen_h.lock().expect("lock") = event.event.file_token.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_TITLE_UPDATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("docxnRenamed"));
    }

    #[test]
    fn typed_drive_file_bitable_field_changed_deserializes_action() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_bitable_field_changed_v1(
                move |event: DriveFileBitableFieldChangedV1| {
                    assert_eq!(event.event.table_id.as_deref(), Some("tbl_1"));
                    assert_eq!(event.event.revision, Some(3));
                    assert_eq!(event.event.update_time, Some(1700000000));
                    let action = event.event.action_list.first().expect("action_list");
                    assert_eq!(action.action.as_deref(), Some("field_edited"));
                    assert_eq!(action.field_id.as_deref(), Some("fld_1"));
                    assert_eq!(
                        action.before_value.as_ref().and_then(|v| v.name.as_deref()),
                        Some("旧名")
                    );
                    let after = action.after_value.as_ref().expect("after_value");
                    assert_eq!(after.name.as_deref(), Some("新名"));
                    let property = after.property.as_ref().expect("property");
                    assert_eq!(property.auto_fill, Some(true));
                    assert_eq!(property.table_name.as_deref(), Some("关联表"));
                    assert_eq!(
                        property
                            .auto_serial
                            .as_ref()
                            .and_then(|s| s.r#type.as_deref()),
                        Some("custom")
                    );
                    assert_eq!(
                        property
                            .auto_serial
                            .as_ref()
                            .and_then(|s| s.options.first())
                            .and_then(|o| o.value.as_deref()),
                        Some("1")
                    );
                    assert_eq!(
                        property.options.first().and_then(|o| o.name.as_deref()),
                        Some("选项A")
                    );
                    *seen_h.lock().expect("lock") = after.id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_BITABLE_FIELD_CHANGED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("fld_1"));
    }

    #[test]
    fn typed_drive_file_bitable_record_changed_deserializes_action() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_bitable_record_changed_v1(
                move |event: DriveFileBitableRecordChangedV1| {
                    assert_eq!(event.event.revision, Some(7));
                    let action = event.event.action_list.first().expect("action_list");
                    assert_eq!(action.action.as_deref(), Some("record_edited"));
                    assert_eq!(action.record_id.as_deref(), Some("rec_1"));
                    assert_eq!(
                        action
                            .before_value
                            .first()
                            .and_then(|f| f.field_id.as_deref()),
                        Some("fld_1")
                    );
                    let after = action.after_value.first().expect("after_value");
                    assert_eq!(after.field_value.as_deref(), Some("{\"text\":\"hi\"}"));
                    let identity = after.field_identity_value.as_ref().expect("identity");
                    let user = identity.users.first().expect("users");
                    assert_eq!(user.name.as_deref(), Some("张三"));
                    assert_eq!(
                        user.user_id.as_ref().and_then(|u| u.open_id.as_deref()),
                        Some("on_fu")
                    );
                    *seen_h.lock().expect("lock") = after.field_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_BITABLE_RECORD_CHANGED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("fld_1"));
    }

    #[test]
    fn typed_drive_file_permission_member_added_deserializes_members() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_permission_member_added_v1(
                move |event: DriveFilePermissionMemberAddedV1| {
                    assert_eq!(event.event.user_list.len(), 2);
                    assert_eq!(event.event.user_list[1].open_id.as_deref(), Some("on_new2"));
                    assert_eq!(event.event.chat_list, vec!["oc_1".to_string()]);
                    assert_eq!(
                        event.event.open_department_id_list,
                        vec!["od_1".to_string()]
                    );
                    *seen_h.lock().expect("lock") = event.event.file_token.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_PERMISSION_MEMBER_ADDED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("docxnPerm"));
    }

    #[test]
    fn typed_drive_file_permission_member_applied_deserializes_application() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_permission_member_applied_v1(
                move |event: DriveFilePermissionMemberAppliedV1| {
                    assert_eq!(
                        event
                            .event
                            .approver_id
                            .as_ref()
                            .and_then(|u| u.open_id.as_deref()),
                        Some("on_owner")
                    );
                    assert_eq!(
                        event
                            .event
                            .application_user_list
                            .first()
                            .and_then(|u| u.open_id.as_deref()),
                        Some("on_applicant")
                    );
                    assert_eq!(event.event.permission.as_deref(), Some("view"));
                    assert_eq!(event.event.application_remark.as_deref(), Some("申请理由"));
                    assert_eq!(event.event.application_chat_list, vec!["oc_1".to_string()]);
                    *seen_h.lock().expect("lock") = event.event.file_token.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_PERMISSION_MEMBER_APPLIED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("docxnApply"));
    }

    #[test]
    fn typed_drive_file_permission_member_removed_deserializes_members() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_file_permission_member_removed_v1(
                move |event: DriveFilePermissionMemberRemovedV1| {
                    assert_eq!(
                        event
                            .event
                            .user_list
                            .first()
                            .and_then(|u| u.open_id.as_deref()),
                        Some("on_rm")
                    );
                    assert_eq!(event.event.chat_list, vec!["oc_2".to_string()]);
                    *seen_h.lock().expect("lock") = event.event.file_token.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(FILE_PERMISSION_MEMBER_REMOVED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("docxnPermRm"));
    }

    #[test]
    fn typed_drive_notice_comment_add_deserializes_notice_meta() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_drive_notice_comment_add_v1(move |event: DriveNoticeCommentAddV1| {
                assert_eq!(event.header.event_type, "drive.notice.comment_add_v1");
                let meta = event.event.notice_meta.as_ref().expect("notice_meta");
                assert_eq!(meta.file_token.as_deref(), Some("docxnCm"));
                assert_eq!(meta.notice_type.as_deref(), Some("add_comment"));
                assert_eq!(
                    meta.from_user_id
                        .as_ref()
                        .and_then(|u| u.open_id.as_deref()),
                    Some("on_author")
                );
                assert_eq!(
                    meta.to_user_id.as_ref().and_then(|u| u.open_id.as_deref()),
                    Some("on_receiver")
                );
                assert_eq!(event.event.is_mentioned, Some(true));
                assert_eq!(event.event.visibility.as_deref(), Some("only_me"));
                assert_eq!(event.event.reply_id.as_deref(), Some("rp_1"));
                *seen_h.lock().expect("lock") = event.event.comment_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(NOTICE_COMMENT_ADD_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("cm_1"));
    }

    /// 事件串常量与票面清单逐字一致。
    #[test]
    fn typed_drive_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_CREATED_IN_FOLDER_V1,
            "drive.file.created_in_folder_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_DELETED_V1,
            "drive.file.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_EDIT_V1,
            "drive.file.edit_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_READ_V1,
            "drive.file.read_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_TRASHED_V1,
            "drive.file.trashed_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_TITLE_UPDATED_V1,
            "drive.file.title_updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_BITABLE_FIELD_CHANGED_V1,
            "drive.file.bitable_field_changed_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_BITABLE_RECORD_CHANGED_V1,
            "drive.file.bitable_record_changed_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_PERMISSION_MEMBER_ADDED_V1,
            "drive.file.permission_member_added_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_PERMISSION_MEMBER_APPLIED_V1,
            "drive.file.permission_member_applied_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_FILE_PERMISSION_MEMBER_REMOVED_V1,
            "drive.file.permission_member_removed_v1"
        );
        assert_eq!(
            EventDispatcherHandler::DRIVE_NOTICE_COMMENT_ADD_V1,
            "drive.notice.comment_add_v1"
        );
    }

    /// typed 注册后再 `register_raw` 同 key 返回 Err；反方向同理。
    #[test]
    fn typed_drive_duplicate_register_fails() {
        struct Noop;
        impl crate::ws_client::EventHandler for Noop {
            fn handle(
                &self,
                _payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                Ok(())
            }
        }
        let err = EventDispatcherHandler::builder()
            .register_drive_file_deleted_v1(|_event: DriveFileDeletedV1| Ok(()))
            .expect("first")
            .register_raw("drive.file.deleted_v1", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("drive.notice.comment_add_v1", Noop)
            .expect("raw first")
            .register_drive_notice_comment_add_v1(|_event: DriveNoticeCommentAddV1| Ok(()))
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }

    #[test]
    fn file_read_operator_id_alias_fills_operator_id_list() {
        let from_alias: DriveFileReadV1 = serde_json::from_str(
            r#"{
              "header":{"event_type":"drive.file.read_v1"},
              "event":{"operator_id":[{"open_id":"ou_alias"}]}
            }"#,
        )
        .expect("alias");
        assert_eq!(from_alias.event.operator_id_list.len(), 1);
        assert_eq!(
            from_alias.event.operator_id_list[0].open_id.as_deref(),
            Some("ou_alias")
        );

        let from_name: DriveFileReadV1 = serde_json::from_str(
            r#"{
              "header":{"event_type":"drive.file.read_v1"},
              "event":{"operator_id_list":[{"open_id":"ou_list"}]}
            }"#,
        )
        .expect("canonical");
        assert_eq!(
            from_name.event.operator_id_list[0].open_id.as_deref(),
            Some("ou_list")
        );
    }
}
