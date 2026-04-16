# 错误上下文统一规范

本规范用于统一 OpenLark 各 crate 的错误上下文字段，避免同类错误在不同模块里出现不同表达方式。

## 目标

对同一条错误，调用方至少能稳定拿到以下信息：

- `operation`：错误发生在哪个稳定步骤
- `component`：错误来自哪个 crate / 模块
- `resource`：错误对应的业务动作或资源描述
- `request_id`：如果服务端返回链路标识，则必须继续向上传播

## 标准字段

### 1. operation

使用**稳定的内部步骤名**，不要直接放自然语言整句。

推荐值：

- `extract_response_data`
- `serialize_params`
- `validate_required_field`
- `Client::from_env`

### 2. component

统一使用 crate 级标识：

- `openlark-docs`
- `openlark-communication`
- `openlark-workflow`
- `openlark-client`

### 3. resource

使用当前业务动作 / 资源描述，优先复用 API helper 已经在传递的中文上下文：

- `查询记录`
- `发送消息`
- `查询任务`

如果没有更好的资源描述，至少不要留空。

### 4. request_id

如果响应里存在 `request_id` / `log_id`，必须继续写入错误上下文。

这一字段用于链路追踪，不应在 helper 层被吞掉。

## 推荐实现方式

优先使用 `CoreError::with_standard_context(...)`：

```rust
error::validation_error("response.data", "服务器没有返回有效的数据")
    .with_standard_context(
        "extract_response_data",
        "openlark-docs",
        "查询记录",
        response.raw_response.request_id.clone(),
    )
```

## 当前落地范围

本轮统一先覆盖高频公共路径：

- `openlark-docs::common::api_utils`
- `openlark-communication::common::api_utils`
- `openlark-workflow::common::api_utils`
- `openlark-client` 既有 `with_operation_context(...)` 调用链

后续新增 API 默认应沿用这套规范；旧模块按高频路径优先逐步迁移。
