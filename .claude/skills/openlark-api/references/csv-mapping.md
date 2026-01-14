# CSV 映射与提取规则（api_list_export.csv）

## 最小提取规则

- `url`：形如 `GET:/open-apis/...`，拆出：
  - HTTP 方法：`GET/POST/PUT/DELETE/PATCH`
  - path：`/open-apis/...` 的路径部分
- `bizTag/meta.Project/meta.Version/meta.Resource/meta.Name`：用于推导落盘路径（见 `file-layout.md`）

## 与代码的对应关系（建议）

- 将 `path` 放到该 crate 的 endpoints 常量或 enum 端点中复用
- `meta.Resource` 若包含 `.`，用于拆目录（见 `file-layout.md`）
