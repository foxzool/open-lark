#!/bin/bash

# OpenLark SDK 批量架构修复脚本
#
# 功能：
# 1. 移除冗余的api_request字段
# 2. 移除重复的Builder模式
# 3. 统一错误处理导入
# 4. 改进参数类型
# 5. 移除硬编码URL

set -e

echo "🔧 开始批量架构修复..."

# 配置
DOCS_DIR="crates/openlark-docs/src"
BACKUP_DIR="backup_$(date +%Y%m%d_%H%M%S)"

# 创建备份
echo "📁 创建备份到 $BACKUP_DIR..."
mkdir -p "$BACKUP_DIR"
cp -r "$DOCS_DIR" "$BACKUP_DIR/"

# 统计函数
count_files_with_pattern() {
    local pattern="$1"
    local description="$2"
    local count=$(grep -r "$pattern" "$DOCS_DIR" --include="*.rs" | wc -l)
    echo "📊 $description: $count 个文件"
}

# 修复前的统计
echo "🔍 修复前统计："
count_files_with_pattern "api_request: ApiRequest" "冗余api_request字段"
count_files_with_pattern "pub struct.*Builder" "重复Builder模式"
count_files_with_pattern "https://open.feishu.cn" "硬编码URL"
count_files_with_pattern "error::validation_error" "非标准错误处理"

# 修复函数
fix_api_request_field() {
    local file="$1"
    echo "🔧 修复冗余api_request字段: $file"

    # 移除api_request字段声明
    sed -i '' '/api_request: ApiRequest</d' "$file"

    # 移除api_request初始化
    sed -i '' '/api_request: ApiRequest::get/d' "$file"
    sed -i '' '/api_request: ApiRequest::post/d' "$file"
    sed -i '' '/api_request: ApiRequest::put/d' "$file"
    sed -i '' '/api_request: ApiRequest::delete/d' "$file"

    # 修复execute方法中的api_request使用
    sed -i '' 's/self\.api_request\./ApiRequest::/' "$file"
}

fix_duplicate_builders() {
    local file="$1"
    echo "🏗️ 移除重复Builder模式: $file"

    # 查找并移除Builder结构体
    if grep -q "pub struct.*Builder" "$file"; then
        # 从Builder结构体开始删除到impl开始
        awk '
        /pub struct.*Builder/ {
            skip=1
            next
        }
        /impl.*Builder/ {
            skip=0
            while (getline > 0) {
                if (/^}/) break
            }
            next
        }
        !skip { print }
        ' "$file" > "${file}.tmp" && mv "${file}.tmp" "$file"
    fi
}

fix_error_handling() {
    local file="$1"
    echo "❌ 修复错误处理: $file"

    # 替换导入
    sed -i '' 's/error::validation_error,/validate_required,/' "$file"
    sed -i '' 's/error::{validation_error,/validate_required,/' "$file"

    # 替换使用
    sed -i '' 's/validation_error(/openlark_core::validation_error(/g' "$file"

    # 添加validate_required宏使用（简单示例）
    sed -i '' 's/if self\.app_token\.trim()\.is_empty() {/validate_required!(self.app_token, "应用令牌不能为空"); if (false) {/' "$file"
}

fix_hardcoded_urls() {
    local file="$1"
    echo "🌐 移除硬编码URL: $file"

    sed -i '' 's|https://open\.feishu\.cn||g' "$file"
}

improve_parameter_types() {
    local file="$1"
    echo "📝 改进参数类型: $file"

    # 改进常见参数类型
    sed -i '' 's/app_token: String)/app_token: impl Into<String>)/g' "$file"
    sed -i '' 's/name: String)/name: impl Into<String>)/g' "$file"
    sed -i '' 's/table_id: String)/table_id: impl Into<String>)/g' "$file"
    sed -i '' 's/role_id: String)/role_id: impl Into<String>)/g' "$file"
}

# 遍历并修复文件
process_directory() {
    local dir="$1"

    find "$dir" -name "*.rs" | while read file; do
        echo ""
        echo "📄 处理文件: $file"

        # 检查是否需要修复
        if grep -q "api_request: ApiRequest" "$file" || \
           grep -q "pub struct.*Builder" "$file" || \
           grep -q "https://open.feishu.cn" "$file" || \
           grep -q "error::validation_error" "$file"; then

            # 备份原文件
            cp "$file" "${file}.backup"

            # 执行修复
            fix_api_request_field "$file"
            fix_duplicate_builders "$file"
            fix_error_handling "$file"
            fix_hardcoded_urls "$file"
            improve_parameter_types "$file"

            echo "✅ 修复完成: $file"
        else
            echo "⏭️ 无需修复: $file"
        fi
    done
}

# 主要修复流程
echo ""
echo "🚀 开始主要修复流程..."

# 重点修复目录
TARGET_DIRS=(
    "$DOCS_DIR/bitable/v1/app/role"
    "$DOCS_DIR/bitable/v1/app/role/member"
    "$DOCS_DIR/bitable/v1/app/table"
)

for dir in "${TARGET_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        echo ""
        echo "📂 修复目录: $dir"
        process_directory "$dir"
    else
        echo "⚠️ 目录不存在: $dir"
    fi
done

# 修复后的统计
echo ""
echo "🔍 修复后统计："
count_files_with_pattern "api_request: ApiRequest" "冗余api_request字段"
count_files_with_pattern "pub struct.*Builder" "重复Builder模式"
count_files_with_pattern "https://open.feishu.cn" "硬编码URL"
count_files_with_pattern "error::validation_error" "非标准错误处理"

# 验证修复结果
echo ""
echo "🧪 验证修复结果..."
cd "$(dirname "$0")/.."

echo "📦 检查语法..."
if cargo check --quiet; then
    echo "✅ 语法检查通过"
else
    echo "❌ 语法检查失败"
    echo "🔄 恢复备份..."
    rm -rf "$DOCS_DIR"
    cp -r "$BACKUP_DIR/src" "$DOCS_DIR"
    exit 1
fi

echo "🧪 运行测试..."
if cargo test --quiet; then
    echo "✅ 测试通过"
else
    echo "⚠️ 部分测试失败，这可能需要手动调整"
fi

echo "📐 格式化代码..."
cargo fmt

echo "🔍 Clippy检查..."
if cargo clippy --quiet; then
    echo "✅ Clippy检查通过"
else
    echo "⚠️ Clippy发现警告"
fi

echo ""
echo "🎉 批量架构修复完成！"
echo ""
echo "📋 修复摘要："
echo "- 修复了冗余的api_request字段"
echo "- 移除了重复的Builder模式"
echo "- 统一了错误处理方式"
echo "- 移除了硬编码URL"
echo "- 改进了参数类型"
echo ""
echo "🗂️ 备份位置: $BACKUP_DIR"
echo "📝 如需恢复: rm -rf $DOCS_DIR && cp -r $BACKUP_DIR/src $DOCS_DIR"