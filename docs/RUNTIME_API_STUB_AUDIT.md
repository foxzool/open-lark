# Runtime API Stub Audit

本文件细化 issue `#107`：追踪仍然留在用户可见运行时代码中的 TODO API stubs。

这些接口与 `tests/` 里的占位测试不同：它们已经在公开 crate 中暴露给用户，但 `execute()` 仍只返回占位 JSON。

## 范围

本轮只覆盖以下 18 个 source-level TODO API stubs：

| Crate | File | Stub methods | Count |
| --- | --- | --- | ---: |
| `openlark-analytics` | `crates/openlark-analytics/src/search/search/v2/query.rs` | `SearchRequest::execute`, `SuggestRequest::execute` | 2 |
| `openlark-analytics` | `crates/openlark-analytics/src/search/search/v2/user.rs` | `SearchUserRequest::execute` | 1 |
| `openlark-user` | `crates/openlark-user/src/settings/v1.rs` | `GetSettingRequest::execute`, `UpdateSettingRequest::execute`, `ListSettingsRequest::execute` | 3 |
| `openlark-user` | `crates/openlark-user/src/preferences/v1.rs` | `GetPreferenceRequest::execute`, `UpdatePreferenceRequest::execute`, `DeletePreferenceRequest::execute`, `ListPreferencesRequest::execute` | 4 |
| `openlark-platform` | `crates/openlark-platform/src/admin/admin/v1/settings.rs` | `GetSettingRequest::execute`, `UpdateSettingRequest::execute`, `ListSettingsRequest::execute` | 3 |
| `openlark-platform` | `crates/openlark-platform/src/admin/admin/v1/users.rs` | `ListAdminUsersRequest::execute`, `DisableUserRequest::execute`, `EnableUserRequest::execute` | 3 |
| `openlark-platform` | `crates/openlark-platform/src/admin/admin/v1/audit.rs` | `QueryAuditLogsRequest::execute`, `GetAuditLogRequest::execute` | 2 |

## 风险分组

### 1. Analytics 搜索 stubs

影响：

- 看起来像可用的搜索 API，但默认只返回占位结构
- 容易让调用方误以为已经接入了真实搜索服务

跟踪 issue：

- `#108` Implement analytics search runtime TODO stubs

### 2. User 设置 / 偏好 stubs

影响：

- `openlark-user` 暴露的是用户可见配置/偏好能力
- 如果继续保留占位返回，容易误导集成方直接在生产代码中使用

跟踪 issue：

- `#109` Implement user settings and preferences runtime TODO stubs

### 3. Platform admin stubs

影响：

- 平台后台管理接口（settings / users / audit）名称清晰、语义明确
- 当前占位实现会造成“存在接口、但并未真正接线”的错觉

跟踪 issue：

- `#110` Implement platform admin runtime TODO stubs

## 当前结论

- 这些 runtime TODO stubs **不应继续处于无主状态**
- 本轮不强行在一个 issue 里实现全部 18 处
- 已按业务域拆成 3 个 follow-up issues，便于分批收敛

## 与 TODO 总审计的关系

- 总量审计见：`docs/TODO_AUDIT_SUMMARY.md`
- 本文件只聚焦 `source_api_stubs` 这一个 p1 桶
