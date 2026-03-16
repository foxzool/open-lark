# openlark-ai Infrastructure Setup Learnings

## Date: 2026-02-19

## Task Summary
创建 openlark-ai crate 的基础设施文件，参考 openlark-cardkit 的实现模式。

## Key Findings

### 1. 文件结构一致性
- openlark-ai 已经存在大部分基础设施文件
- 目录结构与 cardkit 保持一致：`src/common/`, `src/endpoints/`, `src/service.rs`

### 2. API Utils 模式
- `extract_response_data`: 从 Response 中提取 data 字段，处理空响应
- `serialize_params`: 参数序列化为 JSON，处理序列化错误
- `ensure_success`: 无 data 字段的 API 调用，仅用 code 判断成功
- `api_url!` 宏：构建带路径参数的 URL

### 3. Service 结构模式
```rust
pub struct AiClient {
    config: Arc<Config>,
}

impl AiClient {
    pub fn document_ai(&self) -> DocumentAiClient
    pub fn ocr(&self) -> OcrClient  
    pub fn speech_to_text(&self) -> SpeechToTextClient
    pub fn translation(&self) -> TranslationClient
}
```

### 4. 模块导出约定
- `common/mod.rs` 需要重导出 api_utils 中的函数
- `lib.rs` 重导出 service::AiClient 和 endpoints
- prelude 模块重导出 Config 和 SDKResult

### 5. Feature 配置
Cargo.toml 已配置：
```toml
[features]
default = ["v1"]
full = ["v1"]
v1 = []
```

## Changes Made

1. **crates/openlark-ai/src/common/api_utils.rs**
   - 添加 `ensure_success` 函数
   - 与 cardkit 模式保持一致

2. **crates/openlark-ai/src/common/mod.rs**
   - 添加 api_utils 函数重导出
   - `pub use api_utils::{extract_response_data, serialize_params, ensure_success};`

## Verification Commands
```bash
just fmt
cargo check -p openlark-ai --all-features
```

## Notes
- openlark-ai 的 API 使用 `Response<T>` 类型，而 cardkit 使用 `ApiResponse<T>`
- Service 层实现与 cardkit 略有不同，但保持一致的调用链模式
- 测试文件已包含基础单元测试

---

## 2026-02-19: Document AI API 实现

### 本次任务
实现 5 个核心 Document AI API：resume_parse、id_card_recognize、bank_card_recognize、business_license_recognize、vat_invoice_recognize。

### 创建的文件结构

```
crates/openlark-ai/src/document_ai/document_ai/v1/
├── mod.rs
├── recognize/
│   ├── mod.rs
│   ├── resume_parse.rs
│   ├── id_card_recognize.rs
│   ├── bank_card_recognize.rs
│   ├── business_license_recognize.rs
│   └── vat_invoice_recognize.rs
```

另外创建了：
- `crates/openlark-ai/src/document_ai/mod.rs` - Document AI 根模块
- `crates/openlark-ai/src/document_ai/document_ai/mod.rs` - document_ai 子模块
- `crates/openlark-ai/src/common/chain.rs` - Document AI 链式调用入口

### API 文件标准结构

每个 API 文件包含以下组件：

1. **{Name}Body struct** - 请求体
   - 使用 `#[derive(Debug, Clone, Serialize, Deserialize, Default)]`
   - 使用 `#[serde(skip_serializing_if = "Option::is_none")]` 处理可选字段
   - `validate()` 方法进行参数校验（使用 trim() 检查空白字符串）

2. **{Name}Response struct** - 响应体
   - 实现 `openlark_core::api::ApiResponseTrait`

3. **{Name}Result + ParsingResult** - 解析结果结构

4. **{Name}Request struct** - 请求结构
   - `new(config: Config)` 构造函数
   - `execute(body)` 方法
   - `execute_with_options(body, option)` 方法（必须实现）

5. **{Name}RequestBuilder struct** - Builder 模式
   - 字段设置方法（返回 Self）
   - `body()` 方法构建请求体
   - `execute()` 和 `execute_with_options()` 方法

6. **create() / create_with_options()** 辅助函数

### 关键模式

1. **端点常量使用**: 必须使用 endpoints 模块中的常量，禁止硬编码 URL
2. **错误处理**: 使用 `openlark_core::error::validation_error` 创建验证错误
3. **工具函数**: 使用 `crate::common::api_utils` 中的工具函数
4. **链式调用**: 参考 openlark-cardkit 的 chain.rs 模式

### 验证结果

- `cargo check -p openlark-ai` ✅ 通过
- `cargo test -p openlark-ai` ✅ 147 个测试通过

### 注意事项

1. 必须实现 `execute_with_options` 方法，支持自定义 RequestOption
2. 所有公开 API 都需要中文文档注释
3. 每个 API 文件都包含完整的单元测试
4. 使用 `Arc<Config>` 在链式调用中传递配置

---

## 2026-03-15: 实现状态审计（plan 对照）

### 审计结论
- 端点常量（去除兼容别名）共 25 个，已被实现代码引用 24 个，缺失 `OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE`。
- 若按计划口径“27 个 API”统计：当前可判定约为 **24/27**；若按当前 `endpoints/mod.rs` 主常量口径统计：为 **24/25**。

### 目录与实现分布
- `src/document_ai/document_ai/v1/recognize/`：5 个核心 Document AI API 已实现（Task 2 对应）。
- `src/ai/document_ai/v1/**`：18 个 Document AI API 基本齐全（含 Task 6 剩余 API），但与计划落盘路径不一致。
- `src/speech_to_text/speech_to_text/v1/recognize/`：3 个 Speech API 已实现。
- OCR 仅发现 `src/ai/optical_char_recognition/v1/image/basic_recognize.rs`（基础识别），未发现 image_basic 端点实现。
- 翻译 API 在 `src/ai/translation/v1/text/{detect,translate}.rs` 已实现。

### 计划任务完成度（按严格计划结构）
- Task 1：完成。
- Task 2：完成（5 个核心 Document AI + chain）。
- Task 3：未完成（OCR 缺 1 API，且未按计划目录 `ocr/ocr/v1/recognize/` 落盘）。
- Task 4：部分完成（Translation API 已有实现，但未按计划目录 `translation/translation/...`，链路与服务入口不完全一致）。
- Task 5：部分完成（Speech API 在独立模块已实现，`ai/` 下仍有占位实现，服务链路存在分叉）。
- Task 6：功能上接近完成（剩余 Document AI API 已见实现），但未按计划目录组织。
- Task 7：部分完成（crate 内联单测大量存在且通过；`tests/unit/ai/` 计划目录未落地）。

### 验证结果
- `cargo test -p openlark-ai --all-features`：通过（211 unit tests + 4 doc tests）。
- `cargo check -p openlark-ai --all-features`：通过。
