# Open-Lark 0.13 → 0.15 代码迁移示例集

本文档提供详细的代码迁移示例，包含新旧版本对比、适配层实现和最佳实践。

---

## 目录

1. [LarkClient 封装层迁移](#1-larkclient-封装层迁移)
2. [Bitable API 迁移](#2-bitable-api-迁移)
3. [Sheets API 迁移](#3-sheets-api-迁移)
4. [Drive API 迁移](#4-drive-api-迁移)
5. [CustomBot 兼容层](#5-custombot-兼容层)
6. [错误处理迁移](#6-错误处理迁移)
7. [完整业务场景示例](#7-完整业务场景示例)

---

## 1. LarkClient 封装层迁移

### 文件位置
- **旧**: `clients/lark_client/src/client.rs`
- **新**: `clients/lark_client/src/client.rs`

### 1.1 基础 Client 结构

#### 旧版本 (0.13)
```rust
// clients/lark_client/src/client.rs
use open_lark::client::LarkClient as OpenLarkClient;
use open_lark::service::sheets::{
    v2::data_operation::WriteDataToMultiRangesRequest,
    v3::spreadsheet_sheet::{QuerySpreadsheetSheetRequest, Sheet},
};
use open_lark::service::bitable::v1::app_table_record::{
    SearchRecordRequestBuilder, SearchRecordResponse
};

pub struct LarkClient {
    inner: OpenLarkClient,
}

impl LarkClient {
    pub fn new(app_id: &str, app_secret: &str) -> Self {
        let inner = OpenLarkClient::builder(app_id, app_secret).build();
        Self { inner }
    }
}
```

#### 新版本 (0.15)
```rust
// clients/lark_client/src/client.rs
use openlark_client::prelude::*;
use openlark_core::error::CoreError;

pub struct LarkClient {
    inner: Client,
}

pub type Result<T> = std::result::Result<T, LarkClientError>;

impl LarkClient {
    pub fn new(app_id: &str, app_secret: &str) -> Result<Self> {
        let inner = Client::builder()
            .app_id(app_id)
            .app_secret(app_secret)
            .build()
            .map_err(|e| LarkClientError::ClientCreationError(e.to_string()))?;
        
        Ok(Self { inner })
    }

    /// 获取内部 client 引用（供适配器使用）
    pub(crate) fn inner(&self) -> &Client {
        &self.inner
    }
}

/// 错误类型定义
#[derive(Debug, thiserror::Error)]
pub enum LarkClientError {
    #[error("创建客户端失败: {0}")]
    ClientCreationError(String),
    
    #[error("API 调用失败: {0}")]
    ApiError(String),
    
    #[error("数据解析失败: {0}")]
    ParseError(String),
}

impl From<CoreError> for LarkClientError {
    fn from(err: CoreError) -> Self {
        LarkClientError::ApiError(
            err.user_message()
                .unwrap_or_else(|| err.to_string())
        )
    }
}
```

---

## 2. Bitable API 迁移

### 文件位置
- **旧**: `clients/lark_client/src/lib.rs` 和直接使用
- **新**: `clients/lark_client/src/adapters/bitable.rs`

### 2.1 适配器实现

#### 创建文件: `clients/lark_client/src/adapters/mod.rs`
```rust
//! 适配器模块 - 为 0.15 API 提供 0.13 风格的接口

pub mod bitable;
pub mod sheets;
pub mod drive;

pub use bitable::BitableAdapter;
pub use sheets::SheetsAdapter;
pub use drive::DriveAdapter;
```

#### 创建文件: `clients/lark_client/src/adapters/bitable.rs`
```rust
//! Bitable 适配器 - 提供与 0.13 版本兼容的 API

use openlark_client::prelude::*;
use openlark_docs::base::bitable::v1::app::table::record::{
    SearchRecordRequest, SearchRecordResponse,
    CreateRecordRequest, CreateRecordResponse,
    BatchCreateRecordRequest, BatchCreateRecordResponse,
    UpdateRecordRequest, UpdateRecordResponse,
    DeleteRecordRequest, DeleteRecordResponse,
};
use openlark_docs::base::bitable::v1::app::table::record::models::{Record, FieldValue};
use serde_json::Value;
use crate::client::{LarkClient, Result, LarkClientError};

/// Bitable 适配器
/// 
/// 使用示例:
/// ```rust,ignore
/// let client = LarkClient::new(app_id, app_secret)?;
/// let bitable = BitableAdapter::new(&client, "app_token", "table_id");
/// let records = bitable.search_records(filter).await?;
/// ```
pub struct BitableAdapter<'a> {
    config: Config,
    app_token: &'a str,
    table_id: &'a str,
}

impl<'a> BitableAdapter<'a> {
    /// 创建新的 Bitable 适配器
    pub fn new(client: &'a LarkClient, app_token: &'a str, table_id: &'a str) -> Self {
        let config = client.inner().docs.base.bitable().config().clone();
        Self {
            config,
            app_token,
            table_id,
        }
    }

    /// 查询记录
    /// 
    /// 对应旧版本的: `SearchRecordRequestBuilder`
    pub async fn search_records(
        &self,
        filter: Option<Value>,
    ) -> Result<SearchRecordResponse> {
        let mut request = SearchRecordRequest::new(self.config.clone())
            .app_token(self.app_token.to_string())
            .table_id(self.table_id.to_string());

        // 如果有筛选条件，添加到请求
        if let Some(filter_json) = filter {
            // 根据实际需求解析 filter
            // 这里假设 filter 是一个 JSON 对象
            request = request.filter(filter_json.to_string());
        }

        request.execute().await.map_err(|e| e.into())
    }

    /// 创建单条记录
    /// 
    /// 对应旧版本的: `client.bitable.v1.app_table_record.create`
    pub async fn create_record(
        &self,
        fields: Value,
    ) -> Result<CreateRecordResponse> {
        let request = CreateRecordRequest::new(self.config.clone())
            .app_token(self.app_token.to_string())
            .table_id(self.table_id.to_string())
            .fields(fields);

        request.execute().await.map_err(|e| e.into())
    }

    /// 批量创建记录
    pub async fn batch_create_records(
        &self,
        records: Vec<Value>,
    ) -> Result<BatchCreateRecordResponse> {
        let request = BatchCreateRecordRequest::new(self.config.clone())
            .app_token(self.app_token.to_string())
            .table_id(self.table_id.to_string())
            .records(records);

        request.execute().await.map_err(|e| e.into())
    }

    /// 更新记录
    pub async fn update_record(
        &self,
        record_id: &str,
        fields: Value,
    ) -> Result<UpdateRecordResponse> {
        let request = UpdateRecordRequest::new(self.config.clone())
            .app_token(self.app_token.to_string())
            .table_id(self.table_id.to_string())
            .record_id(record_id.to_string())
            .fields(fields);

        request.execute().await.map_err(|e| e.into())
    }

    /// 删除记录
    pub async fn delete_record(
        &self,
        record_id: &str,
    ) -> Result<DeleteRecordResponse> {
        let request = DeleteRecordRequest::new(self.config.clone())
            .app_token(self.app_token.to_string())
            .table_id(self.table_id.to_string())
            .record_id(record_id.to_string());

        request.execute().await.map_err(|e| e.into())
    }
}

/// 重新导出类型，保持与旧版本兼容
pub use openlark_docs::base::bitable::v1::app::table::record::models::Record;
pub use openlark_docs::base::bitable::v1::app::table::record::{
    SearchRecordResponse, CreateRecordResponse,
    BatchCreateRecordResponse, UpdateRecordResponse, DeleteRecordResponse,
};
```

### 2.2 使用方式对比

#### 旧版本使用方式 (0.13)
```rust
// service/src/financial/analysis/sync/committee_adjust.rs
use lark_client::{LarkClient, SearchRecordRequestBuilder};

let client = LarkClient::new(&app_config.lark.app_id, &app_config.lark.app_secret);
let resp = client
    .search_records(SearchRecordRequestBuilder::new(app_token, table_id))
    .await?;
```

#### 新版本使用方式 (0.15)
```rust
// service/src/financial/analysis/sync/committee_adjust.rs
use lark_client::{LarkClient, adapters::BitableAdapter};

let client = LarkClient::new(&app_config.lark.app_id, &app_config.lark.app_secret)?;
let bitable = BitableAdapter::new(&client, app_token, table_id);
let resp = bitable.search_records(None).await?;
```

---

## 3. Sheets API 迁移

### 文件位置
- **新**: `clients/lark_client/src/adapters/sheets.rs`

### 3.1 适配器实现

#### 创建文件: `clients/lark_client/src/adapters/sheets.rs`
```rust
//! Sheets 适配器 - 提供与 0.13 版本兼容的 API

use openlark_client::prelude::*;
use openlark_docs::ccm::sheets::v3::spreadsheet::sheet::{
    query::{query_sheets, QuerySheetResponse},
    get::{get_sheet, GetSheetResponse},
};
use openlark_docs::ccm::sheets::v3::spreadsheet::{
    models::{Spreadsheet, Sheet},
    get::{get_spreadsheet, GetSpreadsheetResponse},
    create::{create_spreadsheet, CreateSpreadsheetRequest, CreateSpreadsheetResponse},
    patch::{patch_spreadsheet, PatchSpreadsheetRequest, PatchSpreadsheetResponse},
};
use serde_json::Value;
use crate::client::{LarkClient, Result, LarkClientError};

/// Sheets 适配器
pub struct SheetsAdapter<'a> {
    config: &'a Config,
}

impl<'a> SheetsAdapter<'a> {
    /// 创建新的 Sheets 适配器
    pub fn new(client: &'a LarkClient) -> Self {
        let config = client.inner().docs.ccm.config();
        Self { config }
    }

    /// 查询工作表列表
    /// 
    /// 对应旧版本的: `client.sheets.v3.spreadsheet_sheet.query`
    pub async fn query_sheets(
        &self,
        spreadsheet_token: &str,
    ) -> Result<QuerySheetResponse> {
        query_sheets(self.config, spreadsheet_token)
            .await
            .map_err(|e| e.into())
    }

    /// 获取工作表详情
    pub async fn get_sheet(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
    ) -> Result<GetSheetResponse> {
        get_sheet(self.config, spreadsheet_token, sheet_id)
            .await
            .map_err(|e| e.into())
    }

    /// 获取电子表格信息
    pub async fn get_spreadsheet(
        &self,
        spreadsheet_token: &str,
    ) -> Result<GetSpreadsheetResponse> {
        get_spreadsheet(self.config, spreadsheet_token)
            .await
            .map_err(|e| e.into())
    }

    /// 创建电子表格
    pub async fn create_spreadsheet(
        &self,
        title: &str,
        folder_token: Option<&str>,
    ) -> Result<CreateSpreadsheetResponse> {
        let request = CreateSpreadsheetRequest::new(title.to_string())
            .folder_token(folder_token.map(|s| s.to_string()));

        create_spreadsheet(self.config, &request)
            .await
            .map_err(|e| e.into())
    }

    /// 更新电子表格属性
    pub async fn patch_spreadsheet(
        &self,
        spreadsheet_token: &str,
        title: Option<&str>,
    ) -> Result<PatchSpreadsheetResponse> {
        let mut request = PatchSpreadsheetRequest::new();
        
        if let Some(t) = title {
            request = request.title(t.to_string());
        }

        patch_spreadsheet(self.config, spreadsheet_token, &request)
            .await
            .map_err(|e| e.into())
    }
}

// TODO: 添加更多 Sheets v2 API 适配
// - write_data_multi_ranges
// - read_data
// - 等等

/// 重新导出类型
pub use openlark_docs::ccm::sheets::v3::spreadsheet::sheet::QuerySheetResponse;
pub use openlark_docs::ccm::sheets::v3::spreadsheet::{
    GetSpreadsheetResponse, CreateSpreadsheetResponse, PatchSpreadsheetResponse,
};
pub use openlark_docs::ccm::sheets::v3::spreadsheet::models::{Spreadsheet, Sheet};
```

### 3.2 使用方式对比

#### 旧版本使用方式 (0.13)
```rust
// service/src/integration/third_parking.rs
use open_lark::{
    client::LarkClient,
    service::sheets::v3::spreadsheet_sheet::QuerySpreadsheetSheetRequest,
};

let client = LarkClient::builder(&config.lark.app_id, &config.lark.app_secret).build();
let req = QuerySpreadsheetSheetRequest::builder()
    .spreadsheet_token(spreadsheet_token)
    .build();
let resp = client.sheets.v3.spreadsheet_sheet.query(req, None).await?;
```

#### 新版本使用方式 (0.15)
```rust
// service/src/integration/third_parking.rs
use lark_client::{LarkClient, adapters::SheetsAdapter};

let client = LarkClient::new(&config.lark.app_id, &config.lark.app_secret)?;
let sheets = SheetsAdapter::new(&client);
let resp = sheets.query_sheets(spreadsheet_token).await?;
```

---

## 4. Drive API 迁移

### 文件位置
- **新**: `clients/lark_client/src/adapters/drive.rs`

### 4.1 适配器实现

#### 创建文件: `clients/lark_client/src/adapters/drive.rs`
```rust
//! Drive 适配器 - 提供与 0.13 版本兼容的 API

use openlark_client::prelude::*;
use openlark_docs::ccm::drive::v1::file::{
    list::{list_files, ListFilesRequest, ListFilesResponse},
    get::{get_file, GetFileResponse},
    copy::{copy_file, CopyFileRequest, CopyFileResponse},
    delete::{delete_file, DeleteFileResponse},
    upload_all::{upload_all, UploadAllRequest, UploadAllResponse},
    download::{download_file, DownloadFileResponse},
};
use crate::client::{LarkClient, Result, LarkClientError};

/// Drive 适配器
pub struct DriveAdapter<'a> {
    config: &'a Config,
}

impl<'a> DriveAdapter<'a> {
    /// 创建新的 Drive 适配器
    pub fn new(client: &'a LarkClient) -> Self {
        let config = client.inner().docs.ccm.config();
        Self { config }
    }

    /// 列出文件夹内容
    /// 
    /// 对应旧版本的: `client.drive.v2.explorer.list_folder`
    pub async fn list_folder(
        &self,
        folder_token: &str,
    ) -> Result<ListFilesResponse> {
        let request = ListFilesRequest::new(folder_token.to_string());
        
        list_files(self.config, &request)
            .await
            .map_err(|e| e.into())
    }

    /// 获取文件信息
    pub async fn get_file(
        &self,
        file_token: &str,
    ) -> Result<GetFileResponse> {
        get_file(self.config, file_token)
            .await
            .map_err(|e| e.into())
    }

    /// 复制文件
    pub async fn copy_file(
        &self,
        file_token: &str,
        dst_folder_token: &str,
    ) -> Result<CopyFileResponse> {
        let request = CopyFileRequest::new()
            .dst_folder_token(dst_folder_token.to_string());

        copy_file(self.config, file_token, &request)
            .await
            .map_err(|e| e.into())
    }

    /// 删除文件
    pub async fn delete_file(
        &self,
        file_token: &str,
    ) -> Result<DeleteFileResponse> {
        delete_file(self.config, file_token)
            .await
            .map_err(|e| e.into())
    }

    /// 上传文件
    pub async fn upload_file(
        &self,
        file_name: &str,
        parent_node: &str,
        file_data: Vec<u8>,
    ) -> Result<UploadAllResponse> {
        let request = UploadAllRequest::new(
            self.config.clone(),
            file_name.to_string(),
            parent_node.to_string(),
            "application/octet-stream".to_string(),
            file_data.len(),
            file_data,
        );

        upload_all(request).await.map_err(|e| e.into())
    }

    /// 下载文件
    pub async fn download_file(
        &self,
        file_token: &str,
    ) -> Result<DownloadFileResponse> {
        download_file(self.config, file_token)
            .await
            .map_err(|e| e.into())
    }
}

/// 重新导出类型
pub use openlark_docs::ccm::drive::v1::file::{
    ListFilesResponse, GetFileResponse, CopyFileResponse,
    DeleteFileResponse, UploadAllResponse, DownloadFileResponse,
};
```

### 4.2 使用方式对比

#### 旧版本使用方式 (0.13)
```rust
// service/src/yitong/handler/income/sync_report/discovery.rs
use open_lark::{
    client::LarkClient,
    service::drive::v2::explorer::{ListFolderRequest, ListFolderResponse},
};

let client = LarkClient::builder(&config.lark_app_id, &config.lark_app_secret).build();
let request = ListFolderRequest {
    folder_token: folder_token.to_string(),
    ..Default::default()
};
let response: ListFolderResponse = client.drive.v2.explorer.list_folder(request, None).await?;
```

#### 新版本使用方式 (0.15)
```rust
// service/src/yitong/handler/income/sync_report/discovery.rs
use lark_client::{LarkClient, adapters::DriveAdapter};

let client = LarkClient::new(&config.lark_app_id, &config.lark_app_secret)?;
let drive = DriveAdapter::new(&client);
let response = drive.list_folder(folder_token).await?;
```

---

## 5. CustomBot 兼容层

### 说明
CustomBot 在新版本中不存在，必须保留兼容层。

### 5.1 保留现有实现

文件: `service/src/open_lark_compat.rs`
```rust
//! Open-Lark 兼容层
//! 
//! 新版本 open-lark 0.15 中不存在的功能，在此提供兼容实现

use anyhow::Result;
use serde_json::Value;

/// 自定义机器人（兼容层）
/// 
/// 注意：新版本的 open-lark 0.15 中没有 CustomBot，
/// 这里使用 reqwest 直接调用飞书 Webhook API
pub struct CustomBot<'a> {
    webhook_url: &'a str,
    secret: Option<&'a str>,
}

impl<'a> CustomBot<'a> {
    /// 创建新的自定义机器人
    pub fn new(webhook_url: &'a str, secret: Option<&'a str>) -> Self {
        Self { webhook_url, secret }
    }

    /// 发送文本消息
    pub async fn send_text(&self, text: &str) -> Result<()> {
        let client = reqwest::Client::new();
        
        let body = serde_json::json!({
            "msg_type": "text",
            "content": {
                "text": text
            }
        });

        let response = client
            .post(self.webhook_url)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("发送消息失败: {}", error_text);
        }

        Ok(())
    }

    /// 发送卡片消息
    pub async fn send_card(&self, card: &MessageCardTemplate) -> Result<()> {
        let client = reqwest::Client::new();

        let body = serde_json::json!({
            "msg_type": "interactive",
            "card": card.content
        });

        let response = client
            .post(self.webhook_url)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("发送卡片失败: {}", error_text);
        }

        Ok(())
    }
}

/// 消息卡片模板
pub struct MessageCardTemplate {
    pub(crate) content: Value,
}

impl MessageCardTemplate {
    /// 创建新的消息卡片模板
    pub fn new(template_id: &str, variables: Value) -> Self {
        let content = serde_json::json!({
            "type": "template",
            "data": {
                "template_id": template_id,
                "template_variables": variables
            }
        });

        Self { content }
    }

    /// 从 JSON 创建
    pub fn from_json(json: Value) -> Self {
        Self { content: json }
    }
}
```

### 5.2 使用方式（无需修改）

```rust
// service/src/yitong/handler/car/notifications.rs
use crate::open_lark_compat::{CustomBot, MessageCardTemplate};

let bot = CustomBot::new(&group_config.url, group_config.secret.as_deref());
let card = MessageCardTemplate::new(template_id, variables);
bot.send_card(&card).await?;
```

---

## 6. 错误处理迁移

### 6.1 错误类型定义

#### 旧版本 (0.13)
```rust
#[derive(Debug, Error)]
pub enum LarkClientError {
    #[error("飞书 API 错误: {0}")]
    ApiError(String),
    #[error("飞书返回数据为空")]
    EmptyDataError,
    #[error("工作表未找到: {0}")]
    SheetNotFound(String),
    #[error("其他错误: {0}")]
    Other(#[from] anyhow::Error),
}
```

#### 新版本 (0.15)
```rust
use openlark_core::error::CoreError;

#[derive(Debug, thiserror::Error)]
pub enum LarkClientError {
    #[error("创建客户端失败: {0}")]
    ClientCreationError(String),
    
    #[error("飞书 API 错误: {0}")]
    ApiError(String),
    
    #[error("飞书返回数据为空")]
    EmptyDataError,
    
    #[error("工作表未找到: {0}")]
    SheetNotFound(String),
    
    #[error("数据解析失败: {0}")]
    ParseError(String),
    
    #[error("其他错误: {0}")]
    Other(#[from] anyhow::Error),
}

impl From<CoreError> for LarkClientError {
    fn from(err: CoreError) -> Self {
        let message = err.user_message()
            .unwrap_or_else(|| err.to_string());
        
        // 可以根据错误类型细分
        if err.is_auth_error() {
            LarkClientError::ApiError(format!("认证错误: {}", message))
        } else if err.is_network_error() {
            LarkClientError::ApiError(format!("网络错误: {}", message))
        } else {
            LarkClientError::ApiError(message)
        }
    }
}
```

---

## 7. 完整业务场景示例

### 7.1 财务分析 - 委员会调整同步

#### 旧版本 (0.13)
```rust
// service/src/financial/analysis/sync/committee_adjust.rs
use lark_client::{LarkClient, SearchRecordRequestBuilder};
use open_lark::service::bitable::v1::Record;

pub async fn sync_committee_adjust(
    app_config: &AppConfig,
    app_token: &str,
    table_id: &str,
) -> Result<()> {
    let client = LarkClient::new(&app_config.lark.app_id, &app_config.lark.app_secret);
    
    // 查询所有记录
    let resp = client
        .search_records(SearchRecordRequestBuilder::new(app_token, table_id))
        .await?;

    for record in resp.data.items {
        let committee_name = record.fields.get("委员会名称")
            .and_then(|v| v.as_str())
            .ok_or("缺少委员会名称")?;
        
        let adjust_amount = record.fields.get("调整金额")
            .and_then(|v| v.as_f64())
            .ok_or("缺少调整金额")?;
        
        // 处理业务逻辑...
        process_adjustment(committee_name, adjust_amount).await?;
    }

    Ok(())
}
```

#### 新版本 (0.15)
```rust
// service/src/financial/analysis/sync/committee_adjust.rs
use lark_client::{LarkClient, adapters::{BitableAdapter, Record}};

pub async fn sync_committee_adjust(
    app_config: &AppConfig,
    app_token: &str,
    table_id: &str,
) -> Result<()> {
    let client = LarkClient::new(&app_config.lark.app_id, &app_config.lark.app_secret)?;
    
    // 使用适配器查询记录
    let bitable = BitableAdapter::new(&client, app_token, table_id);
    let resp = bitable.search_records(None).await?;

    for record in resp.data.items {
        let committee_name = record.fields.get("委员会名称")
            .and_then(|v| v.as_str())
            .ok_or("缺少委员会名称")?;
        
        let adjust_amount = record.fields.get("调整金额")
            .and_then(|v| v.as_f64())
            .ok_or("缺少调整金额")?;
        
        // 处理业务逻辑（保持不变）...
        process_adjustment(committee_name, adjust_amount).await?;
    }

    Ok(())
}
```

**迁移要点**:
1. 导入从 `SearchRecordRequestBuilder` 改为 `BitableAdapter`
2. 创建 `BitableAdapter` 实例
3. 调用 `search_records` 方法
4. 业务逻辑完全保持不变

### 7.2 停车集成 - 读取 Sheets 数据

#### 旧版本 (0.13)
```rust
// service/src/integration/third_parking.rs
use open_lark::{
    client::LarkClient,
    service::sheets::v3::spreadsheet_sheet::QuerySpreadsheetSheetRequest,
};

pub async fn read_parking_config(
    config: &Config,
    spreadsheet_token: &str,
) -> Result<Vec<ParkingConfig>> {
    let client = LarkClient::builder(&config.lark.app_id, &config.lark.app_secret).build();
    
    let req = QuerySpreadsheetSheetRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .build();
    
    let resp = client.sheets.v3.spreadsheet_sheet.query(req, None).await?;
    
    let mut configs = Vec::new();
    for sheet in resp.data.sheets {
        let config = parse_sheet_to_config(&sheet)?;
        configs.push(config);
    }
    
    Ok(configs)
}
```

#### 新版本 (0.15)
```rust
// service/src/integration/third_parking.rs
use lark_client::{LarkClient, adapters::SheetsAdapter};

pub async fn read_parking_config(
    config: &Config,
    spreadsheet_token: &str,
) -> Result<Vec<ParkingConfig>> {
    let client = LarkClient::new(&config.lark.app_id, &config.lark.app_secret)?;
    
    // 使用适配器查询工作表
    let sheets = SheetsAdapter::new(&client);
    let resp = sheets.query_sheets(spreadsheet_token).await?;
    
    let mut configs = Vec::new();
    for sheet in resp.data.sheets {
        let config = parse_sheet_to_config(&sheet)?;
        configs.push(config);
    }
    
    Ok(configs)
}
```

**迁移要点**:
1. 导入改为 `SheetsAdapter`
2. 创建 `SheetsAdapter` 实例
3. 调用 `query_sheets` 方法
4. 响应数据结构保持一致

---

## 📋 迁移检查清单

对于每个业务模块的迁移，请检查：

- [ ] 更新导入语句
- [ ] 替换 Client 创建方式
- [ ] 使用适配器替代直接调用
- [ ] 更新错误处理
- [ ] 编译通过
- [ ] 单元测试通过
- [ ] 集成测试通过

---

## 🎯 最佳实践

1. **优先使用适配器**: 适配器提供了与旧版本相似的 API，减少业务代码修改
2. **分阶段迁移**: 先完成适配层，再逐个业务模块迁移
3. **保持测试**: 每迁移一个模块，立即运行测试
4. **保留兼容层**: CustomBot 等功能继续使用兼容层
5. **错误映射**: 统一错误类型，提供良好的错误消息

---

**代码示例文档完成** ✓

这些示例可以直接用于迁移实施
