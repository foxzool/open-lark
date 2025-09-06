# Repository Guidelines

## 项目结构与模块组织
- `src/`：核心 SDK 与服务接口；按 feature 拆分。
- `crates/`：子包（如 `protobuf`）。
- `examples/`：可运行示例：`basic/`、`api/`、`core/`。
- `tools/`：辅助工具二进制（见 `[[bin]]`）。
- 其它：`docs/` 文档，`scripts/` 脚本，`reports/` 报告。

## 构建、测试与开发命令
- 依赖 `just` 作为任务入口：
  - `just build`：构建所有特性。
  - `just test`：运行单元与文档测试。
  - `just lint`：Clippy 零告警检查。
  - `just fmt` / `just fmt-check`：格式化/检查。
  - `just docs`：生成本地文档。
  - `just check-all`：发布前全检。
  - `just release 0.x.y`：打 tag 触发 CI 发布。
- 等价 cargo 示例：`cargo build --all-features`、`cargo test --all-features`。

## 代码风格与命名
- Rust 2021；`rustfmt` 默认规则。
- Clippy：在 CI 以 `-D warnings` 严格通过；`.clippy.toml` 已适配大仓阈值。
- 命名：模块/函数用 snake_case，类型用 PascalCase；feature 用 kebab-case（如 `cloud-docs`）。
- 最小公开原则：仅暴露必要 API。

## 测试指南
- 框架：内置 `cargo test` 与 doctest；示例位于 `examples/`。
- 约定：测试函数以 `test_*` 命名；需要凭证的示例请复制 `.env-example` 为 `.env`。
- 覆盖：优先核心路径与错误处理，提交需附测试或示例更新。

## 提交与 Pull Request
- 提交规范：采用 emoji + 类型 + 概述，例如：
  - `🐛 fix: restore WebSocket frame parsing`
  - `🎨 style: format code`
  - `🚀 release: bump version to 0.13.1`
- PR 要求：清晰描述、关联 Issue、影响范围、变更清单、测试/文档更新；API 变更需附迁移说明。

## 安全与配置提示（可选）
- 切勿提交敏感信息；`.env` 已忽略。
- 功能开关：通过 `Cargo.toml` features 精简构建（如 `full`、`websocket`、`cloud-docs`）。

