# rcpta_full_hierarchy

用于验证 **rcpta** 的 hierarchy：既包含 Load/Store 语义的 ClassPAG 建边，也集成 rcpta 目前可处理的所有 Rust class syntax，以验证普适性。

## 类设计

- **Entity**（`entity.rs`）：抽象基类，含 `entity_id: i32`，实现接口 **Identifiable**（`get_id()`）。
- **Item**（`item.rs`）：继承 Entity，混入 **Tagged** mixin；无额外字段。用于 Holder 的类引用字段类型及 upcast/downcast/cast_mixin/into() 测试。
- **KeyedItem**（`keyed_item.rs`）：继承 Item，增加字段 `key: i32`；用于多级 upcast/downcast。
- **SpecialItem**（`special_item.rs`）：继承 Item，增加字段 `tag: i32`，重写 `Entity::describe`；用于 **method call 多态解析** 测试（同一 call site 下 pts(receiver) 可对应 3 种具体类型 → 3 个 callee）。
- **Holder**（`holder.rs`）：含类引用字段 `pub late item: CRc<Item>`；`get_item()` / `set_item(...)` 产生 Load/Store 语义。
- **Identifiable**（`interfaces.rs`）：接口（抽象类），用于 `into()` 转为 `CRc<Identifiable>` 的测试。
- **Tagged**（`mixins.rs`）：mixin on Entity，用于 `cast_mixin::<CRc<Tagged>>()` 测试。

## Entry 函数

### 1. `entry_load_store_demo()`（仅 Load/Store）

- 创建两个 Item（_a1、_b1）、两个 Holder（holder_1、holder_2）。
- **Store**：`holder_1.set_item(_a1)`，`holder_2.set_item(_b1)`。
- **Load**：`let _x = holder_1.get_item()`，`let _y = holder_2.get_item()`。

预期 ClassPAG：6 个指针（Item 4 个、Holder 2 个），Store 边 2 条，Load 边 2 条（详见 `doc/rcpta_rust_class_syntax_statements.md` §7）。

### 2. `entry_full_rcpta_demo()`（全量语法）

集中覆盖 rcpta 已支持的源码层语句：

- **Alloc**：`Item::new`、`KeyedItem::new`、`Holder::new`
- **Assign / Clone**：`let item_c = item_a.clone()`
- **Upcast**：`into_superclass::<CRc<Entity>>()`、`into_superclass::<CRc<Item>>()`
- **Downcast + Option::unwrap**：`try_into_subtype::<CRc<Item>>().unwrap()`
- **Cast to mixin**：`cast_mixin::<CRc<Tagged>>()`
- **Interface 转换**：`into()` 得到 `CRc<Identifiable>`
- **Load / Store**：`holder.set_item(...)`、`holder.get_item()`
- **方法调用**：`get_id()`、`describe()`、`describe_tagged()` 等

用于整体验证 rcpta 对上述语句的建边与指针建模是否一致。

### 3. `entry_method_call_demo(choice: u8)`（method call / 多态解析）

用于验证 rcpta 对 **method call** 的建模与基于 pts 的多态解析能力。

- 创建 **Item**、**KeyedItem**、**SpecialItem** 各一；根据 `choice`（0/1/2）将其中之一 upcast 为 `CRc<Entity>` 赋给变量 `e`。
- 调用 **`e.describe()`**：同一 call site，receiver 的 pts 可能为 `{ Item, KeyedItem, SpecialItem }` → 应解析出 **3 个 callee**（Item::describe、KeyedItem::describe、SpecialItem::describe）。
- 调用 **`e.get_id()`**：同一 call site，单一 callee（Entity 的 Identifiable::get_id）。

适合检查：`pts(receiver)` → `dispatch(concrete_type, method_name)` → 为每个 callee 建 CallArg/CallRet。

### 4. `entry_method_call_via_load_demo(choice: u8)`（method call 且 receiver 来自 Load）

与 3 类似，但 `e` 来自 **Holder 的 get_item()** 再 upcast 到 `CRc<Entity>`，用于验证「对从 Load 边得到的指针做 method call 解析」与 Load→Assign→Call 的边组合。

## 如何用 rcpta 验证

对 **classes** 的 test crate `rcpta_full_hierarchy` 跑 rupta，指定：

- entry 函数：`entry_load_store_demo`（仅 Load/Store）、`entry_full_rcpta_demo`（全量）、`entry_method_call_demo`（method call 多态）、`entry_method_call_via_load_demo`（method call + Load）。
- 输出 ClassPAG：`--class-pag-output <path>`。

检查生成的 ClassPAG 是否符合预期（Load/Store 见文档 §7；全量语法见 §二、§三）。

## 测试

```bash
cd rustdsl/classes && cargo test --test rcpta_full_hierarchy
```

- `test_load_store_demo_runs`：执行 `entry_load_store_demo()`，确保无 panic。
- `test_load_store_semantics`：对单个 Holder 做 set_item/get_item 并断言 `get_id()`，验证读写语义。
- `test_full_rcpta_demo_runs`：执行 `entry_full_rcpta_demo()`，确保全量语法用例无 panic。
- `test_method_call_demo_runs`：对 `entry_method_call_demo(0/1/2)` 各执行一次，覆盖三种 concrete type 路径。
- `test_method_call_via_load_demo_runs`：对 `entry_method_call_via_load_demo(0/1/2)` 各执行一次，覆盖「从 Holder Load 再 method call」的三种路径。
