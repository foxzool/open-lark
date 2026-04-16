# Workflow 域 typed coverage 优先级清单

## 当前结论

基于当前覆盖率报告：

- `reports/api_validation/crates/openlark-workflow.md`

当前 `openlark-workflow` 状态为：

- API 总数：112
- 已实现：39
- 未实现：73
- 完成率：34.8%

其中：

- `TASK` 缺口：71
- `APPROVAL` 缺口：2
- `BOARD` 缺口：0

因此，workflow 域的优先级应首先集中在 **Task v2 主闭环**，而不是分散到 approval 尾部查询或旧版兼容 API。

## 高频场景排序依据

结合以下材料：

- `docs/communication-workflow-helper-boundaries.md`
- `examples/01_getting_started/communication_workflows.rs`
- `reports/api_validation/dashboards/core_business.md`

workflow 域的默认频率顺序如下。

### P0-A：Task v2 核心读取路径

优先补齐所有“看得见当前任务状态”的读取链路：

1. comment get / list
2. custom_field get / list
3. section get / list / tasks
4. task get / list / subtask list / tasklists
5. tasklist get / list / tasks
6. activity_subscription get / list

理由：

- 这些接口同时出现在任务查看、任务筛选和任务清单导航中
- 大多是 `get/list/query/search`，实现复杂度最低，最适合作为第一批缺口治理对象

### P0-B：Task v2 主闭环变更路径

在读取链路打通后，再补齐主闭环写操作：

1. task create / patch / delete / complete / uncomplete
2. comment create / delete
3. custom_field create / add / remove / option create / option patch
4. section create / patch / delete
5. task add/remove members, reminders, dependencies, tasklist
6. tasklist create / patch / delete / add/remove members
7. attachment delete / upload
8. subtask create

理由：

- 这些能力才构成“列任务 -> 查看 -> 更新 -> 组织到清单”的完整任务闭环
- 但其中写操作和附件上传复杂度高于读取接口，应排在读取链路之后

### P1：Approval 与 Task v1 兼容层

1. approval district list / search
2. task v1 get / list / comment / follower / collaborator / reminder
3. task v1 create / patch / delete / complete / uncomplete

理由：

- approval 当前只剩 2 个尾部查询缺口
- task v1 仍有兼容价值，但优先级应低于 v2 主路径

## 当前阶段性 backlog

| 阶段 | 范围 | 代表接口 |
|------|------|----------|
| P0-A | Task v2 读取路径 | `task/task/v2/task/list.rs`, `task/task/v2/task/get.rs`, `task/task/v2/tasklist/list.rs` |
| P0-B | Task v2 变更路径 | `task/task/v2/task/create.rs`, `task/task/v2/task/patch.rs`, `task/task/v2/tasklist/create.rs` |
| P1 | Approval 尾部 + Task v1 兼容层 | `approval/approval/v4/district/list.rs`, `task/task/v1/task/list.rs` |

## 执行建议

1. 先完成 **Task v2 读取链路**
2. 再完成 **Task v2 主闭环写操作**
3. 之后再处理 approval 尾部查询与 task v1 兼容层

如果未来 workflow 报告仍显示 task v2 占据绝大多数缺口，就不应把精力提前分散到 approval 尾部接口上。
