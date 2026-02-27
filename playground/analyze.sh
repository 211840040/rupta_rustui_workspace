#!/bin/bash
# RUPTA 自动化分析脚本
# 用法: ./analyze.sh <bin_target>
# 示例: ./analyze.sh simple_method_call

set -e

export PTA_LOG=debug
# export RUST_LOG=rupta=debug
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH


# 检查参数
if [ $# -eq 0 ]; then
    echo "用法: $0 <bin_target>"
    echo "示例: $0 simple_method_call"
    exit 1
fi

BIN_TARGET=$1
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RUPTA_DIR="$(cd "$SCRIPT_DIR/../rupta" && pwd)"
OUTPUT_DIR="$SCRIPT_DIR/analysis_results/$BIN_TARGET"

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

echo "=========================================="
echo "RUPTA 分析脚本"
echo "=========================================="
echo "分析目标: $BIN_TARGET"
echo "输出目录: $OUTPUT_DIR"
echo "=========================================="
echo ""

# 检查 rupta 是否已编译
if [ ! -f "$RUPTA_DIR/target/debug/cargo-pta" ]; then
    echo "警告: cargo-pta 未找到，尝试编译..."
    cd "$RUPTA_DIR"
    cargo build
    cd "$SCRIPT_DIR"
fi

cd "$RUPTA_DIR"
cargo build
cd "$SCRIPT_DIR"
cargo clean

# 运行分析
echo "开始分析..."
"$RUPTA_DIR/target/debug/cargo-pta" pta \
  --bin "$BIN_TARGET" \
  -- \
  --entry-func main \
  --pta-type cs \
  --context-depth 1 \
  --dump-pts "$OUTPUT_DIR/${BIN_TARGET}_pts.txt" \
  --dump-mir "$OUTPUT_DIR/${BIN_TARGET}_mir.txt" \
  --dump-call-graph "$OUTPUT_DIR/call_graph.dot" \
  --dump-class-call-graph "$OUTPUT_DIR/class_cg.txt" \
  --dump-class-pag "$OUTPUT_DIR/class_pag.txt" \
  --dump-class-pts "$OUTPUT_DIR/class_pts.txt" \
  2>&1 | tee "$OUTPUT_DIR/debug.log"
#   --dump-class-type-system "$OUTPUT_DIR/class_type_system.txt" \
#   --dump-class-ptr-system "$OUTPUT_DIR/class_ptr_system.txt" \

echo ""
echo "=========================================="
echo "分析完成！"
echo "=========================================="
echo "输出文件："
echo "  - ${BIN_TARGET}_pts.txt: 指针分析结果"
echo "  - ${BIN_TARGET}_mir.txt: MIR 代码"
echo "  - call_graph.dot: 调用图 (DOT 格式)"
echo "  - class_cg.txt: Class 调用图"
echo "  - class_type_system.txt: Class 类型系统"
echo "  - class_ptr_system.txt: Class 指针系统"
echo "  - debug.log: 分析日志"
echo "=========================================="
