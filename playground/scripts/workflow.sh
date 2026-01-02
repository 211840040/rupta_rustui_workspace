#!/bin/bash
# 完整工作流: 编译 -> 展开 -> 分析
# 用法: ./scripts/workflow.sh example1

set -e

if [ -z "$1" ]; then
    echo "用法: $0 <binary_name>"
    echo "例如: $0 example1"
    exit 1
fi

BIN_NAME="$1"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "=========================================="
echo "  完整工作流: ${BIN_NAME}"
echo "=========================================="
echo ""

# Step 1: 编译检查
echo "[1/3] 编译检查..."
cargo build --bin "$BIN_NAME"
echo "✓ 编译成功"
echo ""

# Step 2: 展开宏
echo "[2/3] 展开宏..."
"$SCRIPT_DIR/expand.sh" "$BIN_NAME"
echo ""

# Step 3: rupta分析 (分析整个项目)
echo "[3/3] rupta分析..."
export PTA_LOG=warn
"$SCRIPT_DIR/analyze.sh"
echo ""

echo "=========================================="
echo "  工作流完成!"
echo "=========================================="
echo ""
echo "查看结果:"
echo "  展开代码: cat expanded/${BIN_NAME}_expanded.rs"
echo "  Points-to: cat analysis_results/pts.txt"
echo "  Call Graph: cat analysis_results/call_graph.dot"
