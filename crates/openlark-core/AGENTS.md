# openlark-core Knowledge Base

**Crate**: Core Infrastructure  
**Purpose**: HTTP 客户端、错误处理、认证和核心工具

## OVERVIEW

OpenLark 核心基础设施 crate，提供所有业务模块共享的底层能力。包含企业级错误处理系统、HTTP 客户端抽象、令牌管理和验证工具。

## STRUCTURE

```
src/
├── api.rs                    # API 响应处理 trait
├── api_endpoints.rs          # 端点常量定义
├── auth/                     # 认证相关（令牌、签名）
│   ├── token.rs             # 令牌管理
│   └── sign.rs              # 请求签名
├── config.rs                 # 客户端配置
├── constants.rs              # 全局常量
├── error/                    # CoreError 错误系统
│   ├── core.rs              # 核心错误类型
│   ├── codes.rs             # 飞书错误码映射
│   ├── observability.rs     # 可观测性集成
│   └── reporter.rs          # 错误报告
├── http.rs                   # HTTP 客户端封装
├── observability.rs          # 日志和追踪
├── query_params.rs          # 查询参数构建
├── req_option.rs            # 请求选项
├── request_builder.rs       # 请求构建器
├── trait_system.rs          # 核心 trait 定义
├── validation.rs            # 参数验证
└── ...
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 添加错误码 | `src/error/codes.rs` | 飞书通用错误码映射 |
| 修改 HTTP 行为 | `src/http.rs` | reqwest 客户端配置 |
| 添加验证规则 | `src/validation/` | 复用 validate_required 宏 |
| 令牌管理 | `src/auth/token.rs` | AccessToken/RefreshToken |
| 常量定义 | `src/constants.rs` | BASE_URL, 默认超时等 |

## CONVENTIONS

### 错误处理
```rust
// 使用 CoreError 统一错误类型
pub type SDKResult<T> = Result<T, CoreError>;

// 验证宏
validate_required!(self.field, "字段不能为空");
validate_required_list!(self.items, 50, "列表不能为空且不能超过 50 项");
```

### 公开 API 设计
- **最小化原则**: 只导出必要的类型（KISS）
- **内部模块**: `mod xxx;` 不导出实现细节
- **Re-export**: 常用类型从 crate root 重新导出

### 错误码优先级
1. 飞书通用 `code`（如 99991671 Token 无效）
2. HTTP `status`
3. 内部业务码

## ANTI-PATTERNS

- ❌ 不要把 core 变成"全家桶"（避免暴露过多内部模块）
- ❌ 不要在 core 引入业务逻辑
- ❌ 不要破坏 Validatable trait 的语义一致性

## COMMANDS

```bash
# 测试 core
cargo test -p openlark-core

# 检查 core 独立编译
cargo check -p openlark-core
```
