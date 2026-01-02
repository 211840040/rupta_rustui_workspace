#!/bin/bash
# 展开宏并保存结果
# 用法: ./scripts/expand.sh example1
#       ./scripts/expand.sh src/bin/example1.rs

set -e

if [ -z "$1" ]; then
    echo "用法: $0 <binary_name|file_path>"
    echo "例如: $0 example1"
    echo "      $0 src/bin/example1.rs"
    exit 1
fi

# 确定输入是bin名还是文件路径
if [[ "$1" == *.rs ]]; then
    FILE="$1"
    BIN_NAME=$(basename "$1" .rs)
else
    BIN_NAME="$1"
    FILE="src/bin/${BIN_NAME}.rs"
fi

OUTPUT_FILE="expanded/${BIN_NAME}_expanded.rs"

echo "=== 展开 ${FILE} ==="
echo "输出到: ${OUTPUT_FILE}"
echo ""

# 使用cargo expand展开宏
# 注意: 需要安装 cargo-expand: cargo install cargo-expand
cargo expand --bin "$BIN_NAME" 2>/dev/null > "$OUTPUT_FILE"

echo "=== 展开完成 ==="
echo "查看展开结果: cat ${OUTPUT_FILE}"
echo "格式化后查看: rustfmt ${OUTPUT_FILE} && cat ${OUTPUT_FILE}"

