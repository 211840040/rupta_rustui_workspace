#!/bin/bash
# 使用rupta分析DSL程序
# 用法: ./scripts/analyze.sh [rupta额外参数]

set -e

RUPTA_DIR="../rupta"
RUPTA_BIN="${RUPTA_DIR}/target/debug/cargo-pta"
ANALYSIS_DIR="analysis_results"

# 检查rupta是否已构建
if [ ! -f "$RUPTA_BIN" ]; then
    echo "rupta未构建,正在构建..."
    (cd "$RUPTA_DIR" && cargo build)
fi

# 创建分析结果目录
mkdir -p "$ANALYSIS_DIR"

echo "=== 使用rupta分析项目 ==="
echo "分析结果将保存到: ${ANALYSIS_DIR}/"
echo ""

# 设置LD_LIBRARY_PATH以确保能找到rustc库
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH

# 设置日志级别
export PTA_LOG=${PTA_LOG:-warn}

# 运行rupta分析
# 注意: cargo-pta 会分析整个项目，而不是单个文件
"$RUPTA_BIN" pta -- \
    --entry-func main \
    --pta-type cs \
    --context-depth 1 \
    --dump-call-graph "${ANALYSIS_DIR}/call_graph.dot" \
    --dump-pts "${ANALYSIS_DIR}/pts.txt" \
    "$@"

echo ""
echo "=== 分析完成 ==="
echo "Call Graph: ${ANALYSIS_DIR}/call_graph.dot"
echo "Points-to Set: ${ANALYSIS_DIR}/pts.txt"
echo ""
echo "可视化Call Graph: dot -Tpng ${ANALYSIS_DIR}/call_graph.dot -o ${ANALYSIS_DIR}/call_graph.png"
