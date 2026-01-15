# 标准示例（以仓库现有风格为准）

下面给出两种“仓库里真实存在”的风格：
- A) 使用端点常量（典型：`openlark-communication`）
- B) 使用 enum 端点（典型：`openlark-docs` / `openlark-auth`）

实现新 API 时优先模仿目标 crate 的现有文件风格。

## A) 端点常量 + Builder + execute（推荐默认）

参考现有文件：`crates/openlark-communication/src/im/im/v1/message/create.rs`

```rust
//! 示例：发送消息（精简结构示例）
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageBody {
    pub receive_id: String,
    pub msg_type: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

pub struct CreateMessageRequest {
    config: Config,
    receive_id_type: Option<String>, // 示例：真实代码里通常是 enum
}

impl CreateMessageRequest {
    pub fn new(config: Config) -> Self {
        Self { config, receive_id_type: None }
    }

    pub fn receive_id_type(mut self, v: impl Into<String>) -> Self {
        self.receive_id_type = Some(v.into());
        self
    }

    pub async fn execute(self, body: CreateMessageBody) -> SDKResult<serde_json::Value> {
        validate_required!(body.receive_id, "receive_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");
        validate_required!(body.content, "content 不能为空");
        let receive_id_type = self.receive_id_type.ok_or_else(|| {
            openlark_core::error::validation_error("receive_id_type 不能为空", "需要指定接收者 ID 类型")
        })?;

        // url: POST:/open-apis/im/v1/messages
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(IM_V1_MESSAGES)
            .query("receive_id_type", receive_id_type)
            .body(serialize_params(&body, "发送消息")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "发送消息")
    }
}
```

## B) enum 端点 + typed response + ApiResponseTrait

参考现有文件：`crates/openlark-docs/src/base/base/v2/app/role/create.rs`

```rust
//! 示例：新增自定义角色（精简结构示例）
//!
//! docPath: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/create-2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::common::api_endpoints::BaseApiV2;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
    pub role_name: String,
    pub table_roles: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub struct Create {
    config: Config,
    app_token: String,
    req: CreateReq,
}

impl Create {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: CreateReq { role_name: String::new(), table_roles: vec![] },
        }
    }

    pub fn app_token(mut self, v: impl Into<String>) -> Self { self.app_token = v.into(); self }
    pub fn role_name(mut self, v: impl Into<String>) -> Self { self.req.role_name = v.into(); self }
    pub fn table_roles(mut self, v: Vec<serde_json::Value>) -> Self { self.req.table_roles = v; self }

    pub async fn send(self) -> SDKResult<CreateResp> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.req.role_name, "role_name 不能为空");

        let api_endpoint = BaseApiV2::RoleCreate(self.app_token);
        let api_request: ApiRequest<CreateResp> = ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&self.req, "新增自定义角色")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "新增自定义角色")
    }
}
```

## 最小导出约定（mod.rs）

在同级 `mod.rs` 中导出模块（模仿目标 crate 现有结构）：

```rust
pub mod create;
pub mod get;
pub mod models;
```

## service.rs 链式调用（给 openlark-client 调用）

`openlark-client` 推荐以“薄包装 + raw 透传”的方式复用各 feature crate 的 service 链路。

### 示例：openlark-docs 的链式结构（真实存在）

入口：`crates/openlark-docs/src/service.rs`

```rust
// openlark_docs::service::DocsService
service.base().v2().app().role().create()
```

其中：
- `base()`（bizTag）在 `DocsService` 上提供
- `v2()`（meta.Version）在 `BaseService` 上提供：`crates/openlark-docs/src/base/service.rs`
- `app().role()`（meta.Project/meta.Resource）在各层 `*Service` 上提供
- `create()`（meta.Name）在 `RoleService` 上提供：`crates/openlark-docs/src/base/base/v2/app/role/mod.rs`

### 对新 API 的要求（建议照此落地）

当你新增 `crates/{feature-crate}/src/{bizTag}/{meta.Project}/{meta.Version}/.../{meta.Name}.rs` 时：
- 确保能从 `crates/{feature-crate}/src/service.rs` 逐层链式访问到该 API builder
- 每一层 service 只做“路由/分组”，最后一层返回具体 API builder（其上再 `.send()`/`.execute()`）
