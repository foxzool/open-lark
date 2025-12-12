# CCM Sheet API 使用指南

## 📖 概述

CCM Sheet API 提供了完整的电子表格操作功能，包括数据读写、样式设置、权限管理、批量处理等 33 个强大的 API 接口。本指南将帮助您快速掌握这些 API 的使用方法。

## 🚀 快速开始

### 1. 基础配置

```rust
use openlark_client::{LarkClient};
use openlark_core::config::Config;

// 创建配置
let config = Config::builder()
    .app_id("your_app_id")
    .app_secret("your_app_secret")
    .build()?;

// 创建客户端
let client = LarkClient::new(config)?;
let sheet_service = client.docs.ccm_sheet.old.v2();
```

### 2. 必要参数获取

在使用 API 之前，您需要准备以下参数：

- **App ID & App Secret**: 从飞书开放平台获取
- **Spreadsheet Token**: 要操作的电子表格 token
- **Sheet ID**: 工作表 ID（通常在元数据中获取）

## 📋 API 分类和使用

### 🔧 基础操作

| API | 功能 | 使用场景 |
|-----|------|----------|
| `getspreadsheetmeta` | 获取表格元数据 | 了解表格结构、工作表信息 |
| `operatesheets` | 操作工作表 | 创建、删除、复制工作表 |
| `updatesheetproperties` | 更新工作表属性 | 修改工作表名称、网格属性 |
| `updatespreadsheetproperties` | 更新表格属性 | 修改表格标题、时区等 |

### 💾 数据操作

| API | 功能 | 推荐用法 |
|-----|------|----------|
| `readsinglerange` | 读取单个范围 | 少量数据读取 |
| `writesinglerange` | 写入单个范围 | 简单数据写入 |
| `readmultipleranges` | 读取多个范围 | 批量读取不同区域 |
| `batchwriteranges` | 批量写入范围 | **推荐**: 高性能批量写入 |
| `appendvalues` | 追加数据 | 动态添加新数据行 |
| `insertvalues` | 插入数据 | 在指定位置插入数据 |

### 🎨 样式和格式

| API | 功能 | 应用场景 |
|-----|------|----------|
| `setstyle` | 设置单元格样式 | 单个区域样式设置 |
| `batchsetstyle` | 批量设置样式 | **推荐**: 高性能样式批量设置 |
| `mergecells` | 合并单元格 | 标题合并、报告格式化 |
| `unmergecells` | 拆分单元格 | 取消合并操作 |

### 🔢 行列操作

| API | 功能 | 使用时机 |
|-----|------|----------|
| `adddimensionrange` | 增加行列 | 预留扩展空间 |
| `insertdimensionrange` | 插入行列 | 在指定位置插入 |
| `updatedimensionrange` | 更新行列 | 调整行列属性 |
| `deletedimensionrange` | 删除行列 | 清理无用行列 |

### 🔐 权限和安全

| API | 功能 | 安全级别 |
|-----|------|----------|
| `addprotectedrange` | 添加保护范围 | 🔒 高安全 |
| `updateprotectedrange` | 更新保护范围 | 🔒 高安全 |
| `getprotectedrange` | 获取保护范围 | 📋 查看 |
| `deleteprotectedrange` | 删除保护范围 | 🔓 解除保护 |

### 🎯 数据验证

| API | 功能 | 应用场景 |
|-----|------|----------|
| `setdropdown` | 设置下拉菜单 | 数据规范输入 |
| `updatedropdown` | 更新验证规则 | 规则调整 |
| `getdropdown` | 获取验证规则 | 规则查看 |
| `deletedropdown` | 删除验证规则 | 移除限制 |

### 🎨 条件格式

| API | 功能 | 效果展示 |
|-----|------|----------|
| `createconditionformat` | 创建条件格式 | 数据可视化 |
| `updateconditionformat` | 更新条件格式 | 规则优化 |
| `getconditionformat` | 获取条件格式 | 规则查看 |
| `deleteconditionformat` | 删除条件格式 | 清理格式 |

### 🖼️ 媒体操作

| API | 功能 | 应用场景 |
|-----|------|----------|
| `writeimage` | 写入图片 | Logo插入、图片报告 |

### 📊 导入导出

| API | 功能 | 用途 |
|-----|------|------|
| `importspreadsheet` | 导入表格 | 数据迁移 |
| `getimportresult` | 查询导入结果 | 导入状态检查 |

## 💡 最佳实践

### 1. 性能优化

**✅ 推荐：使用批量操作**

```rust
// 高性能批量写入
let batch_request = sheet_service.batchwriteranges();
let params = serde_json::json!({
    "spreadsheetToken": token,
    "data": [
        {"range": "Sheet1!A1:C10", "values": [...]},
        {"range": "Sheet2!A1:C10", "values": [...]}
    ]
});
let response = batch_request.execute(params).await?;
```

**❌ 避免：多次单独调用**

```rust
// 性能较差的方式
for i in 1..10 {
    let request = sheet_service.writesinglerange();
    // 每次调用都是一次网络请求
}
```

### 2. 错误处理

```rust
match request.execute(params).await {
    Ok(response) => {
        // 处理成功响应
        println!("操作成功: {:?}", response.data);
    }
    Err(error) => {
        // 智能错误处理
        if error.to_string().contains("token") {
            println!("检查token是否正确");
        } else if error.to_string().contains("permission") {
            println!("检查访问权限");
        }

        // 记录详细错误用于调试
        eprintln!("详细错误: {:?}", error);
    }
}
```

### 3. 数据格式化

**数值格式：**

```rust
// 千分位数字
"numberFormat": {"type": "NUMBER", "pattern": "#,##0"}

// 货币格式
"numberFormat": {"type": "NUMBER", "pattern": "¥#,##0.00"}

// 百分比
"numberFormat": {"type": "NUMBER", "pattern": "0.00%"}
```

**文本格式：**

```rust
// 粗体居中标题
"backgroundColor": {"red": 0.2, "green": 0.4, "blue": 0.8},
"textFormat": {
    "bold": true,
    "fontSize": 14,
    "foregroundColor": {"red": 1.0, "green": 1.0, "blue": 1.0}
},
"horizontalAlignment": "CENTER"
```

### 4. 批量操作优化技巧

**合并连续范围：**

```rust
// 不推荐：多个小范围
[
    {"range": "A1:A1", "values": [["标题1"]]},
    {"range": "B1:B1", "values": [["标题2"]]},
    {"range": "C1:C1", "values": [["标题3"]]}
]

// 推荐：合并为一个大范围
[
    {"range": "A1:C1", "values": [["标题1", "标题2", "标题3"]]}
]
```

## 📚 示例代码

### 快速开始示例

查看 `examples/ccm_sheet_quick_start.rs` 了解最基本的用法：

```bash
cargo run --example ccm_sheet_quick_start --features "docs"
```

### 综合演示示例

查看 `examples/ccm_sheet_comprehensive_demo.rs` 了解所有功能：

```bash
cargo run --example ccm_sheet_comprehensive_demo --features "docs"
```

### 批量操作示例

查看 `examples/ccm_sheet_batch_operations.rs` 学习性能优化：

```bash
cargo run --example ccm_sheet_batch_operations --features "docs"
```

## 🔧 开发调试

### 启用详细日志

```rust
// 在初始化时设置日志级别
env_logger::init();
```

### 检查 API 响应

```rust
println!("完整响应: {:?}", response);
println!("数据部分: {:?}", response.data);
```

### 调试技巧

1. **从小数据量开始**：先测试小范围数据
2. **验证范围格式**：确保 `"Sheet1!A1:C10"` 格式正确
3. **检查数据类型**：确保数据符合 API 期望
4. **使用批量 API**：减少网络请求次数

## 🚨 注意事项

### 安全要求

- 妥善保管 App ID 和 App Secret
- 使用环境变量存储敏感信息
- 定期轮换访问凭证

### API 限制

- 单次批量写入不超过 10,000 个单元格
- 大文件导入可能需要较长时间
- 频繁请求可能触发限流

### 最佳实践

1. **使用批量 API**：提升性能
2. **合理设置范围**：避免过度请求
3. **错误处理**：妥善处理异常情况
4. **资源管理**：及时清理不需要的格式

## 📞 支持和反馈

- 📖 [完整 API 文档](https://open.feishu.cn/document/server-docs/docs/sheets-v3/)
- 🐛 [问题反馈](https://github.com/your-repo/issues)
- 💬 [社区讨论](https://github.com/your-repo/discussions)

---

## 🎉 开始使用

现在您已经了解了 CCM Sheet API 的全部功能，开始构建您的表格应用吧！

```bash
# 运行快速开始示例
cargo run --example ccm_sheet_quick_start --features "docs"
```