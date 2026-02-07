# 对指针流有影响的 Rust Class Syntax 语句（源码层面）

**目的**：梳理在 **Rust class 混合语法**（class DSL）下、**源码层面**与指针流相关的语句/表达式，并映射到 rcpta 的目标输出（ClassPAG 边、ClassPTS 等）。rcpta 的输出应屏蔽 expand/compile 后的 MIR 细节，只呈现**源码层面**关注的内容，类似分析 Java（支持 class 语法）程序时的抽象层次。

**语料范围**：以 `/home/wy/rupta_rustdsl_workspace/rustdsl` 下使用 Rust class syntax 编写的程序为准；rcpta 分析的程序所使用的语法不超出该范围。

**相关文档**：`rcpta_mir_statements.md` 整理的是 MIR 层面对指针流有影响的语句；本文档是**源码层**与 MIR 层之间的“Rust class syntax”对应关系。

**重要**：梳理“**何时建哪条 Class PAG 边**”时，应观察 **rust DSL 是如何用 Rust 语法模拟这些 OO 特性的**（cast、field 的 load/store、assign 如 clone 等），从而从源码与 DSL 实现逻辑自然判断：不同 source code 对应 **what edge kind**。

---

## 一、语料中的 Rust Class 语法来源与 DSL 实现位置

- **class 定义**：`classes_macros` 的 `class! { ... }`（`syntax.rs` 中的 `Class`、`ClassItem`、`ItemFn` 等）。
- **用户代码**：`rustdsl/classes/tests/` 下的 **animal_hierarchy**、**shape_hierarchy**、**vehicle_hierarchy**，以及 `classes_macros/tests/run/` 下的继承、mixin、接口等测试。
- **API 表面**：`classes` crate 的 prelude（`CRc`、`CPtr`）、`ptr.rs` / `macros/mod.rs` 暴露的 `new`、`into_superclass`、`try_into_subtype`、`cast_mixin`、`clone`、`into`（接口转换）等。

**DSL 实现逻辑（供 rcpta 建边时对照）**：
- **`rustdsl/classes/`**：关键 trait 与 struct。如 `ptr.rs`（`RcDyn`、cast/clone 语义）、`get_set.rs`（字段读写的 GetSet/GetSetCopy）、`class.rs`、`object.rs`；`macros/mod.rs`（`_def_class!` 等，包装类型的 Clone/From 委托）。
- **`rustdsl/classes_macros/`**：宏展开逻辑。`expand.rs`（`class!` 展开、构造函数、get_/set_ 生成）、`syntax.rs`（语法结构）、`expand/function_helper.rs`（如 build_superclass 委托 `into_superclass`）。

以下语句分类均来自上述语料与 DSL 实现的对应关系。

---

## 二、DSL 如何用 Rust 语法模拟 OO 特性与 ClassPAG 边对应

rcpta 建 ClassPAG 边时应**对照 DSL 的实现方式**：同一对象/不同对象、读字段/写字段、拷贝引用/类型视图变化，在 DSL 里都有对应的 Rust 调用，据此决定建 **Alloc / Assign / Cast / Load / Store** 中的哪一种。

| OO 特性 | 源码层写法 | DSL 实现（rustdsl） | 指针语义 | ClassPAG 边 |
|--------|------------|----------------------|----------|--------------|
| **对象创建** | `ClassName::new(...)` | 宏展开为 `CRcUninit`/`Rc::new`/`into` 等，最终在 `classes::ptr` 外仅“源码级”调用处分配堆对象 | 新对象，ptr → obj | **Alloc** |
| **赋值/拷贝引用** | `let x = y;`、`x = z;`；**`x.clone()`** | `RcDyn::clone()`（`ptr.rs`）只 clone 内部 `Rc`，指向同一堆对象；包装类型 `Clone` 委托 `self.0.clone()` | 同一对象，多一个引用 | **Assign** |
| **向上/向下/mixin 转型** | `x.into_superclass()`、`x.try_into_subtype::<CRc<Sub>>()`、`x.cast_mixin::<CRc<M>>()` | `ptr.rs` 中 `into_superclass`/`try_into_subtype`/`into_mixin`：`into_raw` → 同一 `data` 的 vtable 转换 → `from_raw`，**不分配新对象** | 同一对象，不同类型视图 | **Cast** |
| **读字段** | `x.get_foo()` | 宏生成 `get_<field>`，内部调用 `GetSet::cell_get` 等（`get_set.rs`）；class 类型时返回 clone 出的 CRc，即从“(base, field)”读出 | (base, field) → result | **Load** |
| **写字段** | `x.set_foo(value)` | 宏生成 `set_<field>`，内部 `GetSet::cell_set` 等；class 类型时 value 写入 base 的 field | value → (base, field) | **Store** |
| **普通方法调用** | `receiver.method(args)` | 宏展开为对 `RcDyn`/包装类型的方法调用，receiver 与 args 流入 callee，返回值流出 | 调用边 | **CallArg / CallRet** |

要点简述：
- **Cast 与 Assign 区分**：DSL 的 cast（`into_superclass`、`try_into_subtype`、`cast_mixin`）在实现上都是**同一块 data 指针 + 不同 vtable**，不新建对象，故建 **Cast** 边；“拷贝引用”的 `clone()` 和 `let a = b` 建 **Assign** 边。
- **Clone 仅指 DSL 的 Clone**：只有来自 `core::clone::Clone::clone`（或 DSL 包装委托到该语义）的调用才视为 **Assign**；用户自定义 `fn clone` 不是“引用拷贝”，不应建 ClassPAG Assign。
- **Load/Store 仅针对 class 类型字段**：getter/setter 的 GetSet 实现中，若字段为 class/CRc 类型，读写才产生指针流，故仅此时建 **Load**/**Store**。

---

## 三、源码层语句与指针流、ClassPAG 映射总览

下表与**第二节“DSL 如何用 Rust 语法模拟 OO 特性”**一一对应：根据 DSL 实现（同一对象 vs 新对象、读字段 vs 写字段、引用拷贝 vs 类型视图）决定建哪类边。

| 源码层语句/表达式 | 指针流含义 | ClassPAG 边 / rcpta 输出 | 备注 |
|-------------------|------------|---------------------------|------|
| **1. 类实例创建** `ClassName::new(...)` | 在堆上分配一个类对象，得到指向该对象的指针 | **Alloc**（ptr → obj） | 仅当“调用方为源码级”时建 ClassObj；与 `rcpta_implementation_log.md` §1 一致 |
| **2. 局部/赋值** `let x = y;` 或 `x = z;`（class 类型） | 指针拷贝/移动：y 或 z 指向的对象与 x 指向同一对象 | **Assign**（src → dst） | 含“调用前 receiver/实参写入参数槽”的隐式赋值 |
| **3. 向上转型** `x.into_superclass()` / `x.into_superclass::<CRc<Super>>()` | 同一对象，类型变为父类/接口；可能先 `clone` 再转 | **Assign** 或 **Cast**（src → dst） | 当前 MIR 多体现为 Copy/Move + Call；Cast 边可统一“同一对象不同视图” |
| **4. 向下转型** `x.try_into_subtype::<CRc<Sub>>()` | 同一对象，类型变为子类；可能先 `clone` 再转；返回 `Option<CRc<Sub>>` | **Cast**（src → dst） | 成功时指向同一对象；取出 Option 内引用见下条 |
| **4'. 类引用的 Option::unwrap** `opt.unwrap()`（`opt` 为 `Option<CRc<T>>`） | 从 Option 壳中取出类引用，赋到左端；同一对象 | **Assign**（Option 内类引用 → 左端） | 归属 Assign 大类；§4.10 |
| **5. Mixin 转型** `x.cast_mixin::<CRc<Mixin>>()` / `x.clone().cast_mixin::<...>()` | 同一对象，视图为某 mixin 类型 | **Assign** 或 **Cast**（src → dst） | animal_hierarchy 中 Flyable/Swimmable 等 |
| **6. 接口转换** `x.into()`（目标为接口，如 `CRc<Drivable>`） | 同向上转型，目标为接口类型 | **Assign** 或 **Cast**（src → dst） | vehicle_hierarchy 中 `car.clone().into()` |
| **7. Clone** `x.clone()`（class 类型） | 新 CRc 指向**同一**类对象（引用计数增加） | **Assign**（src → dst） | 不新建对象，仅指针拷贝语义 |
| **8. Getter 调用** `x.get_name()`、`x.get_breed()` 等 | 读对象字段；若返回 class 类型则“从对象/字段到结果”的读 | **Load**（(base, field) → result） | 已实现（handle_getter_setter） |
| **9. Setter 调用** `x.set_foo(value)` | 写对象字段；若 value 为 class 类型则“值到字段”的写 | **Store**（value → (base, field)） | 已实现（handle_getter_setter） |
| **10. 普通方法调用** `receiver.method(args)` | receiver 与 args 流入 callee，返回值流出 | **CallArg**（receiver/args → callee）、**CallRet**（callee → return） | 与 Tai-e 的 Call 边一致；可带源码级方法名 |

---

## 四、按语料分类的源码层语句

### 4.1 类实例创建（Alloc）

- **写法**：`Dog::new(...)`、`Eagle::new(...)`、`Shark::new(...)`、`Car::new(...)`、`Circle::new(...)` 等。
- **语义**：在堆上分配一个类对象，得到 `CRc<ClassName>`。
- **rcpta**：仅在**源码级调用方**（非构造体内部、非 `into_raw` 等 DSL 内部）处为该 Call 建 **ClassObj** 与 **Alloc** 边；实现见 `is_source_level_allocation_caller` 与 `handle_class_constructor`。

### 4.2 赋值与 Copy/Move（Assign）

- **显式**：`let animal: CRc<Animal> = dog.into_superclass();`、`let bird = eagle.clone().into_superclass::<CRc<Bird>>();`
- **隐式**：方法调用时 receiver 或实参被 copy/move 进参数槽或临时变量；MIR 中体现为 `StatementKind::Assign(place, Rvalue::Use(Copy|Move))`。
- **rcpta**：对两侧均为 class 类型的 Assign 建 **Assign** 边；输出时可保留“左端/右端”的源码对应信息（若能从 debug 信息或命名反推）。

### 4.3 向上转型（Upcast）

- **写法**：
  - `dog.into_superclass()` → `CRc<Animal>`
  - `eagle.clone().into_superclass::<CRc<Bird>>()` → `CRc<Bird>`
  - `bird.into_superclass::<CRc<Animal>>()` → `CRc<Animal>`
  - shape：`circle.clone().into_super()` → `CRc<Shape>`
  - vehicle：`car.clone().into_super()` → `CRc<MotorVehicle>` 等。
- **语义**：同一对象，类型变为父类或接口；不新建对象。
- **rcpta**：MIR 上可能是 Copy/Move + Call；若引入 **Cast** 边，可统一表示“同一对象、不同类型视图”的 ptr → ptr；否则用 **Assign** 表示“结果指针与源指向同一对象”。

### 4.4 向下转型（Downcast）

- **写法**：`animal.try_into_subtype::<CRc<Dog>>()`、`animal.clone().try_into_subtype::<CRc<Bird>>()` 等；返回 `Option<CRc<Sub>>`，后续常用 `.unwrap()` 取出子类引用（见 4.10）。
- **语义**：若运行时类型匹配，返回指向同一对象的子类类型指针；否则 `None`。不新建对象。
- **rcpta**：成功路径上建 **Cast**（receiver → 返回值）；与 Tai-e 对 Cast 的处理一致。

### 4.5 Mixin 转型（Cast to mixin）

- **写法**：`eagle.clone().cast_mixin::<CRc<Flyable>>()`、`duck.clone().cast_mixin::<CRc<Swimmable>>()`、`flying_fish.clone().cast_mixin::<CRc<Flyable>>()`。
- **语义**：同一对象，视图为某 mixin 类型。
- **rcpta**：同 4.3/4.4，**Assign** 或 **Cast**（src → dst），指向同一对象。

### 4.6 接口转换（Into interface）

- **写法**：vehicle_hierarchy 中 `car.clone().into()` 得到 `CRc<Drivable>`、`electric_car.clone().into_super()` 得到 `CRc<Car>` 再 `.into()` 等。
- **语义**：与向上转型一致，目标为接口类型。
- **rcpta**：与 4.3 相同，**Assign** 或 **Cast**。

### 4.7 Clone

- **写法**：`eagle.clone()`、`shark.clone()`、在链式调用中 `eagle.clone().into_superclass::<CRc<Bird>>()`。
- **语义**：新 `CRc` 指向同一类对象（引用计数 +1），无新对象。
- **rcpta**：**Assign**（源指针 → 返回的临时/局部），表示“指向同一对象”。

### 4.8 Getter / Setter

- **Getter**：`dog.get_name()`、`dog.get_breed()`、`eagle.get_wingspan()` 等；返回类型可能是 `Option<String>`、`f64` 或 class 类型。
- **Setter**：`x.set_foo(value)`（语料中多与构造或测试相关）。
- **rcpta**：Getter → **Load**（(base, field) → result）；Setter → **Store**（value → (base, field)）。仅当涉及 **class 类型** 的读/写时建边；实现见 `fpag_builder::handle_getter_setter`：在识别到 getter/setter 调用时对 ClassPAG 调用 `add_load(base_ptr_id, field_name, dst_ptr_id)` / `add_store(base_ptr_id, field_name, src_ptr_id)`，与 Tai-e 的 InstanceLoad/InstanceStore 建模一致（base 为 receiver，field 为字段名，dst/src 为结果/写入值对应的 ClassPtr）。

**Load/Store 的源码级指针一致化**：MIR 中同一源码变量可能对应多个 local（如 set_item 的 receiver 用 `_6 = &_3`，get_item 的 receiver 用 `_12 = &_4`），若直接按 MIR local 建 ClassPAG 指针会导致“同一 holder”对应多条边、指针冗余。rcpta 在建 Load/Store 边时对 **base** 与 **setter 的 value** 做**路径规范化**（`canonicalize_path_for_class_pag`）：若当前函数 MIR 中该 local 由 `&other`（Ref）或 `move other`/`copy other`（Use）定义，则用 **other** 对应的 path 作为 ClassPAG 的 ptr_id。这样 set/get 的 base 统一到“持有者” local（如 holder_1 → local_3），value 统一到“来源” local（如 _a1 → local_1），ClassPAG 输出与源码一一对应（6 个指针、2 条 Store、2 条 Load 等，见 §7）。

**识别与过滤**：仅将**用户 crate** 的 getter/setter 视为字段读写；`identify_getter_setter` 排除 `classes::`（DSL 运行时，如 `GetSet::cell_option_get`/`cell_option_set`），避免把 DSL 内部 Option 等当作 class 字段建边。ClassPAG 的 Load/Store 边仅在 **source-level 上下文**（`is_source_level_context`）下添加，不暴露 DSL 内部函数的指针。

### 4.9 普通方法调用

- **写法**：`animal.make_sound()`、`animal.move_action()`、`vehicle.drive()` 等。
- **语义**：receiver 与实参流入被调用方法，返回值流出。
- **rcpta**：**CallArg**（receiver、各实参 → callee）、**CallRet**（callee → 返回值槽）；输出时可带“源码级方法名”（如 `make_sound`），屏蔽 MIR 内部名。

### 4.10 类引用的 Option::unwrap（Assign）

- **写法**：`let sub = opt.unwrap();`，其中 `opt` 为 `Option<CRc<T>>`（常见于 `try_into_subtype::<CRc<Sub>>()` 的返回值）。
- **语义**：从 `Option::Some` 中取出内部的 `CRc<T>`，与 Option 内包装的类引用指向同一对象，无新分配；属于“把已有引用从 Option 壳里赋到左端”的 **Assign** 语义。
- **rcpta**（归属 **Assign** 大类）：
  - 识别 `Option::unwrap`（`core::option::Option::unwrap` / `std::option::Option::unwrap`）；当 receiver 类型为 `Option<CRc<T>>`（或其实参/拷贝可解析到该类引用 Option）时，建 **Assign** 边：**Option 内部的类引用（Some.0）→ unwrap 结果的左端**。
  - 为保持输出中指针一致、贴近源码，rcpta 用 **Option 的持有者 local**（如 `downcast_to_eagle` 对应的 local）表示“Option.Some.0”的指针 id，不单独暴露 `local_58.as_variant#1.0` 这类 MIR 投影名。
  - 实现见 `is_option_unwrap`、fpag_builder 中 Option::unwrap 分支、`path_to_class_ptr_id` 对 `[Downcast(1), Field(0)]`（Option.Some.0）的规范化。

---

## 五、Rust Class Syntax 与 MIR 的层次关系

```
  源码层（Rust class syntax）            MIR 层                     rcpta 输出
  ─────────────────────────            ───────                    ───────────
  ClassName::new(...)          →  多处 Call + 构造/into_raw 等  →  Alloc（仅源码级 Call）
  let x = y; / x = z;          →  Assign(_, Use(Copy|Move))    →  Assign
  x.into_superclass()          →  Copy/Move + Call + 可能 Cast →  Assign / Cast
  x.try_into_subtype::<T>()    →  Call（返回 Option）           →  Cast
  opt.unwrap()（Option<CRc<T>>）→  Call（Option::unwrap）        →  Assign
  x.cast_mixin::<M>()          →  Call                         →  Assign / Cast
  x.clone()                    →  Call(Clone::clone)           →  Assign
  x.get_foo() / x.set_foo(v)   →  Call + 字段/投影             →  Load / Store
  receiver.method(args)        →  Call                         →  CallArg, CallRet
```

rcpta 的输出应尽量落在**左列（源码层）**的抽象上：例如“某 Alloc 对应某处 `Dog::new`”“某 Assign 对应 `dog.into_superclass()` 的 receiver → 结果变量”，而不是暴露“local_4 → local_27”等 MIR 局部变量名，除非无法映射到源码。

---

## 六、与 Java 分析的类比（源码层面）

| Java 源码层 | Rust class syntax 源码层 | rcpta 目标输出 |
|-------------|--------------------------|----------------|
| `new T()`   | `ClassName::new(...)`    | Alloc（ptr → obj） |
| `a = b`     | `let a = b;` / 传参      | Assign          |
| 向上转型    | `into_superclass()` / `into()` | Assign / Cast |
| 向下转型    | `try_into_subtype::<T>()` | Assign / Cast  |
| 读字段      | `get_foo()`              | Load            |
| 写字段      | `set_foo(v)`             | Store           |
| 方法调用    | `receiver.method(args)`  | CallArg, CallRet |

这样 rcpta 的 ClassPAG / ClassPTS 报告可以像 Java 指针分析一样，以“源码级语句/表达式”为粒度呈现，而不是以 MIR 语句为粒度。

---

## 七、推荐用于 rcpta Load/Store 测试的 entry

- **rcpta_full_hierarchy（推荐）**：用于验证 rcpta Load/Store 建边及全部已支持 Rust class 语法的 hierarchy。
  - 路径：`rustdsl/classes/tests/rcpta_full_hierarchy/`。
  - 类设计：**Item**（简单类）、**Holder**（含类引用字段 `pub late item: CRc<Item>`）；`get_item`/`set_item` 对应 Load/Store。
  - Entry 函数：`entry_load_store_demo()`（在 `main.rs` 中）。行为：创建两个 Item（_a1、_b1）、两个 Holder（holder_1、holder_2），对每个 Holder 做 `set_item`（Store）和 `get_item`（Load），结果存入 _x、_y。
  - **预期 ClassPAG（源码一致）**：
    - **指针 6 个**：Item 4 个（对应 _a1、_b1、_x、_y，即 entry 的 local_1、local_2、local_11、local_13）；Holder 2 个（对应 holder_1、holder_2，即 entry 的 local_3、local_4）。base 与 value 经路径规范化后不再出现 MIR 中的临时/引用 local（如 local_6、local_9、local_12、local_14、local_7、local_10）。
    - **Store 边 2 条**：`(holder_1, "item") ← _a1`、`(holder_2, "item") ← _b1`（即 base local_3/local_4，value local_1/local_2）。
    - **Load 边 2 条**：`(holder_1, "item") → _x`、`(holder_2, "item") → _y`（即 base local_3/local_4，dst local_11/local_13）。
  - 运行 rupta 时以该 test crate 为分析对象，entry 指定为 `entry_load_store_demo`，并设置 `--class-pag-output` 即可核对 Load/Store 边。
- **备选**：`playground` 的 **simple_load_store** 的 `main`（`playground/src/bin/simple_load_store.rs`），用法同上。

## 八、待实现与维护注意

- **Cast 边**：当前以 Assign 覆盖“同一对象不同类型视图”；若引入显式 **Cast** 边，便于与 Tai-e 的 Cast 一致，并在报告中区分“纯赋值”与“类型转换”。
- **Load/Store**：已在 getter/setter 识别路径中建 ClassPAG Load/Store 边；仅对 **class 类型** 字段的读写建边。实现要点：**路径规范化**（`fpag_builder::canonicalize_path_for_class_pag`）保证同一源码级 class ref 对应单一 ptr_id；**仅 source-level 上下文**建 ClassPAG Load/Store；**排除 `classes::` getter/setter**（`analysis::identify_getter_setter`）避免 DSL 内部建边。
- **源码映射**：若有 debug 信息或 span，可把 ClassPAG 的边与源码位置绑定，输出时显示“对应源码行/表达式”，进一步屏蔽 MIR 细节。
- **语料更新**：若 rustdsl 新增语法或 API（如新的 cast/convert 方法），在本文档和 `rcpta_mir_statements.md` 中同步补充对应关系。

---

**文档版本**：与 `rcpta_implementation_log.md`、`rcpta_mir_statements.md` 配套；语料以 rustdsl 仓库为准。
