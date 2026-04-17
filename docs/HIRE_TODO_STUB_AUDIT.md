# Hire generated stub audit

更新时间：2026-04-17

## 背景

`#106` 处理的是 `crates/openlark-hr/src/hire/hire/` 下由生成骨架遗留的 shipped-source TODO/FIXME。

在本轮清理前，`tools/audit_todos.py` 对 Hire 目录统计出：

- **243** 条 TODO/FIXME 注释
- 分布在 **177** 个源码文件中

这些注释主要有两类：

1. **空请求骨架**
   - `// TODO: 添加请求字段`
   - `// TODO: 初始化字段`
   - `// TODO: 添加字段 setter 方法`
2. **响应 schema 占位**
   - `/// TODO: 根据官方文档添加具体字段`

## 本轮处理

### 1. 把 shipped-source TODO 迁移为显式占位说明

为了避免“源码里看起来还没做完，但 issue / 文档里又没有追踪”的双重失真，本轮把 Hire 目录中的 TODO 注释统一替换为更明确的占位说明：

- 空请求骨架改成“当前尚未建模请求字段；补齐 schema 前保持零字段请求”
- `Value` 响应占位改成“当前按未建模 JSON 原样透传；字段收敛后再替换为显式结构”

这样做的目标不是“假装 schema 已完成”，而是把 debt 从源码噪音迁移到可追踪的 issue / 文档层。

### 2. 固化剩余 schema debt 的结构统计

清理后，Hire 目录里不再保留 TODO/FIXME 标记。本轮继续完成 #111 后，当前剩余的 schema debt 为：

- **0 个文件**：零字段请求骨架已全部消除
- **1 个接口**：`talent_object/query` 已确认是官方无参请求，不再视为骨架
- **177 个文件**：响应仍然是 `Value` 直透，需要后续 typed 化

## 分类结论

### A. 零字段请求骨架（已完成）

`#111` 已完成以下收敛：

- 21 个接口已补齐 query/body/path builder 字段
- `crates/openlark-hr/src/hire/hire/v1/talent_object/query.rs` 已确认官方本身无请求参数，因此保留为显式无参请求
- 新增定向验证：`crates/openlark-hr/tests/hire_request_modeling_tests.rs`

已不再存在“零字段请求骨架仍待建模”的剩余项。

### B. `Value` 响应直透（177 files）

这批接口已经具备 endpoint/execute 入口，但公共返回类型仍是 `pub data: Value`。这意味着：

- 运行时可用
- Rust 层 contract 仍弱
- 字段级兼容性无法通过类型系统表达

优先建议聚焦：

- offer / note / job / interview / referral / website / attachment
- v2 `interview_record` / `talent`
- 外部招聘 / 外部面试 / 背调相关路径

跟踪 issue：**#112 Replace Hire Value pass-through responses with typed models**

`#112` 现在是 Hire schema debt 的主跟踪入口。

## 决策

本轮 `#106` 的定位是：

- **完成审计**
- **清理 shipped-source TODO 噪音**
- **把剩余 schema debt 转移到明确 issue 跟踪**

而不是在一个 issue 里一次性为 177 个接口补完所有 typed schema。

## 退出标准

`#106` 可以关闭的条件：

- Hire shipped source 中不再保留骨架式 TODO/FIXME
- 零字段请求骨架已在 `#111` 中完成收敛
- `Value` 响应直透已有独立跟踪（#112）
- 审计结论已固化到仓库文档
