# OpenLark Meeting 模块 API 列表

**生成日期**: 2026-01-03
**来源文件**: `api_list_export.csv`
**筛选条件**: bizTag IN (calendar, vc, meeting_room)
**输出文件**: `crates/openlark-meeting/filtered_apis.csv`

---

## 一、统计摘要

### 1.1 总体统计

| 模块 | API 数量 | 占比 |
|------|---------|------|
| **Calendar** | 44 | 37.6% |
| **VC** | 56 | 47.9% |
| **Meeting Room** | 17 | 14.5% |
| **总计** | **117** | 100% |

### 1.2 按版本分布

| 版本 | 模块 | API 数量 |
|------|------|---------|
| **v4** | Calendar | 44 |
| **v1** | VC | 56 |
| **old/default** | Meeting Room | 17 |

---

## 二、Calendar API (44个)

### 2.1 日历基础操作 (14个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6952888507003076635 | 创建共享日历 | /open-apis/calendar/v4/calendars | POST |
| 6952888507002994715 | 删除共享日历 | /open-apis/calendar/v4/calendars/:calendar_id | DELETE |
| 6952888507002978331 | 查询日历信息 | /open-apis/calendar/v4/calendars/:calendar_id | GET |
| 6952888507002880027 | 查询日历列表 | /open-apis/calendar/v4/calendars | GET |
| 6952888507003158555 | 更新日历信息 | /open-apis/calendar/v4/calendars/:calendar_id | PATCH |
| 6952888507002863643 | 搜索日历 | /open-apis/calendar/v4/calendars/search | POST |
| 7051880392425439236 | 查询主日历信息 | /open-apis/calendar/v4/calendars/primary | POST |
| 7327132452408393732 | 批量获取主日历信息 | /open-apis/calendar/v4/calendars/primarys | POST |
| 7327132452408410116 | 批量查询日历信息 | /open-apis/calendar/v4/calendars/mget | POST |
| 6952888507002748955 | 订阅日历 | /open-apis/calendar/v4/calendars/:calendar_id/subscribe | POST |
| 6952888507003093019 | 取消订阅日历 | /open-apis/calendar/v4/calendars/:calendar_id/unsubscribe | POST |
| 6952888507003060251 | 订阅日历变更事件 | /open-apis/calendar/v4/calendars/subscription | POST |
| 7129706575502098436 | 取消订阅日历变更事件 | /open-apis/calendar/v4/calendars/unsubscription | POST |

### 2.2 日程管理 (12个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6990540948577599491 | 创建日程 | /open-apis/calendar/v4/calendars/:calendar_id/events | POST |
| 6952888507002961947 | 删除日程 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id | DELETE |
| 6952888507003043867 | 更新日程 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id | PATCH |
| 6952888507002699803 | 获取日程 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id | GET |
| 6952888507002798107 | 获取日程列表 | /open-apis/calendar/v4/calendars/:calendar_id/events | GET |
| 6952888507003109403 | 搜索日程 | /open-apis/calendar/v4/calendars/:calendar_id/events/search | POST |
| 6952888507002716187 | 订阅日程变更事件 | /open-apis/calendar/v4/calendars/:calendar_id/events/subscription | POST |
| 7129706575502131204 | 取消订阅日程变更事件 | /open-apis/calendar/v4/calendars/:calendar_id/events/unsubscription | POST |
| 7317471576948834305 | 回复日程 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/reply | POST |
| 7317471576948850689 | 获取重复日程实例 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/instances | GET |
| 7322810271218647043 | 查询日程视图 | /open-apis/calendar/v4/calendars/:calendar_id/events/instance_view | GET |

### 2.3 日程参与人管理 (4个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6952888507003125787 | 添加日程参与人 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees | POST |
| 6952888507002830875 | 删除日程参与人 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/batch_delete | POST |
| 6952888507002896411 | 获取日程参与人列表 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees | GET |
| 6952888507002847259 | 获取日程参与群成员列表 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/:attendee_id/chat_members | GET |

### 2.4 日历访问控制 (5个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6952888507002814491 | 创建访问控制 | /open-apis/calendar/v4/calendars/:calendar_id/acls | POST |
| 6952888507002945563 | 删除访问控制 | /open-apis/calendar/v4/calendars/:calendar_id/acls/:acl_id | DELETE |
| 6953067803433009153 | 获取访问控制列表 | /open-apis/calendar/v4/calendars/:calendar_id/acls | GET |
| 6952888507003027483 | 订阅日历访问控制变更事件 | /open-apis/calendar/v4/calendars/:calendar_id/acls/subscription | POST |
| 7129706575502114820 | 取消订阅日历访问控制变更事件 | /open-apis/calendar/v4/calendars/:calendar_id/acls/unsubscription | POST |

### 2.5 会议群和会议纪要 (3个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 7263360328350744580 | 创建会议群 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat | POST |
| 7263360328350728196 | 解绑会议群 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat | DELETE |
| 7263360328350760964 | 创建会议纪要 | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_minute | POST |

### 2.6 忙闲状态 (2个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6952888507002912795 | 查询主日历日程忙闲信息 | /open-apis/calendar/v4/freebusy/list | POST |
| 7327132452408426500 | 批量查询主日历日程忙闲信息 | /open-apis/calendar/v4/freebusy/batch | POST |

### 2.7 请假事件 (2个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6952888507003142171 | 创建请假日程 | /open-apis/calendar/v4/timeoff_events | POST |
| 6952888507002765339 | 删除请假日程 | /open-apis/calendar/v4/timeoff_events/:timeoff_event_id | DELETE |

### 2.8 配置和 Exchange 集成 (4个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6952888507002929179 | 生成 CalDAV 配置 | /open-apis/calendar/v4/settings/generate_caldav_conf | POST |
| 6963176044560728066 | 将 Exchange 账户绑定到飞书账户 | /open-apis/calendar/v4/exchange_bindings | POST |
| 6963176044560760834 | 解除 Exchange 账户绑定 | /open-apis/calendar/v4/exchange_bindings/:exchange_binding_id | DELETE |
| 6963176044560744450 | 查询 Exchange 账户的绑定状态 | /open-apis/calendar/v4/exchange_bindings/:exchange_binding_id | GET |

---

## 三、VC API (56个)

### 3.1 会议预订 (5个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6960861158129008643 | 预约会议 | /open-apis/vc/v1/reserves/apply | POST |
| 6960861158129041411 | 删除预约 | /open-apis/vc/v1/reserves/:reserve_id | DELETE |
| 6921909217674854427 | 更新预约 | /open-apis/vc/v1/reserves/:reserve_id | PUT |
| 6921909217674936347 | 获取预约 | /open-apis/vc/v1/reserves/:reserve_id | GET |
| 6921909217674952731 | 获取活跃会议 | /open-apis/vc/v1/reserves/:reserve_id/get_active_meeting | GET |

### 3.2 会议管理 (6个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6960861158593134596 | 邀请参会人 | /open-apis/vc/v1/meetings/:meeting_id/invite | PATCH |
| 6997003722790633474 | 移除参会人 | /open-apis/vc/v1/meetings/:meeting_id/kickout | POST |
| 6921909217674805275 | 设置主持人 | /open-apis/vc/v1/meetings/:meeting_id/set_host | PATCH |
| 6960861158593118212 | 结束会议 | /open-apis/vc/v1/meetings/:meeting_id/end | PATCH |
| 6960861158128926723 | 获取会议详情 | /open-apis/vc/v1/meetings/:meeting_id | GET |
| 7013251669786116097 | 获取与会议号关联的会议列表 | /open-apis/vc/v1/meetings/list_by_no | GET |

### 3.3 会议录制 (4个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6960861158593150980 | 开始录制 | /open-apis/vc/v1/meetings/:meeting_id/recording/start | PATCH |
| 6960861158129057795 | 停止录制 | /open-apis/vc/v1/meetings/:meeting_id/recording/stop | PATCH |
| 6960861158593101828 | 获取录制文件 | /open-apis/vc/v1/meetings/:meeting_id/recording | GET |
| 6960861158129025027 | 授权录制文件 | /open-apis/vc/v1/meetings/:meeting_id/recording/set_permission | PATCH |

### 3.4 会议室管理 (7个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 7160517357592084483 | 创建会议室 | /open-apis/vc/v1/rooms | POST |
| 7160517356095979524 | 删除会议室 | /open-apis/vc/v1/rooms/:room_id | DELETE |
| 7160517356095963140 | 更新会议室 | /open-apis/vc/v1/rooms/:room_id | PATCH |
| 7160517357592068099 | 查询会议室详情 | /open-apis/vc/v1/rooms/:room_id | GET |
| 7160517356095946756 | 批量查询会议室详情 | /open-apis/vc/v1/rooms/mget | POST |
| 7160517357592051715 | 查询会议室列表 | /open-apis/vc/v1/rooms | GET |
| 7160517357591937027 | 搜索会议室 | /open-apis/vc/v1/rooms/search | POST |

### 3.5 会议室层级 (7个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 7160517357591986179 | 创建会议室层级 | /open-apis/vc/v1/room_levels | POST |
| 7160517357592002563 | 删除会议室层级 | /open-apis/vc/v1/room_levels/del | POST |
| 7160517356095995908 | 更新会议室层级 | /open-apis/vc/v1/room_levels/:room_level_id | PATCH |
| 7160517357592133635 | 查询会议室层级详情 | /open-apis/vc/v1/room_levels/:room_level_id | GET |
| 7160517356095897604 | 批量查询会议室层级详情 | /open-apis/vc/v1/room_levels/mget | POST |
| 7160517357592117251 | 查询会议室层级列表 | /open-apis/vc/v1/room_levels | GET |
| 7160517356095913988 | 搜索会议室层级 | /open-apis/vc/v1/room_levels/search | GET |

### 3.6 会议室配置 (7个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 7160517356095930372 | 查询会议室配置 | /open-apis/vc/v1/scope_config | GET |
| 7160517357591920643 | 设置会议室配置 | /open-apis/vc/v1/scope_config | POST |
| 7152043170151333892 | 查询会议室预定限制 | /open-apis/vc/v1/reserve_configs/reserve_scope | GET |
| 7152043170151350276 | 更新会议室预定限制 | /open-apis/vc/v1/reserve_configs/:reserve_config_id | PATCH |
| 7194790671877144578 | 查询会议室预定表单 | /open-apis/vc/v1/reserve_configs/:reserve_config_id/form | GET |
| 7194790671877160962 | 更新会议室预定表单 | /open-apis/vc/v1/reserve_configs/:reserve_config_id/form | PATCH |

### 3.7 会议室特定配置 (5个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 7194805625628000259 | 更新会议室预定管理员 | /open-apis/vc/v1/reserve_configs/:reserve_config_id/admin | PATCH |
| 7211447510368534532 | 查询禁用状态变更通知 | /open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform | GET |
| 7211447510368550916 | 更新禁用状态变更通知 | /open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform | PATCH |

### 3.8 导出功能 (6个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 7124235806021238785 | 导出会议明细 | /open-apis/vc/v1/exports/meeting_list | POST |
| 7124195547444477980 | 导出参会人明细 | /open-apis/vc/v1/exports/participant_list | POST |
| 7124195547444494364 | 导出参会人会议质量数据 | /open-apis/vc/v1/exports/participant_quality_list | POST |
| 7124195547444510748 | 导出会议室预定数据 | /open-apis/vc/v1/exports/resource_reservation_list | POST |
| 7124195547444527132 | 查询导出任务结果 | /open-apis/vc/v1/exports/:task_id | GET |
| 7143809848869109763 | 下载导出文件 | /open-apis/vc/v1/exports/download | GET |

### 3.9 报告和统计 (6个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6921909217674821659 | 获取会议报告 | /open-apis/vc/v1/reports/get_daily | GET |
| 6921909217674723355 | 获取 Top 用户列表 | /open-apis/vc/v1/reports/get_top_user | GET |
| 7194805625628033027 | 查询会议明细 | /open-apis/vc/v1/meeting_list | GET |
| 7194805625628147715 | 查询参会人明细 | /open-apis/vc/v1/participant_list | GET |
| 7194805625628049411 | 查询参会人会议质量数据 | /open-apis/vc/v1/participant_quality_list | GET |
| 7194805625628065795 | 查询会议室预定数据 | /open-apis/vc/v1/resource_reservation_list | GET |

### 3.10 告警 (1个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 7146108826851770396 | 获取告警记录 | /open-apis/vc/v1/alerts | GET |

---

## 四、Meeting Room (旧版) API (17个)

### 4.1 建筑管理 (5个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6907569523176636417 | 获取建筑物列表 | /open-apis/meeting_room/building/list | GET |
| 6907569523177635841 | 查询建筑物详情 | /open-apis/meeting_room/building/batch_get | GET |
| 6907569523177766913 | 获取会议室列表 | /open-apis/meeting_room/room/list | GET |
| 6907569523177766913 | 查询会议室详情 | /open-apis/meeting_room/room/batch_get | GET |
| 6907569524100956161 | 查询会议室忙闲 | /open-apis/meeting_room/freebusy/batch_get | GET |

### 4.2 会议相关 (4个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6907569745299226626 | 回复会议室日程实例 | /open-apis/meeting_room/instance/reply | POST |
| 6922096654371831836 | 查询会议室日程主题和会议详情 | /open-apis/meeting_room/summary/batch_get | POST |

### 4.3 地理信息 (2个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 6907569524100808705 | 获取城市列表 | /open-apis/meeting_room/district/list | GET |
| 6907569524100694017 | 获取国家地区列表 | /open-apis/meeting_room/country/list | GET |

### 4.4 会议室特定配置 (6个)

| ID | API 名称 | 端点路径 | 方法 |
|----|---------|----------|------|
| 7070073879629430788 | 创建签到板部署码 | /open-apis/vc/v1/room_configs/set_checkboard_access_code | POST |
| 7070073879629447172 | 创建会议室部署码 | /open-apis/vc/v1/room_configs/set_room_access_code | POST |
| 6921909217674739739 | 查询会议室配置 | /open-apis/vc/v1/room_config/query | GET |
| 692190921767476123 | 设置会议室配置 | /open-apis/vc/v1/room_config/set | POST |

---

## 五、使用说明

### 5.1 文件格式

CSV 文件包含以下字段：

| 字段名 | 说明 | 示例 |
|-------|------|------|
| id | API 唯一标识符 | 6952888507003076635 |
| name | API 名称 | 创建共享日历 |
| bizTag | 业务标签 | calendar |
| meta.Project | 项目名称 | calendar |
| meta.Version | 版本号 | v4 |
| meta.Resource | 资源名称 | calendar |
| meta.Name | 操作名称 | create |
| detail | 详细描述 | 该接口用于为当前身份... |
| chargingMethod | 计费方式 | none, basic |
| fullDose | 是否全额剂量 | true |
| fullPath | 文档路径 | /document/... |
| url | API 路径和 HTTP 方法 | POST:/open-apis/... |
| orderMark | 排序标记 | 0 |
| supportAppTypes | 支持的应用类型 | [""isv"", ""custom""] |
| tags | 标签列表 | [] |
| updateTime | 更新时间 | 1721110838 |
| isCharge | 是否付费 | true, false |
| meta.Type | 类型 | 1 |
| docPath | 完整文档 URL | https://open.feishu.cn/document/... |

### 5.2 筛选命令

```bash
# 从原始 CSV 筛选 calendar、vc、meeting_room
awk -F',' '$3 ~ /calendar|vc|meeting_room/' api_list_export.csv > filtered_apis.csv

# 按 bizTag 分组统计
awk -F',' '{
  if ($3 == "calendar") calendar++;
  else if ($3 == "vc") vc++;
  else if ($3 == "meeting_room") meeting_room++;
}
END {
  printf "Calendar: %d\n", calendar;
  printf "VC: %d\n", vc;
  printf "Meeting Room: %d\n", meeting_room;
  printf "Total: %d\n", calendar + vc + meeting_room;
}' filtered_apis.csv
```

### 5.3 使用 Python 处理

```python
import pandas as pd

# 读取 CSV
df = pd.read_csv('filtered_apis.csv')

# 统计各模块 API 数量
counts = df['bizTag'].value_counts()
print(counts)

# 筛选特定模块
calendar_apis = df[df['bizTag'] == 'calendar']
print(f"Calendar APIs: {len(calendar_apis)}")

# 导出为 Excel
calendar_apis.to_excel('calendar_apis.xlsx', index=False)
```

### 5.4 使用 JavaScript 处理

```javascript
const csv = require('csv-parser');
const fs = require('fs');

fs.createReadStream('filtered_apis.csv')
  .pipe(csv())
  .on('data', (row) => {
    console.log(`${row.name}: ${row.url}`);
  })
  .on('end', () => {
    console.log('CSV parsing completed');
  });
```

---

## 六、版本信息

### 6.1 版本对比

| 版本 | 模块 | API 数量 | 说明 |
|------|------|---------|------|
| **v4** | Calendar | 44 | 最新版本的日历 API |
| **v1** | VC | 56 | 最新版本的视频会议 API |
| **old/default** | Meeting Room | 17 | 旧版会议室 API（历史版本） |

### 6.2 升级建议

**Meeting Room (old/default) API 是历史版本**，建议：

1. **逐步迁移**：将旧版 API 调用迁移到新版 VC API
2. **兼容层**：如需同时支持旧版，实现兼容适配器
3. **文档更新**：更新文档，标注旧版 API 为废弃
4. **通知用户**：提前通知 API 废弃和迁移计划

---

## 七、API 权限说明

### 7.1 权限类型

| 权限类型 | 数量 | 占比 |
|---------|------|------|
| **none (公开)** | 5 | 4.3% |
| **basic (企业认证)** | 112 | 95.7% |

### 7.2 付费 API

| 是否付费 | 数量 | 占比 |
|---------|------|------|
| **免费** | 6 | 5.1% |
| **付费** | 111 | 94.9% |

**注意**：大部分 API 需要企业认证（basic）和付费许可，使用前请确认企业已开通相应服务。

---

## 八、附录

### 8.1 相关文档

- 实施计划：`crates/openlark-meeting/IMPLEMENTATION_PLAN.md`
- 删除报告：`crates/openlark-meeting/CLEANUP_REPORT.md`
- 执行报告：`crates/openlark-meeting/CLEANUP_EXECUTION_REPORT.md`
- 清理脚本：`scripts/cleanup_redundant_apis.sh`

### 8.2 联系支持

如有问题或建议，请联系：
- 项目地址：https://github.com/foxzool/open-lark
- Issue tracker：https://github.com/foxzool/open-lark/issues

---

**文档版本**: 1.0
**生成日期**: 2026-01-03
**最后更新**: 2026-01-03
**状态**: ✅ 完成
