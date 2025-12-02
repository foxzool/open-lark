//! 检索单条记录模块
//!
//! 提供多维表格单条记录的检索功能，根据record_id获取现有记录。

use openlark_core::{
    api::{
        ApiRequest, ApiResponseTrait, BaseResponse, ResponseFormat, HttpMethod,
    },
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use crate::endpoints::BITABLE_V1_RECORD_GET;
use super::AppTableRecordService;

/// 检索单条记录请求
pub struct GetRecordRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 记录的 record_id
    pub record_id: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
    /// 视图的唯一标识符
    pub view_id: Option<String>,
    /// 字段名称，用于指定本次查询返回记录中包含的字段
    pub field_names: Option<Vec<String>>,
    /// 控制是否返回自动计算的字段
    pub automatic: Option<bool>,
}

impl Default for GetRecordRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::get(BITABLE_V1_RECORD_GET.to_string()),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            view_id: None,
            field_names: None,
            automatic: None,
        }
    }
}

impl GetRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::get(
                BITABLE_V1_RECORD_GET.to_string()
            ),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            view_id: None,
            field_names: None,
            automatic: None,
        }
    }

    pub fn builder() -> GetRecordRequestBuilder {
        GetRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetRecordRequestBuilder {
    request: GetRecordRequest,
}

impl GetRecordRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    pub fn record_id(mut self, record_id: impl Into<String>) -> Self {
        self.request.record_id = record_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn view_id(mut self, view_id: impl Into<String>) -> Self {
        self.request.view_id = Some(view_id.into());
        self
    }

    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.request.field_names = Some(field_names);
        self
    }

    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request.automatic = Some(automatic);
        self
    }

    pub fn build(self) -> GetRecordRequest {
        self.request
    }
}

/// 检索单条记录响应
#[derive(Debug, Clone)]
pub struct GetRecordResponse {
    /// 记录信息
    pub record: RecordInfo,
}

/// 记录信息
#[derive(Debug, Clone)]
pub struct RecordInfo {
    /// 记录的record_id
    pub record_id: String,
    /// 记录的字段信息
    pub fields: serde_json::Value,
    /// 记录的创建时间
    pub created_time: String,
    /// 记录的更新时间
    pub last_modified_time: String,
    /// 记录的创建者信息
    pub created_by: Option<CreatorInfo>,
    /// 记录的最后修改者信息
    pub last_modified_by: Option<UpdaterInfo>,
}

/// 创建者信息
#[derive(Debug, Clone)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 更新者信息
#[derive(Debug, Clone)]
pub struct UpdaterInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

impl ApiResponseTrait for GetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

