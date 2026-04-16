# Docs 域 typed coverage 优先级清单

## 当前结论

基于当前批量覆盖率报告：

- `reports/api_validation/crates/openlark-docs.md`

当前 `openlark-docs` 状态为：

- API 总数：205
- 已实现：205
- 未实现：0
- 完成率：100.0%

因此，**当前阶段 docs 域没有待补齐的 typed API 缺口**，正式优先级 backlog 为空。

## 高频场景排序依据

虽然当前缺口为 0，但为了避免未来新增 API 时重新讨论排序口径，docs 域仍保留以下默认优先级顺序。

### P0：主闭环读写场景

这些能力直接支撑 README、helper 设计和入门示例中的主路径：

1. **Drive 文件流转**
   - 参考：`examples/01_getting_started/docs_workflows.rs`
   - 典型动作：列目录、上传文件、下载验证
2. **Spreadsheet 周报处理**
   - 参考：`examples/01_getting_started/docs_workflows.rs`
   - 典型动作：定位 sheet、读取范围、批量写入、追加
3. **Bitable 记录查询 / 读写**
   - 参考：`docs/docs-helper-guidelines.md`
   - 典型动作：记录查询、批量读写、筛选条件表达
4. **Wiki 路径导航**
   - 参考：`docs/docs-helper-guidelines.md`
   - 典型动作：按路径命中节点、按标题查找、空间遍历

### P1：围绕主闭环的增强能力

1. Drive 权限、评论、订阅
2. Bitable 仪表盘 / 视图 / 表单等扩展读写
3. 与主闭环紧邻的批量查询类接口

### P2：尾部或低频场景

1. Baike / Lingo 的词条维护尾部能力
2. Minutes 较低频管理接口
3. 不在 README / helper 主推路径上的补充型接口

## 当前缺口列表

| 优先级 | 结论 |
|--------|------|
| P0 | 无缺口 |
| P1 | 无缺口 |
| P2 | 无缺口 |

## 后续使用方式

当 docs 域未来再次出现 typed API 缺口时，默认按以下顺序补齐：

1. Drive 文件流转
2. Spreadsheet 读写
3. Bitable 记录与批量读写
4. Wiki 导航
5. 其余增强与尾部能力

如果未来报告仍显示 `未实现 = 0`，则本清单只作为排序规则保留，不应为了“满足 issue”而人为制造补齐任务。
