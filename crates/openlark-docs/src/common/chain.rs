//! # openlark-docs 链式调用入口（简化为仅配置获取）
//!
//! ## 设计理念
//!
//! openlark-docs 涵盖多个 bizTag/Project（ccm/base/bitable/baike/minutes 等），
//! 提供简洁的配置获取入口，Request 构建仍使用各 `*RequestBuilder/*Request` 的 `new(config)` / `execute(...)`。
//!
//! ## 推荐入口
//!
//! **公开入口** (推荐用户使用):
//! - `DocsClient` - 文档服务的唯一公开入口
//! - 示例: `DocsClient::new(config).ccm.config().clone()` 用于获取配置
//!
//! ## 推荐调用方式
//!
//! ```rust,ignore
//! use openlark_core::config::Config;
//! use openlark_docs::DocsClient;
//!
//! // 创建客户端
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//! let docs = DocsClient::new(config);
//!
//! // ✅ 推荐：获取配置后构建 Request
//! // 访问云盘服务
//! let config = docs.ccm.config().clone();
//! // let file = UploadAllRequest::new(config, ...).execute().await?;
//!
//! // 访问多维表格
//! let config = docs.base.bitable.config().clone();
//! // let table = CreateTableRequest::new(config, ...).execute().await?;
//!
//! // 访问知识库
//! let config = docs.ccm.wiki.config().clone();
//! // let node = CreateNodeRequest::new(config, ...).execute().await?;
//! ```

use openlark_core::config::Config;
#[cfg(feature = "ccm-core")]
use openlark_core::error::{business_error, validation_error, CoreError};
#[cfg(any(feature = "ccm-core", feature = "bitable"))]
use openlark_core::SDKResult;
use std::sync::Arc;

/// 统一的 typed pagination 返回页。
///
/// 相比直接暴露各 API 的原始分页字段，该结构统一使用 `next_page_token` 命名，
/// 方便后续在 Drive / Docs helper 中复用同一套分页范式。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedPage<T> {
    /// 当前页结果项。
    pub items: Vec<T>,
    /// 是否还有下一页。
    pub has_more: bool,
    /// 下一页分页标记。
    pub next_page_token: Option<String>,
}

impl<T> TypedPage<T> {
    pub fn new(items: Vec<T>, has_more: bool, next_page_token: Option<String>) -> Self {
        Self {
            items,
            has_more,
            next_page_token,
        }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new(), false, None)
    }

    pub fn is_last_page(&self) -> bool {
        !self.has_more
    }

    pub fn into_items(self) -> Vec<T> {
        self.items
    }
}

#[cfg(feature = "ccm-core")]
pub type FolderChildrenPage = TypedPage<crate::ccm::explorer::v2::models::FileItem>;

/// 电子表格范围 helper。
///
/// 统一 sheet 标识与 A1 范围表达，避免业务侧手工拼接
/// `sheet_id!A1:C5` 之类的字符串。
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetRange {
    /// 工作表标识。
    pub sheet_id: String,
    /// 起始单元格。
    pub start_cell: String,
    /// 结束单元格；为空时表示单格或单起点范围。
    pub end_cell: Option<String>,
}

#[cfg(feature = "ccm-core")]
impl SheetRange {
    /// 从工作表 ID + 起始单元格创建范围。
    pub fn new(sheet_id: impl Into<String>, start_cell: impl Into<String>) -> Self {
        Self {
            sheet_id: sheet_id.into(),
            start_cell: start_cell.into(),
            end_cell: None,
        }
    }

    /// 补充结束单元格，形成 `A1:C5` 这类闭区间范围。
    pub fn with_end_cell(mut self, end_cell: impl Into<String>) -> Self {
        self.end_cell = Some(end_cell.into());
        self
    }

    /// 从工作表 ID 与相对范围表达式创建范围。
    ///
    /// `range_expr` 仅应包含单元格部分，例如 `A1` 或 `A1:C5`。
    pub fn from_range_expr(
        sheet_id: impl Into<String>,
        range_expr: impl AsRef<str>,
    ) -> SDKResult<Self> {
        let sheet_id = validate_sheet_range_part("sheet_id", sheet_id.into())?;
        let expr = range_expr.as_ref().trim();
        if expr.is_empty() {
            return Err(validation_error("range_expr", "range_expr 不能为空"));
        }
        if expr.contains('!') {
            return Err(validation_error(
                "range_expr",
                "range_expr 不应包含工作表前缀，请仅传入单元格范围",
            ));
        }

        match expr.split_once(':') {
            Some((start, end)) => Ok(Self::new(
                sheet_id,
                validate_sheet_range_part("start_cell", start)?,
            )
            .with_end_cell(validate_sheet_range_part("end_cell", end)?)),
            None => Ok(Self::new(
                sheet_id,
                validate_sheet_range_part("start_cell", expr)?,
            )),
        }
    }

    /// 解析完整的 A1 表达式，例如 `sheet_id!A1:C5`。
    pub fn parse(a1_notation: impl AsRef<str>) -> SDKResult<Self> {
        let notation = a1_notation.as_ref().trim();
        let (sheet_id, range_expr) = notation.split_once('!').ok_or_else(|| {
            validation_error(
                "a1_notation",
                "A1 表达式必须包含工作表前缀，例如 sheet_id!A1:C5",
            )
        })?;

        Self::from_range_expr(sheet_id, range_expr)
    }

    /// 返回不带工作表前缀的范围表达式。
    pub fn range_expr(&self) -> String {
        match &self.end_cell {
            Some(end_cell) => format!("{}:{}", self.start_cell, end_cell),
            None => self.start_cell.clone(),
        }
    }

    /// 返回完整的 A1 表达式。
    pub fn to_a1_notation(&self) -> String {
        format!("{}!{}", self.sheet_id, self.range_expr())
    }
}

#[cfg(feature = "ccm-core")]
impl std::fmt::Display for SheetRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_a1_notation())
    }
}

#[cfg(feature = "ccm-core")]
impl From<crate::ccm::explorer::v2::models::FolderChildrenData>
    for TypedPage<crate::ccm::explorer::v2::models::FileItem>
{
    fn from(data: crate::ccm::explorer::v2::models::FolderChildrenData) -> Self {
        Self::new(data.items, data.has_more, data.page_token)
    }
}

/// 文件夹子项分页 helper。
///
/// 用于按页读取 Drive Explorer 文件夹内容，并统一分页返回形态。
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone)]
pub struct FolderChildrenPager {
    config: Arc<Config>,
    folder_token: String,
    doc_type: Option<String>,
    page_size: i32,
    next_page_token: Option<String>,
    exhausted: bool,
}

#[cfg(feature = "ccm-core")]
impl FolderChildrenPager {
    fn new(config: Arc<Config>, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            doc_type: None,
            page_size: crate::common::constants::DEFAULT_PAGE_SIZE,
            next_page_token: None,
            exhausted: false,
        }
    }

    /// 设置文件类型过滤。
    pub fn doc_type(mut self, doc_type: impl Into<String>) -> Self {
        self.doc_type = Some(doc_type.into());
        self
    }

    /// 设置分页大小，自动限制在 1..=MAX_PAGE_SIZE 范围内。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = page_size.clamp(1, crate::common::constants::MAX_PAGE_SIZE);
        self
    }

    /// 从指定分页 token 恢复读取。
    pub fn next_page_token(mut self, next_page_token: impl Into<String>) -> Self {
        self.next_page_token = Some(next_page_token.into());
        self
    }

    /// 查看当前即将请求的下一页 token。
    pub fn pending_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    /// 读取下一页结果。
    pub async fn fetch_next_page(&mut self) -> SDKResult<FolderChildrenPage> {
        use crate::ccm::explorer::v2::{get_folder_children, GetFolderChildrenParams};

        if self.exhausted {
            return Ok(TypedPage::empty());
        }

        let response = get_folder_children(
            self.config.as_ref(),
            &self.folder_token,
            Some(GetFolderChildrenParams {
                page_size: Some(self.page_size),
                page_token: self.next_page_token.clone(),
                doc_type: self.doc_type.clone(),
            }),
        )
        .await?;

        let page = response
            .data
            .map(TypedPage::from)
            .unwrap_or_else(TypedPage::empty);
        self.exhausted = !page.has_more;
        self.next_page_token = if page.has_more {
            page.next_page_token.clone()
        } else {
            None
        };

        Ok(page)
    }

    /// 收集当前 pager 剩余的所有结果。
    pub async fn collect_all(
        mut self,
    ) -> SDKResult<Vec<crate::ccm::explorer::v2::models::FileItem>> {
        let mut items = Vec::new();

        loop {
            let page = self.fetch_next_page().await?;
            let is_last_page = page.is_last_page();
            items.extend(page.into_items());
            if is_last_page {
                break;
            }
        }

        Ok(items)
    }
}

/// Docs 链式入口：`docs.ccm.config()` / `docs.base.bitable.config()`（按 feature 裁剪）
#[derive(Debug, Clone)]
pub struct DocsClient {
    config: Arc<Config>,

    #[cfg(feature = "ccm-core")]
    pub ccm: CcmClient,

    #[cfg(any(feature = "base", feature = "bitable"))]
    pub base: BaseClient,

    #[cfg(any(feature = "baike", feature = "lingo"))]
    pub baike: BaikeClient,

    #[cfg(feature = "minutes")]
    pub minutes: MinutesClient,
}

impl DocsClient {
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        Self {
            config: config.clone(),
            #[cfg(feature = "ccm-core")]
            ccm: CcmClient::new(config.clone()),
            #[cfg(any(feature = "base", feature = "bitable"))]
            base: BaseClient::new(config.clone()),
            #[cfg(any(feature = "baike", feature = "lingo"))]
            baike: BaikeClient::new(config.clone()),
            #[cfg(feature = "minutes")]
            minutes: MinutesClient::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建文件夹子项分页 helper。
    #[cfg(feature = "ccm-core")]
    pub fn folder_children_pager(&self, folder_token: impl Into<String>) -> FolderChildrenPager {
        FolderChildrenPager::new(self.config.clone(), folder_token)
    }

    /// 获取文件夹下的全部子项，自动处理分页。
    #[cfg(feature = "ccm-core")]
    pub async fn list_folder_children_all(
        &self,
        folder_token: &str,
        doc_type: Option<&str>,
    ) -> SDKResult<Vec<crate::ccm::explorer::v2::models::FileItem>> {
        let mut pager = self
            .folder_children_pager(folder_token)
            .page_size(crate::common::constants::MAX_PAGE_SIZE);
        if let Some(doc_type) = doc_type {
            pager = pager.doc_type(doc_type);
        }

        pager.collect_all().await
    }

    /// 读取多维表格全部记录，自动处理分页。
    #[cfg(feature = "bitable")]
    pub async fn search_bitable_records_all(
        &self,
        app_token: &str,
        table_id: &str,
    ) -> SDKResult<Vec<crate::base::bitable::v1::app::table::record::models::Record>> {
        use crate::base::bitable::v1::app::table::record::search::SearchRecordRequest;

        SearchRecordRequest::new(self.config().clone())
            .app_token(app_token.to_string())
            .table_id(table_id.to_string())
            .automatic_fields(true)
            .fetch_all()
            .await
    }

    /// 读取多个单元格范围，返回聚合后的范围数据。
    #[cfg(feature = "ccm-core")]
    pub async fn read_multiple_ranges(
        &self,
        spreadsheet_token: &str,
        ranges: Vec<String>,
    ) -> SDKResult<crate::ccm::sheets_v2::v2::data_io::models::MultipleRangeData> {
        use crate::ccm::sheets_v2::v2::data_io::{
            read_multiple_ranges as read_multiple_ranges_api, ReadMultipleRangesParams,
        };

        let response = read_multiple_ranges_api(
            self.config(),
            spreadsheet_token,
            ReadMultipleRangesParams {
                ranges,
                value_render_option: None,
                date_render_option: None,
            },
        )
        .await?;

        response
            .data
            .ok_or_else(|| CoreError::api_data_error("读取多个范围"))
    }

    /// 批量写入多个单元格范围。
    #[cfg(feature = "ccm-core")]
    pub async fn write_multiple_ranges(
        &self,
        spreadsheet_token: &str,
        data: Vec<crate::ccm::sheets_v2::v2::data_io::models::BatchWriteData>,
    ) -> SDKResult<crate::ccm::sheets_v2::v2::data_io::models::BatchUpdateResult> {
        use crate::ccm::sheets_v2::v2::data_io::{batch_write_ranges, BatchWriteRangesParams};

        let response = batch_write_ranges(
            self.config(),
            spreadsheet_token,
            BatchWriteRangesParams {
                data,
                include_style: None,
            },
        )
        .await?;

        response
            .data
            .ok_or_else(|| CoreError::api_data_error("批量写入多个范围"))
    }

    /// 根据工作表标题查找工作表。
    #[cfg(feature = "ccm-core")]
    pub async fn find_sheet_by_title(
        &self,
        spreadsheet_token: &str,
        title: &str,
    ) -> SDKResult<crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo> {
        let sheets = self.list_sheet_infos(spreadsheet_token).await?;

        find_sheet_info(&sheets, title)
            .ok_or_else(|| business_error(format!("未找到工作表: {title}")))
    }

    /// 按工作表标题解析单个范围，返回统一的 `SheetRange` 表达。
    #[cfg(feature = "ccm-core")]
    pub async fn resolve_sheet_range_by_title(
        &self,
        spreadsheet_token: &str,
        title: &str,
        range_expr: &str,
    ) -> SDKResult<SheetRange> {
        let sheet = self.find_sheet_by_title(spreadsheet_token, title).await?;
        SheetRange::from_range_expr(sheet.sheet_id, range_expr)
    }

    #[cfg(feature = "ccm-core")]
    pub async fn list_sheet_infos(
        &self,
        spreadsheet_token: &str,
    ) -> SDKResult<Vec<crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo>> {
        use crate::ccm::sheets::v3::spreadsheet::sheet::query::query_sheets;
        use crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo;

        log::info!(
            "[OPENLARK DEBUG] list_sheet_infos called with token: {}",
            spreadsheet_token
        );

        let response = query_sheets(self.config(), spreadsheet_token).await?;

        log::info!(
            "[OPENLARK DEBUG] query_sheets response count: {}",
            response.sheets.len()
        );

        let sheets: Vec<SpreadsheetSheetInfo> =
            response.sheets.into_iter().map(map_v3_sheet_info).collect();

        if sheets.is_empty() {
            return Err(CoreError::api_data_error("获取工作表列表"));
        }

        Ok(sheets)
    }
}

#[cfg(feature = "ccm-core")]
fn map_v3_sheet_info(
    sheet: crate::ccm::sheets::v3::spreadsheet::Sheet,
) -> crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo {
    crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo {
        sheet_id: sheet.sheet_id,
        title: sheet.title,
        sheet_type: sheet.resource_type,
        row_count: sheet.grid_properties.row_count,
        column_count: sheet.grid_properties.column_count,
    }
}

#[cfg(feature = "ccm-core")]
fn find_sheet_info(
    sheets: &[crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo],
    title: &str,
) -> Option<crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo> {
    sheets.iter().find(|sheet| sheet.title == title).cloned()
}

#[cfg(feature = "ccm-core")]
fn validate_sheet_range_part(field: &str, value: impl Into<String>) -> SDKResult<String> {
    let value = value.into().trim().to_string();
    if value.is_empty() {
        return Err(validation_error(field, &format!("{field} 不能为空")));
    }
    Ok(value)
}

/// ccm：`docs.ccm`（云文档协同）
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone)]
pub struct CcmClient {
    config: Arc<Config>,
}

#[cfg(feature = "ccm-core")]
impl CcmClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// base：`docs.base`（base/bitable 都归口在 base 模块下）
#[cfg(any(feature = "base", feature = "bitable"))]
#[derive(Debug, Clone)]
pub struct BaseClient {
    config: Arc<Config>,
}

#[cfg(any(feature = "base", feature = "bitable"))]
impl BaseClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    #[cfg(feature = "bitable")]
    pub fn bitable(&self) -> BitableClient {
        BitableClient::new(self.config.clone())
    }
}

/// bitable：`docs.base.bitable`（多维表格）
#[cfg(feature = "bitable")]
#[derive(Debug, Clone)]
pub struct BitableClient {
    config: Arc<Config>,
}

#[cfg(feature = "bitable")]
impl BitableClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// baike：`docs.baike`（baike/lingo 相关）
#[cfg(any(feature = "baike", feature = "lingo"))]
#[derive(Debug, Clone)]
pub struct BaikeClient {
    config: Arc<Config>,
}

#[cfg(any(feature = "baike", feature = "lingo"))]
impl BaikeClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// minutes：`docs.minutes`（会议纪要）
#[cfg(feature = "minutes")]
#[derive(Debug, Clone)]
pub struct MinutesClient {
    config: Arc<Config>,
}

#[cfg(feature = "minutes")]
impl MinutesClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }

    #[test]
    fn test_typed_page_last_page_state() {
        let page = TypedPage::new(vec![1, 2], false, None);
        assert!(page.is_last_page());
        assert_eq!(page.into_items(), vec![1, 2]);
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_sheet_range_builds_a1_notation() {
        let range = SheetRange::from_range_expr("sheet_001", "A1:C5").unwrap();

        assert_eq!(range.sheet_id, "sheet_001");
        assert_eq!(range.start_cell, "A1");
        assert_eq!(range.end_cell.as_deref(), Some("C5"));
        assert_eq!(range.to_string(), "sheet_001!A1:C5");
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_sheet_range_parses_single_cell_notation() {
        let range = SheetRange::parse("sheet_001!B2").unwrap();

        assert_eq!(range.sheet_id, "sheet_001");
        assert_eq!(range.start_cell, "B2");
        assert!(range.end_cell.is_none());
        assert_eq!(range.range_expr(), "B2");
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_sheet_range_rejects_embedded_sheet_prefix() {
        let error = SheetRange::from_range_expr("sheet_001", "sheet_002!A1:C5").unwrap_err();
        assert!(error.to_string().contains("range_expr"));
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_folder_children_page_maps_next_page_token() {
        let data = crate::ccm::explorer::v2::models::FolderChildrenData {
            items: vec![crate::ccm::explorer::v2::models::FileItem {
                file_token: "folder_a".to_string(),
                title: "Alpha".to_string(),
                doc_type: "folder".to_string(),
                is_folder: true,
                create_time: 1,
                update_time: 2,
            }],
            has_more: true,
            page_token: Some("page_2".to_string()),
        };

        let page: FolderChildrenPage = TypedPage::from(data);
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].title, "Alpha");
        assert!(page.has_more);
        assert_eq!(page.next_page_token.as_deref(), Some("page_2"));
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_folder_children_pager_resume_token() {
        let client = DocsClient::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );

        let pager = client
            .folder_children_pager("folder_token")
            .page_size(999)
            .doc_type("folder")
            .next_page_token("page_2");

        assert_eq!(pager.pending_page_token(), Some("page_2"));
    }
}
