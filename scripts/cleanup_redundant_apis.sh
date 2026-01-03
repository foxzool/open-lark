#!/bin/bash

# OpenLark Meeting 模块冗余文件删除脚本
# 此脚本将删除在CSV API列表中不存在的冗余实现文件

set -e  # 遇到错误立即退出

echo "=========================================="
echo "OpenLark Meeting 冗余文件清理脚本"
echo "=========================================="
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 基础路径
BASE_DIR="crates/openlark-meeting/src"

# 统计变量
DELETED_COUNT=0
ERROR_COUNT=0

# 删除文件的函数
delete_file() {
    local file_path="$1"
    local description="$2"

    if [ -f "$file_path" ]; then
        echo -e "${YELLOW}删除:${NC} $description"
        echo "  文件: $file_path"
        rm -f "$file_path"
        DELETED_COUNT=$((DELETED_COUNT + 1))
        echo -e "${GREEN}✓ 已删除${NC}"
        echo ""
    else
        echo -e "${RED}✗ 跳过:${NC} $description"
        echo "  文件不存在: $file_path"
        ERROR_COUNT=$((ERROR_COUNT + 1))
        echo ""
    fi
}

# 删除目录的函数
delete_dir() {
    local dir_path="$1"
    local description="$2"

    if [ -d "$dir_path" ]; then
        echo -e "${YELLOW}删除目录:${NC} $description"
        echo "  目录: $dir_path"
        rm -rf "$dir_path"
        echo -e "${GREEN}✓ 已删除${NC}"
        echo ""
    else
        echo -e "${RED}✗ 跳过:${NC} $description"
        echo "  目录不存在: $dir_path"
        ERROR_COUNT=$((ERROR_COUNT + 1))
        echo ""
    fi
}

echo "=========================================="
echo "第一部分：Calendar 模块冗余文件清理"
echo "=========================================="
echo ""

# Calendar 模块冗余文件
delete_file "$BASE_DIR/calendar/calendar/v4/calendar/subscription.rs" \
    "重复的订阅管理（已有calendar/subscription.rs）"

delete_file "$BASE_DIR/calendar/calendar/v4/calendar/unsubscription.rs" \
    "重复的取消订阅管理（已有calendar/unsubscription.rs）"

delete_file "$BASE_DIR/calendar/calendar/v4/calendar/event/subscription.rs" \
    "重复的event订阅管理（已有event/subscription.rs）"

delete_file "$BASE_DIR/calendar/calendar/v4/calendar/event/unsubscription.rs" \
    "重复的event取消订阅管理（已有event/unsubscription.rs）"

delete_file "$BASE_DIR/calendar/calendar/v4/calendar/acl/subscription.rs" \
    "重复的acl订阅管理（已有acl/subscription.rs）"

delete_file "$BASE_DIR/calendar/calendar/v4/calendar/acl/unsubscription.rs" \
    "重复的acl取消订阅管理（已有acl/unsubscription.rs）"

echo "=========================================="
echo "第二部分：VC 模块冗余文件清理"
echo "=========================================="
echo ""

# VC 模块冗余文件
delete_file "$BASE_DIR/vc/vc/v1/reserve/create.rs" \
    "CSV中只有apply.rs（预约），create.rs不在API列表中"

delete_file "$BASE_DIR/vc/vc/v1/room_config/set.rs" \
    "可能是scope_config/create的重复"

delete_file "$BASE_DIR/vc/vc/v1/room_config/query.rs" \
    "可能是scope_config/get的重复"

delete_file "$BASE_DIR/vc/vc/v1/room_config/set_room_access_code.rs" \
    "CSV中无此API"

delete_file "$BASE_DIR/vc/vc/v1/room_config/set_checkboard_access_code.rs" \
    "CSV中无此API（拼写错误）"

echo "=========================================="
echo "第三部分：Meeting Room (旧版) 模块冗余文件清理"
echo "=========================================="
echo ""

# Meeting Room building 目录冗余文件
delete_file "$BASE_DIR/meeting_room/vc_meeting/old/default/building/create.rs" \
    "CSV中无building/create API"

delete_file "$BASE_DIR/meeting_room/vc_meeting/old/default/building/delete.rs" \
    "CSV中无building/delete API"

delete_file "$BASE_DIR/meeting_room/vc_meeting/old/default/building/update.rs" \
    "CSV中无building/update API"

delete_file "$BASE_DIR/meeting_room/vc_meeting/old/default/building/batch_get_id.rs" \
    "CSV中只有batch_get，无batch_get_id"

# Meeting Room room 目录冗余文件
delete_file "$BASE_DIR/meeting_room/vc_meeting/old/default/room/create.rs" \
    "CSV中无room/create API"

delete_file "$BASE_DIR/meeting_room/vc_meeting/old/default/room/delete.rs" \
    "CSV中无room/delete API"

delete_file "$BASE_DIR/meeting_room/vc_meeting/old/default/room/update.rs" \
    "CSV中无room/update API"

delete_file "$BASE_DIR/meeting_room/vc_meeting/old/default/room/batch_get_id.rs" \
    "CSV中只有batch_get，无batch_get_id"

# 删除district和country目录（CSV中无对应API）
delete_dir "$BASE_DIR/meeting_room/vc_meeting/old/default/district" \
    "CSV中无district相关API"

delete_dir "$BASE_DIR/meeting_room/vc_meeting/old/default/country" \
    "CSV中无country相关API"

echo "=========================================="
echo "清理完成！"
echo "=========================================="
echo ""
echo -e "${GREEN}删除文件总数: $DELETED_COUNT${NC}"
echo -e "${YELLOW}跳过/错误总数: $ERROR_COUNT${NC}"
echo ""

# 验证步骤
echo "=========================================="
echo "建议的后续验证步骤"
echo "=========================================="
echo ""
echo "1. 检查编译状态："
echo "   cd crates/openlark-meeting && cargo build"
echo ""
echo "2. 运行测试："
echo "   cd crates/openlark-meeting && cargo test"
echo ""
echo "3. 检查文档："
echo "   cd crates/openlark-meeting && cargo test --doc"
echo ""
echo "4. 运行linter："
echo "   cd crates/openlark-meeting && cargo clippy"
echo ""
echo "5. 如果一切正常，提交更改："
echo "   git add crates/openlark-meeting"
echo "   git commit -m \"chore: 删除openlark-meeting冗余API实现文件\""
echo ""
