# OpenLark Auth APIs 实现计划

## 任务背景

逐个检查 `crates/openlark-auth/src/` 里的API实现，打开 `analysis/data/api_list_export.csv`，过滤出CSV里bizTag=auth，参考docPath里的网页，对照实现。严格按照bizTag, meta.project, meta.version, meta.resource, meta.name进行组织。

## 目标API清单

### meta.project = auth (5个)
1. `app_access_token` (v3/auth/auth) - 商店应用获取app_access_token
2. `app_access_token_internal` (v3/auth/auth) - 自建应用获取app_access_token
3. `tenant_access_token` (v3/auth/auth) - 商店应用获取tenant_access_token
4. `tenant_access_token_internal` (v3/auth/auth) - 自建应用获取tenant_access_token
5. `app_ticket_resend` (v3/auth/auth) - 重新获取app_ticket

### meta.project = authen (5个)
1. `user_info` (v1/authen/user_info) - 获取用户信息
2. `oidc.access_token` (v1/authen/oidc/access_token) - 获取user_access_token
3. `oidc.refresh_access_token` (v1/authen/oidc/refresh_access_token) - 刷新user_access_token
4. `access_token` (v1/authen/access_token) - 获取user_access_token（v1版本）
5. `refresh_access_token` (v1/authen/refresh_access_token) - 刷新user_access_token（v1版本）

### meta.project = oauth (1个)
1. `default` (old/oauth/default) - 获取登录预授权码

## 目录结构规范

严格按照 `biztag/project/version/resource/name.rs` 进行组织：
- auth项目：`src/api/auth/v3/auth/{name}.rs`
- authen项目：`src/api/authen/v1/{resource}/{name}.rs`
- oauth项目：`src/api/oauth/old/default/{name}.rs`

## 技术要求

- 使用 openlark-core 的错误处理系统
- 正确的客户端调用方式
- 零警告编译
- 类型安全的API设计
- 中文文档注释

## 实施状态

- [x] 需求分析
- [x] 方案设计
- [x] 详细计划
- [ ] 当前实现状态检查
- [ ] API实现和修复
- [ ] 模块导出更新
- [ ] 质量验证

## 上下文

项目根目录：`/Users/zool/RustroverProjects/open-lark/crates/openlark-auth`
工作模式：渐进式重构实现
质量标准：企业级代码规范