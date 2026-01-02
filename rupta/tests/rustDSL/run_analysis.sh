#!/bin/bash
# 运行 RUPTA 分析脚本

set -e

# 设置工作目录
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RUPTA_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUTPUT_DIR="$RUPTA_DIR/tests/rustDSL/analysis_results"

# 设置库路径
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH

# 创建输出目录（使用绝对路径）
mkdir -p "$OUTPUT_DIR"

echo "=========================================="
echo "开始运行 RUPTA 分析..."
echo "工作目录: $RUPTA_DIR"
echo "输出目录: $OUTPUT_DIR"
echo "=========================================="

# 切换到 rupta 目录
cd "$RUPTA_DIR"

# 运行分析（指定入口函数 test_entry，使用绝对路径）
./target/debug/cargo-pta pta \
  --manifest-path tests/rustDSL/Cargo.toml \
  --lib \
  -- \
  --entry-func test_entry \
  --pta-type cs \
  --context-depth 1 \
  --dump-call-graph "$OUTPUT_DIR/callgraph.dot" \
  --dump-pts "$OUTPUT_DIR/points_to.txt" \
  --dump-mir "$OUTPUT_DIR/mir.txt" \
  --dump-stats \
  2>&1 | tee "$OUTPUT_DIR/analysis.log"

echo ""
echo "=========================================="
echo "分析完成！"
echo "结果文件："
echo "  - callgraph.dot: 调用图"
echo "  - points_to.txt: 指针分析结果"
echo "  - mir.txt: MIR 代码"
echo "  - analysis.log: 分析日志"
echo "=========================================="

