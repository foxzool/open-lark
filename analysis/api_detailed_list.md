# 详细 API 清单

## 按 biztag 排序的详细 API 列表

### ccm (174 个 API)

| API名称 | biztag | project-category | 版本 | 资源类型 |
|---------|--------|------------------|------|----------|
| 获取群公告基本信息 | ccm | docx | v1 | chat.announcement |
| 获取群公告所有块 | ccm | docx | v1 | chat.announcement |
| 在群公告中创建块 | ccm | docx | v1 | chat.announcement.block.children |
| 批量更新群公告块的内容 | ccm | docx | v1 | chat.announcement.block |
| 获取群公告块的内容 | ccm | docx | v1 | chat.announcement.block |
| 获取所有子块 | ccm | docx | v1 | chat.announcement.block.children |
| 删除群公告中的块 | ccm | docx | v1 | chat.announcement.block.children |
| 获取我的空间（根文件夹）元数据 | ccm | ccm_drive_explorer | old | v2/root_folder/meta |
| 获取文件夹中的文件清单 | ccm | drive | v1 | file |
| 获取文件夹元数据 | ccm | ccm_drive_explorer | old | v2/folder/:folderToken/meta |
| 新建文件夹 | ccm | drive | v1 | file |
| 查询异步任务状态 | ccm | drive | v1 | file |
| 获取文件元数据 | ccm | drive | v1 | meta |
| 获取文件统计信息 | ccm | drive | v1 | file.statistics |
| 获取文件访问记录 | ccm | drive | v1 | file.view_record |
| 复制文件 | ccm | drive | v1 | file |
| 移动文件或文件夹 | ccm | drive | v1 | file |
| 删除文件或文件夹 | ccm | drive | v1 | file |
| 创建文件快捷方式 | ccm | drive | v1 | file |
| 搜索云文档 | ccm | ccm_docs | old | docs-api/search/object |

(以下省略 154 个 ccm API...)

### base (49 个 API)

| API名称 | biztag | project-category | 版本 | 资源类型 |
|---------|--------|------------------|------|----------|
| 获取租户访问令牌 | base | auth | v1 | auth |
| 获取用户访问令牌 | base | auth | v1 | auth |
| 刷新用户访问令牌 | base | auth | v1 | auth |
| 获取应用访问令牌 | base | auth | v1 | auth |
| 撤销应用访问令牌 | base | auth | v1 | auth |
| 获取应用已授权用户范围 | base | contact | v3 | user |
| 获取用户已授权应用范围 | base | contact | v3 | user |
| 获取用户已授权应用范围 | base | contact | v3 | user |
| 获取部门已授权应用范围 | base | contact | v3 | department |
| 获取事件出口 IP | base | event | v1 | outbound_ip |
| 获取应用已授权部门范围 | base | contact | v3 | department |
| 获取应用已授权群组范围 | base | contact | v3 | group |
| 获取部门已授权应用范围 | base | contact | v3 | department |
| 获取应用已授权用户范围 | base | contact | v3 | user |
| 获取用户已授权应用范围 | base | contact | v3 | user |
| 获取部门已授权应用范围 | base | contact | v3 | department |
| 获取群组已授权应用范围 | base | contact | v3 | group |
| 获取应用已授权群组范围 | base | contact | v3 | group |
| 获取用户已授权应用范围 | base | contact | v3 | user |
| 获取应用已授权部门范围 | base | contact | v3 | department |

(以下省略 29 个 base API...)

### baike (27 个 API)

| API名称 | biztag | project-category | 版本 | 资源类型 |
|---------|--------|------------------|------|----------|
| 创建实体 | baike | entity | entity | create |
| 获取实体 | baike | entity | entity | get |
| 更新实体 | baike | entity | entity | update |
| 删除实体 | baike | entity | entity | delete |
| 搜索实体 | baike | entity | entity | search |
| 匹配实体 | baike | entity | entity | match |
| 获取实体列表 | baike | entity | entity | list |
| 提取实体 | baike | entity | entity | extract |
| 获取实体高亮 | baike | entity | entity | highlight |
| 创建草稿 | baike | draft | draft | create |
| 更新草稿 | baike | draft | draft | update |
| 上传文件 | baike | file | v1 | upload |
| 下载文件 | baike | file | v1 | download |
| 获取仓库列表 | baike | repo | v1 | list |
| 获取分类列表 | baike | classification | v1 | list |

### minutes (4 个 API)

| API名称 | biztag | project-category | 版本 | 资源类型 |
|---------|--------|------------------|------|----------|
| 获取会议纪要基本信息 | minutes | minutes | v1 | meeting.minutes |
| 创建会议纪要 | minutes | minutes | v1 | meeting.minutes |
| 更新会议纪要 | minutes | minutes | v1 | meeting.minutes |
| 删除会议纪要 | minutes | minutes | v1 | meeting.minutes |
