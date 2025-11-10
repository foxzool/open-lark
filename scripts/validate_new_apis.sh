#!/bin/bash

# API验证脚本
# 用于验证新实现的API是否正确集成

set -e

echo "🚀 开始API验证流程..."

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 检查函数
check_compilation() {
    log_info "检查代码编译..."
    if cargo check --quiet; then
        log_success "代码编译成功"
        return 0
    else
        log_error "代码编译失败"
        cargo check
        return 1
    fi
}

check_clippy() {
    log_info "运行代码质量检查..."
    if cargo clippy --quiet -- -Dwarnings; then
        log_success "代码质量检查通过"
        return 0
    else
        log_warning "代码质量检查发现问题"
        cargo clippy
        return 1
    fi
}

check_tests() {
    log_info "运行测试..."
    if cargo test --quiet --lib; then
        log_success "所有测试通过"
        return 0
    else
        log_error "测试失败"
        cargo test --lib
        return 1
    fi
}

check_rust_api_mapper() {
    log_info "运行rust-api-mapper验证..."

    local output_file="api_validation_result.md"
    local json_file="api_validation_result.json"

    cd tools/rust-api-mapper
    if cargo run -- \
        --service-dir ../../src/service \
        --api-list ../../server_api_list.csv \
        --markdown-output "../../$output_file" \
        --json-output "../../$json_file" 2>/dev/null; then

        log_success "rust-api-mapper验证完成"

        # 统计新增的已实现API
        local implemented_count=$(grep -c "✅" "../../$output_file" || echo "0")
        local total_count=$(grep -c "|" "../../$output_file" | head -1 || echo "0")

        if [ "$implemented_count" -gt 0 ]; then
            log_success "发现 $implemented_count 个已实现的API"
            log_info "总API数: $total_count"
            log_info "实现率: $(echo "scale=1; $implemented_count * 100 / $total_count" | bc 2>/dev/null || echo "N/A")%"
        else
            log_warning "没有发现新实现的API"
        fi

        return 0
    else
        log_error "rust-api-mapper验证失败"
        return 1
    fi
}

check_documentation() {
    log_info "检查文档生成..."
    if cargo doc --quiet --no-deps; then
        log_success "文档生成成功"
        return 0
    else
        log_error "文档生成失败"
        return 1
    fi
}

check_coverage() {
    log_info "检查测试覆盖率..."

    if command -v cargo-llvm-cov &> /dev/null; then
        if cargo llvm-cov --quiet --lib --html; then
            log_success "测试覆盖率报告已生成"
            log_info "报告位置: target/llvm-cov/html/index.html"
            return 0
        else
            log_warning "测试覆盖率检查失败"
            return 1
        fi
    else
        log_warning "未安装 cargo-llvm-cov，跳过覆盖率检查"
        log_info "安装命令: cargo install cargo-llvm-cov"
        return 0
    fi
}

# 生成验证报告
generate_report() {
    log_info "生成验证报告..."

    local report_file="api_validation_report.md"
    local timestamp=$(date "+%Y-%m-%d %H:%M:%S")

    cat > "$report_file" << EOF
# API验证报告

**生成时间**: $timestamp
**项目**: open-lark Sheets模块

## 验证结果

$(check_compilation && echo "✅ 编译检查通过" || echo "❌ 编译检查失败")
$(check_clippy && echo "✅ 代码质量检查通过" || echo "❌ 代码质量检查失败")
$(check_tests && echo "✅ 单元测试通过" || echo "❌ 单元测试失败")
$(check_rust_api_mapper && echo "✅ API映射验证通过" || echo "❌ API映射验证失败")
$(check_documentation && echo "✅ 文档生成成功" || echo "❌ 文档生成失败")
$(check_coverage && echo "✅ 测试覆盖率检查完成" || echo "⚠️ 测试覆盖率检查跳过")

## 下一步行动

1. 如果有失败的检查项，请优先修复
2. 运行 \`cargo run --example <example_name>\` 测试具体API
3. 检查生成的文档和覆盖率报告
4. 更新CHANGELOG和README

EOF

    log_success "验证报告已生成: $report_file"
}

# 主函数
main() {
    echo "🔍 验证新实现的API..."
    echo ""

    local failed_checks=0

    # 执行各项检查
    check_compilation || ((failed_checks++))
    check_clippy || ((failed_checks++))
    check_tests || ((failed_checks++))
    check_rust_api_mapper || ((failed_checks++))
    check_documentation || ((failed_checks++))
    check_coverage || ((failed_checks++))

    echo ""

    # 生成报告
    generate_report

    if [ $failed_checks -eq 0 ]; then
        log_success "🎉 所有验证检查通过！"
        echo ""
        echo "📋 建议下一步行动:"
        echo "1. 运行具体示例测试API功能"
        echo "2. 检查API映射报告中的新增实现"
        echo "3. 更新相关文档"
        exit 0
    else
        log_error "❌ 有 $failed_checks 项检查失败，请修复后重试"
        exit 1
    fi
}

# 显示帮助信息
show_help() {
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  -h, --help     显示帮助信息"
    echo "  --quick       快速检查（仅编译和基本测试）"
    echo "  --coverage    包含覆盖率检查"
    echo ""
    echo "此脚本用于验证新实现的API是否正确集成，包括："
    echo "- 代码编译检查"
    echo "- 代码质量检查 (clippy)"
    echo "- 单元测试"
    echo "- rust-api-mapper验证"
    echo "- 文档生成"
    echo "- 测试覆盖率"
}

# 解析命令行参数
case "${1:-}" in
    -h|--help)
        show_help
        exit 0
        ;;
    --quick)
        log_info "执行快速检查..."
        check_compilation
        check_tests
        check_rust_api_mapper
        log_success "快速检查完成"
        ;;
    --coverage)
        main
        ;;
    "")
        main
        ;;
    *)
        log_error "未知选项: $1"
        show_help
        exit 1
        ;;
esac