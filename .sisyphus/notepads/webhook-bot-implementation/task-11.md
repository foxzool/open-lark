## 2026-03-07 Task 11 Integration Tests

- 新增 `crates/openlark-webhook/tests/integration_webhook.rs`，使用 wiremock `MockServer` 覆盖 webhook 发送全流程。
- 覆盖 HTTP 成功与错误状态：200、400、429、500，并校验错误类型为 `WebhookError::Http`。
- 覆盖响应解析：200 成功 JSON 可解析到 `SendWebhookMessageResponse`，非法响应体返回 `WebhookError::Serialization`。
- 覆盖请求格式校验：text/post/image/file 的 JSON 结构与 Feishu webhook 格式一致。
- 在 `card` feature 下增加 interactive 卡片消息请求格式校验，确保消息类型覆盖完整。
- 验证命令输出已记录到 `.sisyphus/evidence/task-11-integration-tests.txt`。
