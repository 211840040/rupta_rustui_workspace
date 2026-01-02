# 手动使用 RUPTA 分析 DSL 程序的完整步骤

## 完整流程（以 example_dsl.rs 为例）

### 步骤 1: 展开 DSL 宏

```bash
# 1.1 创建临时 Cargo 项目用于展开
cd /home/wy/rupta_rustdsl_workspace/rupta/tests/rustDSL
mkdir -p temp_expand
cd temp_expand
cargo init --name temp_expand

# 1.2 添加依赖
cat >> Cargo.toml << 'EOF'
[dependencies]
classes = { path = "../../../../rustdsl/classes" }
classes_macros = { path = "../../../../rustdsl/classes_macros" }
EOF

# 1.3 复制 DSL 代码
cp ../example_dsl.rs src/main.rs

# 1.4 展开宏（需要先安装 cargo-expand: cargo install cargo-expand）
cargo expand --bin temp_expand 2>&1 | sed -n '/^#\[/,$p' > ../example_dsl_expanded.rs

# 1.5 清理展开后的代码
cd ..
sed -i '1i extern crate classes;' example_dsl_expanded.rs
sed -i '/^use classes_macros::classes;$/d' example_dsl_expanded.rs
sed -i '/^extern crate std;$/d' example_dsl_expanded.rs
sed -i '/^#\[prelude_import\]$/d' example_dsl_expanded.rs
sed -i '/^use std::prelude::rust_2021::\*;$/d' example_dsl_expanded.rs
sed -i '/^#\[macro_use\]$/d' example_dsl_expanded.rs

# 1.6 清理临时文件
rm -rf temp_expand
```

### 步骤 2: 将展开后的代码复制到项目并修复

```bash
cd /home/wy/rupta_rustdsl_workspace/rupta/tests/rustDSL

# 复制展开后的代码
cp example_dsl_expanded.rs src/lib.rs

# 添加必要的特性标志（修复编译错误）
# 这些特性标志是展开后的代码所需要的
sed -i '1i #![feature(panic_internals)]' src/lib.rs
sed -i '2i #![feature(derive_clone_copy)]' src/lib.rs
```

### 步骤 3: 检查编译

```bash
cargo check --lib
```

如果还有编译错误，检查错误信息并相应修复。

### 步骤 4: 使用 RUPTA 分析

#### 方式 1: 使用脚本（推荐）

```bash
cd /home/wy/rupta_rustdsl_workspace/rupta/tests/rustDSL
chmod +x run_analysis.sh
./run_analysis.sh
```

脚本会自动使用绝对路径创建输出目录，避免路径问题。

#### 方式 2: 手动运行命令

```bash
cd /home/wy/rupta_rustdsl_workspace/rupta
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH

# 创建输出目录（使用绝对路径）
OUTPUT_DIR="/home/wy/rupta_rustdsl_workspace/rupta/tests/rustDSL/analysis_results"
mkdir -p "$OUTPUT_DIR"

# 运行分析（使用多行格式，注意每个参数之间要有空格）
# 注意：库项目需要指定一个入口函数，我们在 lib.rs 中添加了 test_entry 函数
# 使用绝对路径避免路径问题
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
  --dump-stats
```

**注意**：
- `pta` 和 `--manifest-path` 之间必须有空格
- `cs` 和 `--context-depth` 之间必须有空格
- 使用反斜杠 `\` 进行多行输入时，反斜杠后面不能有空格
- 分析结果会保存在 `tests/rustDSL/analysis_results/` 目录下
- **重要**：对于库项目（lib.rs），RUPTA 需要一个入口函数才能进行分析。我们在 `lib.rs` 中添加了 `test_entry` 函数作为入口点

### 步骤 5: 查看分析结果

分析完成后，结果文件会保存在 `tests/rustDSL/analysis_results/` 目录下：

- `callgraph.dot`: 调用图（DOT 格式），可用 Graphviz 可视化
- `points_to.txt`: 指针分析结果
- `mir.txt`: MIR 代码（可能很大）
- `analysis.log`: 分析过程的日志

查看结果：
```bash
cd /home/wy/rupta_rustdsl_workspace/rupta/tests/rustDSL/analysis_results
ls -lh
cat callgraph.dot
cat points_to.txt
```

## 关键点

1. **展开宏**：使用 `cargo expand` 将 DSL 代码展开成标准 Rust 代码
2. **清理代码**：移除展开后代码中不必要的导入（如 `classes_macros`、`std::prelude` 等）
3. **添加声明**：添加 `extern crate classes;` 声明
4. **使用 Cargo**：通过 Cargo 管理依赖，然后使用 `cargo-pta` 进行分析

## 注意事项

- 展开后的代码会非常大（example_dsl.rs 展开后约 5600 行）
- 需要确保 `classes` crate 已经构建
- 如果遇到编译错误，检查展开后的代码是否正确

