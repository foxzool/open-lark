# Release Notes 中的 Crate 级质量状态

本文件定义发布说明里 crate 级质量状态摘要的来源、维度和生成方式。

## 维度

发布说明中的 crate 级质量状态当前采用以下维度：

1. **typed coverage**
   - 数据来源：`reports/api_validation/summary.json`
   - 表示该 crate 在 typed API 覆盖率上的当前完成度
2. **剩余缺口**
   - 数据来源：同上
   - 表示剩余未实现 API 数量
3. **contract tests**
   - 数据来源：crate 内 `tests/*contract*.rs`
   - 表示该 crate 是否已有 request/response contract test 保护
4. **snapshot tests**
   - 数据来源：crate 内 `tests/*snapshot*.rs` 或 `tests/snapshots/*.snap`
   - 表示该 crate 是否已有高价值输出快照保护
5. **备注**
   - 若该 crate 属于 typed coverage 统计对象，则显示：
     - `无 typed API 缺口`
     - 或 `缺口：P0=..., P1=...`
   - 若不属于 typed coverage 统计对象，则显示：
     - `非 typed API 覆盖统计对象`

## 生成方式

先生成 typed coverage 汇总：

```bash
python3 tools/validate_apis.py --all-crates
```

再生成发布说明片段：

```bash
python3 tools/release_quality_status.py --output release_quality_status.md
```

## 与发布流程的关系

该摘要片段会被 release workflow 附加到 GitHub Release body。

这样做的目的不是给每个 crate 打一个绝对“成熟度分数”，而是让用户快速看到：

- 哪些 crate 已经没有 typed API 缺口
- 哪些 crate 还存在高优先级缺口
- 哪些 crate 已经补上 contract / snapshot 保护

## 非目标

本文件与脚本当前不尝试：

- 穷举所有质量维度
- 替代完整测试报告
- 给每个 crate 做主观成熟度评级

重点是提供**稳定、可复用、可自动生成**的 release note 质量摘要。
