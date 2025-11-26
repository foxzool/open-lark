#!/bin/bash
# ACS和Security&Compliance API实现验证脚本
# 使用方法: ./scripts/validate_security_apis.sh

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT=$(cd "$(dirname "$0")/.." && pwd)
SECURITY_CRATE="${PROJECT_ROOT}/crates/openlark-security"
REPORT_DIR="${PROJECT_ROOT}/.claude/reports"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

echo -e "${BLUE}🔍 ACS和Security&Compliance API实现验证开始...${NC}"

# 创建报告目录
mkdir -p "$REPORT_DIR"

# 1. 基础编译检查
echo -e "\n${YELLOW}📦 1. 基础编译检查${NC}"
echo "检查 openlark-security crate 编译状态..."

if cargo check -p openlark-security --all-features > "$REPORT_DIR/compile_check_$TIMESTAMP.log" 2>&1; then
    echo -e "${GREEN}✅ 编译检查通过${NC}"
    COMPILE_STATUS="PASS"
else
    echo -e "${RED}❌ 编译检查失败${NC}"
    echo "查看详细日志: $REPORT_DIR/compile_check_$TIMESTAMP.log"
    COMPILE_STATUS="FAIL"
fi

# 2. Clippy代码质量检查
echo -e "\n${YELLOW}🔧 2. Clippy代码质量检查${NC}"
echo "运行 cargo clippy 检查..."

if cargo clippy -p openlark-security --all-features -- -D warnings > "$REPORT_DIR/clippy_check_$TIMESTAMP.log" 2>&1; then
    echo -e "${GREEN}✅ Clippy检查通过${NC}"
    CLIPPY_STATUS="PASS"
else
    echo -e "${RED}❌ Clippy检查失败${NC}"
    echo "查看详细日志: $REPORT_DIR/clippy_check_$TIMESTAMP.log"
    CLIPPY_STATUS="FAIL"
fi

# 3. 单元测试
echo -e "\n${YELLOW}🧪 3. 单元测试${NC}"
echo "运行单元测试..."

if cargo test -p openlark-security --all-features > "$REPORT_DIR/unit_tests_$TIMESTAMP.log" 2>&1; then
    echo -e "${GREEN}✅ 单元测试通过${NC}"
    TEST_STATUS="PASS"
else
    echo -e "${RED}❌ 单元测试失败${NC}"
    echo "查看详细日志: $REPORT_DIR/unit_tests_$TIMESTAMP.log"
    TEST_STATUS="FAIL"
fi

# 4. API文件存在性检查
echo -e "\n${YELLOW}4. API模块文件检查${NC}"

# 预期的API模块文件列表
expected_files=(
    "src/acs/v1/users/mod.rs"
    "src/acs/v1/user_faces/mod.rs"
    "src/acs/v1/rule_external/mod.rs"
    "src/acs/v1/visitors/mod.rs"
    "src/acs/v1/devices/mod.rs"
    "src/acs/v1/access_records/mod.rs"
    "src/security_and_compliance/v2/device_records/mod.rs"
    "src/security_and_compliance/v2/device_apply_records/mod.rs"
    "src/security_and_compliance/v1/openapi_logs/mod.rs"
)

files_exist=0
files_total=${#expected_files[@]}

for file_path in "${expected_files[@]}"; do
    full_path="${SECURITY_CRATE}/${file_path}"
    if [ -f "$full_path" ]; then
        echo -e "  ${GREEN}✅ ${file_path}${NC}"
        ((files_exist++))
    else
        echo -e "  ${RED}❌ ${file_path}${NC} (文件不存在)"
    fi
done

files_percentage=$((files_exist * 100 / files_total))
echo -e "\n文件存在率: ${files_exist}/${files_total} (${files_percentage}%)"

# 5. API路径实现检查
echo -e "\n${YELLOW}5. API路径实现检查${NC}"

# API检查列表 (方法 路径 文件)
api_patterns=(
    "PATCH /open-apis/acs/v1/users/:user_id acs/v1/users/mod.rs"
    "GET /open-apis/acs/v1/users/:user_id acs/v1/users/mod.rs"
    "GET /open-apis/acs/v1/users acs/v1/users/mod.rs"
    "PUT /open-apis/acs/v1/users/:user_id/face acs/v1/user_faces/mod.rs"
    "GET /open-apis/acs/v1/users/:user_id/face acs/v1/user_faces/mod.rs"
    "POST /open-apis/acs/v1/rule_external acs/v1/rule_external/mod.rs"
    "GET /open-apis/acs/v1/rule_external acs/v1/rule_external/mod.rs"
    "DELETE /open-apis/acs/v1/rule_external acs/v1/rule_external/mod.rs"
    "POST /open-apis/acs/v1/visitors acs/v1/visitors/mod.rs"
    "DELETE /open-apis/acs/v1/visitors/:visitor_id acs/v1/visitors/mod.rs"
    "GET /open-apis/security_and_compliance/v2/device_records/mine security_and_compliance/v2/device_records/mod.rs"
    "POST /open-apis/security_and_compliance/v2/device_records security_and_compliance/v2/device_records/mod.rs"
    "GET /open-apis/security_and_compliance/v2/device_records security_and_compliance/v2/device_records/mod.rs"
    "PUT /open-apis/security_and_compliance/v2/device_records/:device_record_id security_and_compliance/v2/device_records/mod.rs"
    "DELETE /open-apis/security_and_compliance/v2/device_records/:device_record_id security_and_compliance/v2/device_records/mod.rs"
)

apis_found=0
apis_total=${#api_patterns[@]}

for api_info in "${api_patterns[@]}"; do
    http_method=$(echo "$api_info" | cut -d' ' -f1)
    rest_info=$(echo "$api_info" | cut -d' ' -f2-)
    api_path=$(echo "$rest_info" | cut -d' ' -f1)
    file_path=$(echo "$rest_info" | cut -d' ' -f2-)

    full_path="${SECURITY_CRATE}/${file_path}"
    if [ -f "$full_path" ]; then
        # 检查文件中是否包含对应的API路径
        base_path=$(echo "$api_path" | sed 's|:[^/]*||g') # 移除路径参数
        if grep -q "$base_path" "$full_path"; then
            echo -e "  ${GREEN}✅ $http_method $api_path${NC}"
            ((apis_found++))
        else
            echo -e "  ${RED}❌ $http_method $api_path${NC} (路径未在文件中找到)"
        fi
    else
        echo -e "  ${RED}❌ $http_method $api_path${NC} (文件不存在: $file_path)"
    fi
done

apis_percentage=$((apis_found * 100 / apis_total))
echo -e "\nAPI实现率: ${apis_found}/${apis_total} (${apis_percentage}%)"

# 6. 文档生成检查
echo -e "\n${YELLOW}📖 6. 文档生成检查${NC}"

if cargo doc -p openlark-security --no-deps --all-features > "$REPORT_DIR/doc_generation_$TIMESTAMP.log" 2>&1; then
    echo -e "${GREEN}✅ 文档生成成功${NC}"
    DOC_STATUS="PASS"
else
    echo -e "${RED}❌ 文档生成失败${NC}"
    echo "查看详细日志: $REPORT_DIR/doc_generation_$TIMESTAMP.log"
    DOC_STATUS="FAIL"
fi

# 7. 生成综合报告
echo -e "\n${YELLOW}📊 7. 生成综合报告${NC}"

REPORT_FILE="$REPORT_DIR/api_validation_report_$TIMESTAMP.md"
cat > "$REPORT_FILE" << EOF
# ACS和Security&Compliance API实现验证报告

**生成时间**: $(date +"%Y-%m-%d %H:%M:%S")
**验证范围**: openlark-security crate
**项目版本**: $(git -C "$PROJECT_ROOT" describe --tags --always 2>/dev/null || echo "unknown")

## 验证结果总览

| 检查项目 | 状态 | 说明 |
|---------|------|------|
| 编译检查 | $COMPILE_STATUS | cargo check -p openlark-security |
| Clippy检查 | $CLIPPY_STATUS | 代码质量和风格检查 |
| 单元测试 | $TEST_STATUS | cargo test -p openlark-security |
| 文件存在性 | $files_exist/$files_total ($files_percentage%) | API模块文件完整性 |
| API路径实现 | $apis_found/$apis_total ($apis_percentage%) | API路径和方法实现 |
| 文档生成 | $DOC_STATUS | cargo doc 生成文档 |

## 详细分析

### API模块文件状态
EOF

for file_path in "${expected_files[@]}"; do
    full_path="${SECURITY_CRATE}/${file_path}"
    if [ -f "$full_path" ]; then
        echo "| $file_path | ✅ 存在 | - |" >> "$REPORT_FILE"
    else
        echo "| $file_path | ❌ 缺失 | - |" >> "$REPORT_FILE"
    fi
done

cat >> "$REPORT_FILE" << EOF

### API实现状态
EOF

for api_info in "${api_patterns[@]}"; do
    http_method=$(echo "$api_info" | cut -d' ' -f1)
    rest_info=$(echo "$api_info" | cut -d' ' -f2-)
    api_path=$(echo "$rest_info" | cut -d' ' -f1)
    file_path=$(echo "$rest_info" | cut -d' ' -f2-)

    full_path="${SECURITY_CRATE}/${file_path}"
    if [ -f "$full_path" ]; then
        base_path=$(echo "$api_path" | sed 's|:[^/]*||g')
        if grep -q "$base_path" "$full_path"; then
            echo "| $http_method $api_path | ✅ 已实现 | $file_path |" >> "$REPORT_FILE"
        else
            echo "| $http_method $api_path | ❌ 未找到 | $file_path |" >> "$REPORT_FILE"
        fi
    else
        echo "| $http_method $api_path | ❌ 文件缺失 | $file_path |" >> "$REPORT_FILE"
    fi
done

cat >> "$REPORT_FILE" << EOF

## 改进建议

### 高优先级
1. 实现缺失的API模块文件
2. 修复编译和Clippy警告
3. 补充单元测试覆盖率

### 中优先级
1. 完善API文档和示例
2. 添加集成测试
3. 优化错误处理机制

### 低优先级
1. 性能优化和基准测试
2. 代码重构和架构改进
3. 添加更多使用示例

## 相关文件
- 编译日志: \`compile_check_$TIMESTAMP.log\`
- Clippy日志: \`clippy_check_$TIMESTAMP.log\`
- 测试日志: \`unit_tests_$TIMESTAMP.log\`
- 文档生成日志: \`doc_generation_$TIMESTAMP.log\`

---

**报告生成者**: API验证脚本
**下次运行**: \`./scripts/validate_security_apis.sh\`
EOF

echo -e "${GREEN}✅ 综合报告已生成: $REPORT_FILE${NC}"

# 8. 总结
echo -e "\n${BLUE}🎯 验证总结${NC}"

# 计算整体评分
compile_score=20
clippy_score=15
test_score=20
files_score=$((files_percentage * 15 / 100))
apis_score=$((apis_percentage * 20 / 100))
doc_score=10

OVERALL_SCORE=$compile_score
[ "$CLIPPY_STATUS" = "PASS" ] && OVERALL_SCORE=$((OVERALL_SCORE + clippy_score))
[ "$TEST_STATUS" = "PASS" ] && OVERALL_SCORE=$((OVERALL_SCORE + test_score))
OVERALL_SCORE=$((OVERALL_SCORE + files_score + apis_score))
[ "$DOC_STATUS" = "PASS" ] && OVERALL_SCORE=$((OVERALL_SCORE + doc_score))

if [ $OVERALL_SCORE -ge 80 ]; then
    echo -e "${GREEN}🌟 整体评分: $OVERALL_SCORE/100 - 优秀${NC}"
elif [ $OVERALL_SCORE -ge 60 ]; then
    echo -e "${YELLOW}⭐ 整体评分: $OVERALL_SCORE/100 - 良好${NC}"
else
    echo -e "${RED}⚠️  整体评分: $OVERALL_SCORE/100 - 需要改进${NC}"
fi

echo -e "\n📋 关键指标:"
echo -e "  • 文件完整性: $files_percentage%"
echo -e "  • API实现率: $apis_percentage%"
echo -e "  • 代码质量: $CLIPPY_STATUS"
echo -e "  • 测试状态: $TEST_STATUS"

if [ "$COMPILE_STATUS" = "FAIL" ] || [ "$CLIPPY_STATUS" = "FAIL" ] || [ "$TEST_STATUS" = "FAIL" ]; then
    echo -e "\n${RED}⚠️  发现严重问题，请优先解决编译、代码质量或测试问题${NC}"
    exit 1
else
    echo -e "\n${GREEN}✅ 验证完成！查看详细报告: $REPORT_FILE${NC}"
fi