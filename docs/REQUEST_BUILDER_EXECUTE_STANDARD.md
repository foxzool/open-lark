# Request / Builder / Execute 统一设计规范（跨 crate）

> 适用范围：OpenLark 全仓库新增 API 与增量重构场景。
> 
> 目标：在同一业务域内仅保留一套执行范式，避免并存导致的对外 API 混乱。

## 1. 统一范式选型

全仓默认采用 **范式 A（Request 自持 Config）**：

- `Request::new(config, ...)`：请求对象持有 `Arc<Config>`。
- `Request::validate(&self)`：参数校验前置，失败返回 `SDKResult` 错误。
- `Request::execute(self)`：默认走 `RequestOption::default()`。
- `Request::execute_with_options(self, option: RequestOption)`：对外统一的高级执行入口。

### 1.1 Service 层职责

- Service 仅负责资源分组与入口组织，不重复持有独立 HTTP 客户端。
- Service 层执行方法必须透传 `RequestOption`。
- 不在 Service 与 Request 之间引入第二套执行协议（例如同域混用 `build().execute(&service)` 和 `request.execute_with_options(...)`）。

## 2. 新增 API 标准模板

### 2.1 Request 骨架

```rust
pub struct CreateXxxRequest {
    config: Arc<Config>,
    // path/query/body 字段...
}

impl CreateXxxRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn validate(&self) -> SDKResult<()> {
        // validate_required!(...)
        Ok(())
    }

    pub async fn execute(self) -> SDKResult<CreateXxxResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CreateXxxResponse> {
        self.validate()?;
        // RequestExecutor::json_request / query_request / multipart_request ...
        unimplemented!()
    }
}
```

### 2.2 Builder 规则

- Builder 与 Request 字段一一对应，setter 命名使用 `snake_case`。
- 必填参数优先在 `new(...)` 中声明；可选参数使用链式 setter。
- `build()` 只负责组装，不执行网络调用。

### 2.3 导出规则

- `mod.rs` 中同时导出 `Request` 与 `RequestBuilder`。
- version 层（如 `v1/mod.rs`）只导出稳定对外类型。
- 避免“同义导出”（多入口语义重复）。

## 3. execute / execute_with_options 一致性约束

### 3.1 强制约束

1. 所有可执行 Request 必须同时提供 `execute` 与 `execute_with_options`。
2. `execute` 只能作为 `execute_with_options(RequestOption::default())` 的薄封装。
3. 所有请求在执行前必须调用 `validate()`。
4. 任何公共 API 不得使用 `unwrap()` / `expect()`。

### 3.2 RequestOption 透传

- Service 包装层、资源层、聚合入口层不得吞掉 `RequestOption`。
- 如果新增便捷方法，必须保留对等的 `_with_options` 版本。

## 4. 不一致实现整改清单模板

按业务域（如 docs/wiki/v2、hr/corehr/v2）持续维护：

- [ ] **P0** 同一目录同时存在两套执行范式（阻断新增，先收敛）。
- [ ] **P1** 缺失 `execute_with_options` 或未透传 `RequestOption`。
- [ ] **P1** 缺失 `validate()` 或校验不完整（必填参数漏检）。
- [ ] **P2** `mod.rs` 导出不完整（Request/Builder 漏导出）。
- [ ] **P2** 存在硬编码路径字符串，未使用 endpoints 常量/枚举。
- [ ] **P3** 测试仅覆盖 happy path，缺少 builder/validate/序列化边界测试。

## 5. 新增 API 验收检查点

每个新增 API 至少满足：

1. 结构：包含 `Request`、`RequestBuilder`、`Response`。
2. 执行：包含 `execute` + `execute_with_options`。
3. 校验：包含 `validate`（必填与边界参数）。
4. 导出：`mod.rs` 与上层模块导出完整。
5. 测试：至少包含 builder 链式与执行入口行为测试。

## 6. 渐进迁移策略（历史代码）

- 新增 API：**立即强制遵循本规范**。
- 存量 API：按业务域批次收敛，单次 PR 不跨多个顶层域。
- 兼容策略：
  - 旧入口保留一个版本周期并标记 `#[deprecated]`；
  - 文档示例优先切换到新入口；
  - 同步更新 `mod.rs` 导出，避免新旧入口混杂在 prelude。

