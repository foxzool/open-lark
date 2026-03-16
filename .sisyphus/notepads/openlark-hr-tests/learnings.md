
## 文档编写心得 (2026-02-19)
- **Builder 测试模式提取**：通过分析 `tests/unit/im/builder_tests.rs`，发现该项目的 Builder 测试重点在于验证 `query_params` 和序列化后的 `body` 是否正确。
- **Mock 模式**：项目中广泛使用 `wiremock` 进行异步 HTTP 模拟，这在编写测试指南时应作为核心推荐工具。
- **文档结构一致性**：参考了 `crates/openlark-docs/AGENTS.md` 的结构，确保文档风格与项目整体保持一致。
- **Checklist 的重要性**：在 SDK 这种大规模项目中，具体的 Checklist 有助于维持成百上千个 API 接口实现的一致性。
