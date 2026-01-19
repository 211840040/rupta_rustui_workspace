#!/bin/bash
# 调试脚本：运行 RUPTA 分析并查看调试输出

set -e

# 设置调试日志环境变量
# 使用 RUST_LOG 来指定日志级别和模块
# rupta=debug 表示 rupta crate 及其所有子模块都输出 debug 级别日志
export RUST_LOG=rupta=debug
export RUST_LOG_STYLE=always

# 设置库路径
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH

# 切换到 playground 目录
cd /home/wy/rupta_rustdsl_workspace/playground

# 创建输出目录
mkdir -p analysis_results/simple_load_store

# 运行分析，将调试输出保存到文件
echo "开始运行 RUPTA 分析（调试模式）..."
echo "调试日志将保存到: analysis_results/simple_load_store/debug.log"
echo ""

/home/wy/rupta_rustdsl_workspace/rupta/target/debug/cargo-pta pta \
  --manifest-path Cargo.toml \
  --bin simple_load_store \
  -- \
  --entry-func main \
  --pta-type cs \
  --context-depth 1 \
  --dump-call-graph analysis_results/simple_load_store/simple_load_store_cg.dot \
  --dump-pts analysis_results/simple_load_store/simple_load_store_pts.txt \
  --dump-mir analysis_results/simple_load_store/simple_load_store_mir.txt \
  2>&1 | tee analysis_results/simple_load_store/debug.log

echo ""
echo "分析完成！"
echo ""
echo "查看关键调试信息："
echo "1. DSL class 类型识别："
grep "is_dsl_class_type" analysis_results/simple_load_store/debug.log | head -20
echo ""
echo "2. 参数传播："
grep "add_inter_procedural_edges.*arg\[1\]" analysis_results/simple_load_store/debug.log | head -10
echo ""
echo "3. 返回值传播："
grep "add_inter_procedural_edges.*ret" analysis_results/simple_load_store/debug.log | head -10
