[![crates.io](https://img.shields.io/crates/v/open-lark)](https://crates.io/crates/open-lark)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![crates.io](https://img.shields.io/crates/d/open-lark)](https://crates.io/crates/open-lark)
[![CI](https://github.com/foxzool/open-lark/workflows/CI/badge.svg)](https://github.com/foxzool/open-lark/actions)
[![Documentation](https://docs.rs/open-lark/badge.svg)](https://docs.rs/open-lark)

# 飞书开放平台相关API调用

## 已完成

### 认证与授权

- [x] 自建应用获取 tenant_access_token

### 飞书卡片

#### 卡片组件

- 容器
    - [x] 分栏
    - [x] 表单容器
    - [x] 交互容器
    - [x] 折叠面板
- 展示
    - [x] 标题
    - [x] 普通文本
    - [x] 富文本(Markdown)
    - [x] 图片
    - [x] 多图混排
    - [x] 分割线
    - [x] 人员
    - [x] 人员列表
    - [x] 图表
    - [x] 表格
    - [x] 备注
- 交互
    - [x] 输入框
    - [x] 按钮
    - [x] 折叠按钮组
    - [x] 下拉选择-单选
    - [x] 下拉选择-多选
    - [x] 人员选择-单选
    - [x] 人员选择-多选
    - [x] 日期选择器
    - [x] 时间选择器
    - [x] 日期时间选择器
    - [x] 多图选择
    - [x] 勾选器

- [x] 卡片回传交互

### 消息

- [x] 发送消息
- [x] 获取会话历史消息
- 消息内容结构
    - 发送消息内容
        - [x] 文本
        - [x] 富文本
        - [x] 图片
        - [x] 卡片

### 群组

- [x] 获取用户或机器人所在的群列表