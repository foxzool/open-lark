//! 浮动图片管理服务
//!
//! 提供电子表格浮动图片的完整管理功能，包括：
//! - 创建和插入浮动图片
//! - 更新图片位置和大小
//! - 查询和获取图片信息
//! - 删除浮动图片
//! - 图片格式和尺寸管理

use serde::{Deserialize, Serialize};

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};
use reqwest::Method;

// v3模块核心类型定义
pub type SpreadsheetToken = String;
pub type SheetId = String;
pub type CellValue = serde_json::Value;
pub type SheetPagedResponse<T> = Vec<T>;

/// 浮动图片管理服务
///
/// 提供电子表格浮动图片的完整管理功能，支持图片在工作表中的灵活布局。
/// 浮动图片可以独立于单元格网格进行位置调整，支持精确定位和尺寸控制。
///
/// # 功能特性
///
/// ## 图片管理
/// - **创建图片**: 在指定位置插入浮动图片
/// - **更新图片**: 调整图片位置、大小和属性
/// - **查询图片**: 获取图片列表和详细信息
/// - **删除图片**: 移除不需要的浮动图片
///
/// ## 布局控制
/// - **精确定位**: 支持像素级位置控制
/// - **尺寸调整**: 自定义宽度和高度设置
/// - **偏移控制**: 精确的单元格内偏移调整
/// - **范围绑定**: 图片与单元格范围的关联
///
/// # 常见应用场景
///
/// ```rust
/// # use open_lark::prelude::*;
/// # use open_lark::service::sheets::v3::FloatImagesService;
/// # use open_lark::service::sheets::v3::models::spreadsheet::SpreadsheetToken;
/// # use open_lark::service::sheets::v3::models::sheet::SheetId;
///
/// let service = FloatImagesService::new(client_config);
///
/// // 场景1: 插入公司Logo到指定位置
/// let request = CreateFloatImageRequest::builder()
///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
///     .sheet_id("0XXXXXXXXXX".to_string())
///     .float_image_token("img_token_123456".to_string())
///     .range("A1".to_string()) // 只支持单个单元格
///     .width(200) // 设置宽度
///     .height(80)  // 设置高度
///     .offset_x(50) // X轴偏移
///     .offset_y(20) // Y轴偏移
/// .build()?;
///
/// let image = service.create(&request).await?;
///
/// // 场景2: 更新图片位置和大小
/// let update_request = UpdateFloatImageRequest::builder()
///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
///     .sheet_id("0XXXXXXXXXX".to_string())
///     .float_image_id("img_789".to_string())
///     .range("B2".to_string())
///     .width(300)
///     .height(120)
///     .offset_x(0)
///     .offset_y(0)
///     .build()?;
///
/// let updated_image = service.update(&update_request).await?;
/// ```
#[derive(Clone, Debug)]
pub struct FloatImagesService {
    config: openlark_core::config::Config,
}

/// 浮动图片信息
///
/// 表示一个浮动图片的完整信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatImage {
    /// 浮动图片ID
    #[serde(rename = "floatImageId")]
    pub float_image_id: String,
    /// 浮动图片Token
    #[serde(rename = "floatImageToken")]
    pub float_image_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 所在范围（单个单元格）
    pub range: String,
    /// 图片宽度（像素）
    pub width: i32,
    /// 图片高度（像素）
    pub height: i32,
    /// X轴偏移（像素）
    #[serde(rename = "offsetX")]
    pub offset_x: i32,
    /// Y轴偏移（像素）
    #[serde(rename = "offsetY")]
    pub offset_y: i32,
    /// 创建时间戳
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// 更新时间戳
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    /// 图片状态
    pub status: String,
    /// 图片类型
    #[serde(rename = "imageType")]
    pub image_type: String,
}

/// 创建浮动图片请求
///
/// 用于在指定工作表中插入新的浮动图片。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 浮动图片Token（上传图片后获得）
    #[serde(rename = "floatImageToken")]
    pub float_image_token: String,
    /// 图片所在范围（单个单元格）
    pub range: String,
    /// 浮动图片ID（可选，不填自动生成）
    #[serde(rename = "floatImageId", skip_serializing_if = "Option::is_none")]
    pub float_image_id: Option<String>,
    /// 图片宽度（可选，使用图片真实宽高）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 图片高度（可选，使用图片真实宽高）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// X轴偏移（可选，默认0）
    #[serde(rename = "offsetX", skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<i32>,
    /// Y轴偏移（可选，默认0）
    #[serde(rename = "offsetY", skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<i32>,
}

impl CreateFloatImageRequest {
    /// 创建浮动图片请求构建器
    pub fn builder() -> CreateFloatImageRequestBuilder {
        CreateFloatImageRequestBuilder::new()
    }
}

/// 创建浮动图片请求构建器
pub struct CreateFloatImageRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    float_image_token: Option<String>,
    range: Option<String>,
    float_image_id: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
    offset_x: Option<i32>,
    offset_y: Option<i32>,
}

impl CreateFloatImageRequestBuilder {
    /// 创建新的创建浮动图片请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            float_image_token: None,
            range: None,
            float_image_id: None,
            width: None,
            height: None,
            offset_x: None,
            offset_y: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置浮动图片Token
    pub fn float_image_token(mut self, float_image_token: String) -> Self {
        self.float_image_token = Some(float_image_token);
        self
    }

    /// 设置图片所在范围
    pub fn range(mut self, range: String) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置浮动图片ID
    pub fn float_image_id(mut self, float_image_id: String) -> Self {
        self.float_image_id = Some(float_image_id);
        self
    }

    /// 设置图片宽度
    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    /// 设置图片高度
    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    /// 设置图片尺寸
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// 设置X轴偏移
    pub fn offset_x(mut self, offset_x: i32) -> Self {
        self.offset_x = Some(offset_x);
        self
    }

    /// 设置Y轴偏移
    pub fn offset_y(mut self, offset_y: i32) -> Self {
        self.offset_y = Some(offset_y);
        self
    }

    /// 设置偏移量
    pub fn offset(mut self, offset_x: i32, offset_y: i32) -> Self {
        self.offset_x = Some(offset_x);
        self.offset_y = Some(offset_y);
        self
    }

    /// 构建创建浮动图片请求对象
    pub fn build(self) -> openlark_core::error::SDKResult<CreateFloatImageRequest> {
        // 验证必需参数
        match (
            &self.spreadsheet_token,
            &self.sheet_id,
            &self.float_image_token,
            &self.range,
        ) {
            (Some(_), Some(_), Some(float_image_token), Some(range)) => {
                // 验证浮动图片Token
                if float_image_token.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "浮动图片Token不能为空".to_string(),
                    ));
                }

                // 验证范围格式
                if range.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "图片范围不能为空".to_string(),
                    ));
                }

                // 验证浮动图片ID格式（如果提供）
                if let Some(ref id) = self.float_image_id {
                    if id.is_empty() {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "浮动图片ID不能为空".to_string(),
                        ));
                    }
                    if id.len() > 10 {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "浮动图片ID长度不能超过10个字符".to_string(),
                        ));
                    }
                }

                // 验证尺寸参数
                if let Some(width) = self.width {
                    if width <= 0 {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "图片宽度必须大于0".to_string(),
                        ));
                    }
                    if width > 10000 {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "图片宽度不能超过10000像素".to_string(),
                        ));
                    }
                }

                if let Some(height) = self.height {
                    if height <= 0 {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "图片高度必须大于0".to_string(),
                        ));
                    }
                    if height > 10000 {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "图片高度不能超过10000像素".to_string(),
                        ));
                    }
                }

                // 验证偏移量
                if let Some(offset_x) = self.offset_x {
                    if offset_x < -10000 || offset_x > 10000 {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "X轴偏移量范围应为-10000到10000".to_string(),
                        ));
                    }
                }

                if let Some(offset_y) = self.offset_y {
                    if offset_y < -10000 || offset_y > 10000 {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "Y轴偏移量范围应为-10000到10000".to_string(),
                        ));
                    }
                }

                Ok(CreateFloatImageRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_token: float_image_token.clone(),
                    range: range.clone(),
                    float_image_id: self.float_image_id,
                    width: self.width,
                    height: self.height,
                    offset_x: self.offset_x,
                    offset_y: self.offset_y,
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token、工作表ID、浮动图片Token和范围都是必需的".to_string(),
            )),
        }
    }
}

/// 创建浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageResponse {
    /// 浮动图片信息
    pub data: FloatImage,
}

/// 更新浮动图片请求
///
/// 用于更新现有浮动图片的位置和大小。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 浮动图片ID
    #[serde(rename = "floatImageId")]
    pub float_image_id: String,
    /// 更新字段列表
    pub fields: Vec<String>,
    /// 图片所在范围（单个单元格）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// 图片宽度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 图片高度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// X轴偏移
    #[serde(rename = "offsetX", skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<i32>,
    /// Y轴偏移
    #[serde(rename = "offsetY", skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<i32>,
}

impl UpdateFloatImageRequest {
    /// 创建更新浮动图片请求构建器
    pub fn builder() -> UpdateFloatImageRequestBuilder {
        UpdateFloatImageRequestBuilder::new()
    }
}

/// 更新浮动图片请求构建器
pub struct UpdateFloatImageRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    float_image_id: Option<String>,
    fields: Vec<String>,
    range: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
    offset_x: Option<i32>,
    offset_y: Option<i32>,
}

impl UpdateFloatImageRequestBuilder {
    /// 创建新的更新浮动图片请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            float_image_id: None,
            fields: vec![],
            range: None,
            width: None,
            height: None,
            offset_x: None,
            offset_y: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置浮动图片ID
    pub fn float_image_id(mut self, float_image_id: String) -> Self {
        self.float_image_id = Some(float_image_id);
        self
    }

    /// 添加更新字段
    pub fn add_field(mut self, field: String) -> Self {
        self.fields.push(field);
        self
    }

    /// 设置更新字段列表
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
        self
    }

    /// 更新所有字段
    pub fn update_all(self) -> Self {
        self.fields(vec![
            "range".to_string(),
            "width".to_string(),
            "height".to_string(),
            "offsetX".to_string(),
            "offsetY".to_string(),
        ])
    }

    /// 设置图片所在范围
    pub fn range(mut self, range: String) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置图片宽度
    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    /// 设置图片高度
    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    /// 设置图片尺寸
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// 设置X轴偏移
    pub fn offset_x(mut self, offset_x: i32) -> Self {
        self.offset_x = Some(offset_x);
        self
    }

    /// 设置Y轴偏移
    pub fn offset_y(mut self, offset_y: i32) -> Self {
        self.offset_y = Some(offset_y);
        self
    }

    /// 设置偏移量
    pub fn offset(mut self, offset_x: i32, offset_y: i32) -> Self {
        self.offset_x = Some(offset_x);
        self.offset_y = Some(offset_y);
        self
    }

    /// 构建更新浮动图片请求对象
    pub fn build(self) -> openlark_core::error::SDKResult<UpdateFloatImageRequest> {
        // 验证必需参数
        match (
            &self.spreadsheet_token,
            &self.sheet_id,
            &self.float_image_id,
        ) {
            (Some(_), Some(_), Some(float_image_id)) => {
                // 验证浮动图片ID
                if float_image_id.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "浮动图片ID不能为空".to_string(),
                    ));
                }

                // 验证更新字段
                if self.fields.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "至少需要指定一个更新字段".to_string(),
                    ));
                }

                // 验证字段有效性
                let valid_fields = ["range", "width", "height", "offsetX", "offsetY"];
                for field in &self.fields {
                    if !valid_fields.contains(&field.as_str()) {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            format!("无效的更新字段: {}", field),
                        ));
                    }
                }

                // 验证提供的字段与字段列表一致性
                if self.range.is_none() && self.fields.contains(&"range".to_string()) {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "指定了range字段但未提供range值".to_string(),
                    ));
                }
                if self.width.is_none() && self.fields.contains(&"width".to_string()) {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "指定了width字段但未提供width值".to_string(),
                    ));
                }
                if self.height.is_none() && self.fields.contains(&"height".to_string()) {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "指定了height字段但未提供height值".to_string(),
                    ));
                }
                if self.offset_x.is_none() && self.fields.contains(&"offsetX".to_string()) {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "指定了offsetX字段但未提供offsetX值".to_string(),
                    ));
                }
                if self.offset_y.is_none() && self.fields.contains(&"offsetY".to_string()) {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "指定了offsetY字段但未提供offsetY值".to_string(),
                    ));
                }

                Ok(UpdateFloatImageRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_id: float_image_id.clone(),
                    fields: self.fields,
                    range: self.range,
                    width: self.width,
                    height: self.height,
                    offset_x: self.offset_x,
                    offset_y: self.offset_y,
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token、工作表ID和浮动图片ID都是必需的".to_string(),
            )),
        }
    }
}

/// 更新浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageResponse {
    /// 浮动图片信息
    pub data: FloatImage,
}

/// 查询浮动图片请求
///
/// 用于查询工作表中的浮动图片列表。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFloatImagesRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 浮动图片ID列表（可选）
    #[serde(rename = "floatImageIds", skip_serializing_if = "Vec::is_empty")]
    pub float_image_ids: Vec<String>,
    /// 页大小
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页令牌
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl QueryFloatImagesRequest {
    /// 创建查询浮动图片请求构建器
    pub fn builder() -> QueryFloatImagesRequestBuilder {
        QueryFloatImagesRequestBuilder::new()
    }
}

/// 查询浮动图片请求构建器
pub struct QueryFloatImagesRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    float_image_ids: Vec<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl QueryFloatImagesRequestBuilder {
    /// 创建新的查询浮动图片请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            float_image_ids: vec![],
            page_size: None,
            page_token: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 添加浮动图片ID
    pub fn add_float_image_id(mut self, float_image_id: String) -> Self {
        self.float_image_ids.push(float_image_id);
        self
    }

    /// 设置浮动图片ID列表
    pub fn float_image_ids(mut self, float_image_ids: Vec<String>) -> Self {
        self.float_image_ids = float_image_ids;
        self
    }

    /// 设置页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页令牌
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 构建查询浮动图片请求对象
    pub fn build(self) -> openlark_core::error::SDKResult<QueryFloatImagesRequest> {
        match (&self.spreadsheet_token, &self.sheet_id) {
            (Some(_), Some(_)) => {
                // 验证图片ID数量
                if self.float_image_ids.len() > 10 {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "单次最多支持查询10个浮动图片ID".to_string(),
                    ));
                }

                // 验证页大小
                if let Some(page_size) = self.page_size {
                    if page_size <= 0 {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "页大小必须大于0".to_string(),
                        ));
                    }
                    if page_size > 100 {
                        return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                            "页大小不能超过100".to_string(),
                        ));
                    }
                }

                Ok(QueryFloatImagesRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_ids: self.float_image_ids,
                    page_size: self.page_size,
                    page_token: self.page_token,
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token和工作表ID都是必需的".to_string(),
            )),
        }
    }
}

/// 查询浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFloatImagesResponse {
    /// 浮动图片列表数据
    pub data: SheetPagedResponse<FloatImage>,
}

/// 获取浮动图片请求
///
/// 用于获取指定浮动图片的详细信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFloatImageRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 浮动图片ID
    #[serde(rename = "floatImageId")]
    pub float_image_id: String,
}

impl GetFloatImageRequest {
    /// 创建获取浮动图片请求构建器
    pub fn builder() -> GetFloatImageRequestBuilder {
        GetFloatImageRequestBuilder::new()
    }
}

/// 获取浮动图片请求构建器
pub struct GetFloatImageRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    float_image_id: Option<String>,
}

impl GetFloatImageRequestBuilder {
    /// 创建新的获取浮动图片请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            float_image_id: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置浮动图片ID
    pub fn float_image_id(mut self, float_image_id: String) -> Self {
        self.float_image_id = Some(float_image_id);
        self
    }

    /// 构建获取浮动图片请求对象
    pub fn build(self) -> openlark_core::error::SDKResult<GetFloatImageRequest> {
        match (
            &self.spreadsheet_token,
            &self.sheet_id,
            &self.float_image_id,
        ) {
            (Some(_), Some(_), Some(float_image_id)) => {
                if float_image_id.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "浮动图片ID不能为空".to_string(),
                    ));
                }

                Ok(GetFloatImageRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_id: float_image_id.clone()
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token、工作表ID和浮动图片ID都是必需的".to_string(),
            )),
        }
    }
}

/// 获取浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFloatImageResponse {
    /// 浮动图片信息
    pub data: FloatImage,
}

impl ApiResponseTrait for GetFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除浮动图片请求
///
/// 用于删除指定的浮动图片。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFloatImageRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 浮动图片ID
    #[serde(rename = "floatImageId")]
    pub float_image_id: String,
}

impl DeleteFloatImageRequest {
    /// 创建删除浮动图片请求构建器
    pub fn builder() -> DeleteFloatImageRequestBuilder {
        DeleteFloatImageRequestBuilder::new()
    }
}

/// 删除浮动图片请求构建器
pub struct DeleteFloatImageRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    float_image_id: Option<String>,
}

impl DeleteFloatImageRequestBuilder {
    /// 创建新的删除浮动图片请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            float_image_id: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置浮动图片ID
    pub fn float_image_id(mut self, float_image_id: String) -> Self {
        self.float_image_id = Some(float_image_id);
        self
    }

    /// 构建删除浮动图片请求对象
    pub fn build(self) -> openlark_core::error::SDKResult<DeleteFloatImageRequest> {
        match (
            &self.spreadsheet_token,
            &self.sheet_id,
            &self.float_image_id,
        ) {
            (Some(_), Some(_), Some(float_image_id)) => {
                if float_image_id.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "浮动图片ID不能为空".to_string(),
                    ));
                }

                Ok(DeleteFloatImageRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_id: float_image_id.clone()
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token、工作表ID和浮动图片ID都是必需的".to_string(),
            )),
        }
    }
}

/// 删除浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFloatImageResponse {
    /// 删除结果
    pub data: SheetPagedResponse<serde_json::Value>,
}

impl FloatImagesService {
    /// 创建浮动图片服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 创建浮动图片
    ///
    /// 在指定的工作表中插入新的浮动图片。
    ///
    /// # 参数
    /// - `request`: 创建浮动图片请求，包含图片配置信息
    ///
    /// # 返回
    /// 返回创建成功的浮动图片信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{FloatImagesService, CreateFloatImageRequest};
    ///
    /// let service = FloatImagesService::new(client_config);
    ///
    /// // 创建浮动图片
    /// let request = CreateFloatImageRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .float_image_token("img_token_123456".to_string())
    ///     .range("A1".to_string())
    ///     .size(200, 80)
    ///     .offset(50, 20)
    ///     .build()?;
    ///
    /// let image = service.create(&request).await?;
    /// println!("创建浮动图片成功: {}", image.data.float_image_id);
    /// ```
    ///
    /// # 图片限制
    /// - 表格内不重复的图片（浮动图片+单元格图片）总数不超过4000
    /// - 浮动图片Token需要先通过上传图片API获得
    /// - 范围只支持单个单元格
    /// - 图片ID长度不超过10个字符
    ///
    /// # 错误处理
    /// - 如果电子表格不存在，返回相应错误
    /// - 如果工作表不存在，返回相应错误
    /// - 如果图片Token无效，返回相应错误
    /// - 如果图片数量超限，返回相应错误
    pub async fn create(
        &self,
        request: &CreateFloatImageRequest,
    ) -> openlark_core::error::SDKResult<Response<CreateFloatImageResponse>> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str()
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &url);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response =
            Transport::<CreateFloatImageResponse>::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        Ok(response)
    }

    /// 更新浮动图片
    ///
    /// 更新现有的浮动图片位置和大小。
    ///
    /// # 参数
    /// - `request`: 更新浮动图片请求，包含图片ID和更新字段
    ///
    /// # 返回
    /// 返回更新后的浮动图片信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{FloatImagesService, UpdateFloatImageRequest};
    ///
    /// let service = FloatImagesService::new(client_config);
    ///
    /// // 更新图片位置和大小
    /// let request = UpdateFloatImageRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .float_image_id("img_789".to_string())
    ///     .update_all()
    ///     .range("B2".to_string())
    ///     .size(300, 120)
    ///     .offset(0, 0)
    ///     .build()?;
    ///
    /// let image = service.update(&request).await?;
    /// println!("更新浮动图片成功: {}", image.data.float_image_id);
    /// ```
    ///
    /// # 更新字段
    /// - `range`: 图片所在范围（单个单元格）
    /// - `width`: 图片宽度
    /// - `height`: 图片高度
    /// - `offsetX`: X轴偏移
    /// - `offsetY`: Y轴偏移
    ///
    /// # 注意事项
    /// - 浮动图片ID和float_image_token不能更新
    /// - 更新操作会保留其他未指定的属性
    pub async fn update(
        &self,
        request: &UpdateFloatImageRequest,
    ) -> openlark_core::error::SDKResult<Response<UpdateFloatImageResponse>> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str(),
            request.float_image_id
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::PATCH, &url);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response =
            Transport::<UpdateFloatImageResponse>::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        Ok(response)
    }

    /// 查询浮动图片列表
    ///
    /// 查询指定工作表中的浮动图片列表。
    ///
    /// # 参数
    /// - `request`: 查询浮动图片请求，包含筛选条件和分页参数
    ///
    /// # 返回
    /// 返回浮动图片列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{FloatImagesService, QueryFloatImagesRequest};
    ///
    /// let service = FloatImagesService::new(client_config);
    ///
    /// // 查询所有浮动图片
    /// let request = QueryFloatImagesRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .page_size(20)
    ///     .build()?;
    ///
    /// let response = service.query(&request).await?;
    ///
    /// for image in &response.data.items {
    ///     println!("图片ID: {}, 范围: {}, 尺寸: {}x{}",
    ///         image.float_image_id, image.range, image.width, image.height);
    /// }
    /// ```
    ///
    /// # 分页
    /// - 支持`page_size`和`page_token`参数进行分页查询
    /// - 默认`page_size`为20，最大为100
    /// - 可以通过指定浮动图片ID列表进行精确查询
    pub async fn query(
        &self,
        request: &QueryFloatImagesRequest,
    ) -> openlark_core::error::SDKResult<Response<QueryFloatImagesResponse>> {
        let mut url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/query",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str()
        );

        // 构建查询参数
        let mut params = vec![];

        if !request.float_image_ids.is_empty() {
            params.push(format!(
                "float_image_ids={}",
                request.float_image_ids.join(",")
            ));
        }

        if let Some(page_size) = request.page_size {
            params.push(format!("page_size={}", page_size));
        }

        if let Some(page_token) = &request.page_token {
            params.push(format!("page_token={}", page_token));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        // 创建HTTP请求
        let api_request = ApiRequest::with_method_and_path(Method::GET, &url);

        // 发送请求并获取响应
        let response =
            Transport::<QueryFloatImagesResponse>::request(api_request, &self.config, None).await?;

        // 检查响应是否成功
        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        Ok(response)
    }

    /// 获取浮动图片信息
    ///
    /// 获取指定浮动图片的详细信息。
    ///
    /// # 参数
    /// - `request`: 获取浮动图片请求，包含图片ID
    ///
    /// # 返回
    /// 返回浮动图片的详细信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{FloatImagesService, GetFloatImageRequest};
    ///
    /// let service = FloatImagesService::new(client_config);
    ///
    /// let request = GetFloatImageRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .float_image_id("img_789".to_string())
    ///     .build()?;
    ///
    /// let image = service.get(&request).await?;
    ///
    /// println!("图片ID: {}", image.data.float_image_id);
    /// println!("图片Token: {}", image.data.float_image_token);
    /// println!("位置: {}, 偏移: ({}, {})",
    ///     image.data.range, image.data.offset_x, image.data.offset_y);
    /// println!("尺寸: {}x{}", image.data.width, image.data.height);
    /// ```
    pub async fn get(
        &self,
        request: &GetFloatImageRequest,
    ) -> SDKResult<Response<GetFloatImageResponse>> {
        let endpoint = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str(),
            request.float_image_id
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::GET, &endpoint);
        api_request.supported_access_token_types =
            vec![AccessTokenType::Tenant, AccessTokenType::User];

        Transport::<GetFloatImageResponse>::request(api_request, &self.config, None).await
    }

    /// 删除浮动图片
    ///
    /// 删除指定的浮动图片。
    ///
    /// # 参数
    /// - `request`: 删除浮动图片请求，包含图片ID
    ///
    /// # 返回
    /// 返回删除操作结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{FloatImagesService, DeleteFloatImageRequest};
    ///
    /// let service = FloatImagesService::new(client_config);
    ///
    /// let request = DeleteFloatImageRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .float_image_id("img_789".to_string())
    ///     .build()?;
    ///
    /// let response = service.delete(&request).await?;
    /// println!("删除浮动图片成功");
    /// ```
    ///
    /// # 注意事项
    /// - 删除操作不可撤销
    /// - 需要相应的管理权限
    /// - 删除后图片ID会释放，可以重复使用
    pub async fn delete(
        &self,
        request: &DeleteFloatImageRequest,
    ) -> openlark_core::error::SDKResult<Response<DeleteFloatImageResponse>> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str(),
            request.float_image_id
        );

        // 创建HTTP请求
        let api_request = ApiRequest::with_method_and_path(Method::DELETE, &url);

        // 发送请求并获取响应
        let response =
            Transport::<DeleteFloatImageResponse>::request(api_request, &self.config, None).await?;

        // 检查响应是否成功
        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        Ok(response)
    }

    /// 创建浮动图片构建器
    ///
    /// 提供链式调用的构建器模式，便于快速创建浮动图片。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::FloatImagesService;
    ///
    /// let service = FloatImagesService::new(client_config);
    ///
    /// let image = service
    ///     .create_builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .float_image_token("img_token_123456".to_string())
    ///     .range("A1")
    ///     .size(200, 80)
    ///     .offset(50, 20)
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn create_builder(&self) -> FloatImagesServiceBuilder<'_> {
        FloatImagesServiceBuilder {
            service: self,
            operation_type: FloatImageOperationType::Create,
            spreadsheet_token: None,
            sheet_id: None,
            float_image_id: None,
            float_image_token: None,
            range: None,
            width: None,
            height: None,
            offset_x: None,
            offset_y: None,
            fields: vec![],
            float_image_ids: vec![],
            page_size: None,
            page_token: None,
        }
    }

    /// 更新浮动图片构建器
    ///
    /// 提供链式调用的构建器模式，便于快速更新浮动图片。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::FloatImagesService;
    ///
    /// let service = FloatImagesService::new(client_config);
    ///
    /// let image = service
    ///     .update_builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .float_image_id("img_789")
    ///     .update_all()
    ///     .range("B2")
    ///     .size(300, 120)
    ///     .offset(0, 0)
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn update_builder(&self) -> FloatImagesServiceBuilder<'_> {
        FloatImagesServiceBuilder {
            service: self,
            operation_type: FloatImageOperationType::Update,
            spreadsheet_token: None,
            sheet_id: None,
            float_image_id: None,
            float_image_token: None,
            range: None,
            width: None,
            height: None,
            offset_x: None,
            offset_y: None,
            fields: vec![],
            float_image_ids: vec![],
            page_size: None,
            page_token: None,
        }
    }
}

/// 浮动图片操作类型
enum FloatImageOperationType {
    Create,
    Update,
    Query,
    Get,
    Delete,
}

/// 浮动图片服务构建器
///
/// 提供流畅的API用于构建浮动图片操作请求。
pub struct FloatImagesServiceBuilder<'a> {
    service: &'a FloatImagesService,
    operation_type: FloatImageOperationType,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    float_image_id: Option<String>,
    float_image_token: Option<String>,
    range: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
    offset_x: Option<i32>,
    offset_y: Option<i32>,
    fields: Vec<String>,
    float_image_ids: Vec<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl<'a> FloatImagesServiceBuilder<'a> {
    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置浮动图片ID
    pub fn float_image_id(mut self, float_image_id: &str) -> Self {
        self.float_image_id = Some(float_image_id.to_string());
        self
    }

    /// 设置浮动图片Token
    pub fn float_image_token(mut self, float_image_token: String) -> Self {
        self.float_image_token = Some(float_image_token);
        self
    }

    /// 设置图片所在范围
    pub fn range(mut self, range: &str) -> Self {
        self.range = Some(range.to_string());
        self
    }

    /// 设置图片宽度
    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    /// 设置图片高度
    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    /// 设置图片尺寸
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// 设置X轴偏移
    pub fn offset_x(mut self, offset_x: i32) -> Self {
        self.offset_x = Some(offset_x);
        self
    }

    /// 设置Y轴偏移
    pub fn offset_y(mut self, offset_y: i32) -> Self {
        self.offset_y = Some(offset_y);
        self
    }

    /// 设置偏移量
    pub fn offset(mut self, offset_x: i32, offset_y: i32) -> Self {
        self.offset_x = Some(offset_x);
        self.offset_y = Some(offset_y);
        self
    }

    /// 添加更新字段（仅用于更新操作）
    pub fn add_field(mut self, field: &str) -> Self {
        self.fields.push(field.to_string());
        self
    }

    /// 更新所有字段（仅用于更新操作）
    pub fn update_all(self) -> Self {
        self.fields(vec!["range", "width", "height", "offsetX", "offsetY"])
    }

    /// 设置更新字段列表（仅用于更新操作）
    pub fn fields(mut self, fields: Vec<&str>) -> Self {
        self.fields = fields.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// 添加浮动图片ID（仅用于查询操作）
    pub fn add_float_image_id(mut self, float_image_id: &str) -> Self {
        self.float_image_ids.push(float_image_id.to_string());
        self
    }

    /// 设置浮动图片ID列表（仅用于查询操作）
    pub fn float_image_ids(mut self, float_image_ids: Vec<&str>) -> Self {
        self.float_image_ids = float_image_ids.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// 设置页大小（仅用于查询操作）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页令牌（仅用于查询操作）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行操作
    pub async fn execute(self) -> openlark_core::error::SDKResult<serde_json::Value> {
        match self.operation_type {
            FloatImageOperationType::Create => {
                let request = CreateFloatImageRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_token: self.float_image_token.unwrap(),
                    range: self.range.unwrap(),
                    float_image_id: self.float_image_id,
                    width: self.width,
                    height: self.height,
                    offset_x: self.offset_x,
                    offset_y: self.offset_y,
                };
                let response = self.service.create(&request).await?;
                Ok(serde_json::to_value(response.data)?)
            }
            FloatImageOperationType::Update => {
                let request = UpdateFloatImageRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_id: self.float_image_id.unwrap(),
                    fields: self.fields,
                    range: self.range,
                    width: self.width,
                    height: self.height,
                    offset_x: self.offset_x,
                    offset_y: self.offset_y,
                };
                let response = self.service.update(&request).await?;
                Ok(serde_json::to_value(response.data)?)
            }
            FloatImageOperationType::Query => {
                let request = QueryFloatImagesRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_ids: self.float_image_ids,
                    page_size: self.page_size,
                    page_token: self.page_token,
                };
                let response = self.service.query(&request).await?;
                Ok(serde_json::to_value(response.data)?)
            }
            FloatImageOperationType::Get => {
                let request = GetFloatImageRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_id: self.float_image_id.unwrap(),
                };
                let response = self.service.get(&request).await?;
                Ok(serde_json::to_value(response.data)?)
            }
            FloatImageOperationType::Delete => {
                let request = DeleteFloatImageRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    float_image_id: self.float_image_id.unwrap(),
                };
                let response = self.service.delete(&request).await?;
                Ok(serde_json::to_value(response.data)?)
            }
        }
    }
}

// 为响应类型实现 ApiResponseTrait
impl ApiResponseTrait for CreateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QueryFloatImagesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_float_images_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = FloatImagesService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_create_float_image_request_builder() {
        let request = CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("img_token_123456".to_string())
            .range("A1".to_string())
            .float_image_id("img_001".to_string())
            .size(200, 80)
            .offset(50, 20)
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.float_image_token, "img_token_123456");
        assert_eq!(req.range, "A1");
        assert_eq!(req.float_image_id, Some("img_001".to_string()));
        assert_eq!(req.width, Some(200));
        assert_eq!(req.height, Some(80));
        assert_eq!(req.offset_x, Some(50));
        assert_eq!(req.offset_y, Some(20));
    }

    #[test]
    fn test_update_float_image_request_builder() {
        let request = UpdateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_id("img_789".to_string())
            .update_all()
            .range("B2".to_string())
            .size(300, 120)
            .offset(0, 0)
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.float_image_id, "img_789");
        assert_eq!(req.fields.len(), 5);
        assert_eq!(req.range, Some("B2".to_string()));
        assert_eq!(req.width, Some(300));
        assert_eq!(req.height, Some(120));
        assert_eq!(req.offset_x, Some(0));
        assert_eq!(req.offset_y, Some(0));
    }

    #[test]
    fn test_query_float_images_request_builder() {
        let request = QueryFloatImagesRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .add_float_image_id("img_001".to_string())
            .add_float_image_id("img_002".to_string())
            .page_size(20)
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.float_image_ids.len(), 2);
        assert_eq!(req.page_size, Some(20));
    }

    #[test]
    fn test_get_float_image_request_builder() {
        let request = GetFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_id("img_789".to_string())
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.float_image_id, "img_789");
    }

    #[test]
    fn test_delete_float_image_request_builder() {
        let request = DeleteFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_id("img_789".to_string())
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.float_image_id, "img_789");
    }

    #[test]
    fn test_float_images_request_validation() {
        // 测试缺少必需参数
        let no_token_request = CreateFloatImageRequest::builder()
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("img_token_123456".to_string())
            .range("A1".to_string())
            .build();
        assert!(no_token_request.is_err());

        // 测试空浮动图片Token
        let empty_token = CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("".to_string())
            .range("A1".to_string())
            .build();
        assert!(empty_token.is_err());

        // 测试空范围
        let empty_range = CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("img_token_123456".to_string())
            .range("".to_string())
            .build();
        assert!(empty_range.is_err());

        // 测试过长的图片ID
        let too_long_id = CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("img_token_123456".to_string())
            .range("A1".to_string())
            .float_image_id("12345678901".to_string()) // 11个字符
            .build();
        assert!(too_long_id.is_err());

        // 测试无效尺寸
        let invalid_width = CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("img_token_123456".to_string())
            .range("A1".to_string())
            .width(-10) // 负数宽度
            .build();
        assert!(invalid_width.is_err());

        // 测试过大尺寸
        let too_large_size = CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("img_token_123456".to_string())
            .range("A1".to_string())
            .width(20000) // 超过限制
            .build();
        assert!(too_large_size.is_err());

        // 测试过大偏移
        let too_large_offset = CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("img_token_123456".to_string())
            .range("A1".to_string())
            .offset_x(20000) // 超过限制
            .build();
        assert!(too_large_offset.is_err());
    }

    #[test]
    fn test_float_images_service_builder() {
        let config = openlark_core::config::Config::default();
        let service = FloatImagesService::new(config);

        // 测试创建构建器
        let create_builder = service
            .create_builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("img_token_123456".to_string())
            .range("A1")
            .size(200, 80)
            .offset(50, 20);

        assert_eq!(
            create_builder.spreadsheet_token.unwrap().as_str(),
            "shtcnmBRWQKbsJRHXXXXXXXXXX"
        );
        assert_eq!(create_builder.sheet_id.unwrap().as_str(), "0XXXXXXXXXX");
        assert_eq!(
            create_builder.float_image_token.unwrap(),
            "img_token_123456"
        );
        assert_eq!(create_builder.range.unwrap(), "A1");
        assert_eq!(create_builder.width, Some(200));
        assert_eq!(create_builder.height, Some(80));
        assert_eq!(create_builder.offset_x, Some(50));
        assert_eq!(create_builder.offset_y, Some(20));

        // 测试更新构建器
        let update_builder = service
            .update_builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_id("img_789")
            .update_all()
            .range("B2")
            .size(300, 120)
            .offset(0, 0);

        assert_eq!(update_builder.float_image_id.unwrap(), "img_789");
        assert_eq!(update_builder.fields.len(), 5);
        assert_eq!(update_builder.range.unwrap(), "B2");
    }

    #[test]
    fn test_complete_float_image_workflow() {
        let config = openlark_core::config::Config::default();
        let service = FloatImagesService::new(config);

        // 场景1: 创建公司Logo图片
        let logo_request = CreateFloatImageRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_token("company_logo_token".to_string())
            .range("A1")
            .float_image_id("logo_001".to_string())
            .size(300, 100)
            .offset(0, 0)
            .build()
            .unwrap();

        assert_eq!(logo_request.float_image_id, Some("logo_001".to_string()));
        assert_eq!(logo_request.width, Some(300));
        assert_eq!(logo_request.height, Some(100));

        // 场景2: 批量创建产品图片
        let product_images = vec![
            ("product_a_token", "A5", "prod_a_001", 150, 150),
            ("product_b_token", "A6", "prod_b_002", 150, 150),
            ("product_c_token", "A7", "prod_c_003", 150, 150),
        ];

        for (token, range, id, width, height) in product_images {
            let request = CreateFloatImageRequest::builder()
                .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
                .sheet_id("0XXXXXXXXXX".to_string())
                .float_image_token(token.to_string())
                .range(range.to_string())
                .float_image_id(id.to_string())
                .size(width, height)
                .build()
                .unwrap();

            assert_eq!(request.float_image_token, token);
            assert_eq!(request.range, range);
            assert_eq!(request.width, Some(width));
            assert_eq!(request.height, Some(height));
        }

        // 场景3: 查询和管理图片
        let query_request = QueryFloatImagesRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .float_image_ids(vec![
                "logo_001".to_string(),
                "prod_a_001".to_string(),
                "prod_b_002".to_string(),
            ])
            .page_size(10)
            .build()
            .unwrap();

        assert_eq!(query_request.float_image_ids.len(), 3);
        assert_eq!(query_request.page_size, Some(10));

        // 场景4: 批量更新图片位置
        let update_requests = vec![
            ("logo_001", "B1", 350, 120, 0, 0),
            ("prod_a_001", "B5", 180, 180, 10, 10),
            ("prod_b_001", "C5", 180, 180, 10, 10),
        ];

        for (id, range, width, height, offset_x, offset_y) in update_requests {
            let request = UpdateFloatImageRequest::builder()
                .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
                .sheet_id("0XXXXXXXXXX".to_string())
                .float_image_id(id.to_string())
                .update_all()
                .range(range.to_string())
                .size(width, height)
                .offset(offset_x, offset_y)
                .build()
                .unwrap();

            assert_eq!(request.float_image_id, id);
            assert_eq!(request.range.unwrap(), range);
            assert_eq!(request.width, Some(width));
            assert_eq!(request.height, Some(height));
            assert_eq!(request.offset_x, Some(offset_x));
            assert_eq!(request.offset_y, Some(offset_y));
        }
    }
}
