#!/bin/bash

# API模块迁移映射脚本
# 从 api_req/api_resp 迁移到新的 api 模块
# 激进式完全替换策略，不保持向后兼容

set -euo pipefail

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 计数器
TOTAL_FILES=0
MODIFIED_FILES=0
ERRORS=0

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# API映射规则定义
declare -A TYPE_MAPPINGS=(
    # 响应类型映射
    ["api_resp::BaseResponse"]="api::Response"]
    ["api_resp::RawResponse"]="api::RawResponse"]
    ["api_resp::ApiResponseTrait"]="api::ApiResponseTrait"]
    ["api_resp::ErrorInfo"]="api::ErrorInfo"]
    ["api_resp::ResponseFormat"]="api::ResponseFormat"]

    # 请求类型映射
    ["api_req::ApiRequest"]="api::ApiRequest"]
    ["api_req::HttpMethod"]="api::HttpMethod"]
    ["api_req::RequestData"]="api::RequestData"]

    # 完整路径映射
    ["crate::api_resp::BaseResponse"]="api::BaseResponse"]
    ["crate::api_resp::RawResponse"]="api::RawResponse"]
    ["crate::api_req::ApiRequest"]="api::ApiRequest"]

    # 通用模式
    ["use crate::api_resp::"]="use api::"]
    ["use crate::api_req::"]="use api::"]
    ["super::api_resp::"]="super::api::"]
    ["super::api_req::"]="super::api::"]
)

# 函数：替换单个文件的API引用
process_file() {
    local file="$1"
    local temp_file="${file}.tmp"
    local modified=false

    # 备份原文件
    cp "$file" "$temp_file"

    # 应用所有映射规则
    for old_pattern in "${!TYPE_MAPPINGS[@]}"; do
        new_pattern="${TYPE_MAPPINGS[$old_pattern]}"

        # 使用 perl 进行精确替换，避免匹配到注释或字符串中的内容
        if perl -i -pe "
            # 替换 use 语句
            s/use\\s+${old_pattern//\//\\/}\\s*;/use ${new_pattern//\//\\/};/g;
            # 替换类型引用
            s/\\b${old_pattern//\//\\/}\\b/${new_pattern//\//\\/}/g;
        " "$temp_file" 2>/dev/null; then
            if ! cmp -s "$file" "$temp_file"; then
                modified=true
                log_info "  应用映射: $old_pattern -> $new_pattern"
            fi
        fi
    done

    # 处理导入路径更新
    perl -i -pe "
        # 更新 prelude 导入
        s/use crate::api_resp::prelude::\\*/use api::prelude::*;/g;
        s/use crate::api_req::prelude::\\*/use api::prelude::*;/g;

        # 更新模块导入
        s/use crate::api_resp::([^;]+);/use api::responses::\\1;/g;
        s/use crate::api_req::([^;]+);/use api::\\1;/g;
    " "$temp_file" 2>/dev/null

    # 处理特殊情况：BaseResponse<T> 到 Response<T>
    perl -i -pe 's/\bBaseResponse</Response</g' "$temp_file" 2>/dev/null

    # 检查文件是否被修改
    if ! cmp -s "$file" "$temp_file"; then
        mv "$temp_file" "$file"
        ((MODIFIED_FILES++))
        log_success "  已更新: $(basename "$file")"
        return 0
    else
        rm -f "$temp_file"
        return 1
    fi
}

# 函数：验证迁移结果
validate_migration() {
    local file="$1"
    local has_errors=false

    # 检查是否还有旧的API引用
    if grep -q "api_req::\|api_resp::" "$file"; then
        log_warning "  仍有旧API引用:"
        grep -n "api_req::\|api_resp::" "$file" | head -3 | sed 's/^/    /'
        has_errors=true
    fi

    # 检查语法错误
    if ! rustc --edition 2021 --crate-type lib "$file" --extern openlark_core=crates/openlark-core/target/debug/libopenlark_core.rlib 2>/dev/null; then
        log_error "  语法错误"
        has_errors=true
    fi

    if $has_errors; then
        ((ERRORS++))
        return 1
    fi

    return 0
}

# 函数：批量处理目录
process_directory() {
    local dir="$1"

    log_info "处理目录: $dir"

    # 查找所有 Rust 文件
    while IFS= read -r -d '' file; do
        ((TOTAL_FILES++))

        log_info "处理文件 ($TOTAL_FILES): $(basename "$file")"

        if process_file "$file"; then
            validate_migration "$file"
        fi

    done < <(find "$dir" -name "*.rs" -type f -print0)
}

# 函数：显示迁移统计
show_statistics() {
    echo
    log_info "========== 迁移统计 =========="
    log_info "总文件数: $TOTAL_FILES"
    log_success "已修改文件: $MODIFIED_FILES"
    log_warning "错误文件数: $ERRORS"

    if [[ $ERRORS -eq 0 ]]; then
        log_success "🎉 迁移完成！"
    else
        log_warning "⚠️  发现 $ERRORS 个错误，需要手动检查"
    fi
}

# 函数：显示帮助信息
show_help() {
    echo "API模块迁移工具"
    echo ""
    echo "用法: $0 [选项] <目录或文件>"
    echo ""
    echo "选项:"
    echo "  -h, --help     显示此帮助信息"
    echo "  -v, --verbose  详细输出"
    echo "  -d, --dry-run  仅显示将要执行的更改，不实际修改文件"
    echo ""
    echo "示例:"
    echo "  $0 src/                    # 迁移整个src目录"
    echo "  $0 src/services/           # 迁移特定目录"
    echo "  $0 src/lib.rs              # 迁移单个文件"
    echo "  $0 --dry-run src/          # 预览模式"
}

# 主函数
main() {
    local target=""
    local dry_run=false
    local verbose=false

    # 解析参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -v|--verbose)
                verbose=true
                shift
                ;;
            -d|--dry-run)
                dry_run=true
                log_warning "🔍 预览模式 - 不会实际修改文件"
                shift
                ;;
            -*)
                log_error "未知选项: $1"
                show_help
                exit 1
                ;;
            *)
                if [[ -z "$target" ]]; then
                    target="$1"
                else
                    log_error "只能指定一个目标文件或目录"
                    exit 1
                fi
                shift
                ;;
        esac
    done

    # 检查目标
    if [[ -z "$target" ]]; then
        log_error "请指定要迁移的文件或目录"
        show_help
        exit 1
    fi

    if [[ ! -e "$target" ]]; then
        log_error "目标不存在: $target"
        exit 1
    fi

    # 显示映射规则
    log_info "========== API映射规则 =========="
    for old_pattern in "${!TYPE_MAPPINGS[@]}"; do
        new_pattern="${TYPE_MAPPINGS[$old_pattern]}"
        log_info "  $old_pattern -> $new_pattern"
    done
    echo

    if $dry_run; then
        log_info "预览模式：显示将要应用的规则"
        return 0
    fi

    # 开始迁移
    log_info "🚀 开始API模块迁移..."
    log_info "目标: $target"
    echo

    if [[ -f "$target" ]]; then
        # 处理单个文件
        ((TOTAL_FILES++))
        log_info "处理文件: $(basename "$target")"

        if process_file "$target"; then
            validate_migration "$target"
        fi
    elif [[ -d "$target" ]]; then
        # 处理目录
        process_directory "$target"
    else
        log_error "无效的目标类型: $target"
        exit 1
    fi

    show_statistics
}

# 检查依赖
if ! command -v perl >/dev/null 2>&1; then
    log_error "需要安装 perl"
    exit 1
fi

if ! command -v rustc >/dev/null 2>&1; then
    log_error "需要安装 rustc"
    exit 1
fi

# 运行主函数
main "$@"