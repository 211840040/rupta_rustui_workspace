# Playground - rustdsl + rupta 开发测试环境

这个playground用于编写使用rustdsl DSL语法的程序，观察宏展开结果，并使用rupta进行指针分析。

## 目录结构

```
playground/
├── src/bin/          # DSL程序源文件
│   └── example1.rs   # 示例程序
├── expanded/         # 宏展开结果
├── analysis_results/ # rupta分析结果
└── scripts/          # 辅助脚本
    ├── expand.sh     # 展开宏
    ├── analyze.sh    # rupta分析
    └── workflow.sh   # 完整工作流
```

## 环境准备

### 1. 安装cargo-expand (展开宏)
```bash
cargo install cargo-expand
```

### 2. 构建rupta
```bash
cd ../rupta
cargo build
```

### 3. 设置环境变量 (可选，已在脚本中自动设置)
```bash
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
export PTA_LOG=info  # 或 debug, warn, error
```

## 使用方法

### 方法1: 使用脚本 (推荐)

```bash
cd playground

# 完整工作流: 编译 -> 展开 -> 分析
./scripts/workflow.sh example1

# 或者分步执行:
./scripts/expand.sh example1      # 仅展开宏
./scripts/analyze.sh example1     # 仅rupta分析
```

### 方法2: 手动执行

```bash
# 1. 编译
cargo build --bin example1

# 2. 展开宏
cargo expand --bin example1 > expanded/example1_expanded.rs

# 3. rupta分析
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
../rupta/target/debug/pta expanded/example1_expanded.rs \
    --entry-func main \
    --pta-type cs \
    --context-depth 1 \
    --dump-call-graph analysis_results/example1/call_graph.dot \
    --dump-pts analysis_results/example1/pts.txt \
    --dump-mir analysis_results/example1/mir.txt
```

## 添加新的测试程序

1. 在 `src/bin/` 下创建新的 `.rs` 文件
2. 在 `Cargo.toml` 中添加对应的 `[[bin]]` 配置（可选，cargo会自动检测）
3. 使用脚本测试: `./scripts/workflow.sh <新程序名>`

## rupta 常用参数

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `--entry-func` | 入口函数名 | main |
| `--pta-type` | 分析类型: cs(callsite-sensitive), ander(andersen) | cs |
| `--context-depth` | callsite-sensitive分析的上下文深度 | 1 |
| `--dump-call-graph` | 输出call graph (DOT格式) | - |
| `--dump-pts` | 输出points-to分析结果 | - |
| `--dump-mir` | 输出所有可达函数的MIR | - |

## 查看分析结果

### Call Graph 可视化
```bash
# 转换为PNG图片
dot -Tpng analysis_results/example1/call_graph.dot -o analysis_results/example1/call_graph.png

# 或转换为SVG (推荐，可缩放)
dot -Tsvg analysis_results/example1/call_graph.dot -o analysis_results/example1/call_graph.svg
```

### Points-to Set
```bash
cat analysis_results/example1/pts.txt
```

### MIR
```bash
cat analysis_results/example1/mir.txt
```

## DSL 语法快速参考

```rust
use classes::prelude::CRc;
use classes_macros::classes;

classes! {
    // 定义类
    class ClassName {
        struct {
            field1: Type,
            field2: Type = default_value,  // 带默认值
            final field3: Type,            // final字段(不可变)
        }
        
        pub fn new(args) -> Self {
            Self { field1, field2, .. }
        }
        
        pub fn method(&self) {
            // 方法体
        }
    }
    
    // 继承
    class SubClass extends SuperClass {
        pub fn new() -> Self {
            Self { super: Super::new(), .. }
        }
        
        pub override fn method(&self) {
            // 重写方法
        }
    }
}
```

