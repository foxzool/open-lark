/// 记录管理服务模块
///
/// 提供多维表格记录的创建、更新、删除、查询等操作功能。
///
/// # 使用示例
///
/// ```rust,no_run
/// use openlark_core::Config;
/// use openlark_docs::base::bitable::v1::app::table::record::*;
/// use openlark_docs::base::bitable::v1::field_types::{RecordFieldsBuilder, RecordFieldValue};
/// use serde_json::json;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config = Config::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .build();
///
/// // 1. 使用字段类型创建记录（推荐方式）
/// let fields = RecordFieldsBuilder::new()
///     .add_text("姓名", "张三")
///     .add_number("年龄", 25)
///     .add_bool("在职", true)
///     .add_user_id("负责人", "user_123")
///     .build();
///
/// let create_req = CreateRecordRequest::new(config.clone())
///     .app_token("your_app_token")
///     .table_id("your_table_id")
///     .fields(json!(fields));
/// let response = create_req.execute().await?;
/// println!("记录ID: {}", response.record.record_id);
///
/// // 2. 查询记录（带过滤器）
/// let search_req = SearchRecordRequest::new(config.clone())
///     .app_token("your_app_token")
///     .table_id("your_table_id")
///     .filter(FilterInfo {
///         conjunction: "and".to_string(),
///         conditions: vec![FilterCondition {
///             field_name: "姓名".to_string(),
///             operator: "is".to_string(),
///             value: Some(vec!["张三".to_string()]),
///         }],
///     });
/// let results = search_req.execute().await?;
/// println!("找到 {} 条记录", results.items.len());
///
/// // 3. 更新记录
/// let update_req = UpdateRecordRequest::new(config.clone())
///     .app_token("your_app_token")
///     .table_id("your_table_id")
///     .record_id("record_id")
///     .fields(json!(fields));
/// update_req.execute().await?;
///
/// // 4. 删除记录
/// let delete_req = DeleteRecordRequest::new(config)
///     .app_token("your_app_token")
///     .table_id("your_table_id")
///     .record_id("record_id");
/// delete_req.execute().await?;
/// # Ok(())
/// # }
/// ```
///
/// # 字段类型说明
///
/// `RecordFieldsBuilder` 和 `RecordFieldValue` 提供类型安全的字段构建：
///
/// ## 常用字段类型
///
/// - **文本**: `RecordFieldValue::Text("value")`
/// - **数字**: `RecordFieldValue::Number(123)`
/// - **布尔**: `RecordFieldValue::Bool(true)`
/// - **日期**: `RecordFieldValue::Date(timestamp)`
/// - **用户ID**: `RecordFieldValue::SingleId("user_123")` 或 `MultipleIds(vec!["user_123", "user_456"])`
/// - **附件**: `RecordFieldValue::Attachment(vec![attachment_info])`
/// - **单选**: `RecordFieldValue::Select("option_name")` 或 `MultipleSelect(vec!["opt1", "opt2"])`
/// - **进度条**: `RecordFieldValue::Progress(progress, status)`
///
/// ## RecordFieldsBuilder 方法
///
/// - `add_text(field_name, value)`: 添加文本字段
/// - `add_number(field_name, value)`: 添加数字字段
/// - `add_bool(field_name, value)`: 添加布尔字段
/// - `add_date(field_name, timestamp)`: 添加日期字段
/// - `add_user_id(field_name, user_id)`: 添加单个用户ID
/// - `add_user_ids(field_name, user_ids)`: 添加多个用户ID
/// - `add_attachment(field_name, attachment)`: 添加附件
/// - `add_select(field_name, value)`: 添加单选字段
/// - `add_multi_select(field_name, values)`: 添加多选字段
/// - `add_progress(field_name, progress, status)`: 添加进度条
///
/// ## 类型转换
///
/// `RecordFieldsBuilder::build()` 返回 `RecordFields` 类型
/// `RecordFieldValue::Text/Number/Bool`...` 枚举会自动序列化为 JSON 值
///
/// ## 示例
///
/// ```rust,no_run
/// let fields = RecordFieldsBuilder::new()
///     .add_text("产品", "iPhone")
///     .add_number("价格", 6999)
///     .add_bool("在售", false)
///     .build();
///
/// // fields 会自动转换为 serde_json::Value
/// let request = CreateRecordRequest::new(config)
///     .app_token("app_token")
///     .table_id("table_id")
///     .fields(json!(fields));
/// ```
///
/// # 迁移 RecordFieldValue
///
/// `RecordFieldValue::to_json_value(&self)` 提供向后兼容性
///
/// ```rust,no_run
/// // 向后兼容的代码
/// let fields = serde_json::json!({
///     "field1": "value1",
///     "field2": 123,
/// });
///
/// // 新方式（推荐）
/// let new_fields = RecordFieldsBuilder::new()
///     .add_text("field1", "value1");
///     .build();
/// ```
use openlark_core::config::Config;

// 导入子模块
pub mod batch_create;
pub mod batch_delete;
pub mod batch_get;
pub mod batch_update;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod search;
pub mod update;

// 重新导出主要类型
pub use batch_create::{
    BatchCreateRecordRequest, BatchCreateRecordRequestBuilder, BatchCreateRecordResponse,
};
pub use batch_delete::{
    BatchDeleteRecordRequest, BatchDeleteRecordRequestBuilder, BatchDeleteRecordResponse,
};
pub use batch_get::{BatchGetRecordRequest, BatchGetRecordRequestBuilder, BatchGetRecordResponse};
pub use batch_update::{
    BatchUpdateRecordRequest, BatchUpdateRecordRequestBuilder, BatchUpdateRecordResponse,
};
pub use create::{CreateRecordRequest, CreateRecordRequestBuilder, CreateRecordResponse};
pub use delete::{DeleteRecordRequest, DeleteRecordRequestBuilder, DeleteRecordResponse};
pub use get::{GetRecordRequest, GetRecordRequestBuilder, GetRecordResponse};
pub use list::{ListRecordRequest, ListRecordRequestBuilder, ListRecordResponse};
pub use models::{Person, Record};
pub use search::{SearchRecordRequest, SearchRecordRequestBuilder, SearchRecordResponse};
pub use update::{UpdateRecordRequest, UpdateRecordRequestBuilder, UpdateRecordResponse};

/// 记录服务
pub struct AppTableRecordService {
    config: Config,
}

impl AppTableRecordService {
    /// 创建记录服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 批量获取记录
    pub async fn batch_get(&self) -> BatchGetRecordRequestBuilder {
        BatchGetRecordRequestBuilder::new(self.config.clone())
    }
}

// Type alias for compatibility
pub type ServiceType = AppTableRecordService;
