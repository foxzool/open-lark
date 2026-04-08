# Issue #41 静态检查闸门使用说明

`tools/issue41_guardrail.py` 用于持续检查 issue #41 的关键一致性规则，默认只扫描业务 crate：

- `openlark-docs`
- `openlark-meeting`
- `openlark-communication`
- `openlark-hr`

## 检查项

- `E001`：存在 `execute()` 但缺失 `execute_with_options()`
- `E002`：`execute()` 未委托默认 `RequestOption::default()`
- `E003`：业务 API 文件中出现 `Transport::request(..., None)`
- `W001`：启发式判断存在必填校验场景，但未发现 `validate_required!`

## 运行方式

```bash
python3 tools/issue41_guardrail.py
```

如果你在仓库根目录，也可以使用 just 入口：

```bash
just issue41-guardrail
```

## 常用参数

只扫指定 crate：

```bash
python3 tools/issue41_guardrail.py --crates openlark-meeting
```

将警告也作为失败：

```bash
python3 tools/issue41_guardrail.py --strict-warn
```

## 退出码

- 默认：存在 `ERROR` 时退出码为 `1`
- 启用 `--strict-warn`：存在 `ERROR` 或 `WARN` 时退出码为 `1`
