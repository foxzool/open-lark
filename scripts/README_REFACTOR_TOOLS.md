# Calendar API 自动化重构工具

本目录包含用于自动化重构 Calendar API 的工具和脚本。

## 工具概览

### 1. refactor_calendar_api.sh
**Bash 半自动化重构脚本**

#### 功能
- ✅ 自动检测 API 类型（GET/POST/DELETE/PATCH）
- ✅ 生成响应结构模板
- ✅ 检测常见问题（unwrap_or_default, ResponseFormat::Custom等）
- ✅ 提供详细的修复建议
- ✅ 自动备份原文件
- ✅ 彩色输出，易于阅读

#### 使用方法

```bash
# 基本用法
./scripts/refactor_calendar_api.sh <module_path> <api_name>

# 示例 1: 分析 event/list
./scripts/refactor_calendar_api.sh calendar/v4/event list

# 示例 2: 分析 calendar/create
./scripts/refactor_calendar_api.sh calendar/v4/calendar create
```

#### 输出内容
脚本会输出：
1. API 的 HTTP 方法类型
2. 响应结构模板（保存到 `/tmp/response_template.rs`）
3. 在线文档链接
4. 发现的问题列表
5. 修复建议
6. API endpoint 枚举使用情况
7. 下一步操作指南

#### 备份机制
每次运行都会创建时间戳备份：
```
crates/openlark-meeting/src/calendar/v4/event/list.rs.backup_20250115_143025
```

#### 回退方法
如需回退，使用备份文件：
```bash
cp crates/openlark-meeting/src/calendar/v4/event/list.rs.backup_20250115_143025 \
   crates/openlark-meeting/src/calendar/v4/event/list.rs
```

---

### 2. refactor_calendar_apis.py
**Python 全分析和批量处理工具**

#### 功能
- ✅ 扫描所有 Calendar API 文件
- ✅ 按模块统计重构进度
- ✅ 生成详细的 JSON 格式分析报告
- ✅ 支持自动修复常见问题
- ✅ 彩色终端输出
- ✅ 详细的代码模式识别

#### 使用方法

```bash
# 查看帮助
python3 scripts/refactor_calendar_apis.py --help

# 分析所有 Calendar API
python3 scripts/refactor_calendar_apis.py --analyze

# 分析并保存报告
python3 scripts/refactor_calendar_apis.py --analyze --report

# 重构指定文件（开发中）
python3 scripts/refactor_calendar_apis.py --refactor <file_path>
```

#### 输出示例

```
============================================================
ℹ Calendar API 重构状态摘要
============================================================
总 API 数量: 45
✓ 已使用强类型: 5
⚠ 使用弱类型: 39
进度: 5/45 (11%)

按模块统计:
  calendar/acl                    0/ 5  ⚠0%
  calendar/create.rs              1/ 1  ✓
  calendar/event                  0/18 ⚠0%
  ...
```

#### 报告格式

JSON 报告保存在 `/tmp/calendar_api_refactor_report.json`：

```json
{
  "total_apis": 45,
  "strong_typed": 5,
  "weak_typed": 39,
  "progress": "11%",
  "details": [
    {
      "file_path": "calendar/v4/calendar/create.rs",
      "api_name": "create",
      "issues": [],
      "recommendations": [],
      "uses_strong_types": true,
      "uses_api_endpoint_enum": false,
      "uses_weak_type": false,
      "response_type": "CreateCalendarResponse"
    },
    ...
  ]
}
```

#### 自定义路径

```bash
# 使用不同的基础路径
python3 scripts/refactor_calendar_apis.py --analyze --base-path /path/to/src
```

---

## 常见使用场景

### 场景 1: 开始重构新 API

使用 Bash 脚本分析目标文件：

```bash
./scripts/refactor_calendar_api.sh calendar/v4/event get
```

按照输出的建议：
1. 查看 `/tmp/response_template.rs`
2. 访问在线文档
3. 填充响应结构体字段
4. 手动完成重构
5. 运行 `cargo check` 验证

### 场景 2: 检查重构进度

使用 Python 脚本分析所有 API：

```bash
python3 scripts/refactor_calendar_apis.py --analyze --report
```

查看摘要报告，了解：
- 哪些 API 已完成
- 哪些模块最需要关注
- 整体进度百分比

### 场景 3: 选择下一个重构目标

根据分析报告的结果，选择：
1. 使用频率最高的模块（如 `calendar/event`）
2. 依赖关系最少的 API
3. 测试覆盖最全面的 API

### 场景 4: 验证重构质量

重构完成后，运行：

```bash
# 1. 编译检查
cargo check -p openlark-meeting --all-features

# 2. Lint 检查
just lint

# 3. 运行测试
just test

# 4. 格式化代码
just fmt
```

---

## 重构模板和参考

### GET 请求模板

参考文件：`crates/openlark-meeting/src/calendar/v4/calendar/get.rs`

关键点：
- 使用 `ApiRequest::get()`
- 使用 `CalendarApiV4::CalendarGet()` 枚举
- 返回类型：`SDKResult<GetCalendarResponse>`
- 数据提取：使用 `.ok_or_else(|| validation_error(...))`

### POST 请求模板

参考文件：`crates/openlark-meeting/src/calendar/v4/calendar/create.rs`

关键点：
- 使用 `ApiRequest::post().body()`
- 使用 `CalendarApiV4::CalendarCreate` 枚举
- 参数：`body: serde_json::Value`
- 数据提取：使用 `serde_json::from_value()`

### DELETE 请求模板

参考文件：`crates/openlark-meeting/src/calendar/v4/calendar/delete.rs`

关键点：
- 使用 `ApiRequest::delete()`
- 使用 `CalendarApiV4::CalendarDelete()` 枚举
- 返回类型：空响应结构体 `DeleteCalendarResponse {}`
- 数据提取：`.ok_or_else(...)`

### PATCH 请求模板

参考文件：`crates/openlark-meeting/src/calendar/v4/calendar/patch.rs`

关键点：
- 使用 `ApiRequest::patch().body()`
- 使用 `CalendarApiV4::CalendarPatch()` 枚举
- 参数：`body: serde_json::Value`
- 数据提取：使用 `serde_json::from_value()`

### LIST 请求模板

参考文件：`crates/openlark-meeting/src/calendar/v4/calendar/list.rs`

关键点：
- 使用 `ApiRequest::get()`
- 使用 `CalendarApiV4::CalendarList` 枚举
- 响应结构：包含分页信息 `page_token`, `has_more`
- 数据类型：`Vec<CalendarData>`

---

## 常见问题和解决方案

### Q1: ResponseFormat::Custom 报错

**错误信息**：
```
no variant named `Custom` in enum `ResponseFormat`
```

**解决方案**：
```rust
// ❌ 错误
ResponseFormat::Custom("calendar".to_string())

// ✅ 正确
ResponseFormat::Data
```

### Q2: unwrap_or_default() 警告

**警告信息**：
```
warning: use of unwrap_or_default() can hide errors
```

**解决方案**：
```rust
// ❌ 错误（掩盖空响应问题）
let data = response.data.unwrap_or_default()?;

// ✅ 正确（明确报错）
let data = response
    .data
    .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))?;
```

### Q3: 重复定义类型

**错误信息**：
```
error[E0428]: the name `CalendarData` is defined multiple times
```

**原因**：复制代码时忘记删除原有的重复定义

**解决方案**：
- 检查文件中是否有重复的 `struct` 定义
- 只保留一个定义
- 或将共享类型移到 `models.rs` 中

### Q4: CalendarApiV4 枚举不存在

**错误信息**：
```
error[E0433]: failed to resolve: use of undeclared type `CalendarApiV4`
```

**解决方案**：
1. 添加导入：`use crate::common::api_endpoints::CalendarApiV4;`
2. 检查 `api_endpoints.rs` 中是否有对应的枚举变体
3. 如果没有，需要先添加枚举变体

---

## 工作流建议

### 单人开发工作流

```bash
# 1. 早晨：查看进度
python3 scripts/refactor_calendar_apis.py --analyze --report

# 2. 选择目标 API（优先级：event > acl > freebusy > 其他）
./scripts/refactor_calendar_api.sh calendar/v4/event get

# 3. 手动完成重构
#    - 填充响应结构
#    - 修改 execute 方法
#    - 更新文档注释

# 4. 验证编译
cargo check -p openlark-meeting --all-features

# 5. 运行测试
just test

# 6. 重复步骤 2-5

# 7. 一天结束时：再次分析进度
python3 scripts/refactor_calendar_apis.py --analyze --report
```

### 团队协作工作流

```bash
# 开发者 A: 处理 calendar/event 模块
git checkout -b refactor/event-module
./scripts/refactor_calendar_api.sh calendar/v4/event get
# ... 完成 18 个 event API
git commit -m "feat: 重构 calendar/event 模块为强类型"

# 开发者 B: 处理 calendar/acl 模块
git checkout -b refactor/acl-module
./scripts/refactor_calendar_api.sh calendar/v4/acl list
# ... 完成 5 个 acl API
git commit -m "feat: 重构 calendar/acl 模块为强类型"

# 合并前：运行完整检查
git checkout main
git merge refactor/event-module
git merge refactor/acl-module
cargo test --all-features
just lint
```

---

## 性能优化建议

### 批量处理

如果要重构多个相关 API，可以创建一个批量脚本：

```bash
#!/bin/bash
# 批量重构 event 模块

apis=("get" "list" "create" "delete" "patch")

for api in "${apis[@]}"; do
    echo "=== 开始重构 event/$api ==="
    ./scripts/refactor_calendar_api.sh calendar/v4/event $api
    echo "=== 完成 event/$api ==="
    echo ""
done
```

### 并行分析

使用 Python 的多进程能力加速分析：

```bash
# 在 refactor_calendar_apis.py 中使用 multiprocessing
# 这需要修改脚本，但可以大幅提升大项目的分析速度
```

---

## 进度追踪

### 当前状态（2025-01-15）

- ✅ **已完成**: 5/45 (11%)
  - calendar/create.rs
  - calendar/delete.rs
  - calendar/get.rs
  - calendar/list.rs
  - calendar/patch.rs

- 🔄 **进行中**: 0/45

- ⏳ **待处理**: 40/45 (89%)

### 下一步计划

1. **优先级 1**: `calendar/event` 模块 (18 API)
   - 使用频率最高
   - 影响范围最大

2. **优先级 2**: `calendar/acl` 模块 (5 API)
   - 权限控制核心功能

3. **优先级 3**: 其他模块 (22 API)

---

## 技术支持

### 相关文档

- 项目根目录：`AGENTS.md`, `ARCHITECTURE.md`
- Calendar 模块：`crates/openlark-meeting/src/calendar/v4/`
- API 端点定义：`crates/openlark-meeting/src/common/api_endpoints.rs`

### 在线资源

- 飞书开放平台文档：https://open.feishu.cn/document/
- Rust 类型系统：https://doc.rust-lang.org/book/ch10-00-generics.html
- Serde 文档：https://serde.rs/

### 获取帮助

如遇到问题：
1. 查看 `/tmp/` 下的临时文件和日志
2. 参考 `/tmp/CALNDAR_REFACTOR_COMPLETION_REPORT.md`
3. 检查 LSP 诊断输出
4. 运行 `cargo check` 查看详细错误信息

---

## 贡献指南

### 改进脚本

如果要改进这些脚本：

1. Fork 项目
2. 创建功能分支
3. 修改脚本
4. 添加测试
5. 提交 Pull Request

### 添加新功能

建议添加的功能：
- ✨ 自动从在线文档提取响应结构
- ✨ 自动生成测试用例
- ✨ Git 集成（自动提交）
- ✨ 性能统计（重构时间、编译时间）
- ✨ 可视化进度图表

---

## 许可证

与项目主许可证一致：MIT OR Apache-2.0

---

**最后更新**: 2025-01-15
**维护者**: Sisyphus Agent
