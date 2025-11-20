use serde::{Deserialize, Serialize};

use openlark_core::event::EventHandler};

#[derive(Clone, Debug)]
pub struct P2DriveFileUpdatedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2DriveFileUpdatedV1Data,
pub(crate) struct P2DriveFileUpdatedV1ProcessorImpl<F>,
where
    F: Fn(P2DriveFileUpdatedV1) + 'static,
{
    f: F,
impl<F> EventHandler for P2DriveFileUpdatedV1ProcessorImpl<F>,
where
    F: Fn(P2DriveFileUpdatedV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {,
let event: P2DriveFileUpdatedV1 = serde_json::from_slice(payload)?;
        (self.f)(event);
Ok(()),
    }
impl<F> P2DriveFileUpdatedV1ProcessorImpl<F>,
where
    F: Fn(P2DriveFileUpdatedV1) + 'static,
{,
pub(crate) fn new(f: F) -> Self {
        P2DriveFileUpdatedV1ProcessorImpl { f }
/// 云文档文件更新事件数据,
#[derive(Clone, Debug)]
pub struct P2DriveFileUpdatedV1Data {
    /// 事件对象
    pub object: DriveFileEventObject,
    /// 更新前的文件信息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub old_object: Option<DriveFileEventObject>,
/// 云文档文件事件对象,
#[derive(Clone, Debug)]
pub struct DriveFileEventObject {
    /// 对象类型 (file)
    pub object_type: String,
    /// 文件信息
    pub file: DriveFile,
/// 云文档文件信息,
#[derive(Clone, Debug)]
pub struct DriveFile {
    /// 文件token
    pub file_token: String,
    /// 文件类型 (doc, sheet, bitable, mindnote, file, folder)
    pub file_type: String,
    /// 文件名
    pub name: String,
    /// 文件URL,
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
    /// 最后修改者用户ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// 创建时间 (Unix时间戳，单位：秒),
#[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 修改时间 (Unix时间戳，单位：秒),
#[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 文件状态,
#[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DriveFileStatus>,
    /// 文件权限信息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DriveFilePermissions>,
    /// 版本信息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<FileVersion>,
    /// 更新类型,
#[serde(skip_serializing_if = "Option::is_none")]
    pub update_type: Option<FileUpdateType>,
    /// 是否是快捷方式,
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_shortcut: Option<bool>,
    /// 如果是快捷方式，指向的原文件token,
#[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_target_token: Option<String>,
/// 文件状态,
#[derive(Clone, Debug)]
pub struct DriveFileStatus {
    /// 是否被删除,
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// 是否在回收站,
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_in_trash: Option<bool>,
    /// 是否被锁定,
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
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
/// 文件版本信息,
#[derive(Clone, Debug)]
pub struct FileVersion {
    /// 版本号,
#[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 版本名称,
#[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    /// 是否为当前版本,
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_current: Option<bool>,
/// 文件更新类型,
#[derive(Clone, Debug)]
pub struct FileUpdateType {
    /// 更新类型 (content, metadata, permission, location)
    pub update_category: String,
    /// 具体更新字段列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub updated_fields: Option<Vec<String>>,
    /// 更新描述,
#[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
