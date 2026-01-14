# 目录与命名（校正版）

> 新增 API 时优先“模仿已有模块”：以 `crates/*/src/` 下目标业务的现有目录层级为准。

## 核心口径（推荐）

从 CSV 取值后，优先按以下公式落盘：

```
crates/{feature-crate}/src/{bizTag}/{meta.Project}/{meta.Version}/{meta.Resource...}/{meta.Name}.rs
```

- `meta.Resource` 若包含 `.`，拆成多级目录
- 若 crate 内现有结构不符合该公式（例如出现重复层级、历史 `old/default` 目录），以现有目录为准

## bizTag → feature crate（常用）

见 `SKILL.md` 中的映射表。

## 反查方法（避免落错目录）

当你不确定 `{bizTag}/{meta.Project}/{meta.Version}` 的真实层级时：

1) 先在目标 crate 下查找同 bizTag 或 project 目录
2) 若已存在同资源目录（例如 `.../v1/message/`），新文件放同级
3) 若该模块采用 enum 端点系统（例如存在 `common/api_endpoints.rs`），优先沿用现有 enum/常量定义的位置与命名（示例见 `standard-example.md`）
