#!/bin/bash

# 文档状态检查脚本
# 用于自动检查文档的时效性和准确性

set -e

PROJECT_ROOT="/Users/zool/RustroverProjects/open-lark"
ARCHIVED_DIR="$PROJECT_ROOT/docs/archived"

echo "🔍 open-lark 文档状态检查"
echo "=============================="

# 检查函数
check_document() {
    local file="$1"
    local description="$2"

    if [ -f "$file" ]; then
        local size=$(du -h "$file" | cut -f1)
        local modified=$(stat -f "%Sm" -t "%Y-%m-%d" "$file")
        local lines=$(wc -l < "$file" 2>/dev/null || echo "0")

        # 检查是否有过时标记
        local outdated_markers=0
        if grep -q "TODO\|FIXME\|DEPRECATED\|OUTDATED\|WIP" "$file" 2>/dev/null; then
            outdated_markers=$(grep -c "TODO\|FIXME\|DEPRECATED\|OUTDATED\|WIP" "$file" 2>/dev/null || echo "0")
        fi

        echo "📄 $description"
        echo "   路径: $file"
        echo "   大小: $size"
        echo "   修改: $modified"
        echo "   行数: $lines"

        if [ "$outdated_markers" -gt 0 ]; then
            echo "   ⚠️  发现 $outdated_markers 个过时标记"
        else
            echo "   ✅ 无过时标记"
        fi
        echo ""
    else
        echo "❌ 缺失: $description ($file)"
        echo ""
    fi
}

# 检查核心文档
echo "🔴 核心文档状态"
echo "------------------"

check_document "$PROJECT_ROOT/README.md" "项目主文档"
check_document "$PROJECT_ROOT/CLAUDE.md" "开发指南"
check_document "$PROJECT_ROOT/docs/documentation-review-guidelines.md" "文档审查指南"

# 检查架构文档
echo "🏗️  架构文档状态"
echo "------------------"

find "$PROJECT_ROOT/docs/architecture" -name "*.md" -not -path "*/archived/*" | while read -r file; do
    filename=$(basename "$file")
    check_document "$file" "架构文档: $filename"
done

# 检查报告文档
echo "📊 报告文档状态"
echo "------------------"

find "$PROJECT_ROOT/reports" -name "*.md" | while read -r file; do
    filename=$(basename "$file")
    check_document "$file" "报告: $filename"
done

# 检查存档文档数量
echo "🗂️ 存档文档状态"
echo "------------------"

if [ -d "$ARCHIVED_DIR" ]; then
    archived_count=$(find "$ARCHIVED_DIR" -name "*.md" | wc -l)
    archived_size=$(du -sh "$ARCHIVED_DIR" 2>/dev/null | cut -f1)
    echo "📁 存档文档: $archived_count 个文件 ($archived_size)"
else
    echo "📁 存档目录: 不存在"
fi
echo ""

# 检查潜在问题文档
echo "⚠️  潜在问题文档"
echo "------------------"

# 查找包含过时标记的文档
echo "🔍 搜索包含过时标记的文档..."
problematic_files=$(find "$PROJECT_ROOT/docs" -name "*.md" -exec grep -l "TODO\|FIXME\|DEPRECATED\|OUTDATED\|WIP" {} \; 2>/dev/null)

if [ -n "$problematic_files" ]; then
    echo "发现以下文档包含过时标记:"
    echo "$problematic_files" | while read -r file; do
        echo "  - $(basename "$file"): $(grep -c "TODO\|FIXME\|DEPRECATED\|OUTDATED\|WIP" "$file") 个标记"
    done
else
    echo "✅ 未发现包含过时标记的文档"
fi
echo ""

# 检查空文档
echo "🔍 搜索空文档..."
empty_files=$(find "$PROJECT_ROOT/docs" -name "*.md" -size 0 2>/dev/null)

if [ -n "$empty_files" ]; then
    echo "发现以下空文档:"
    echo "$empty_files" | while read -r file; do
        echo "  - $file"
    done
else
    echo "✅ 未发现空文档"
fi
echo ""

# 统计总结
total_docs=$(find "$PROJECT_ROOT/docs" -name "*.md" | wc -l)
archived_docs=$(find "$PROJECT_ROOT/docs/archived" -name "*.md" 2>/dev/null | wc -l || echo "0")
active_docs=$((total_docs - archived_docs))

echo "📈 文档统计"
echo "------------"
echo "总文档数: $total_docs"
echo "存档文档: $archived_docs"
echo "活跃文档: $active_docs"
echo ""

if [ "$active_docs" -lt 10 ]; then
    echo "⚠️  警告: 活跃文档数量较少 ($active_docs)，建议增加文档覆盖"
elif [ "$active_docs" -gt 50 ]; then
    echo "💡 提示: 活跃文档数量较多 ($active_docs)，建议定期审查和清理"
else
    echo "✅ 文档数量合理 ($active_docs)"
fi

echo ""
echo "🎯 建议操作:"
echo "1. 根据文档审查指南定期检查文档状态"
echo "2. 及时更新包含过时标记的文档"
echo "3. 将过时文档移至存档目录"
echo "4. 定期清理空文档和重复内容"
echo ""
echo "✅ 文档状态检查完成！"