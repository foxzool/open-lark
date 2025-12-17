# API Analysis Tools

这个目录包含了用于分析和管理open-lark项目API实现的工具脚本。

## 工具说明

### 1. `check_ccm_implementation.py`
检查CCM（云文档管理）相关的API实现情况。

### 2. `check_exact_ccm_implementation.py`
精确检查CCM API的实现状态，提供详细的覆盖率统计。

### 3. `check_missing_apis.py`
扫描并识别项目中缺失的API实现。

### 4. `filter_ccm_apis.py`
从API列表中过滤出CCM相关的API。

### 5. `api_check_tool.py`
通用的API检查工具，可以检查不同服务模块的API实现情况。

## 使用方法

这些脚本通常需要：
1. API导出CSV文件（`api_list_export.csv`）
2. 正确的Python环境（Python 3.6+）

```bash
# 示例：检查CCM API实现
python check_ccm_implementation.py

# 示例：检查缺失的API
python check_missing_apis.py
```

## 注意事项

- 这些工具主要用于开发和维护阶段
- 运行前请确保有最新的API数据文件
- 某些脚本可能需要根据项目结构更新路径配置