# AGENTS.md (openlark-meeting)

## OVERVIEW
飞书会议与日程服务实现。涵盖 117 个 API（pub async fn），175 个源码文件。核心特征是日历（calendar）、视频会议（vc）、会议室（meeting_room）三大业务域。

## STRUCTURE
- **Calendar（日历）**: v4 (44 APIs) - 日程管理、忙闲查询、权限控制
- **VC（视频会议）**: v1 (56 APIs) - 会议管理、录制、导出、统计
- **Meeting Room（会议室）**: 历史版本 (17 APIs) - 建筑、房间、预约管理

## WHERE TO LOOK
- **日程管理**: `src/calendar/v4/calendar/event/`
- **忙闲查询**: `src/calendar/v4/freebusy/`
- **视频会议**: `src/vc/v1/meeting/`
- **会议室管理**: `src/meeting_room/room/`
- **Endpoint 定义**: `src/common/api_endpoints.rs` (基于 Enum 的类型安全映射)

## CONVENTIONS
- **路径规范**: `src/{bizTag}/{version}/{resource}/{name}.rs`
  - 示例: `src/calendar/v4/calendar/event/get.rs`
  - 示例: `src/vc/v1/meeting/list_by_no.rs`
- **版本策略**:
  - `v4`: calendar（现行）
  - `v1`: vc（现行）、meeting_room（历史版本）
- **URL 映射**: 严格遵循 `/open-apis/{project}/v{version}/{resource}/{action}`
- **编译控制**: 细粒度 Feature flags (如 `calendar`, `vc`, `meeting-room`)

## ANTI-PATTERNS
- **版本混用**: 在 `v1` 目录下实现 `v4` 逻辑
- **过时依赖**: 新功能开发调用已弃用的 API
- **硬编码 URL**: 绕过 `ApiEndpoint` 枚举手动拼接字符串
- **缺失 RequestOption**: 不实现 `execute_with_options` 方法
