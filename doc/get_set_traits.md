# GetSet Traits 学习记录

## 概述

`get_set.rs` 中的 trait 主要用于为**字段**实现统一的访问器模式，支持 Rust 的**内部可变性（Interior Mutability）**机制。根据字段的**修饰符**和**类型**，DSL 宏展开时会自动调用对应的 trait 方法。

## 核心 Trait 说明

### 1. GetSetCopy

**用途**: 为 `Copy` 类型的字段提供 getter/setter

**特点**:
- 适用于基本类型（`i32`, `bool` 等）
- 直接复制值，无需处理引用计数
- 支持 `default` 和 `late` 字段

**主要方法**:
- `cell_get(this: &Cell<Self>) -> Self` - 获取默认字段值
- `cell_set(this: &Cell<Self>, value: Self)` - 设置默认字段值
- `cell_option_get(this: &Cell<Option<Self>>) -> Self` - 获取 `late` 字段值
- `cell_option_set(this: &Cell<Option<Self>>, value: Self)` - 设置 `late` 字段值

### 2. GetSet

**用途**: 为非 `Copy` 类型（如类、`Rc`）的字段提供 getter/setter

**特点**:
- 适用于引用计数类型、类对象
- 需要处理引用计数的升级/降级
- 支持 `default`、`late` 和 `late final` 字段

**关联类型**:
- `type Get` - getter 返回类型
- `type OptionGet` - `late` 字段 getter 返回类型
- `type Set` - setter 接受类型

**主要方法**:
- `cell_get(this: &Cell<Self>) -> Self::Get` - 获取默认字段值
- `cell_set(this: &Cell<Self>, value: Self::Set)` - 设置默认字段值
- `cell_option_get(this: &Cell<Option<Self>>) -> Self::OptionGet` - 获取 `late` 字段值
- `once_cell_get(this: &OnceCell<Self>) -> Self::OptionGet` - 获取 `late final` 字段值
- `once_cell_set(this: &OnceCell<Self>, value: Self::Set)` - 设置 `late final` 字段值

### 3. GetSetOnce

**用途**: 为 `late final` 字段提供 getter/setter（只能初始化一次）

**特点**:
- 使用 `OnceCell<T>` 确保只能初始化一次
- 适用于 `Copy` 类型的 `late final` 字段

### 4. New / NewCopy

**用途**: 在构造函数中初始化字段

**方法**:
- `new_cell(value)` - 创建 `default` 字段
- `new_cell_option(value)` - 创建 `late` 字段
- `new_once_cell(value)` - 创建 `late final` 字段

## Trait 实现情况

### 自动实现（Blanket Implementations）

```rust
// 所有 Copy 类型自动实现
impl<T: Copy> GetSetCopy for T {}
impl<T: Copy> NewCopy for T {}
impl<T> GetSetOnce for T {}
impl<T> NewOnce for T {}
impl<T: GetSet> New for T {}
```

### 手动实现

| 类型 | 实现的 Trait | 原因 |
|------|------------|------|
| `T: ClassRcWeak` | `GetSet` | 类对象的弱引用，需要 upgrade/downgrade |
| `Option<T: ClassRcWeak>` | `GetSet` | 可选的类对象弱引用 |
| `Rc<T>` | `GetSet` | 标准引用计数类型，需要 clone |
| `Option<Rc<T>>` | `GetSet` | 可选的 Rc |
| `RcLike<T>` | `GetSet` | 类似 Rc 的包装类型（如 `HashTrieMap`） |
| `Option<RcLike<T>>` | `GetSet` | 可选的 RcLike |

## 字段修饰符与 Trait 方法映射

| 字段修饰符 | 类型分类 | 使用的 Trait | 调用的方法 | 示例字段 |
|-----------|---------|------------|-----------|---------|
| **`default`** | `NonRc` (Copy) | `GetSetCopy` | `cell_get()`, `cell_set()` | `x: i32` |
| **`default`** | `Rc` (非 Copy) | `GetSet` | `cell_get()`, `cell_set()` | `x: Rc<SomeClass>` |
| **`late`** | `NonRc` | `GetSetCopy` | `cell_option_get()`, `cell_option_set()` | `late x: i32` |
| **`late`** | `Rc` | `GetSet` | `cell_option_get()`, `cell_option_set()` | `late x: Rc<Class>` |
| **`late final`** | `NonRc` | `GetSetOnce` | `get()`, `set()` | `late final x: i32` |
| **`late final`** | `Rc` | `GetSet` | `once_cell_get()`, `once_cell_set()` | `late final x: Rc<Class>` |
| **`final`** | - | 不使用 trait | 直接返回引用 | `final x: i32` |
| **`raw`** | - | 不使用 trait | 直接返回引用 | `raw x: i32` |
| **`mutable`** | - | 不使用 trait | 使用 `RefCell::borrow()` | `mutable x: i32` |
| **`takecell`** | - | 不使用 trait | 使用 `TakeCell` 方法 | `takecell x: i32` |

## 宏展开逻辑

宏展开时根据字段修饰符和类型选择对应的 trait 方法：

```rust
// 来自 expand.rs 的简化逻辑
match field_kind {
    Default(kind) => match kind {
        Rc => GetSet::cell_get/set,
        NonRc => GetSetCopy::cell_get/set,
    },
    Late(kind) => match kind {
        Rc => GetSet::cell_option_get/set,
        NonRc => GetSetCopy::cell_option_get/set,
    },
    LateFinal(kind) => match kind {
        Rc => GetSet::once_cell_get/set,
        NonRc => GetSetOnce::get/set,
    },
    // ...
}
```

## Expanded 代码中的使用示例

### 1. 字段访问器方法

```rust
// 使用 GetSetCopy::cell_get 和 cell_set
pub fn get_x(&self) -> i32 {
    ::classes::get_set::GetSetCopy::cell_get(&self.0.x)
}

pub fn set_x<_T: Into<i32>>(&self, x: _T) {
    ::classes::get_set::GetSetCopy::cell_set(&self.0.x, x.into());
}
```

### 2. 构造函数中的字段初始化

```rust
pub fn new(...) -> CRc<Self> {
    unsafe {
        ::core::ptr::write(
            &raw mut (*_self.as_mut_ptr()).x,
            ::classes::get_set::NewCopy::new_cell(x),  // 使用 NewCopy
        );
        ::core::ptr::write(
            &raw mut (*_self.as_mut_ptr()).y,
            ::classes::get_set::NewCopy::new_cell(y),  // 使用 NewCopy
        );
    }
}
```

## 设计目的

这些 trait 提供了统一的字段访问抽象，使得 DSL 宏能够：

1. **自动生成 getter/setter 方法** - 根据字段类型和修饰符自动生成合适的访问器
2. **处理不同类型的字段** - 统一处理 Copy 类型和非 Copy 类型
3. **支持多种字段语义** - 支持 default、late、final 等不同的字段语义
4. **安全地处理内部可变性** - 通过 `Cell` 和 `OnceCell` 实现无需 `&mut` 的字段访问

## 关键理解

✅ **`get_set.rs` 中的 trait 主要是为字段实现的**

✅ **根据字段变量修饰符和类型的不同，程序展开时会调用对应 trait 的方法**

✅ **这提供了统一的字段访问抽象，同时适配不同的字段语义和类型**

## 参考资源

- **Rust Interior Mutability**: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
- **Rust Cell and RefCell**: https://doc.rust-lang.org/std/cell/index.html
- **Rust Trait Objects and Dynamic Dispatch**: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
- **Rust 宏展开和代码生成**: https://doc.rust-lang.org/book/ch19-06-macros.html
- **Rust Trait 和泛型编程**: https://doc.rust-lang.org/book/ch10-02-traits.html

