# openlark-docs API 详细清单

## 总览
- **总计**: 254 个 API
- **已分析**: 254 个 API (100%)
- **待实现**: 254 个 API
- **状态**: 规划阶段

---

## 📁 drive - 云空间文件管理 (59 APIs)

### drive/v1 - 核心文件操作 (56 APIs)

#### file - 文件基础操作 (15 APIs)
1. `POST:/open-apis/drive/v1/files/create_folder` - 新建文件夹
2. `GET:/open-apis/drive/v1/files/task_check` - 查询异步任务状态
3. `POST:/open-apis/drive/v1/metas/batch_query` - 获取文件元数据
4. `GET:/open-apis/drive/v1/files/:file_token/statistics` - 获取文件统计信息
5. `GET:/open-apis/drive/v1/files/:file_token/view_records` - 获取文件访问记录
6. `POST:/open-apis/drive/v1/files/:file_token/copy` - 复制文件
7. `POST:/open-apis/drive/v1/files/:file_token/move` - 移动文件或文件夹
8. `DELETE:/open-apis/drive/v1/files/:file_token` - 删除文件或文件夹
9. `POST:/open-apis/drive/v1/files/create_shortcut` - 创建文件快捷方式
10. `POST:/open-apis/drive/v1/files/upload_all` - 上传文件
11. `POST:/open-apis/drive/v1/files/upload_prepare` - 分片上传文件-预上传
12. `POST:/open-apis/drive/v1/files/upload_part` - 分片上传文件-上传分片
13. `POST:/open-apis/drive/v1/files/upload_finish` - 分片上传文件-完成上传
14. `GET:/open-apis/drive/v1/files/:file_token/download` - 下载文件

#### permission.member - 成员权限管理 (7 APIs)
15. `POST:/open-apis/drive/v1/permissions/members/batch_add` - 批量添加权限成员
16. `POST:/open-apis/drive/v1/permissions/members/batch_update` - 批量更新权限成员
17. `POST:/open-apis/drive/v1/permissions/members/batch_delete` - 批量删除权限成员
18. `POST:/open-apis/drive/v1/permissions/members/add` - 添加权限成员
19. `POST:/open-apis/drive/v1/permissions/members/update` - 更新权限成员
20. `DELETE:/open-apis/drive/v1/permissions/members/delete` - 删除权限成员

#### media - 素材文件管理 (6 APIs)
21. `POST:/open-apis/drive/v1/medias/upload_all` - 上传素材
22. `POST:/open-apis/drive/v1/medias/upload_prepare` - 分片上传素材-预上传
23. `POST:/open-apis/drive/v1/medias/upload_part` - 分片上传素材-上传分片
24. `POST:/open-apis/drive/v1/medias/upload_finish` - 分片上传素材-完成上传
25. `GET:/open-apis/drive/v1/medias/:file_token/download` - 下载素材
26. `GET:/open-apis/drive/v1/medias/batch_get_tmp_download_url` - 获取素材临时下载链接

#### file.comment - 文件评论功能 (5 APIs)
27. `POST:/open-apis/drive/v1/files/:file_token/comments` - 创建文件评论
28. `GET:/open-apis/drive/v1/files/:file_token/comments` - 获取文件评论列表
29. `PATCH:/open-apis/drive/v1/files/:file_token/comments/:comment_id` - 更新文件评论
30. `DELETE:/open-apis/drive/v1/files/:file_token/comments/:comment_id` - 删除文件评论

#### file.version - 文档版本管理 (4 APIs)
31. `POST:/open-apis/drive/v1/files/:file_token/versions` - 创建文档版本
32. `GET:/open-apis/drive/v1/files/:file_token/versions` - 获取文档版本列表
33. `GET:/open-apis/drive/v1/files/:file_token/versions/:version_id` - 获取文档版本信息
34. `DELETE:/open-apis/drive/v1/files/:file_token/versions/:version_id` - 删除文档版本

#### 其他资源 (19 APIs)
35. `POST:/open-apis/drive/v1/files/:file_token/import` - 创建导入任务
36. `GET:/open-apis/drive/v1/import_tasks/:ticket` - 查询导入任务结果
37. `POST:/open-apis/drive/v1/files/:file_token/export` - 创建导出任务
38. `GET:/open-apis/drive/v1/export_tasks/:task_id` - 查询导出任务结果
39. `GET:/open-apis/drive/export_tasks/file/:file_token/download` - 下载导出文件
40. `POST:/open-apis/drive/v1/files/:file_token/subscriptions/add` - 添加订阅
41. `GET:/open-apis/drive/v1/files/:file_token/subscriptions/list` - 获取订阅列表
42. `DELETE:/open-apis/drive/v1/files/:file_token/subscriptions/delete` - 删除订阅
43. `POST:/open-apis/drive/v1/permissions/public/add` - 添加公开链接
44. `GET:/open-apis/drive/v1/permissions/public` - 获取公开链接信息
45. `DELETE:/open-apis/drive/v1/permissions/public/delete` - 删除公开链接
46. `POST:/open-apis/drive/v1/permissions/public/password/add` - 添加公开链接密码
47. `GET:/open-apis/drive/v1/permissions/public/password` - 获取公开链接密码信息
48. `DELETE:/open-apis/drive/v1/permissions/public/password/delete` - 删除公开链接密码
49. `POST:/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies` - 创建评论回复
50. `GET:/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies` - 获取评论回复列表
51. `PATCH:/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies/:reply_id` - 更新评论回复
52. `DELETE:/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies/:reply_id` - 删除评论回复
53. `GET:/open-apis/drive/v1/files/:file_token/statistics` - 获取文件统计信息
54. `GET:/open-apis/drive/v1/files/:file_token/view_records` - 获取文件访问记录

#### file.comment.reply - 评论回复 (3 APIs)
55. `POST:/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies` - 创建评论回复
56. `GET:/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies` - 获取评论回复列表
57. `DELETE:/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies/:reply_id` - 删除评论回复

#### import_task - 导入任务 (2 APIs)
58. `POST:/open-apis/drive/v1/files/:file_token/import` - 创建导入任务
59. `GET:/open-apis/drive/v1/import_tasks/:ticket` - 查询导入任务结果

### drive/v2 - 增强功能 (3 APIs)

#### file.like - 点赞功能 (1 API)
60. `GET:/open-apis/drive/v2/files/:file_token/likes` - 获取云文档的点赞者列表

#### permission.public - 公开链接增强 (2 APIs)
61. `PUT:/open-apis/drive/v2/public_links/:link_token` - 更新公开链接
62. `GET:/open-apis/drive/v2/public_links/:link_token/permissions` - 获取公开链接权限

---

## 📁 bitable - 多维表格 (46 APIs)

### bitable/v1 - 完整的多维表格功能

#### app.table.record - 记录CRUD操作 (10 APIs)
63. `POST:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records` - 新增记录
64. `PUT:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id` - 更新记录
65. `POST:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/search` - 查询记录
66. `DELETE:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id` - 删除记录
67. `POST:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_create` - 新增多条记录
68. `POST:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_update` - 更新多条记录
69. `POST:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_get` - 批量获取记录
70. `POST:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_delete` - 删除多条记录
71. `GET:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id` - 检索记录
72. `GET:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records` - 列出记录

#### app.table - 数据表管理 (6 APIs)
73. `POST:/open-apis/bitable/v1/apps/:app_token/tables` - 新增一个数据表
74. `POST:/open-apis/bitable/v1/apps/:app_token/tables/batch_create` - 新增多个数据表
75. `PATCH:/open-apis/bitable/v1/apps/:app_token/tables/:table_id` - 更新数据表
76. `GET:/open-apis/bitable/v1/apps/:app_token/tables` - 列出数据表
77. `DELETE:/open-apis/bitable/v1/apps/:app_token/tables/:table_id` - 删除一个数据表
78. `POST:/open-apis/bitable/v1/apps/:app_token/tables/batch_delete` - 删除多个数据表

#### app.role.member - 协作者管理 (5 APIs)
79. `POST:/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members` - 新增协作者
80. `POST:/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_create` - 批量新增协作者
81. `GET:/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members` - 列出协作者
82. `DELETE:/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/:member_id` - 删除协作者
83. `POST:/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_delete` - 批量删除协作者

#### app.table.view - 视图管理 (5 APIs)
84. `POST:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views` - 新增视图
85. `PATCH:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id` - 更新视图
86. `GET:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views` - 列出视图
87. `GET:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id` - 获取视图
88. `DELETE:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id` - 删除视图

#### app.table.field - 字段管理 (4 APIs)
89. `POST:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields` - 新增字段
90. `PUT:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields/:field_id` - 更新字段
91. `GET:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields` - 列出字段
92. `DELETE:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields/:field_id` - 删除字段

#### app - 应用管理 (4 APIs)
93. `POST:/open-apis/bitable/v1/apps` - 创建多维表格
94. `POST:/open-apis/bitable/v1/apps/:app_token/copy` - 复制多维表格
95. `GET:/open-apis/bitable/v1/apps/:app_token` - 获取多维表格元数据
96. `PUT:/open-apis/bitable/v1/apps/:app_token` - 更新多维表格元数据

#### app.table.form - 表单管理 (2 APIs)
97. `PATCH:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id` - 更新表单元数据
98. `GET:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id` - 获取表单元数据

#### app.table.form.field - 表单问题管理 (2 APIs)
99. `PATCH:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/fields/:field_id` - 更新表单问题
100. `GET:/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/fields` - 列出表单问题

#### app.role - 角色管理 (4 APIs)
101. `POST:/open-apis/bitable/v1/apps/:app_token/roles` - 新增自定义角色
102. `GET:/open-apis/bitable/v1/apps/:app_token/roles` - 列出自定义角色
103. `PUT:/open-apis/bitable/v1/apps/:app_token/roles/:role_id` - 更新自定义角色
104. `DELETE:/open-apis/bitable/v1/apps/:app_token/roles/:role_id` - 删除自定义角色

#### app.dashboard - 仪表盘管理 (2 APIs)
105. `POST:/open-apis/bitable/v1/apps/:app_token/dashboards/:block_id/copy` - 复制仪表盘
106. `GET:/open-apis/bitable/v1/apps/:app_token/dashboards` - 列出仪表盘

#### app.workflow - 自动化流程 (2 APIs)
107. `GET:/open-apis/bitable/v1/apps/:app_token/workflows` - 列出自动化流程
108. `PUT:/open-apis/bitable/v1/apps/:app_token/workflows/:workflow_id` - 更新自动化流程状态

---

## 📁 sheets - 电子表格(新版) (27 APIs)

### sheets/v3 - 现代电子表格API

#### spreadsheet.sheet.filter_view - 筛选视图 (5 APIs)
109. `POST:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views` - 添加筛选视图
110. `GET:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views` - 获取筛选视图列表
111. `PUT:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id` - 更新筛选视图
112. `DELETE:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id` - 删除筛选视图

#### spreadsheet.sheet.filter_view.condition - 筛选条件 (5 APIs)
113. `POST:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions` - 添加筛选条件
114. `GET:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions` - 获取筛选条件列表
115. `PUT:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id` - 更新筛选条件
116. `DELETE:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id` - 删除筛选条件

#### spreadsheet.sheet.float_image - 浮动图片 (5 APIs)
117. `POST:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images` - 添加浮动图片
118. `GET:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images` - 获取浮动图片列表
119. `PUT:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id` - 更新浮动图片
120. `DELETE:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id` - 删除浮动图片

#### spreadsheet.sheet - 工作表操作 (5 APIs)
121. `GET:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/query` - 获取工作表
122. `GET:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id` - 查询工作表
123. `POST:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/values` - 写入工作表值
124. `GET:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/values` - 读取工作表值
125. `POST:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/values/batch_update` - 批量更新工作表值

#### spreadsheet - 表格基础操作 (3 APIs)
126. `POST:/open-apis/sheets/v3/spreadsheets` - 创建电子表格
127. `PATCH:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token` - 修改电子表格属性
128. `GET:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token` - 获取电子表格信息

#### spreadsheet.sheet.filter - 数据筛选 (4 APIs)
129. `POST:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` - 添加筛选
130. `GET:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` - 获取筛选信息
131. `PUT:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` - 更新筛选
132. `DELETE:/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` - 删除筛选

---

## 📁 wiki - 知识库管理 (16 APIs)

### wiki/v1 - 搜索功能 (1 API)
133. `POST:/open-apis/wiki/v1/nodes/search` - 搜索 Wiki

### wiki/v2 - 完整知识库功能 (15 APIs)

#### space.node - 空间节点管理 (6 APIs)
134. `POST:/open-apis/wiki/v2/spaces/:space_id/nodes` - 创建知识空间节点
135. `POST:/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/move` - 移动知识空间节点
136. `POST:/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/update_title` - 更新知识空间节点标题
137. `POST:/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/copy` - 创建知识空间节点副本
138. `GET:/open-apis/wiki/v2/spaces/get_node` - 获取知识空间节点信息
139. `GET:/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/children` - 获取知识空间子节点列表

#### space - 知识空间管理 (4 APIs)
140. `POST:/open-apis/wiki/v2/spaces` - 创建知识空间
141. `GET:/open-apis/wiki/v2/spaces` - 获取知识空间列表
142. `GET:/open-apis/wiki/v2/spaces/:space_id` - 获取知识空间信息
143. `POST:/open-apis/wiki/v2/spaces/:space_id/nodes/move_docs_to_wiki` - 移动云空间文档至知识空间

#### space.member - 成员管理 (3 APIs)
144. `GET:/open-apis/wiki/v2/spaces/:space_id/members` - 获取知识空间成员列表
145. `POST:/open-apis/wiki/v2/spaces/:space_id/members` - 添加知识空间成员
146. `DELETE:/open-apis/wiki/v2/spaces/:space_id/members/:member_id` - 删除知识空间成员

#### space.setting - 空间设置 (1 API)
147. `PUT:/open-apis/wiki/v2/spaces/:space_id/setting` - 更新知识空间设置

#### task - 任务管理 (1 API)
148. `GET:/open-apis/wiki/v2/tasks/:task_id` - 获取任务结果

---

## 📁 docx - 文档处理 (19 APIs)

### docx/v1 - 文档操作

#### document.block - 文档块操作 (4 APIs)
149. `GET:/open-apis/docx/v1/documents/:document_id/blocks` - 获取文档所有块
150. `PATCH:/open-apis/docx/v1/documents/:document_id/blocks/:block_id` - 更新块的内容
151. `GET:/open-apis/docx/v1/documents/:document_id/blocks/:block_id` - 获取块的内容
152. `PATCH:/open-apis/docx/v1/documents/:document_id/blocks/batch_update` - 批量更新块的内容

#### document - 文档基础操作 (4 APIs)
153. `POST:/open-apis/docx/v1/documents` - 创建文档
154. `GET:/open-apis/docx/v1/documents/:document_id` - 获取文档基本信息
155. `GET:/open-apis/docx/v1/documents/:document_id/raw_content` - 获取文档纯文本内容
156. `POST:/open-apis/docx/v1/documents/:document_id/batch_update` - 批量更新文档

#### document.block.children - 块子元素 (3 APIs)
157. `POST:/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children` - 创建块
158. `GET:/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children` - 获取所有子块
159. `DELETE:/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children/batch_delete` - 删除块

#### chat.announcement.block - 群公告块 (3 APIs)
160. `GET:/open-apis/docx/v1/chats/:chat_id/announcement/blocks` - 获取群公告所有块
161. `PATCH:/open-apis/docx/v1/chats/:chat_id/announcement/blocks/batch_update` - 批量更新群公告块的内容
162. `GET:/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id` - 获取群公告块的内容

#### chat.announcement - 群公告 (1 API)
163. `GET:/open-apis/docx/v1/chats/:chat_id/announcement` - 获取群公告基本信息

#### chat.announcement.block.children - 群公告块子元素 (3 APIs)
164. `GET:/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children` - 获取所有子块
165. `DELETE:/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children/batch_delete` - 删除群公告中的块

#### document.block.descendant - 块后代 (1 API)
166. `GET:/open-apis/docx/v1/documents/:document_id/blocks/:block_id/descendants` - 获取文档块的后代

---

## 📁 minutes - 会议纪要 (4 APIs)

### minutes/v1 - 妙记功能

167. `GET:/open-apis/minutes/v1/minutes/:minute_token` - 获取妙记信息
168. `GET:/open-apis/minutes/v1/minutes/:minute_token/media` - 下载妙记音视频文件
169. `GET:/open-apis/minutes/v1/minutes/:minute_token/transcript` - 导出妙记文字记录
170. `GET:/open-apis/minutes/v1/minutes/:minute_token/statistics` - 获取妙记统计数据

---

## 📁 baike - 知识库(旧版) (13 APIs)

### baike/v1 - 词条管理

#### entity - 词条操作 (8 APIs)
171. `POST:/open-apis/baike/v1/entities` - 创建免审词条
172. `PUT:/open-apis/baike/v1/entities/:entity_id` - 更新免审词条
173. `GET:/open-apis/baike/v1/entities/:entity_id` - 获取词条详情
174. `GET:/open-apis/baike/v1/entities` - 获取词条列表
175. `POST:/open-apis/baike/v1/entities/match` - 精准搜索词条
176. `POST:/open-apis/baike/v1/entities/search` - 模糊搜索词条
177. `POST:/open-apis/baike/v1/entities/highlight` - 词条高亮
178. `POST:/open-apis/baike/v1/entities/extract` - 提取潜在的词条

#### draft - 草稿管理 (2 APIs)
179. `POST:/open-apis/baike/v1/drafts` - 创建草稿
180. `PUT:/open-apis/baike/v1/drafts/:draft_id` - 更新草稿

#### classification - 分类管理 (1 API)
181. `GET:/open-apis/baike/v1/classifications` - 获取词典分类

#### file - 文件管理 (2 APIs)
182. `POST:/open-apis/baike/v1/files/upload` - 上传图片
183. `GET:/open-apis/baike/v1/files/:file_token/download` - 下载图片

---

## 📁 lingo - 语言服务(新版知识库) (14 APIs)

### lingo/v1 - 增强知识库

#### entity - 词条管理 (8 APIs)
184. `POST:/open-apis/lingo/v1/entities` - 创建免审词条
185. `PUT:/open-apis/lingo/v1/entities/:entity_id` - 更新免审词条
186. `DELETE:/open-apis/lingo/v1/entities/:entity_id` - 删除免审词条
187. `GET:/open-apis/lingo/v1/entities/:entity_id` - 获取词条详情
188. `GET:/open-apis/lingo/v1/entities` - 获取词条列表
189. `POST:/open-apis/lingo/v1/entities/match` - 精准搜索词条
190. `POST:/open-apis/lingo/v1/entities/search` - 模糊搜索词条
191. `POST:/open-apis/lingo/v1/entities/highlight` - 词条高亮

#### draft - 草稿管理 (2 APIs)
192. `POST:/open-apis/lingo/v1/drafts` - 创建草稿
193. `PUT:/open-apis/lingo/v1/drafts/:draft_id` - 更新草稿

#### classification - 分类管理 (1 API)
194. `GET:/open-apis/lingo/v1/classifications` - 获取词典分类

#### file - 文件管理 (2 APIs)
195. `POST:/open-apis/lingo/v1/files/upload` - 上传图片
196. `GET:/open-apis/lingo/v1/files/:file_token/download` - 下载图片

#### repo - 词典管理 (1 API)
197. `GET:/open-apis/lingo/v1/repos` - 获取词库列表

---

## 📁 base - 多维表格增强权限 (3 APIs)

### base/v2 - 高级权限管理

#### app.role - 自定义角色管理 (3 APIs)
198. `POST:/open-apis/base/v2/apps/:app_token/roles` - 新增自定义角色
199. `PUT:/open-apis/base/v2/apps/:app_token/roles/:role_id` - 更新自定义角色
200. `GET:/open-apis/base/v2/apps/:app_token/roles` - 列出自定义角色

---

## 📁 docs - 内容服务 (1 API)

### docs/v1 - 内容管理 (1 API)

#### content - 内容操作 (1 API)
201. `POST:/open-apis/suite/docs-api/search/object` - 搜索云文档

---

## 📁 旧版API (需要迁移) (52 APIs)

### ccm_sheet/old - 旧版电子表格 (33 APIs)
*(建议迁移到 sheets/v3)*

### ccm_drive_explorer/old - 旧版云空间 (8 APIs)
*(建议迁移到 drive/v1)*

### ccm_doc/old - 旧版文档 (6 APIs)
*(建议迁移到 docx/v1)*

### ccm_docs/old - 旧版文档搜索 (2 APIs)
*(建议迁移到 docs/v1)*

### ccm_drive_permission/old - 旧版权限 (3 APIs)
*(建议迁移到 drive/v1)*

---

## 实现状态跟踪

| 模块 | API总数 | 已实现 | 进行中 | 待实现 | 完成率 |
|------|---------|--------|--------|--------|--------|
| drive | 59 | 0 | 0 | 59 | 0% |
| bitable | 46 | 0 | 0 | 46 | 0% |
| sheets | 27 | 0 | 0 | 27 | 0% |
| wiki | 16 | 0 | 0 | 16 | 0% |
| docx | 19 | 0 | 0 | 19 | 0% |
| minutes | 4 | 0 | 0 | 4 | 0% |
| baike | 13 | 0 | 0 | 13 | 0% |
| lingo | 14 | 0 | 0 | 14 | 0% |
| base | 3 | 0 | 0 | 3 | 0% |
| docs | 1 | 0 | 0 | 1 | 0% |
| 旧版 | 52 | 0 | 0 | 52 | 0% |
| **总计** | **254** | **0** | **0** | **254** | **0%** |

---

## 下一步行动

1. **优先实现核心模块**: drive, bitable, docx
2. **建立基础架构**: 错误处理、认证、通用类型
3. **实现第一组API**: 文件基础操作、多维表格记录管理
4. **逐步扩展**: 按业务需求优先级实现其他模块
5. **版本管理**: 明确支持新版API，标记旧版API为deprecated