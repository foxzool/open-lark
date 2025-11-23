#!/bin/bash

# 质量检查脚本
# 运行完整的代码质量检查，包括格式、lint、测试和覆盖率

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# 检查结果
FAILED=0

echo -e "${BLUE}==================== Open-Lark 质量检查 ====================${NC}"

# 1. 检查代码格式
log_info "检查代码格式..."
if cargo fmt --all -- --check; then
    log_success "代码格式检查通过"
else
    log_error "代码格式检查失败，请运行 'cargo fmt' 修复"
    FAILED=1
fi

# 2. 运行 Clippy 检查
log_info "运行 Clippy 代码质量检查..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    log_success "Clippy 检查通过"
else
    log_error "Clippy 检查失败，请修复警告"
    FAILED=1
fi

# 3. 构建检查
log_info "构建检查..."
if cargo build --all-features; then
    log_success "构建检查通过"
else
    log_error "构建检查失败"
    FAILED=1
fi

# 4. 运行测试
log_info "运行测试..."
if cargo test --all-features; then
    log_success "测试通过"
else
    log_error "测试失败"
    FAILED=1
fi

# 5. 检查文档
log_info "检查文档生成..."
if cargo doc --all-features --no-deps --quiet; then
    log_success "文档生成检查通过"
else
    log_error "文档生成失败"
    FAILED=1
fi

# 6. 运行基准测试（如果可用）
log_info "运行性能基准测试..."
if cargo bench --bench config_performance --quiet 2>/dev/null; then
    log_success "性能基准测试完成"
else
    log_warning "性能基准测试跳过（criterion 可能未安装）"
fi

# 7. 检查测试覆盖率（如果 cargo-llvm-cov 可用）
if command -v cargo-llvm-cov &> /dev/null; then
    log_info "生成测试覆盖率报告..."
    if cargo llvm-cov --all-features --quiet; then
        log_success "测试覆盖率报告生成完成"
        # 显示覆盖率统计
        cargo llvm-cov --all-features --summary | tail -10
    else
        log_warning "测试覆盖率生成失败"
    fi
else
    log_warning "跳过测试覆盖率检查（cargo-llvm-cov 未安装）"
fi

# 8. 安全审计（如果 cargo-audit 可用）
if command -v cargo-audit &> /dev/null; then
    log_info "运行安全审计..."
    if cargo audit; then
        log_success "安全审计通过"
    else
        log_error "安全审计发现安全问题"
        FAILED=1
    fi
else
    log_warning "跳过安全审计（cargo-audit 未安装）"
fi

# 总结
echo -e "${BLUE}==================== 质量检查总结 ====================${NC}"

if [ $FAILED -eq 0 ]; then
    log_success "🎉 所有质量检查通过！"
    exit 0
else
    log_error "❌ 质量检查失败，请修复上述问题后重试"
    exit 1
fi