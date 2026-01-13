# OpenLark Communication Knowledge Base

## OVERVIEW
通讯协作核心模块，涵盖即时通讯 (IM)、通讯录 (Contact)、邮件服务 (Mail)、视频会议 (VC) 及朋友圈 (Moments)。支持 300+ 核心 API。

## STRUCTURE
```
src/
├── im/im/v1/           # IM 核心：消息 (message)、群聊 (chat)、文件/图片
├── contact/contact/v3/ # 通讯录：用户 (user)、部门 (department)、用户组 (group)
├── mail/               # 邮件服务：邮件组 (mailgroup)、邮箱管理
├── vc/                 # 视频会议：会议室 (room)、会议管理 (meeting)
├── moments/            # 动态分享：朋友圈 (post)、互动 (interaction)
├── endpoints/          # 端点常量：全量 API 路径定义 (IM_V1_MESSAGES 等)
└── common/             # 通用类型与工具
```

## WHERE TO LOOK
| 任务 | 路径 | 备注 |
|------|------|------|
| 发送/撤回消息 | `src/im/im/v1/message/` | 支持富文本、图片、卡片 |
| 上传/下载资源 | `src/im/im/v1/file/` | 包含 image 模块 |
| 组织架构管理 | `src/contact/contact/v3/` | 用户 CRUD 与部门搜索 |
| 视频会议控制 | `src/vc/v1/meeting/` | 创建、邀请、踢出、结束 |
| 端点定义查找 | `src/endpoints/mod.rs` | 包含所有业务端点常量 |

## CONVENTIONS
- **文件路径规范**: `src/{bizTag}/{project}/{version}/{resource}/{name}.rs`
  - 示例: `src/im/im/v1/message/create.rs`
- **Builder 模式**: 推荐使用 `create_message_builder()` 等方法，通过 `.execute()` 发起异步请求。
- **端点引用**: 强制使用 `endpoints` 模块中的常量，严禁硬编码 URL。
- **Prelude**: `use openlark_communication::prelude::*;` 可快速获取 `Config` 和 `SDKResult`。

## ANTI-PATTERNS
- **避免**: 在业务代码中手动拼接 API 路径。
- **避免**: 绕过 `ServiceRegistry` 直接构建子服务实例。
- **避免**: 忽视 `cfg(feature = "im")` 等条件编译，导致非必要模块被包含。
- **避免**: 在 `src/lib.rs` 中直接定义端点，应统一归口到 `endpoints`。
