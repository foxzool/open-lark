#!/bin/bash
# 端点迁移脚本 - 基于bizTag到crate的映射

set -euo pipefail

# 定义映射关系
declare -A MIGRATION_MAP=(
    # admin crate
    ["acs"]="openlark-admin"
    ["admin"]="openlark-admin"
    ["mdm"]="openlark-admin"
    ["security_and_compliance"]="openlark-admin"
    ["tenant"]="openlark-admin"
    ["trust_party"]="openlark-admin"
    ["workplace"]="openlark-admin"

    # ai crate
    ["ai"]="openlark-ai"
    ["aily"]="openlark-ai"

    # auth crate (部分保留)
    ["passport"]="openlark-auth"
    ["verification"]="openlark-auth"

    # collab crate
    ["calendar"]="openlark-collab"
    ["minutes"]="openlark-collab"
    ["task"]="openlark-collab"

    # comm crate
    ["im"]="openlark-comm"
    ["mail"]="openlark-comm"
    ["vc"]="openlark-comm"

    # docs crate
    ["cloud_docs"]="openlark-docs"
    ["drive"]="openlark-docs"
    ["cardkit"]="openlark-docs"
    ["report"]="openlark-docs"

    # helpdesk crate
    ["lingo"]="openlark-helpdesk"
    ["helpdesk"]="openlark-helpdesk"
    ["search"]="openlark-helpdesk"

    # hr crate
    ["attendance"]="openlark-hr"
    ["corehr"]="openlark-hr"
    ["ehr"]="openlark-hr"
    ["okr"]="openlark-hr"
    ["payroll"]="openlark-hr"
    ["performance"]="openlark-hr"

    # hire crate
    ["hire"]="openlark-hire"

    # people crate
    ["contact"]="openlark-people"
    ["directory"]="openlark-people"
    ["personal_settings"]="openlark-people"
)

# 保留在core中的端点
declare -a CORE_ENDPOINTS=(
    "auth"
    "application"
    "platform_integration"
    "apass"
    "analytics"
    "ai_embedding"  # AI基础功能
    "ai_workflow"   # AI工作流
)

echo "🚀 开始端点迁移..."

# 创建目标目录
for crate in "${MIGRATION_MAP[@]}"; do
    mkdir -p "crates/${crate}/src/endpoints"
    echo "📁 创建目录: crates/${crate}/src/endpoints"
done

# 迁移端点文件
for endpoint_file in crates/openlark-core/src/endpoints/*.rs; do
    filename=$(basename "$endpoint_file" .rs)

    # 跳过mod.rs和特殊文件
    if [[ "$filename" == "mod" || "$filename" == "endpoints_original" ]]; then
        echo "⏭️  跳过: $filename.rs"
        continue
    fi

    # 检查是否保留在core中
    if [[ " ${CORE_ENDPOINTS[@]} " =~ " ${filename} " ]]; then
        echo "🔒 保留在core: $filename.rs"
        continue
    fi

    # 查找目标crate
    target_crate=""
    for biztag in "${!MIGRATION_MAP[@]}"; do
        if [[ "$filename" == "$biztag"* ]] || [[ "$biztag" == "$filename"* ]]; then
            target_crate="${MIGRATION_MAP[$biztag]}"
            break
        fi
    done

    # 如果没有找到精确匹配，尝试模糊匹配
    if [[ -z "$target_crate" ]]; then
        case "$filename" in
            *contact*|*directory*|*personal_settings*)
                target_crate="openlark-people"
                ;;
            *attendance*|*corehr*|*ehr*|*okr*|*payroll*|*performance*)
                target_crate="openlark-hr"
                ;;
            *im*|*mail*|*vc*)
                target_crate="openlark-comm"
                ;;
            *drive*|*docs*|*cardkit*|*report*)
                target_crate="openlark-docs"
                ;;
            *ai*|*aily*)
                target_crate="openlark-ai"
                ;;
            *admin*|*acs*|*mdm*|*security*|*tenant*|*trust_party*|*workplace*)
                target_crate="openlark-admin"
                ;;
            *calendar*|*minutes*|*task*)
                target_crate="openlark-collab"
                ;;
            *hire*)
                target_crate="openlark-hire"
                ;;
            *auth*|*passport*|*verification*)
                target_crate="openlark-auth"
                ;;
            *lingo*|*helpdesk*|*search*)
                target_crate="openlark-helpdesk"
                ;;
            *)
                echo "⚠️  未找到匹配的crate: $filename.rs"
                continue
                ;;
        esac
    fi

    # 执行迁移
    if [[ -n "$target_crate" ]]; then
        cp "$endpoint_file" "crates/${target_crate}/src/endpoints/${filename}.rs"
        echo "📦 迁移: $filename.rs -> ${target_crate}"
    fi
done

echo "✅ 端点文件迁移完成!"

# 生成各crate的endpoints/mod.rs文件
for crate_dir in crates/*/src/endpoints; do
    if [[ -d "$crate_dir" && "$crate_dir" != "crates/openlark-core/src/endpoints" ]]; then
        crate_name=$(basename $(dirname $(dirname "$crate_dir")))
        echo "🔧 生成 $crate_name 的 mod.rs..."

        cat > "$crate_dir/mod.rs" << EOF
//! $crate_name 服务端点定义
//!
//! 此模块包含 $crate_name 服务的所有API端点常量

// 导入核心端点
pub use openlark_core::endpoints::core::*;

EOF

        # 添加模块声明
        for endpoint_file in "$crate_dir"/*.rs; do
            if [[ -f "$endpoint_file" && "$(basename "$endpoint_file")" != "mod.rs" ]]; then
                module_name=$(basename "$endpoint_file" .rs)
                echo "pub mod $module_name;" >> "$crate_dir/mod.rs"
                echo "pub use $module_name::*;" >> "$crate_dir/mod.rs"
            fi
        done
    fi
done

echo "🎯 生成mod.rs文件完成!"