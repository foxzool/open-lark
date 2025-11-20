use serde::{Deserialize, Serialize};

use openlark_core::event::EventHandler};

#[derive(Clone, Debug)]
pub struct P2DriveFileDeletedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2DriveFileDeletedV1Data,
pub(crate) struct P2DriveFileDeletedV1ProcessorImpl<F>,
where
    F: Fn(P2DriveFileDeletedV1) + 'static,
{
    f: F,
impl<F> EventHandler for P2DriveFileDeletedV1ProcessorImpl<F>,
where
    F: Fn(P2DriveFileDeletedV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {,
let event: P2DriveFileDeletedV1 = serde_json::from_slice(payload)?;
        (self.f)(event);
Ok(()),
    }
impl<F> P2DriveFileDeletedV1ProcessorImpl<F>,
where
    F: Fn(P2DriveFileDeletedV1) + 'static,
{,
pub(crate) fn new(f: F) -> Self {
        P2DriveFileDeletedV1ProcessorImpl { f }
/// 云文档文件删除事件数据,
#[derive(Clone, Debug)]
pub struct P2DriveFileDeletedV1Data {
    /// 事件对象
    pub object: DriveFileEventObject,
    /// 删除前的文件信息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub old_object: Option<DriveFileEventObject>,
/// 云文档文件事件对象,
#[derive(Clone, Debug)]
pub struct DriveFileEventObject {
    /// 对象类型 (file)
    pub object_type: String,
    /// 文件信息
    pub file: DeletedDriveFile,
/// 被删除的云文档文件信息,
#[derive(Clone, Debug)]
pub struct DeletedDriveFile {
    /// 文件token
    pub file_token: String,
    /// 文件类型 (doc, sheet, bitable, mindnote, file, folder)
    pub file_type: String,
    /// 文件名
    pub name: String,
    /// 文件URL（删除时可能失效）,
#[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 文件大小（字节）,
#[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 文件MIME类型,
#[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// 父文件夹token,
#[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    /// 创建者用户ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// 删除者用户ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    /// 创建时间 (Unix时间戳，单位：秒),
#[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 删除时间 (Unix时间戳，单位：秒),
#[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_time: Option<String>,
    /// 删除信息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_info: Option<FileDeletionInfo>,
    /// 文件权限信息（删除前的快照）,
#[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DriveFilePermissions>,
    /// 是否在回收站（软删除）,
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_in_trash: Option<bool>,
    /// 是否是快捷方式,
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_shortcut: Option<bool>,
    /// 如果是快捷方式，指向的原文件token,
#[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_target_token: Option<String>,
/// 文件删除信息,
#[derive(Clone, Debug)]
pub struct FileDeletionInfo {
    /// 删除类型 (user_delete, auto_delete, system_delete, trash_delete)
    pub delete_type: String,
    /// 删除原因,
#[serde(skip_serializing_if = "Option::is_none")]
    pub delete_reason: Option<String>,
    /// 是否可恢复,
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_recoverable: Option<bool>,
    /// 如果在回收站，自动清理时间 (Unix时间戳，单位：秒),
#[serde(skip_serializing_if = "Option::is_none")]
    pub auto_clean_time: Option<String>,
    /// 备份信息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub backup_info: Option<FileBackupInfo>,
/// 文件备份信息,
#[derive(Clone, Debug)]
pub struct FileBackupInfo {
    /// 是否有备份
    pub has_backup: bool,
    /// 备份位置,
#[serde(skip_serializing_if = "Option::is_none")]
    pub backup_location: Option<String>,
    /// 备份过期时间 (Unix时间戳，单位：秒),
#[serde(skip_serializing_if = "Option::is_none")]
    pub backup_expire_time: Option<String>,
/// 文件权限信息,
#[derive(Clone, Debug)]
pub struct DriveFilePermissions {
    /// 是否可编辑,
#[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    /// 是否可查看,
#[serde(skip_serializing_if = "Option::is_none")]
    pub can_view: Option<bool>,
    /// 是否可分享,
#[serde(skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    /// 是否可复制,
#[serde(skip_serializing_if = "Option::is_none")]
    pub can_copy: Option<bool>,
    /// 是否可下载,
#[serde(skip_serializing_if = "Option::is_none")]
    pub can_download: Option<bool>,
