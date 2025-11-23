//! 筛选视图服务
//!
//! 提供飞书电子表格v3版本的筛选视图功能，包括：
//! - 创建、获取、更新、删除筛选视图
//! - 筛选条件的管理和操作
//! - 多条件筛选支持
//! - 筛选状态和数据同步

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 筛选条件操作符
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FilterOperator {
    /// 等于
    Equal,
    /// 不等于
    NotEqual,
    /// 包含
    Contains,
    /// 不包含
    DoesNotContain,
    /// 开始于
    BeginsWith,
    /// 结束于
    EndsWith,
    /// 为空
    IsEmpty,
    /// 不为空
    IsNotEmpty,
    /// 大于
    GreaterThan,
    /// 大于等于
    GreaterThanOrEqual,
    /// 小于
    LessThan,
    /// 小于等于
    LessThanOrEqual,
    /// 介于
    Between,
    /// 不介于
    NotBetween,
}

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// 筛选条件ID
    #[serde(rename = "conditionId", skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    /// 列索引
    #[serde(rename = "columnIndex")]
    pub column_index: i32,
    /// 操作符
    #[serde(rename = "operator")]
    pub operator: FilterOperator,
    /// 筛选值
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    /// 是否忽略大小写
    #[serde(rename = "ignoreCase", skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
}

impl FilterCondition {
    /// 创建新的筛选条件
    pub fn new(column_index: i32, operator: FilterOperator) -> Self {
        Self {
            condition_id: None,
            column_index,
            operator,
            values: None,
            ignore_case: Some(false),
        }
    }

    /// 设置条件ID
    pub fn condition_id(mut self, condition_id: impl Into<String>) -> Self {
        self.condition_id = Some(condition_id.into());
        self
    }

    /// 设置筛选值
    pub fn values(mut self, values: Vec<impl Into<String>>) -> Self {
        let converted_values: Vec<String> = values.into_iter().map(|v| v.into()).collect();
        self.values = Some(converted_values);
        self
    }

    /// 设置单个筛选值
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.values = Some(vec![value.into()]);
        self
    }

    /// 设置是否忽略大小写
    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = Some(ignore_case);
        self
    }

    /// 验证筛选条件
    pub fn validate(&self) -> Result<(), String> {
        if self.column_index < 0 {
            return Err("列索引不能小于0".to_string());
        }

        // 检查特定操作符的值要求
        match self.operator {
            FilterOperator::Equal
            | FilterOperator::NotEqual
            | FilterOperator::Contains
            | FilterOperator::DoesNotContain
            | FilterOperator::BeginsWith
            | FilterOperator::EndsWith => {
                if self.values.as_ref().map_or(true, |v| v.is_empty()) {
                    return Err(format!("{:?} 操作符需要设置筛选值", self.operator));
                }
            }
            FilterOperator::Between | FilterOperator::NotBetween => {
                let values_count = self.values.as_ref().map_or(0, |v| v.len());
                if values_count != 2 {
                    return Err(format!("{:?} 操作符需要设置2个筛选值", self.operator));
                }
            }
            FilterOperator::GreaterThan
            | FilterOperator::GreaterThanOrEqual
            | FilterOperator::LessThan
            | FilterOperator::LessThanOrEqual => {
                if self.values.as_ref().map_or(true, |v| v.is_empty()) {
                    return Err(format!("{:?} 操作符需要设置筛选值", self.operator));
                }
                // 验证数字格式
                if let Some(ref values) = self.values {
                    for value in values {
                        if value.trim().parse::<f64>().is_err() {
                            return Err(format!(
                                "{:?} 操作符的筛选值必须是有效的数字: {}",
                                self.operator, value
                            ));
                        }
                    }
                }
            }
            FilterOperator::IsEmpty | FilterOperator::IsNotEmpty => {
                if self.values.is_some() && !self.values.as_ref().unwrap().is_empty() {
                    return Err(format!("{:?} 操作符不应设置筛选值", self.operator));
                }
            }
        }

        // 检查值数量限制（最多10个值）
        if let Some(ref values) = self.values {
            if values.len() > 10 {
                return Err("单个筛选条件最多支持10个筛选值".to_string());
            }
        }

        Ok(())
    }
}

/// 筛选视图配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterView {
    /// 筛选视图ID
    #[serde(rename = "filterViewId", skip_serializing_if = "Option::is_none")]
    pub filter_view_id: Option<String>,
    /// 筛选视图名称
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否显示在工具栏
    #[serde(rename = "showInToolbar", skip_serializing_if = "Option::is_none")]
    pub show_in_toolbar: Option<bool>,
    /// 筛选条件列表
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<FilterCondition>>,
}

impl FilterView {
    /// 创建新的筛选视图配置
    pub fn new() -> Self {
        Self {
            filter_view_id: None,
            name: None,
            show_in_toolbar: Some(true),
            conditions: None,
        }
    }

    /// 设置筛选视图ID
    pub fn filter_view_id(mut self, filter_view_id: impl Into<String>) -> Self {
        self.filter_view_id = Some(filter_view_id.into());
        self
    }

    /// 设置筛选视图名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置是否显示在工具栏
    pub fn show_in_toolbar(mut self, show: bool) -> Self {
        self.show_in_toolbar = Some(show);
        self
    }

    /// 添加筛选条件
    pub fn add_condition(mut self, condition: FilterCondition) -> Self {
        let conditions = self.conditions.unwrap_or_default();
        let mut new_conditions = conditions;
        new_conditions.push(condition);
        self.conditions = Some(new_conditions);
        self
    }

    /// 批量添加筛选条件
    pub fn add_conditions(mut self, conditions: Vec<FilterCondition>) -> Self {
        let mut all_conditions = self.conditions.unwrap_or_default();
        all_conditions.extend(conditions);
        self.conditions = Some(all_conditions);
        self
    }

    /// 验证筛选视图配置
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("筛选视图名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("筛选视图名称长度不能超过100个字符".to_string());
            }
        }

        if let Some(ref conditions) = self.conditions {
            if conditions.is_empty() {
                return Err("筛选视图至少需要包含一个筛选条件".to_string());
            }
            if conditions.len() > 50 {
                return Err("单个筛选视图最多支持50个筛选条件".to_string());
            }

            // 验证每个筛选条件
            for (index, condition) in conditions.iter().enumerate() {
                if let Err(err) = condition.validate() {
                    return Err(format!("第{}个筛选条件验证失败: {}", index + 1, err));
                }
            }
        }

        Ok(())
    }
}

/// 创建筛选视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 筛选视图配置
    #[serde(rename = "filterView")]
    pub filter_view: FilterView,
}

impl CreateFilterViewRequest {
    /// 创建新的筛选视图请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        filter_view: FilterView,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            filter_view,
        }
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        self.filter_view.validate()
    }
}

/// 创建筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewResponse {
    /// 响应数据
    pub data: CreateFilterViewData,
}

/// 创建筛选视图数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 筛选视图信息
    #[serde(rename = "filterView")]
    pub filter_view: FilterViewResponse,
}

/// 筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterViewResponse {
    /// 筛选视图ID
    #[serde(rename = "filterViewId")]
    pub filter_view_id: String,
    /// 筛选视图名称
    #[serde(rename = "name")]
    pub name: String,
    /// 是否显示在工具栏
    #[serde(rename = "showInToolbar")]
    pub show_in_toolbar: bool,
    /// 筛选条件数量
    #[serde(rename = "conditionCount")]
    pub condition_count: i32,
    /// 创建时间
    #[serde(rename = "createTime")]
    pub create_time: String,
    /// 更新时间
    #[serde(rename = "updateTime")]
    pub update_time: String,
}

impl Default for CreateFilterViewResponse {
    fn default() -> Self {
        Self {
            data: CreateFilterViewData {
                spreadsheet_token: String::new(),
                filter_view: FilterViewResponse {
                    filter_view_id: String::new(),
                    name: String::new(),
                    show_in_toolbar: false,
                    condition_count: 0,
                    create_time: String::new(),
                    update_time: String::new(),
                },
            },
        }
    }
}

impl ApiResponseTrait for CreateFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterViewRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 筛选视图ID
    #[serde(rename = "filterViewId")]
    pub filter_view_id: String,
}

impl GetFilterViewRequest {
    /// 创建新的获取筛选视图请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        filter_view_id: impl Into<String>,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            filter_view_id: filter_view_id.into(),
        }
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        if self.filter_view_id.trim().is_empty() {
            return Err("筛选视图ID不能为空".to_string());
        }

        Ok(())
    }
}

/// 获取筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterViewResponse {
    /// 响应数据
    pub data: GetFilterViewData,
}

/// 获取筛选视图数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterViewData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 筛选视图信息
    #[serde(rename = "filterView")]
    pub filter_view: FilterViewResponse,
    /// 筛选条件列表
    #[serde(rename = "conditions")]
    pub conditions: Vec<FilterCondition>,
}

impl Default for GetFilterViewResponse {
    fn default() -> Self {
        Self {
            data: GetFilterViewData {
                spreadsheet_token: String::new(),
                filter_view: FilterViewResponse {
                    filter_view_id: String::new(),
                    name: String::new(),
                    show_in_toolbar: false,
                    condition_count: 0,
                    create_time: String::new(),
                    update_time: String::new(),
                },
                conditions: vec![],
            },
        }
    }
}

impl ApiResponseTrait for GetFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 筛选视图ID
    #[serde(rename = "filterViewId")]
    pub filter_view_id: String,
    /// 更新的筛选视图配置
    #[serde(rename = "filterView")]
    pub filter_view: FilterView,
}

impl UpdateFilterViewRequest {
    /// 创建新的更新筛选视图请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        filter_view_id: impl Into<String>,
        filter_view: FilterView,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            filter_view_id: filter_view_id.into(),
            filter_view,
        }
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        if self.filter_view_id.trim().is_empty() {
            return Err("筛选视图ID不能为空".to_string());
        }

        self.filter_view.validate()
    }
}

/// 更新筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewResponse {
    /// 响应数据
    pub data: UpdateFilterViewData,
}

/// 更新筛选视图数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 筛选视图信息
    #[serde(rename = "filterView")]
    pub filter_view: FilterViewResponse,
}

impl Default for UpdateFilterViewResponse {
    fn default() -> Self {
        Self {
            data: UpdateFilterViewData {
                spreadsheet_token: String::new(),
                filter_view: FilterViewResponse {
                    filter_view_id: String::new(),
                    name: String::new(),
                    show_in_toolbar: false,
                    condition_count: 0,
                    create_time: String::new(),
                    update_time: String::new(),
                },
            },
        }
    }
}

impl ApiResponseTrait for UpdateFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterViewRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 筛选视图ID
    #[serde(rename = "filterViewId")]
    pub filter_view_id: String,
}

impl DeleteFilterViewRequest {
    /// 创建新的删除筛选视图请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        filter_view_id: impl Into<String>,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            filter_view_id: filter_view_id.into(),
        }
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        if self.filter_view_id.trim().is_empty() {
            return Err("筛选视图ID不能为空".to_string());
        }

        Ok(())
    }
}

/// 删除筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterViewResponse {
    /// 响应数据
    pub data: DeleteFilterViewData,
}

/// 删除筛选视图数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterViewData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 筛选视图ID
    #[serde(rename = "filterViewId")]
    pub filter_view_id: String,
    /// 删除状态
    #[serde(rename = "status")]
    pub status: String,
}

impl Default for DeleteFilterViewResponse {
    fn default() -> Self {
        Self {
            data: DeleteFilterViewData {
                spreadsheet_token: String::new(),
                filter_view_id: String::new(),
                status: "success".to_string(),
            },
        }
    }
}

impl ApiResponseTrait for DeleteFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 筛选视图服务
#[derive(Clone, Debug)]
pub struct FilterViewsService {
    config: Config,
}

impl FilterViewsService {
    /// 创建筛选视图服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建筛选视图
    ///
    /// 为指定的工作表创建新的筛选视图，支持多个筛选条件的组合。
    ///
    /// # 参数
    /// - `request`: 创建筛选视图请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let condition1 = FilterCondition::new(0, FilterOperator::Equal)
    ///     .values(vec!["北京".to_string(), "上海".to_string()]);
    ///
    /// let condition2 = FilterCondition::new(1, FilterOperator::Contains)
    ///     .value("技术部");
    ///
    /// let filter_view = FilterView::new()
    ///     .name("销售数据筛选")
    ///     .show_in_toolbar(true)
    ///     .add_condition(condition1)
    ///     .add_condition(condition2);
    ///
    /// let request = CreateFilterViewRequest::new(
    ///     "spreadsheet_token",
    ///     "sheet1",
    ///     filter_view
    /// );
    ///
    /// let response = service.create_filter_view(request, None).await?;
    /// println!("筛选视图ID: {}", response.data.filter_view.filter_view_id);
    /// ```
    pub async fn create_filter_view(
        &self,
        request: CreateFilterViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CreateFilterViewResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::IllegalParamError(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::POST,
            format!(
                "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views",
                &request.spreadsheet_token, &request.sheet_id
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<CreateFilterViewResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 获取筛选视图
    ///
    /// 获取指定筛选视图的详细信息，包括所有筛选条件。
    ///
    /// # 参数
    /// - `request`: 获取筛选视图请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let request = GetFilterViewRequest::new(
    ///     "spreadsheet_token",
    ///     "sheet1",
    ///     "filter_view_123"
    /// );
    ///
    /// let response = service.get_filter_view(request, None).await?;
    /// println!("筛选视图名称: {}", response.data.filter_view.name);
    /// println!("筛选条件数量: {}", response.data.conditions.len());
    /// ```
    pub async fn get_filter_view(
        &self,
        request: GetFilterViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<GetFilterViewResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::IllegalParamError(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::GET,
            format!(
                "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}",
                &request.spreadsheet_token, &request.sheet_id, &request.filter_view_id
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 发送请求
        let api_resp =
            Transport::<GetFilterViewResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 更新筛选视图
    ///
    /// 更新已存在的筛选视图的配置，包括名称、显示设置和筛选条件。
    ///
    /// # 参数
    /// - `request`: 更新筛选视图请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let updated_filter_view = FilterView::new()
    ///     .name("更新后的筛选视图")
    ///     .show_in_toolbar(false)
    ///     .add_condition(FilterCondition::new(2, FilterOperator::NotEmpty));
    ///
    /// let request = UpdateFilterViewRequest::new(
    ///     "spreadsheet_token",
    ///     "sheet1",
    ///     "filter_view_123",
    ///     updated_filter_view
    /// );
    ///
    /// let response = service.update_filter_view(request, None).await?;
    /// println!("更新状态: {}", response.data.filter_view.name);
    /// ```
    pub async fn update_filter_view(
        &self,
        request: UpdateFilterViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UpdateFilterViewResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::IllegalParamError(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::PATCH,
            format!(
                "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}",
                &request.spreadsheet_token, &request.sheet_id, &request.filter_view_id
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<UpdateFilterViewResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 删除筛选视图
    ///
    /// 删除指定的筛选视图，删除后所有相关的筛选条件将被清除。
    ///
    /// # 参数
    /// - `request`: 删除筛选视图请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let request = DeleteFilterViewRequest::new(
    ///     "spreadsheet_token",
    ///     "sheet1",
    ///     "filter_view_123"
    /// );
    ///
    /// let response = service.delete_filter_view(request, None).await?;
    /// println!("删除状态: {}", response.data.status);
    /// ```
    pub async fn delete_filter_view(
        &self,
        request: DeleteFilterViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<DeleteFilterViewResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::IllegalParamError(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::DELETE,
            format!(
                "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}",
                &request.spreadsheet_token, &request.sheet_id, &request.filter_view_id
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 发送请求
        let api_resp =
            Transport::<DeleteFilterViewResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 创建筛选视图构建器
    pub fn create_filter_view_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
    ) -> CreateFilterViewBuilder {
        CreateFilterViewBuilder::new(
            self.config.clone(),
            spreadsheet_token.into(),
            sheet_id.into(),
        )
    }

    /// 更新筛选视图构建器
    pub fn update_filter_view_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        filter_view_id: impl Into<String>,
    ) -> UpdateFilterViewBuilder {
        UpdateFilterViewBuilder::new(
            self.config.clone(),
            spreadsheet_token.into(),
            sheet_id.into(),
            filter_view_id.into(),
        )
    }
}

/// 创建筛选视图构建器
#[derive(Clone, Debug)]
pub struct CreateFilterViewBuilder {
    config: Config,
    spreadsheet_token: String,
    sheet_id: String,
    filter_view: FilterView,
}

impl CreateFilterViewBuilder {
    /// 创建新的筛选视图构建器实例
    pub fn new(config: Config, spreadsheet_token: String, sheet_id: String) -> Self {
        Self {
            config,
            spreadsheet_token,
            sheet_id,
            filter_view: FilterView::new(),
        }
    }

    /// 设置筛选视图名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.filter_view = self.filter_view.name(name);
        self
    }

    /// 设置是否显示在工具栏
    pub fn show_in_toolbar(mut self, show: bool) -> Self {
        self.filter_view = self.filter_view.show_in_toolbar(show);
        self
    }

    /// 添加筛选条件
    pub fn add_condition(mut self, condition: FilterCondition) -> Self {
        self.filter_view = self.filter_view.add_condition(condition);
        self
    }

    /// 批量添加筛选条件
    pub fn add_conditions(mut self, conditions: Vec<FilterCondition>) -> Self {
        self.filter_view = self.filter_view.add_conditions(conditions);
        self
    }

    /// 执行创建请求
    pub async fn execute(self) -> SDKResult<Response<CreateFilterViewResponse>> {
        let request =
            CreateFilterViewRequest::new(self.spreadsheet_token, self.sheet_id, self.filter_view);

        let service = FilterViewsService {
            config: self.config,
        };
        service.create_filter_view(request, None).await
    }
}

/// 更新筛选视图构建器
#[derive(Clone, Debug)]
pub struct UpdateFilterViewBuilder {
    config: Config,
    spreadsheet_token: String,
    sheet_id: String,
    filter_view_id: String,
    filter_view: FilterView,
}

impl UpdateFilterViewBuilder {
    /// 创建新的更新筛选视图构建器实例
    pub fn new(
        config: Config,
        spreadsheet_token: String,
        sheet_id: String,
        filter_view_id: String,
    ) -> Self {
        Self {
            config,
            spreadsheet_token,
            sheet_id,
            filter_view_id,
            filter_view: FilterView::new(),
        }
    }

    /// 设置筛选视图名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.filter_view = self.filter_view.name(name);
        self
    }

    /// 设置是否显示在工具栏
    pub fn show_in_toolbar(mut self, show: bool) -> Self {
        self.filter_view = self.filter_view.show_in_toolbar(show);
        self
    }

    /// 清除现有筛选条件并添加新条件
    pub fn clear_conditions(mut self) -> Self {
        self.filter_view = FilterView::new()
            .name(self.filter_view.name.as_deref().unwrap_or_default())
            .show_in_toolbar(self.filter_view.show_in_toolbar.unwrap_or(true));
        self
    }

    /// 添加筛选条件
    pub fn add_condition(mut self, condition: FilterCondition) -> Self {
        self.filter_view = self.filter_view.add_condition(condition);
        self
    }

    /// 批量添加筛选条件
    pub fn add_conditions(mut self, conditions: Vec<FilterCondition>) -> Self {
        self.filter_view = self.filter_view.add_conditions(conditions);
        self
    }

    /// 执行更新请求
    pub async fn execute(self) -> SDKResult<Response<UpdateFilterViewResponse>> {
        let request = UpdateFilterViewRequest::new(
            self.spreadsheet_token,
            self.sheet_id,
            self.filter_view_id,
            self.filter_view,
        );

        let service = FilterViewsService {
            config: self.config,
        };
        service.update_filter_view(request, None).await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_operator_serialization() {
        let equal = FilterOperator::Equal;
        let serialized = serde_json::to_string(&equal).unwrap();
        assert_eq!(serialized, "\"EQUAL\"");

        let contains = FilterOperator::Contains;
        let serialized = serde_json::to_string(&contains).unwrap();
        assert_eq!(serialized, "\"CONTAINS\"");

        let between = FilterOperator::Between;
        let serialized = serde_json::to_string(&between).unwrap();
        assert_eq!(serialized, "\"BETWEEN\"");

        let is_empty = FilterOperator::IsEmpty;
        let serialized = serde_json::to_string(&is_empty).unwrap();
        assert_eq!(serialized, "\"IS_EMPTY\"");
    }

    #[test]
    fn test_filter_condition_creation() {
        let condition = FilterCondition::new(1, FilterOperator::Equal)
            .values(vec!["测试".to_string(), "示例".to_string()])
            .ignore_case(true);

        assert_eq!(condition.column_index, 1);
        assert_eq!(condition.operator, FilterOperator::Equal);
        assert_eq!(
            condition.values,
            Some(vec!["测试".to_string(), "示例".to_string()])
        );
        assert_eq!(condition.ignore_case, Some(true));
    }

    #[test]
    fn test_filter_condition_validation() {
        // 测试正常条件
        let valid_condition = FilterCondition::new(0, FilterOperator::Equal).value("测试值");
        assert!(valid_condition.validate().is_ok());

        // 测试 Between 操作符需要两个值
        let between_condition = FilterCondition::new(1, FilterOperator::Between)
            .values(vec!["10".to_string(), "20".to_string()]);
        assert!(between_condition.validate().is_ok());

        let invalid_between =
            FilterCondition::new(1, FilterOperator::Between).values(vec!["10".to_string()]);
        assert!(invalid_between.validate().is_err());

        // 测试数字操作符需要数字值
        let number_condition = FilterCondition::new(2, FilterOperator::GreaterThan).value("100.5");
        assert!(number_condition.validate().is_ok());

        let invalid_number = FilterCondition::new(2, FilterOperator::GreaterThan).value("abc");
        assert!(invalid_number.validate().is_err());

        // 测试 IsEmpty 不应该有值
        let empty_condition = FilterCondition::new(3, FilterOperator::IsEmpty);
        assert!(empty_condition.validate().is_ok());

        let invalid_empty = FilterCondition::new(3, FilterOperator::IsEmpty).value("测试");
        assert!(invalid_empty.validate().is_err());

        // 测试列索引不能为负数
        let invalid_column = FilterCondition::new(-1, FilterOperator::Equal);
        assert!(invalid_column.validate().is_err());

        // 测试值数量限制
        let too_many_values = FilterCondition::new(0, FilterOperator::Equal);
        let mut values = vec![];
        for i in 1..=11 {
            values.push(format!("值{}", i));
        }
        let too_many_condition = too_many_values.values(values);
        assert!(too_many_condition.validate().is_err());
    }

    #[test]
    fn test_filter_view_creation() {
        let condition1 = FilterCondition::new(0, FilterOperator::Equal)
            .values(vec!["北京".to_string(), "上海".to_string()]);
        let condition2 = FilterCondition::new(1, FilterOperator::Contains).value("技术");

        let filter_view = FilterView::new()
            .name("销售数据筛选")
            .show_in_toolbar(true)
            .add_condition(condition1)
            .add_condition(condition2);

        assert_eq!(filter_view.name, Some("销售数据筛选".to_string()));
        assert_eq!(filter_view.show_in_toolbar, Some(true));
        assert_eq!(filter_view.conditions.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_filter_view_validation() {
        // 测试正常的筛选视图
        let valid_filter = FilterView::new()
            .name("测试筛选")
            .add_condition(FilterCondition::new(0, FilterOperator::Equal).value("测试"));
        assert!(valid_filter.validate().is_ok());

        // 测试空名称
        let empty_name = FilterView::new()
            .name("")
            .add_condition(FilterCondition::new(0, FilterOperator::Equal).value("测试"));
        assert!(empty_name.validate().is_err());

        // 测试过长名称
        let long_name = FilterView::new()
            .name(&"a".repeat(101))
            .add_condition(FilterCondition::new(0, FilterOperator::Equal).value("测试"));
        assert!(long_name.validate().is_err());

        // 测试无筛选条件
        let no_conditions = FilterView::new().name("测试筛选");
        assert!(no_conditions.validate().is_err());

        // 测试过多条件
        let mut too_many_conditions = FilterView::new().name("测试筛选");
        for i in 1..=51 {
            too_many_conditions = too_many_conditions.add_condition(
                FilterCondition::new(0, FilterOperator::Equal).value(format!("值{}", i)),
            );
        }
        assert!(too_many_conditions.validate().is_err());
    }

    #[test]
    fn test_create_filter_view_request() {
        let filter_view = FilterView::new()
            .name("测试筛选视图")
            .add_condition(FilterCondition::new(0, FilterOperator::Equal).value("测试"));

        let request = CreateFilterViewRequest::new("test_token", "sheet1", filter_view);
        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_filter_view_request() {
        let request = GetFilterViewRequest::new("test_token", "sheet1", "filter_view_123");
        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.filter_view_id, "filter_view_123");
        assert!(request.validate().is_ok());

        // 测试空值验证
        let empty_token = GetFilterViewRequest::new("", "sheet1", "filter_view_123");
        assert!(empty_token.validate().is_err());

        let empty_sheet = GetFilterViewRequest::new("test_token", "", "filter_view_123");
        assert!(empty_sheet.validate().is_err());

        let empty_filter_id = GetFilterViewRequest::new("test_token", "sheet1", "");
        assert!(empty_filter_id.validate().is_err());
    }

    #[test]
    fn test_update_filter_view_request() {
        let filter_view = FilterView::new()
            .name("更新后的筛选")
            .add_condition(FilterCondition::new(1, FilterOperator::NotEmpty));

        let request =
            UpdateFilterViewRequest::new("test_token", "sheet1", "filter_456", filter_view);
        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.filter_view_id, "filter_456");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_delete_filter_view_request() {
        let request = DeleteFilterViewRequest::new("test_token", "sheet1", "filter_789");
        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.filter_view_id, "filter_789");
        assert!(request.validate().is_ok());

        // 测试空值验证
        let empty_token = DeleteFilterViewRequest::new("", "sheet1", "filter_789");
        assert!(empty_token.validate().is_err());

        let empty_sheet = DeleteFilterViewRequest::new("test_token", "", "filter_789");
        assert!(empty_sheet.validate().is_err());

        let empty_filter_id = DeleteFilterViewRequest::new("test_token", "sheet1", "");
        assert!(empty_filter_id.validate().is_err());
    }

    #[test]
    fn test_filter_views_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = FilterViewsService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_create_filter_view_builder() {
        let config = openlark_core::config::Config::default();
        let builder = FilterViewsService::new(config.clone())
            .create_filter_view_builder("token".to_string(), "sheet1".to_string())
            .name("销售数据筛选")
            .show_in_toolbar(true)
            .add_condition(
                FilterCondition::new(0, FilterOperator::Equal)
                    .values(vec!["北京".to_string(), "上海".to_string()]),
            )
            .add_condition(FilterCondition::new(1, FilterOperator::Contains).value("技术"));

        assert_eq!(builder.spreadsheet_token, "token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.filter_view.name, Some("销售数据筛选".to_string()));
        assert_eq!(builder.filter_view.conditions.as_ref().unwrap().len(), 2);
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_update_filter_view_builder() {
        let config = openlark_core::config::Config::default();
        let builder = FilterViewsService::new(config.clone())
            .update_filter_view_builder(
                "token".to_string(),
                "sheet1".to_string(),
                "filter_123".to_string(),
            )
            .name("更新后的筛选")
            .clear_conditions()
            .show_in_toolbar(false)
            .add_condition(FilterCondition::new(2, FilterOperator::NotEmpty));

        assert_eq!(builder.spreadsheet_token, "token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.filter_view_id, "filter_123");
        assert_eq!(builder.filter_view.name, Some("更新后的筛选".to_string()));
        assert_eq!(builder.filter_view.conditions.as_ref().unwrap().len(), 1);
        assert_eq!(builder.filter_view.show_in_toolbar, Some(false));
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_response_default() {
        let create_response = CreateFilterViewResponse::default();
        assert!(create_response.data.spreadsheet_token.is_empty());
        assert!(create_response.data.filter_view.filter_view_id.is_empty());

        let get_response = GetFilterViewResponse::default();
        assert!(get_response.data.spreadsheet_token.is_empty());
        assert!(get_response.data.conditions.is_empty());

        let update_response = UpdateFilterViewResponse::default();
        assert!(update_response.data.spreadsheet_token.is_empty());
        assert!(update_response.data.filter_view.filter_view_id.is_empty());

        let delete_response = DeleteFilterViewResponse::default();
        assert!(delete_response.data.spreadsheet_token.is_empty());
        assert!(delete_response.data.filter_view_id.is_empty());
        assert_eq!(delete_response.data.status, "success");
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            CreateFilterViewResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(GetFilterViewResponse::data_format(), ResponseFormat::Data);
        assert_eq!(
            UpdateFilterViewResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            DeleteFilterViewResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_condition = FilterCondition::new(5, FilterOperator::Between)
            .values(vec!["100".to_string(), "200".to_string()])
            .ignore_case(true);

        let serialized = serde_json::to_string(&original_condition).unwrap();
        let deserialized: FilterCondition = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_condition.column_index, deserialized.column_index);
        assert_eq!(original_condition.operator, deserialized.operator);
        assert_eq!(original_condition.ignore_case, deserialized.ignore_case);

        let values = deserialized.values.unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], "100");
        assert_eq!(values[1], "200");
    }

    #[test]
    fn test_complex_filter_scenarios() {
        // 测试复合筛选条件
        let complex_filter = FilterView::new()
            .name("复合筛选")
            .show_in_toolbar(true)
            .add_condition(
                FilterCondition::new(0, FilterOperator::Equal)
                    .values(vec!["产品A", "产品B", "产品C", "产品D"])
                    .ignore_case(true),
            )
            .add_condition(
                FilterCondition::new(1, FilterOperator::Between)
                    .values(vec!["100".to_string(), "1000".to_string()]),
            )
            .add_condition(FilterCondition::new(2, FilterOperator::NotEqual).value("已删除"))
            .add_condition(
                FilterCondition::new(3, FilterOperator::Contains)
                    .values(vec!["技术", "研发", "产品"]),
            );

        assert!(complex_filter.validate().is_ok());
        assert_eq!(complex_filter.conditions.as_ref().unwrap().len(), 4);

        // 验证每个条件的具体设置
        let conditions = complex_filter.conditions.as_ref().unwrap();

        // 第一个条件：等于操作符，多个值，忽略大小写
        let first_condition = &conditions[0];
        assert_eq!(first_condition.column_index, 0);
        assert_eq!(first_condition.operator, FilterOperator::Equal);
        assert_eq!(first_condition.values.as_ref().unwrap().len(), 4);
        assert_eq!(first_condition.ignore_case, Some(true));

        // 第二个条件：范围操作符，数字值
        let second_condition = &conditions[1];
        assert_eq!(second_condition.operator, FilterOperator::Between);
        assert_eq!(second_condition.values.as_ref().unwrap().len(), 2);
        assert_eq!(second_condition.values.as_ref().unwrap()[0], "100");
        assert_eq!(second_condition.values.as_ref().unwrap()[1], "1000");

        // 第三个条件：不等于操作符
        let third_condition = &conditions[2];
        assert_eq!(third_condition.operator, FilterOperator::NotEqual);
        assert_eq!(third_condition.values.as_ref().unwrap().len(), 1);
        assert_eq!(third_condition.values.as_ref().unwrap()[0], "已删除");

        // 第四个条件：包含操作符，多个值
        let fourth_condition = &conditions[3];
        assert_eq!(fourth_condition.operator, FilterOperator::Contains);
        assert_eq!(fourth_condition.values.as_ref().unwrap().len(), 3);
        assert!(fourth_condition
            .values
            .as_ref()
            .unwrap()
            .contains(&"技术".to_string()));
        assert!(fourth_condition
            .values
            .as_ref()
            .unwrap()
            .contains(&"研发".to_string()));
        assert!(fourth_condition
            .values
            .as_ref()
            .unwrap()
            .contains(&"产品".to_string()));
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界值 - 最小列索引
        let min_column = FilterCondition::new(0, FilterOperator::Equal).value("测试");
        assert!(min_column.validate().is_ok());

        // 测试边界值 - 最大单个值
        let single_value = FilterCondition::new(10, FilterOperator::Equal).value("单个值");
        assert!(single_value.validate().is_ok());

        // 测试边界值 - 10个值的限制
        let max_values = FilterCondition::new(0, FilterOperator::Equal);
        let mut values = vec![];
        for i in 1..=10 {
            values.push(format!("值{}", i));
        }
        let max_condition = max_values.values(values);
        assert!(max_condition.validate().is_ok());

        // 测试边界值 - 超过10个值
        let too_many_values = FilterCondition::new(0, FilterOperator::Equal);
        let mut too_many_values = vec![];
        for i in 1..=11 {
            too_many_values.push(format!("值{}", i));
        }
        let too_many_condition = too_many_values.values(too_many_values);
        assert!(too_many_condition.validate().is_err());

        // 测试边界值 - 最大列索引
        let max_column = FilterCondition::new(999999, FilterOperator::Equal).value("测试");
        assert!(max_column.validate().is_ok());

        // 测试边界值 - 超过最大列索引
        let too_large_column = FilterCondition::new(1000000, FilterOperator::Equal).value("测试");
        assert!(too_large_column.validate().is_err());
    }

    #[test]
    fn test_filter_view_name_edge_cases() {
        // 测试最小长度名称
        let min_name = FilterView::new()
            .name("A")
            .add_condition(FilterCondition::new(0, FilterOperator::Equal).value("测试"));
        assert!(min_name.validate().is_ok());

        // 测试最大长度名称
        let max_name = FilterView::new()
            .name(&"n".repeat(100))
            .add_condition(FilterCondition::new(0, FilterOperator::Equal).value("测试"));
        assert!(max_name.validate().is_ok());

        // 测试超长名称
        let too_long_name = FilterView::new()
            .name(&"n".repeat(101))
            .add_condition(FilterCondition::new(0, Filter().value("测试")));
        assert!(too_long_name.validate().is_err());

        // 测试空格名称
        let space_name = FilterView::new()
            .name(" ")
            .add_condition(FilterCondition::new(0, FilterOperator::Equal).value("测试"));
        assert!(space_name.validate().is_err());

        // 测试带特殊字符的名称
        let special_name = FilterView::new()
            .name("销售数据-筛选_2024")
            .add_condition(FilterCondition::new(0, FilterOperator::Equal).value("测试"));
        assert!(special_name.validate().is_ok());
    }

    /// 查询筛选视图
    ///
    /// 获取指定工作表中的所有筛选视图列表。
    ///
    /// # 参数
    /// - `request`: 查询筛选视图请求
    ///
    /// # 返回
    /// 返回筛选视图列表
    pub async fn query_filter_views(
        &self,
        request: &QueryFilterViewsRequest,
    ) -> openlark_core::error::SDKResult<QueryFilterViewsResponse> {
        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/query",
            request.spreadsheet_token, request.sheet_id
        );

        let mut api_request = ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Post, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response: Response<QueryFilterViewsResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 创建筛选视图条件
    ///
    /// 在指定筛选视图中添加新的筛选条件。
    ///
    /// # 参数
    /// - `request`: 创建筛选视图条件请求
    ///
    /// # 返回
    /// 返回创建后的筛选条件信息
    pub async fn create_filter_view_condition(
        &self,
        request: &CreateFilterViewConditionRequest,
    ) -> openlark_core::error::SDKResult<CreateFilterViewConditionResponse> {
        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/filter_views/{}/conditions",
            request.spreadsheet_token, request.filter_view_id
        );

        let mut api_request = ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Post, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response: Response<CreateFilterViewConditionResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 更新筛选视图条件
    ///
    /// 更新指定筛选视图中的筛选条件。
    ///
    /// # 参数
    /// - `request`: 更新筛选视图条件请求
    ///
    /// # 返回
    /// 返回更新后的筛选条件信息
    pub async fn update_filter_view_condition(
        &self,
        request: &UpdateFilterViewConditionRequest,
    ) -> openlark_core::error::SDKResult<UpdateFilterViewConditionResponse> {
        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/filter_views/{}/conditions/{}",
            request.spreadsheet_token, request.filter_view_id, request.condition_id
        );

        let mut api_request = ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Put, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response: Response<UpdateFilterViewConditionResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 删除筛选视图条件
    ///
    /// 删除指定筛选视图中的筛选条件。
    ///
    /// # 参数
    /// - `request`: 删除筛选视图条件请求
    ///
    /// # 返回
    /// 返回删除结果
    pub async fn delete_filter_view_condition(
        &self,
        request: &DeleteFilterViewConditionRequest,
    ) -> openlark_core::error::SDKResult<DeleteFilterViewConditionResponse> {
        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/filter_views/{}/conditions/{}",
            request.spreadsheet_token, request.filter_view_id, request.condition_id
        );

        let api_request = ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Delete, &endpoint);

        let response: Response<DeleteFilterViewConditionResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }
}

// 新增缺失的请求和响应类型

/// 查询筛选视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterViewsRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 分页大小
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    /// 页码
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
}

/// 查询筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterViewsResponse {
    /// 筛选视图列表
    #[serde(rename = "filter_views")]
    pub filter_views: Vec<FilterView>,
    /// 是否有更多数据
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 下一页token
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for QueryFilterViewsResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// 创建筛选视图条件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewConditionRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 筛选视图ID
    #[serde(rename = "filter_view_id")]
    pub filter_view_id: String,
    /// 筛选条件
    #[serde(rename = "condition")]
    pub condition: FilterCondition,
}

/// 创建筛选视图条件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewConditionResponse {
    /// 筛选条件ID
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    /// 筛选条件
    #[serde(rename = "condition")]
    pub condition: FilterCondition,
}

impl openlark_core::api::ApiResponseTrait for CreateFilterViewConditionResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// 更新筛选视图条件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewConditionRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 筛选视图ID
    #[serde(rename = "filter_view_id")]
    pub filter_view_id: String,
    /// 条件ID
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    /// 筛选条件
    #[serde(rename = "condition")]
    pub condition: FilterCondition,
}

/// 更新筛选视图条件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewConditionResponse {
    /// 筛选条件ID
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    /// 筛选条件
    #[serde(rename = "condition")]
    pub condition: FilterCondition,
}

impl openlark_core::api::ApiResponseTrait for UpdateFilterViewConditionResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// 删除筛选视图条件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterViewConditionRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 筛选视图ID
    #[serde(rename = "filter_view_id")]
    pub filter_view_id: String,
    /// 条件ID
    #[serde(rename = "condition_id")]
    pub condition_id: String,
}

/// 删除筛选视图条件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterViewConditionResponse {
    /// 是否成功
    #[serde(rename = "success")]
    pub success: bool,
}

impl openlark_core::api::ApiResponseTrait for DeleteFilterViewConditionResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}
