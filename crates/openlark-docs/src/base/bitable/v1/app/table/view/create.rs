//! Bitable 新增视图
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 新增视图请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CreateViewRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图信息
    view: CreateViewData,
}

impl CreateViewRequest {
    /// 创建新增视图请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            view: CreateViewData::default(),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置视图信息
    pub fn view(mut self, view: CreateViewData) -> Self {
        self.view = view;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateViewResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        if self.view.view_name.trim().is_empty() {
            return Err(validation_error("view.view_name", "视图名称不能为空"));
        }

        if self.view.view_name.len() > 100 {
            return Err(validation_error(
                "view.view_name",
                "视图名称长度不能超过100个字符",
            ));
        }

        // 视图名称不能包含 [ ]
        if self.view.view_name.contains('[') || self.view.view_name.contains(']') {
            return Err(validation_error(
                "view.view_name",
                "视图名称不能包含 '[' 或 ']'",
            ));
        }

        // 验证视图类型
        if let Some(ref view_type) = self.view.view_type {
            let valid_types = ["grid", "kanban", "gallery", "gantt", "form"];
            if !valid_types.contains(&view_type.as_str()) {
                return Err(validation_error(
                    "view.view_type",
                    "视图类型必须是以下之一: grid, kanban, gallery, gantt, form",
                ));
            }
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::ViewCreate(self.app_token.clone(), self.table_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CreateViewResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(&self.view)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 创建视图Builder
pub struct CreateViewRequestBuilder {
    request: CreateViewRequest,
}

impl CreateViewRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateViewRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    /// 设置视图信息
    pub fn view(mut self, view: CreateViewData) -> Self {
        self.request = self.request.view(view);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateViewRequest {
        self.request
    }
}

#[derive(Serialize, Default, Debug, Clone)]
/// 视图数据
pub struct CreateViewData {
    /// 视图名称
    pub view_name: String,
    /// 视图类型，可选值：grid (表格视图)、kanban (看板视图)、gallery (画册视图)、gantt (甘特视图)
    pub view_type: Option<String>,
}

impl CreateViewData {
    pub fn new(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: None,
        }
    }

    /// 创建表格视图
    pub fn grid_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("grid".to_string()),
        }
    }

    /// 创建看板视图
    pub fn kanban_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("kanban".to_string()),
        }
    }

    /// 创建画册视图
    pub fn gallery_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("gallery".to_string()),
        }
    }

    /// 创建甘特视图
    pub fn gantt_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("gantt".to_string()),
        }
    }

    /// 设置视图类型
    pub fn with_view_type(mut self, view_type: impl ToString) -> Self {
        self.view_type = Some(view_type.to_string());
        self
    }
}

/// 创建视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateViewResponse {
    /// 视图信息
    pub view: View,
}

impl ApiResponseTrait for CreateViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
