# Communication 域 typed coverage 优先级清单

## 当前结论

基于当前覆盖率报告：

- `reports/api_validation/crates/openlark-communication.md`

当前 `openlark-communication` 状态为：

- API 总数：165
- 已实现：164
- 未实现：1
- 完成率：99.4%

当前 backlog 只有 1 个缺口，因此 communication 域的实现顺序非常明确。

## 高频场景排序依据

结合以下材料：

- `docs/communication-workflow-helper-boundaries.md`
- `examples/01_getting_started/communication_workflows.rs`

communication 域的默认频率顺序为：

### P0：前置发现动作

1. **user lookup**
   - `search_users_all`
   - `find_user_by_name`
   - 缺失 API：`contact/contact/v3/user/basic_batch.rs`
2. **chat lookup**
   - `search_chats_all`
   - `find_chat_by_name`

原因：发送动作、审批通知、目录同步都会先依赖“查人/查群”。

### P1：高频发送动作

1. 发送文本 / 富文本消息
2. 回复消息
3. 转发 thread

### P2：高频媒体动作与尾部管理能力

1. 图片 / 文件上传与发送
2. 组织目录、群成员、Aily 等尾部扩展能力

## 当前缺口列表

| 优先级 | API | 预期文件 | 说明 |
|--------|-----|----------|------|
| P0 | 通过 ID 获取用户姓名 | `contact/contact/v3/user/basic_batch.rs` | 目录集成高频前置动作，且实现复杂度低 |

## 阶段性实现建议

1. **先补 `contact/contact/v3/user/basic_batch.rs`**
   - 它是当前 communication 域唯一缺口
   - 同时位于高频“查人 -> 发送”链路的最前置位置
2. 只有在该缺口完成后，才考虑 communication 域的其他尾部增量

换言之：在 communication 域里，当前不存在需要进一步排序的第二梯队 backlog。
