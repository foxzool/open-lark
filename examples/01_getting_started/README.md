# 入门示例

这一组示例对齐 `0.15.0` 的根 crate 用法，重点是 `Client` 单入口和按需 feature。

## 示例列表

| 示例 | 说明 | 运行命令 |
|------|------|----------|
| `readme_quick_start` | 根 README 对齐示例，覆盖文档 helper 入口 | `cargo run --example readme_quick_start --features "auth,docs-bitable"` |
| `simple_api_call` | 最小 communication 示例，验证客户端初始化和模块访问 | `cargo run --example simple_api_call --features "auth,communication"` |
| `docs_helpers` | 演示 docs helper，包括文件夹遍历、sheet 查找、批量读范围、多维表格全量读取 | `cargo run --example docs_helpers --features "auth,docs-bitable"` |
| `websocket_echo_bot` | 长连接消息回显 | `cargo run --example websocket_echo_bot --features "communication,websocket"` |

## 环境变量

```bash
cp examples/01_getting_started/.env.example .env
export OPENLARK_APP_ID="your_app_id"
export OPENLARK_APP_SECRET="your_app_secret"
```

可选变量：

```bash
export OPENLARK_BASE_URL="https://open.feishu.cn"
export OPENLARK_FOLDER_TOKEN="folder_token"
export OPENLARK_SPREADSHEET_TOKEN="spreadsheet_token"
export OPENLARK_SHEET_TITLE="汇总表"
export OPENLARK_BITABLE_APP_TOKEN="app_token"
export OPENLARK_BITABLE_TABLE_ID="table_id"
```

## 学习顺序

1. 先运行 `simple_api_call`，确认 `Client::from_env()` 和 `Client::builder()` 都能正常工作。
2. 再运行 `readme_quick_start` 或 `docs_helpers`，理解 `0.15.0` 推荐的文档 helper 能力。
3. 最后运行 `websocket_echo_bot`，验证长连接能力。
