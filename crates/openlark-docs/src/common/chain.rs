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
use openlark_core::error::{business_error, CoreError};
#[cfg(any(feature = "ccm-core", feature = "bitable"))]
use openlark_core::SDKResult;
use std::sync::Arc;

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

    /// 获取文件夹下的全部子项，自动处理分页。
    #[cfg(feature = "ccm-core")]
    pub async fn list_folder_children_all(
        &self,
        folder_token: &str,
        doc_type: Option<&str>,
    ) -> SDKResult<Vec<crate::ccm::explorer::v2::models::FileItem>> {
        use crate::ccm::explorer::v2::{get_folder_children, GetFolderChildrenParams};

        let mut items = Vec::new();
        let mut page_token = None;

        loop {
            let response = get_folder_children(
                self.config(),
                folder_token,
                Some(GetFolderChildrenParams {
                    page_size: Some(crate::common::constants::MAX_PAGE_SIZE),
                    page_token: page_token.clone(),
                    doc_type: doc_type.map(str::to_owned),
                }),
            )
            .await?;

            let Some(data) = response.data else {
                break;
            };

            items.extend(data.items);

            if !data.has_more {
                break;
            }

            page_token = data.page_token;
        }

        Ok(items)
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

    #[cfg(feature = "ccm-core")]
    pub async fn list_sheet_infos(
        &self,
        spreadsheet_token: &str,
    ) -> SDKResult<Vec<crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo>> {
        use crate::ccm::sheets_v2::v2::spreadsheet::{get_spreadsheet, GetSpreadsheetParams};

        let response = get_spreadsheet(
            self.config(),
            spreadsheet_token,
            GetSpreadsheetParams {
                include_sheet: Some(true),
            },
        )
        .await?;

        let spreadsheet = response
            .data
            .ok_or_else(|| CoreError::api_data_error("获取表格信息"))?;

        spreadsheet
            .sheets
            .ok_or_else(|| CoreError::api_data_error("获取工作表列表"))
    }
}

#[cfg(feature = "ccm-core")]
fn find_sheet_info(
    sheets: &[crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo],
    title: &str,
) -> Option<crate::ccm::sheets_v2::v2::spreadsheet::models::SpreadsheetSheetInfo> {
    sheets.iter().find(|sheet| sheet.title == title).cloned()
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
}
