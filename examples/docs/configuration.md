# OpenLark 示例配置

本文档只描述当前仓库里真实存在的示例配置方式。

## 必需环境变量

```bash
export OPENLARK_APP_ID="your_app_id"
export OPENLARK_APP_SECRET="your_app_secret"
```

## 可选环境变量

```bash
export OPENLARK_BASE_URL="https://open.feishu.cn"
export OPENLARK_FOLDER_TOKEN="folder_token"
export OPENLARK_SPREADSHEET_TOKEN="spreadsheet_token"
export OPENLARK_SHEET_TITLE="汇总表"
export OPENLARK_BITABLE_APP_TOKEN="app_token"
export OPENLARK_BITABLE_TABLE_ID="table_id"
```

如果你使用国际版 Lark：

```bash
export OPENLARK_BASE_URL="https://open.larksuite.com"
```

## 推荐做法

```bash
cp examples/.env.example .env
```

然后把上面的变量写进 `.env`。

## 运行示例

```bash
cargo run --example simple_api_call --features "auth,communication"
cargo run --example docs_helpers --features "auth,docs-bitable"
cargo run --example websocket_echo_bot --features "communication,websocket"
```

## 说明

- `simple_api_call` 只验证根 crate 单入口和 communication 模块是否可用。
- `docs_helpers` 会按你是否提供相关 token，分别演示文件夹遍历、sheet 查找和多维表格读取。
- `websocket_echo_bot` 需要额外完成飞书事件订阅和长连接配置。
