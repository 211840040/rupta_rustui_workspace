# 对指针流有影响的 Rust Class Syntax 语句（源码层面）

**目的**：梳理在 **Rust class 混合语法**（class DSL）下、**源码层面**与指针流相关的语句/表达式，并映射到 rcpta 的目标输出（ClassPAG 边、ClassPTS 等）。rcpta 的输出应屏蔽 expand/compile 后的 MIR 细节，只呈现**源码层面**关注的内容，类似分析 Java（支持 class 语法）程序时的抽象层次。

**语料范围**：以 `/home/wy/rupta_rustdsl_workspace/rustdsl` 下使用 Rust class syntax 编写的程序为准；rcpta 分析的程序所使用的语法不超出该范围。

**相关文档**：`rcpta_mir_statements.md` 整理的是 MIR 层面对指针流有影响的语句；本文档是**源码层**与 MIR 层之间的“Rust class syntax”对应关系。

---

## 一、语料中的 Rust Class 语法来源

- **class 定义**：`classes_macros` 的 `class! { ... }`（`syntax.rs` 中的 `Class`、`ClassItem`、`ItemFn` 等）。
- **用户代码**：`rustdsl/classes/tests/` 下的 **animal_hierarchy**、**shape_hierarchy**、**vehicle_hierarchy**，以及 `classes_macros/tests/run/` 下的继承、mixin、接口等测试。
- **API 表面**：`classes` crate 的 prelude（`CRc`、`CPtr`）、`ptr.rs` / `macros/mod.rs` 暴露的 `new`、`into_superclass`、`try_into_subtype`、`cast_mixin`、`clone`、`into`（接口转换）等。

以下语句分类均来自上述语料中的实际写法。

---

## 二、源码层语句与指针流、ClassPAG 映射总览

| 源码层语句/表达式 | 指针流含义 | ClassPAG 边 / rcpta 输出 | 备注 |
|-------------------|------------|---------------------------|------|
| **1. 类实例创建** `ClassName::new(...)` | 在堆上分配一个类对象，得到指向该对象的指针 | **Alloc**（ptr → obj） | 仅当“调用方为源码级”时建 ClassObj；与 `rcpta_implementation_log.md` §1 一致 |
| **2. 局部/赋值** `let x = y;` 或 `x = z;`（class 类型） | 指针拷贝/移动：y 或 z 指向的对象与 x 指向同一对象 | **Assign**（src → dst） | 含“调用前 receiver/实参写入参数槽”的隐式赋值 |
| **3. 向上转型** `x.into_superclass()` / `x.into_superclass::<CRc<Super>>()` | 同一对象，类型变为父类/接口；可能先 `clone` 再转 | **Assign** 或 **Cast**（src → dst） | 当前 MIR 多体现为 Copy/Move + Call；Cast 边可统一“同一对象不同视图” |
| **4. 向下转型** `x.try_into_subtype::<CRc<Sub>>()` / `.unwrap()` | 同一对象，类型变为子类；可能先 `clone` 再转 | **Assign** 或 **Cast**（src → dst） | 成功时指向同一对象 |
| **5. Mixin 转型** `x.cast_mixin::<CRc<Mixin>>()` / `x.clone().cast_mixin::<...>()` | 同一对象，视图为某 mixin 类型 | **Assign** 或 **Cast**（src → dst） | animal_hierarchy 中 Flyable/Swimmable 等 |
| **6. 接口转换** `x.into()`（目标为接口，如 `CRc<Drivable>`） | 同向上转型，目标为接口类型 | **Assign** 或 **Cast**（src → dst） | vehicle_hierarchy 中 `car.clone().into()` |
| **7. Clone** `x.clone()`（class 类型） | 新 CRc 指向**同一**类对象（引用计数增加） | **Assign**（src → dst） | 不新建对象，仅指针拷贝语义 |
| **8. Getter 调用** `x.get_name()`、`x.get_breed()` 等 | 读对象字段；若返回 class 类型则“从对象/字段到结果”的读 | **Load**（(base, field) → result） | 待实现；需区分返回 Copy 与 class 类型 |
| **9. Setter 调用** `x.set_foo(value)` | 写对象字段；若 value 为 class 类型则“值到字段”的写 | **Store**（value → (base, field)） | 待实现 |
| **10. 普通方法调用** `receiver.method(args)` | receiver 与 args 流入 callee，返回值流出 | **CallArg**（receiver/args → callee）、**CallRet**（callee → return） | 与 Tai-e 的 Call 边一致；可带源码级方法名 |

---

## 三、按语料分类的源码层语句

### 3.1 类实例创建（Alloc）

- **写法**：`Dog::new(...)`、`Eagle::new(...)`、`Shark::new(...)`、`Car::new(...)`、`Circle::new(...)` 等。
- **语义**：在堆上分配一个类对象，得到 `CRc<ClassName>`。
- **rcpta**：仅在**源码级调用方**（非构造体内部、非 `into_raw` 等 DSL 内部）处为该 Call 建 **ClassObj** 与 **Alloc** 边；实现见 `is_source_level_allocation_caller` 与 `handle_class_constructor`。

### 3.2 赋值与 Copy/Move（Assign）

- **显式**：`let animal: CRc<Animal> = dog.into_superclass();`、`let bird = eagle.clone().into_superclass::<CRc<Bird>>();`
- **隐式**：方法调用时 receiver 或实参被 copy/move 进参数槽或临时变量；MIR 中体现为 `StatementKind::Assign(place, Rvalue::Use(Copy|Move))`。
- **rcpta**：对两侧均为 class 类型的 Assign 建 **Assign** 边；输出时可保留“左端/右端”的源码对应信息（若能从 debug 信息或命名反推）。

### 3.3 向上转型（Upcast）

- **写法**：
  - `dog.into_superclass()` → `CRc<Animal>`
  - `eagle.clone().into_superclass::<CRc<Bird>>()` → `CRc<Bird>`
  - `bird.into_superclass::<CRc<Animal>>()` → `CRc<Animal>`
  - shape：`circle.clone().into_super()` → `CRc<Shape>`
  - vehicle：`car.clone().into_super()` → `CRc<MotorVehicle>` 等。
- **语义**：同一对象，类型变为父类或接口；不新建对象。
- **rcpta**：MIR 上可能是 Copy/Move + Call；若引入 **Cast** 边，可统一表示“同一对象、不同类型视图”的 ptr → ptr；否则用 **Assign** 表示“结果指针与源指向同一对象”。

### 3.4 向下转型（Downcast）

- **写法**：`animal.try_into_subtype::<CRc<Dog>>()`、`animal.clone().try_into_subtype::<CRc<Bird>>()`、`.unwrap()`。
- **语义**：若运行时类型匹配，返回指向同一对象的子类类型指针；否则 `None`。
- **rcpta**：成功路径上建 **Assign** 或 **Cast**（src → 返回值）；与 Tai-e 对 Cast 的处理一致，可作为后续 Cast 边实现的参考。

### 3.5 Mixin 转型（Cast to mixin）

- **写法**：`eagle.clone().cast_mixin::<CRc<Flyable>>()`、`duck.clone().cast_mixin::<CRc<Swimmable>>()`、`flying_fish.clone().cast_mixin::<CRc<Flyable>>()`。
- **语义**：同一对象，视图为某 mixin 类型。
- **rcpta**：同 3.3/3.4，**Assign** 或 **Cast**（src → dst），指向同一对象。

### 3.6 接口转换（Into interface）

- **写法**：vehicle_hierarchy 中 `car.clone().into()` 得到 `CRc<Drivable>`、`electric_car.clone().into_super()` 得到 `CRc<Car>` 再 `.into()` 等。
- **语义**：与向上转型一致，目标为接口类型。
- **rcpta**：与 3.3 相同，**Assign** 或 **Cast**。

### 3.7 Clone

- **写法**：`eagle.clone()`、`shark.clone()`、在链式调用中 `eagle.clone().into_superclass::<CRc<Bird>>()`。
- **语义**：新 `CRc` 指向同一类对象（引用计数 +1），无新对象。
- **rcpta**：**Assign**（源指针 → 返回的临时/局部），表示“指向同一对象”。

### 3.8 Getter / Setter

- **Getter**：`dog.get_name()`、`dog.get_breed()`、`eagle.get_wingspan()` 等；返回类型可能是 `Option<String>`、`f64` 或 class 类型。
- **Setter**：`x.set_foo(value)`（语料中多与构造或测试相关）。
- **rcpta**：Getter → **Load**（(base, field) → result）；Setter → **Store**（value → (base, field)）。仅当涉及 **class 类型** 的读/写时建边，可与 MIR 的 Call 与字段投影对应。

### 3.9 普通方法调用

- **写法**：`animal.make_sound()`、`animal.move_action()`、`vehicle.drive()` 等。
- **语义**：receiver 与实参流入被调用方法，返回值流出。
- **rcpta**：**CallArg**（receiver、各实参 → callee）、**CallRet**（callee → 返回值槽）；输出时可带“源码级方法名”（如 `make_sound`），屏蔽 MIR 内部名。

---

## 四、Rust Class Syntax 与 MIR 的层次关系

```
  源码层（Rust class syntax）            MIR 层                     rcpta 输出
  ─────────────────────────            ───────                    ───────────
  ClassName::new(...)          →  多处 Call + 构造/into_raw 等  →  Alloc（仅源码级 Call）
  let x = y; / x = z;          →  Assign(_, Use(Copy|Move))    →  Assign
  x.into_superclass()          →  Copy/Move + Call + 可能 Cast →  Assign / Cast
  x.try_into_subtype::<T>()    →  Call + Option 解包           →  Assign / Cast
  x.cast_mixin::<M>()          →  Call                         →  Assign / Cast
  x.clone()                    →  Call(Clone::clone)           →  Assign
  x.get_foo() / x.set_foo(v)   →  Call + 字段/投影             →  Load / Store
  receiver.method(args)        →  Call                         →  CallArg, CallRet
```

rcpta 的输出应尽量落在**左列（源码层）**的抽象上：例如“某 Alloc 对应某处 `Dog::new`”“某 Assign 对应 `dog.into_superclass()` 的 receiver → 结果变量”，而不是暴露“local_4 → local_27”等 MIR 局部变量名，除非无法映射到源码。

---

## 五、与 Java 分析的类比（源码层面）

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

## 六、待实现与维护注意

- **Cast 边**：当前以 Assign 覆盖“同一对象不同类型视图”；若引入显式 **Cast** 边，便于与 Tai-e 的 Cast 一致，并在报告中区分“纯赋值”与“类型转换”。
- **Load/Store**：需在 MIR 层识别 getter/setter 的 Call 与字段投影，仅对 **class 类型** 的读写建 Load/Store。
- **源码映射**：若有 debug 信息或 span，可把 ClassPAG 的边与源码位置绑定，输出时显示“对应源码行/表达式”，进一步屏蔽 MIR 细节。
- **语料更新**：若 rustdsl 新增语法或 API（如新的 cast/convert 方法），在本文档和 `rcpta_mir_statements.md` 中同步补充对应关系。

---

**文档版本**：与 `rcpta_implementation_log.md`、`rcpta_mir_statements.md` 配套；语料以 rustdsl 仓库为准。
