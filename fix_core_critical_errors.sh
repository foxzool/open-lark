#!/bin/bash

echo "=== 修复剩余的核心编译错误 ==="

# 专门修复api_req.rs和其他核心文件的语法错误
echo "修复api_req.rs文件..."

# 修复未闭合的大括号问题
echo "修复未闭合的大括号..."

# 查找包含未闭合大括号的文件
find crates -name "*.rs" -exec grep -l ";\s*$" {} \; | while IFS=; do
    file="$IFS"
    # 检查是否需要修复
    if grep -q ";\s*$}" "$file" >/dev/null 2>&1; then
        echo "需要修复: $file"

        # 统计未匹配的大括号数量
        unclosed_count=$(grep -c ";\s*$" "$file" | wc -l | tr -d ' ' | wc -c)

        if [ "$unclosed_count" -gt 0 ]; then
            echo "  发现 $unclosed_count 个未匹配的大括号，进行修复..."

            # 使用更安全的修复方法：先添加缺失的分号，然后移除多余的
            # 为每个未匹配的开始大括号添加对应的结束大括号
            for i in $(seq 1 $unclosed_count); do
                # 找到对应行号的开始位置
                start_line=$(sed -n "${i}q" "s/[^{]*}]" "$file" | head -n "${i}" | tail -n +1)

                # 如果这一行是函数开始，但缺少结束大括号
                if [[ "$start_line" == *"def" || "$start_line" == *"fn" || "$start_line" == *"impl" || "$start_line" == *"async" || "$start_line" == *"pub" || "$start_line" == *"enum" || "$start_line" == *"struct" || "$start_line" == *"trait" || "$start_line" == *"mod" ]]; then
                    echo "  为第$i个未闭合大括号添加结束大括号"
                    # 在对应位置后插入 }
                    sed -i '' "${i}a)" "$file"' > "${file}.tmp" && mv "${file}.tmp" "$file"
                    echo "  ✅ 修复第$i个未闭合大括号"
                else
                    echo "  ⚠️  第$i个大括号已正确闭合"
                fi
            done

            # 移除临时文件
            rm -f "${file}.tmp" 2>/dev/null

            echo "  ✅ 修复完成: $file ($unclosed_count 个大括号修复)"
        fi
    else
        echo "  ⚠️  无需修复的大括号"
    fi
done

echo "修复完成！修复的未闭合大括号: $(find crates -name "*.rs" -exec grep -l ";\s*$" {} \; | while IFS=; do
    file="$IFS"

    # 计算仍需修复的文件数
    remaining_files=$(find crates -name "*.rs" -exec grep -l ";\s*$" {} \; | wc -l | tr -d ' ' | wc -c)
    if [ "$remaining_files" -gt 0 ]; then
        echo "⚠️  仍有 $remaining_files 个文件包含未匹配的大括号"
        find crates -name "*.rs" -exec grep -l ";\s*$" {} \; | while IFS=; do
            echo "  需要检查的文件: $file"
            # 简单检查每个文件的问题
            open issues=0
            grep ";\s*$" "$file" | while IFS= read -r; do
                if [[ "$REPLY" =~ *[;{}]*$ ]]; then
                    ((open_issues+=1))
                elif [[ "$LINE" =~ [^[\s]*$ ]]; then
                    ((open_issues+=1))
                else
                    echo "  - 正常代码行"
                fi
                IFS= read -r "$file"
            done

            if [ "$open_issues" -gt 0 ]; then
                echo " ❌ 文件 $file 存在语法问题，需要手动检查"
            elif [[ "$LINE" =~ [^http_method.*Method::[^;]*,$ ]] && [[ "$LINE" =~ [,\s*}\s*$[^}]*} ]]; then
                ((open_issues+=1))
                echo " ❌ 发现HTTP方法声明问题"
            elif [[ "$LINE" =~ [,\s*}\s*$[^}]*}, ]] && [[ "$LINE" =~ [,\s*}\s*\s*\s*\s*$[^}]*} ]]; then
                ((open_issues+=1))
                echo " ❌ 发现缺少分号或多余符号"
            fi
            elif [[ "$LINE" =~ [^\.\.default\(\)\s*,\)]]; then
                ((open_issues+=1))
                echo " ❌ 发现不必要的..Default::default()调用"
            fi
            fi

            IFS= read -r "$file"
            done
        fi

        if [ "$open_issues" -gt 0 ]; then
            echo " 严重错误：$file 有 $open_issues 个问题需要修复"
        elif [ "$open_issues" -gt 10 ]; then
            echo "错误太多，建议手动检查: $file"
        else
            echo " ✅ 文件 $file 语法正确"
        fi
    else
        echo " ⚠️ 未知错误，需要进一步检查"
    fi
done

echo "=== 修复完成 ==="
echo "已修复 $(find crates -name "*.rs" -exec grep -l ";\s*$" {} \; | wc -l | tr -d ' ' | wc -c | while IFS=; do
    file="$IFS"

    # 使用简单的sed命令直接替换常见错误模式
    # 1. 修复..Default::default() 模式问题
    find crates -name "*.rs" -type f -print0 | xargs -0 grep -l "\.\.\.default\(\)" "$file" | while IFS= read -r "$file"; do
        # 检查并替换
        if grep -q "\.\.\.default\(\)" "$file" >/dev/null; then
            echo "修复..Default::default()模式: $file"
            sed -i '' 's/\.\.default\(\)/mut api_req = ApiRequest::default();/g' "$file"
        done

        # 2. 修复问题函数
        find crates -name "*.rs" -type f -exec grep -l "fn test.*{$" "$file" | while IFS= read -r "$file"; do
            # 检查测试函数的问题
            # 如果函数没有结束大括号但有 }，则在行尾添加 }
            if grep -q "fn test.*{$" "$file" | grep -v "}$" >/dev/null; then
                # 检查该函数的结束位置
                end_line=$(grep -n "fn test.*{$" "$file" | tail -n 10 | head -1)
                if [[ "$end_line" =~ .*[^}]\s*$ ]] && [[ "$end_line" != *"{" ]]; then
                    # 在函数结束前添加 }
                    sed -i '' "${end_line} };" "$file"' > "${file}.tmp" && mv "${file}.tmp" "$file"
                    echo "  ✅ 修复测试函数: $file"
                else
                    echo " ⚠️ 测试函数结构正常"
                fi

                IFS= read -r "${file}.tmp"
                rm -f "${file}.tmp" 2>/dev/null
            done
        else
                echo " ❌ 函数有其他语法问题"
            fi
        fi

            IFS= read -r "$file"
            done
        fi
    done
        fi
    done
    else
            echo " ⚠️ 文件 $file 可能没有问题"
        fi
    done

        if grep -q "#\[test\]" "$file" >/dev/null; then
            # 测试模块导入是否正确
            test_import_result=$(grep -c "use.*open_lark_core::" "$file" | wc -l)
            if [ "$test_import_result" -gt 0 ]; then
                echo " ❌ 发现import问题，需要修复: $file"
            fi
        else
                echo " ✅ 模块导入正确"
            fi
        else
            echo " ⚠️ 文件 $file 可能没有测试模块"
            fi
        fi
    done
    fi

    echo "=== 验证修复结果 ==="

    # 再次统计剩余错误
    remaining=$(find crates -name "*.rs" -exec grep -l "\;\s*$" {} \; | wc -l | tr -d ' ' | wc -c)
    echo "剩余未修复文件数: $remaining"
done

echo "=== 错误修复脚本执行完成 ==="