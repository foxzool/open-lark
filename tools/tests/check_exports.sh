#!/bin/bash
set -e

echo "🔍 检查通配符导出..."

# 查找所有通配符导出
WILDCARD_COUNT=$(rg "pub\s+use\s+[\w:]+\s*::\*\s*;" crates/openlark-docs/src --type rust -c | awk '{sum+=$1} END {print sum}')

echo "📊 当前通配符导出数量: $WILDCARD_COUNT"

# 显示前 20 个通配符导出
if [ "$WILDCARD_COUNT" -gt 0 ]; then
    echo ""
    echo "⚠️  发现的通配符导出（前 20 个）:"
    rg "pub\s+use\s+[\w:]+\s*::\*\s*;" crates/openlark-docs/src --type rust -n | head -20
fi

echo ""
echo "✅ 导出检查完成"
