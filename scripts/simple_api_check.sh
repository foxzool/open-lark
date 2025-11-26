#!/bin/bash
# 简化的API实现检查脚本

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

PROJECT_ROOT=$(cd "$(dirname "$0")/.." && pwd)
SECURITY_CRATE="${PROJECT_ROOT}/crates/openlark-security"

echo -e "${BLUE}🔍 简化版API实现验证${NC}"

# 1. 文件存在性检查
echo -e "\n${YELLOW}📁 API模块文件检查${NC}"

files=(
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

exist_count=0
total_count=${#files[@]}

for file in "${files[@]}"; do
    full_path="${SECURITY_CRATE}/${file}"
    if [ -f "$full_path" ]; then
        echo -e "  ${GREEN}✅ ${file}${NC}"
        ((exist_count++))
    else
        echo -e "  ${RED}❌ ${file}${NC}"
    fi
done

percentage=$((exist_count * 100 / total_count))
echo -e "\n文件存在率: ${exist_count}/${total_count} (${percentage}%)"

# 2. API内容检查
echo -e "\n${YELLOW}🔗 API实现内容检查${NC}"

# 检查关键API是否存在
check_api_in_file() {
    local file="$1"
    local pattern="$2"
    local description="$3"

    if [ -f "${SECURITY_CRATE}/${file}" ]; then
        if grep -q "$pattern" "${SECURITY_CRATE}/${file}"; then
            echo -e "  ${GREEN}✅ ${description}${NC}"
            return 0
        else
            echo -e "  ${RED}❌ ${description} (模式未找到)${NC}"
            return 1
        fi
    else
        echo -e "  ${RED}❌ ${description} (文件不存在)${NC}"
        return 1
    fi
}

api_count=0
total_apis=15

# ACS APIs
check_api_in_file "src/acs/v1/users/mod.rs" "open-apis/acs/v1/users" "ACS用户管理APIs" && ((api_count+=3))
check_api_in_file "src/acs/v1/user_faces/mod.rs" "open-apis/acs/v1/users" "ACS人脸识别APIs" && ((api_count+=2))
check_api_in_file "src/acs/v1/rule_external/mod.rs" "open-apis/acs/v1/rule_external" "ACS权限规则APIs" && ((api_count+=4))
check_api_in_file "src/acs/v1/visitors/mod.rs" "open-apis/acs/v1/visitors" "ACS访客管理APIs" && ((api_count+=2))

# Security&Compliance APIs
check_api_in_file "src/security_and_compliance/v2/device_records/mod.rs" "open-apis/security_and_compliance/v2/device_records" "设备记录管理APIs" && ((api_count+=6))

api_percentage=$((api_count * 100 / total_apis))
echo -e "\nAPI实现率: ${api_count}/${total_apis} (${api_percentage}%)"

# 3. 编译检查
echo -e "\n${YELLOW}📦 编译检查${NC}"
if cargo check -p openlark-security --all-features > /dev/null 2>&1; then
    echo -e "  ${GREEN}✅ 编译成功${NC}"
    compile_status="PASS"
else
    echo -e "  ${RED}❌ 编译失败${NC}"
    compile_status="FAIL"
fi

# 4. 总结
echo -e "\n${BLUE}📊 验证总结${NC}"

overall_score=$((percentage * 40 / 100 + api_percentage * 60 / 100))

if [ "$compile_status" = "FAIL" ]; then
    echo -e "${RED}⚠️  存在编译问题，评分暂时无效${NC}"
else
    if [ $overall_score -ge 80 ]; then
        echo -e "${GREEN}🌟 整体评分: ${overall_score}/100 - 优秀${NC}"
    elif [ $overall_score -ge 60 ]; then
        echo -e "${YELLOW}⭐ 整体评分: ${overall_score}/100 - 良好${NC}"
    else
        echo -e "${RED}⚠️  整体评分: ${overall_score}/100 - 需要改进${NC}"
    fi
fi

echo -e "\n📋 关键指标:"
echo -e "  • 文件完整性: ${percentage}%"
echo -e "  • API实现率: ${api_percentage}%"
echo -e "  • 编译状态: $compile_status"

if [ "$compile_status" = "FAIL" ]; then
    exit 1
fi