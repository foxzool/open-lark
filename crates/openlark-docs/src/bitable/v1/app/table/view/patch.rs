//! Bitable 更新视图API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/view/patch

use openlark_core::{
    api::ApiRequest,
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 视图信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct View {
    /// 视图ID
    pub view_id: String,
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    pub view_type: String,
    /// 视图配置
    pub property: Option<Value>,
    /// 创建时间
    pub created_time: String,
    /// 更新时间
    pub modified_time: String,
}

/// 更新视图请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PatchViewRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<PatchViewResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 视图信息
    view: PatchViewData,
}

impl PatchViewRequest {
    /// 创建更新视图请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::put(""),
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
            user_id_type: None,
            view: PatchViewData::default(),
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

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.view_id = view_id;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置视图信息
    pub fn view(mut self, view: PatchViewData) -> Self {
        self.view = view;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchViewResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        if self.view_id.trim().is_empty() {
            return Err(validation_error("view_id", "视图ID不能为空"));
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

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::ViewPatch(self.app_token.clone(), self.table_id.clone(), self.view_id.clone());

        // 构建请求体
        let request_body = PatchViewRequestBody { view: self.view };

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<PatchViewResponse> = ApiRequest::put(&api_endpoint.to_url())
            .body(openlark_core::api::RequestData::Json(serde_json::to_value(&request_body)?));

        // 设置查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;

        // 解析响应数据
        let view_data: View = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析更新视图响应失败", "响应数据格式不正确"))?;

        Ok(PatchViewResponse {
            view: view_data,
            success: response.raw_response.is_success(),
        })
    }
}

/// 更新视图Builder
pub struct PatchViewRequestBuilder {
    request: PatchViewRequest,
}

impl PatchViewRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchViewRequest::new(config),
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

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.request = self.request.view_id(view_id);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置视图信息
    pub fn view(mut self, view: PatchViewData) -> Self {
        self.request = self.request.view(view);
        self
    }

    /// 构建请求
    pub fn build(self) -> PatchViewRequest {
        self.request
    }
}

/// 视图数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchViewData {
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    pub view_type: Option<String>,
    /// 视图的自定义属性
    pub property: Option<Value>,
}

impl PatchViewData {
    pub fn new(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
            view_type: None,
            property: None,
        }
    }

    /// 创建表格视图
    pub fn grid_view(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
            view_type: Some("grid".to_string()),
            property: None,
        }
    }

    /// 创建看板视图
    pub fn kanban_view(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
            view_type: Some("kanban".to_string()),
            property: None,
        }
    }

    /// 创建画册视图
    pub fn gallery_view(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
            view_type: Some("gallery".to_string()),
            property: None,
        }
    }

    /// 创建甘特视图
    pub fn gantt_view(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
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

/// 更新视图请求体（内部使用）
#[derive(Serialize)]
struct PatchViewRequestBody {
    view: PatchViewData,
}

/// 更新视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchViewResponse {
    /// 视图信息
    pub view: View,
    /// 操作结果
    pub success: bool,
}
