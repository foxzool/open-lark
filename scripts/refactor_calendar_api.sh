#!/bin/bash
#
# Calendar API 自动化重构脚本
#
# 用途：将使用 serde_json::Value 的 Calendar API 自动转换为强类型
# 使用方法：./scripts/refactor_calendar_api.sh <module_path> <api_name>
#
# 示例：
#   ./scripts/refactor_calendar_api.sh calendar/v4/event list
#   ./scripts/refactor_calendar_api.sh calendar/v4/event get
#

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# 检查参数
if [ $# -lt 2 ]; then
    print_error "用法: $0 <module_path> <api_name>"
    print_info "示例: $0 calendar/v4/event list"
    exit 1
fi

MODULE_PATH=$1
API_NAME=$2
FILE_PATH="crates/openlark-meeting/src/${MODULE_PATH}/${API_NAME}.rs"

# 检查文件是否存在
if [ ! -f "$FILE_PATH" ]; then
    print_error "文件不存在: $FILE_PATH"
    exit 1
fi

print_info "开始重构: $FILE_PATH"

# 备份原文件
BACKUP_PATH="${FILE_PATH}.backup_$(date +%Y%m%d_%H%M%S)"
cp "$FILE_PATH" "$BACKUP_PATH"
print_success "备份已创建: $BACKUP_PATH"

# 分析 API 类型
HTTP_METHOD=$(grep -m1 "ApiRequest::" "$FILE_PATH" | sed 's/.*ApiRequest::\([a-z]*\).*/\1/')

print_info "检测到 HTTP 方法: $HTTP_METHOD"

# 生成强类型响应结构模板
if [ "$API_NAME" = "get" ] || [ "$API_NAME" = "list" ]; then
    # get 和 list 类型的响应模板
    RESPONSE_TYPE=$(echo "${API_NAME^}Response")

    # 检查是否已经使用强类型
    if grep -q "SDKResult<$RESPONSE_TYPE>" "$FILE_PATH"; then
        print_warning "文件已使用强类型，跳过"
        exit 0
    fi

    print_info "正在生成响应结构..."

    # 提取原始 API 信息
    DOC_PATH=$(grep -m1 "docPath:" "$FILE_PATH" | sed 's/.*docPath: //')

    # 构建新的响应结构
    cat > /tmp/response_template.rs << 'EOF'
/// 响应数据结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseData {
    // TODO: 根据在线文档填充字段
}

impl ApiResponseTrait for ResponseType {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
EOF

    print_success "响应结构模板已生成到 /tmp/response_template.rs"
    print_info "请根据在线文档填充 ResponseData 结构体字段"
    print_info "文档链接: $DOC_PATH"

    # 提供下一步建议
    print_warning "需要手动步骤:"
    echo "1. 查看在线文档: $DOC_PATH"
    echo "2. 修改 /tmp/response_template.rs，填充 ResponseData 字段"
    echo "3. 将响应结构插入到 $FILE_PATH"
    echo "4. 修改 execute() 方法的返回类型"
    echo "5. 修改响应数据提取逻辑"

elif [ "$API_NAME" = "create" ] || [ "$API_NAME" = "update" ] || [ "$API_NAME" = "patch" ]; then
    # create、update、patch 类型的响应模板
    print_info "检测到写入操作 API"

    # 检查是否已经使用强类型
    if grep -q "SDKResult<.*Response>" "$FILE_PATH"; then
        print_warning "文件可能已使用强类型，跳过"
        exit 0
    fi

    print_info "写入操作通常返回创建/更新的资源"

else
    print_warning "API 类型 '$API_NAME' 未在自动化脚本中定义"
    print_info "请参考 get.rs 或 list.rs 手动重构"
fi

# 检查 API endpoint 枚举使用情况
if grep -q "CalendarApiV4::" "$FILE_PATH"; then
    print_success "文件已使用 CalendarApiV4 枚举（推荐）"
elif grep -q "CALENDAR_V4_" "$FILE_PATH"; then
    print_warning "文件使用字符串常量，建议迁移到 CalendarApiV4"
fi

# 检查 ResponseFormat 使用情况
if grep -q "ResponseFormat::Custom" "$FILE_PATH"; then
    print_error "发现无效的 ResponseFormat::Custom"
    print_info "请修复为 ResponseFormat::Data"
fi

# 检查 unwrap_or_default 使用情况
if grep -q "unwrap_or_default()" "$FILE_PATH"; then
    print_warning "发现 unwrap_or_default()，建议改为 ok_or_else()"
    print_info "参考: .ok_or_else(|| validation_error(\"响应数据为空\", \"服务器没有返回有效的数据\"))"
fi

print_success "分析完成"
echo ""
print_info "下一步操作:"
echo "1. 检查生成的模板: /tmp/response_template.rs"
echo "2. 查看在线文档获取响应结构"
echo "3. 手动完成重构"
echo "4. 运行 cargo check 验证"
echo "5. 提交前运行 just test && just lint"

# 恢复说明
echo ""
print_info "如需回退，使用: cp $BACKUP_PATH $FILE_PATH"
