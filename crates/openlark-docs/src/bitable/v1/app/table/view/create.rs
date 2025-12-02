//! Bitable V1 创建视图API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 新增视图请求
#[derive(Debug, Clone)]
pub struct CreateViewRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<CreateViewResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图信息
    view: ViewData,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl CreateViewRequest {
    /// 创建新增视图请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post(""),
            app_token: String::new(),
            table_id: String::new(),
            view: ViewData::default(),
            user_id_type: None,
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
    pub fn view(mut self, view: ViewData) -> Self {
        self.view = view;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
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

        // 验证视图类型
        if let Some(ref view_type) = self.view.view_type {
            let valid_types = ["grid", "kanban", "gallery", "gantt"];
            if !valid_types.contains(&view_type.as_str()) {
                return Err(validation_error(
                    "view.view_type",
                    "视图类型必须是以下之一: grid, kanban, gallery, gantt",
                ));
            }
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views",
            self.config.base_url, self.app_token, self.table_id
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request.url = format!("{}?user_id_type={}", api_request.url, user_id_type);
        }

        // 构建请求体
        let request_body = CreateViewRequestBody { view: self.view };

        // 设置请求体
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::post(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let view_data: View = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析创建视图响应失败", "响应数据格式不正确"))?;

        Ok(CreateViewResponse {
            view: view_data,
            success: response.raw_response.is_success(),
        })
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
    pub fn view(mut self, view: ViewData) -> Self {
        self.request = self.request.view(view);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateViewRequest {
        self.request
    }
}

#[derive(Serialize, Default, Debug, Clone)]
/// 视图数据
pub struct ViewData {
    /// 视图名称
    pub view_name: String,
    /// 视图类型，可选值：grid (表格视图)、kanban (看板视图)、gallery (画册视图)、gantt (甘特视图)
    pub view_type: Option<String>,
    /// 视图的自定义属性，当前支持的视图自定义属性参考视图类型
    pub property: Option<Value>,
}

impl ViewData {
    pub fn new(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: None,
            property: None,
        }
    }

    /// 创建表格视图
    pub fn grid_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("grid".to_string()),
            property: None,
        }
    }

    /// 创建看板视图
    pub fn kanban_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("kanban".to_string()),
            property: None,
        }
    }

    /// 创建画册视图
    pub fn gallery_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("gallery".to_string()),
            property: None,
        }
    }

    /// 创建甘特视图
    pub fn gantt_view(view_name: impl ToString) -> Self {
        Self {
            view_name: view_name.to_string(),
            view_type: Some("gantt".to_string()),
            property: None,
        }
    }

    /// 设置视图类型
    pub fn with_view_type(mut self, view_type: impl ToString) -> Self {
        self.view_type = Some(view_type.to_string());
        self
    }

    /// 设置视图属性
    pub fn with_property(mut self, property: Value) -> Self {
        self.property = Some(property);
        self
    }
}

/// 请求体结构
#[derive(Serialize)]
struct CreateViewRequestBody {
    view: ViewData,
}

/// 创建视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateViewResponse {
    /// 视图信息
    pub view: View,
    /// 操作结果
    pub success: bool,
}
