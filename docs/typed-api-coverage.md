# Typed API 覆盖率报告说明

本文档定义 `tools/validate_apis.py` 的覆盖率统计口径与生成流程，用于支持按 crate 的缺口跟踪、里程碑规划与发布节奏管理。

## 1. 统计口径

- 数据源：仓库根目录 `api_list_export.csv`。
- 实现源：`tools/api_coverage.toml` 中配置的 crate 源码目录与 bizTag 映射。
- 默认排除 `meta.Version=old`（脚本默认开启 `--skip-old`）。
- 覆盖率定义：`已实现 / API 总数 * 100%`。

## 2. 报告产物

执行批量模式后，会生成以下文件：

- `reports/api_validation/summary.md`：面向人读的汇总看板（总览 + 各 crate 指标）。
- `reports/api_validation/summary.json`：机器可读汇总（便于 CI/看板系统消费）。
- `reports/api_validation/crates/<crate>.md`：每个 crate 的详细报告（含未实现清单）。

每个 crate 的报告至少包含：

- API 总数
- 已实现数量
- 未实现数量
- 完成率

## 3. 使用方式

### 3.1 生成全量覆盖率（推荐）

```bash
just api-coverage
```

等价命令：

```bash
python3 tools/validate_apis.py --all-crates
```

### 3.2 单 crate 验证

```bash
python3 tools/validate_apis.py --crate openlark-docs
```

### 3.3 包含 old 版本 API

```bash
python3 tools/validate_apis.py --all-crates --include-old
```

## 4. 规划建议

- 以 `summary.md` 的 `未实现` 列作为季度缺口清单输入。
- 以 `completion_rate` 追踪里程碑完成趋势。
- 优先处理高流量核心 crate 的高缺口模块，再补尾部模块。
