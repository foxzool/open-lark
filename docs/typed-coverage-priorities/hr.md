# HR 域 typed coverage 优先级清单

## 当前结论

基于当前覆盖率报告：

- `reports/api_validation/crates/openlark-hr.md`

当前 `openlark-hr` 状态为：

- API 总数：551
- 已实现：546
- 未实现：5
- 完成率：99.1%

所有缺口都集中在 `FEISHU_PEOPLE / CoreHR` 相关场景，因此 HR 域不需要重新做大范围排序，只需要按剩余 5 个缺口的频率和复杂度收敛。

## 当前缺口优先级

### P1：时间轴查询（优先先补）

1. `feishu_people/corehr/v2/company/query_multi_timeline.rs`
2. `feishu_people/corehr/v2/location/query_multi_timeline.rs`

理由：

- 都是查询型接口
- 已被覆盖率模型判为 `P1`
- 更适合作为 CoreHR 缺口治理的第一批收敛项

### P2：流程与写操作（随后补齐）

1. `feishu_people/corehr/v2/probation/edit.rs`
2. `feishu_people/corehr/v2/process/query_flow_data_template/create.rs`
3. `feishu_people/corehr/v2/process_start/create.rs`

理由：

- 都属于流程发起 / 模板 / 写操作路径
- 请求体和语义约束更复杂
- 频率低于时间轴查询类接口

## 默认频率顺序

如果未来 HR 域再次出现新缺口，默认按以下顺序补齐：

1. **组织与时间轴查询**
   - company / location / department / employee timeline
2. **员工状态变更**
   - probation、入转调离等高频变更
3. **流程模板与流程发起**
   - query_flow_data_template / process_start
4. **其余尾部能力**

## 阶段性实现建议

| 阶段 | 范围 | 代表接口 |
|------|------|----------|
| P1 | CoreHR 时间轴查询 | `company/query_multi_timeline`, `location/query_multi_timeline` |
| P2 | CoreHR 变更与流程 | `probation/edit`, `process/query_flow_data_template/create`, `process_start/create` |

当前 HR 域的关键点不是“还有很多没做”，而是**把 CoreHR 最后 5 个缺口按低复杂度查询优先收口**。
