#!/bin/bash

# 批量修复ApiRequest结构体初始化中的私有字段访问

echo "开始批量修复ApiRequest结构体初始化..."

# 找到所有需要修复的Rust文件
files=$(find crates/open-lark-communication crates/open-lark-collaboration -name "*.rs" -not -path "*/target/*" -not -name "mod.rs" | xargs grep -l "ApiRequest {" | head -20)

total_files=$(echo "$files" | wc -l)
echo "找到 $total_files 个需要修复的文件"

current_file=0

for file in $files; do
    current_file=$((current_file + 1))
    echo "[$current_file/$total_files] 检查文件: $file"

    # 检查是否包含私有字段初始化
    if grep -q "http_method:" "$file" || grep -q "supported_access_token_types:" "$file"; then
        echo "  ✓ 需要修复"

        # 创建临时文件
        temp_file=$(mktemp)

        # 应用修复规则 - 将结构体初始化改为方法调用
        perl -pe '
            # 处理ApiRequest结构体初始化
            if ( /let\s+\w+\s*=\s*(\w*::)?ApiRequest\s*\{/ ) {
                $in_struct_init = 1;
                $struct_line = $_;
                next;
            }

            if ($in_struct_init) {
                # 收集所有字段
                if ( /\}/ ) {
                    $in_struct_init = 0;
                    # 检查是否有私有字段
                    if ($struct_content =~ /http_method:|supported_access_token_types:/) {
                        # 提取其他字段
                        my @other_fields = ();
                        if ($struct_content =~ /api_path:\s*([^,]+)/) {
                            push @other_fields, "api_path: $1";
                        }
                        if ($struct_content =~ /body:\s*([^,]+)/) {
                            push @other_fields, "body: $1";
                        }
                        if ($struct_content =~ /query_params:\s*([^,]+)/) {
                            push @other_fields, "query_params: $1";
                        }
                        if ($struct_content =~ /file:\s*([^,]+)/) {
                            push @other_fields, "file: $1";
                        }

                        # 提取http_method和supported_access_token_types
                        my $http_method = "Method::GET"; # 默认值
                        my $token_types = "vec![]"; # 默认值

                        if ($struct_content =~ /http_method:\s*([^,]+)/) {
                            $http_method = $1;
                        }
                        if ($struct_content =~ /supported_access_token_types:\s*([^,]+)/) {
                            $token_types = $1;
                        }

                        # 生成新的代码
                        my $var_name = ($struct_line =~ /let\s+(\w+)\s*=/)[0];
                        my $result = "let mut $var_name = ApiRequest::with_method($http_method);\n";

                        # 添加token types
                        $result .= "        $var_name.set_supported_access_token_types($token_types);\n";

                        # 添加其他字段
                        foreach my $field (@other_fields) {
                            if ($field =~ /(\w+):\s*(.+)/) {
                                my $field_name = $1;
                                my $field_value = $2;
                                if ($field_name eq "api_path") {
                                    $result .= "        $var_name.set_api_path($field_value);\n";
                                } elsif ($field_name eq "body") {
                                    $result .= "        $var_name.set_body($field_value);\n";
                                } elsif ($field_name eq "file") {
                                    $result .= "        $var_name.set_file($field_value);\n";
                                }
                            }
                        }

                        $_ = $result;
                    } else {
                        # 没有私有字段，保持原样
                        $_ = $struct_line . $struct_content . "}\n";
                    }
                    $struct_content = "";
                } else {
                    $struct_content .= $_;
                    next;
                }
            } else {
                # 不在结构体初始化中，保持原样
            }
        ' "$file" > "$temp_file"

        # 检查是否有变化
        if ! cmp -s "$file" "$temp_file"; then
            mv "$temp_file" "$file"
            echo "  ✓ 已修复"
        else
            rm "$temp_file"
            echo "  - 无需修复"
        fi
    else
        echo "  - 无需修复"
    fi
done

echo "批量修复完成！"