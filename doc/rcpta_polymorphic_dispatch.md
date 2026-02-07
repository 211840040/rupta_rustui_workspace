# rcpta 多态调用解析：基于 Points-to 将 Method Call 解析为 Callee

本文档说明如何根据 **receiver 的 points-to 集合** 和 **class/interface/mixin 层次**，把一个 method call 解析成可能被调用的 callee 集合，用于建 CallArg/CallRet 边或做 on-the-fly 传播。

---

## 一、Tai-e 的做法（对照）

Tai-e 在 **solver** 里做基于 points-to 的解析（`DefaultSolver.java`）：

1. **时机**：当某个变量的 **points-to 集合** 发生变化时（`onNewPointsToSet`），若该变量是某条 **invoke** 的 receiver，则对该 invoke 做解析。
2. **输入**：receiver 变量 `recv`，以及其当前 points-to 集合 `pts`（若干抽象对象）。
3. **对每个对象**：`recvObj` ∈ pts：
   - 取对象的 **具体类型**：`type = recvObj.getObject().getType()`（即该 allocation site 对应的类）。
   - **Dispatch**：`callee = CallGraphs.resolveCallee(type, callSite)`。
   - 对 virtual/interface 调用，内部即：`hierarchy.dispatch(type, methodRef)` —— 根据 **具体类型** 和 **方法引用**（声明类 + 方法签名）查表得到实际被调用的方法。
4. **建边**：对得到的每个 `callee` 建 call edge，并把 **receiver 对象** 传给 callee 的 `this`（即 CallArg：actual receiver → formal this）。

因此：**多态解析 = receiver 的 pts 中每个对象的“具体类型” + 一次 dispatch(具体类型, 方法引用) → 得到该 call site 下可能的所有 callee**。

---

## 二、rcpta 中“根据 Points-to 解析成 callee”的具体步骤

### 2.1 前提与约定

- **Call site** 的静态信息：receiver 的 **静态类型**（如 `CRc<Animal>`）、被调用的 **方法名**（如 `make_sound`），以及（若需要）**声明处**（如 `Animal` 的虚方法 / 接口 `Drivable` 的 `drive` / mixin `Tagged` 的 `describe_tagged`）。
- **Points-to**：在 solver 或单独阶段，对每个 ClassPAG 中的 **指针** 维护 `pts(ptr_id) ⊆ { obj_id_1, obj_id_2, ... }`。
- **对象类型**：每个 `obj_id` 对应 ClassPAG 中的一个 **ClassObj**，其 `class_type` 即为该 allocation 的 **具体类**（如 `Dog`、`Car`）。

以下用 **“声明类 + 方法名”** 表示一次调用（class 继承、interface、mixin 统一用同一套 dispatch 表即可，见 2.3）。

### 2.2 算法：从 receiver 的 pts 到 callee 集合

**输入**：

- `receiver_ptr_id`：call site 的 receiver 在 ClassPAG 中的指针 id。
- `method_name`：被调用的方法名（如 `make_sound`、`drive`、`describe_tagged`）。
- `pts_map`：`ptr_id -> { obj_id }`（receiver 的 points-to 集合）。
- `class_pag`：含 `get_obj(obj_id) -> ClassObj`，从而得到每个对象的 `class_type`。
- `class_type_system`：含 `dispatch(concrete_type, method_name) -> Option<func_name>`（见下文 2.3）。

**输出**：可能被调用的 callee 的 **func_name** 集合（可再映射为 FuncId 用于建边）。

**步骤**：

1. 取 receiver 的 points-to：`obj_ids = pts_map.get(receiver_ptr_id).unwrap_or_default()`。
2. 对每个 `obj_id` ∈ obj_ids：
   - `obj = class_pag.get_obj(obj_id)`；若没有则跳过。
   - **具体类型**：`concrete_type = obj.class_type`（例如 `Dog`、`Car`）。
   - **Dispatch**：`callee_func_name = class_type_system.dispatch(concrete_type, method_name)`。
   - 若 `Some(name)`，把 `name` 加入 callee 集合。
3. 去重后返回的集合即为 **该 call site 在当前 pts 下解析出的所有 callee**。

要点：**每个可能指向的对象对应一个具体类型，每个具体类型通过 dispatch 得到至多一个实现方法**；合起来就是“基于 points-to 的多态 callee 集合”。

### 2.3 Dispatch 表：class / interface / mixin 统一

rcpta 用 **ClassTypeSystem** 维护：

- **继承**：每个类的 `parent`、`subclasses`（已有）。
- **方法实现表**：每个类上有 `method_impls: method_name -> func_name`，表示“该类实现的某个方法对应的具体函数全名”。

**Dispatch 逻辑**（与 Tai-e 的 `hierarchy.dispatch(type, methodRef)` 对应）：

- **输入**：`concrete_type`（如 `Dog`）、`method_name`（如 `make_sound`）。
- **查表**：先在该类的 `method_impls` 里查 `method_name`：
  - 若存在 → 返回该 `func_name`（即该类的实现）。
  - 若不存在 → 沿 `parent` 向上找，对父类递归做同样查找，直到找到或没有父类。
- **输出**：`Option<func_name>`。

这样：

- **Class 继承**：子类重写方法时，在子类上登记 `method_impls`，dispatch 会优先得到子类的实现。
- **Interface**：在 DSL 中，实现接口的类会有一个“实现接口方法”的函数（如 `Car::drive` 实现 `Drivable::drive`）。只要在 **该实现类** 上登记 `(class_name, method_name)` → 该函数的 `func_name`，dispatch 时用 receiver 的 **具体类型**（如 `Car`）查表即可得到 `Car::drive`，无需单独区分 interface。
- **Mixin**：同理，带 mixin 的类会有一个实现 mixin 方法的函数；在该类上登记 `(class_name, method_name)` → 对应 `func_name`，dispatch 时用 receiver 的 **具体类型** 即可。

即：**不区分 class/interface/mixin，统一用“具体类型 + 方法名”查 method_impls + 父链**。

### 2.4 何时注册 method_impl

- 在 **建 PAG** 时，每当 **识别到一次 class method 调用**（`identify_class_method`），就用当前 **静态解析到的 callee** 做一次登记：  
  `register_method_impl(callee_class_name, method_name, callee_func_name)`。  
  这样，所有“在程序中作为 callee 出现过的”实现都会进入 dispatch 表；之后用 pts 解析时，对任意 concrete_type，只要该类型在该 call 的 pts 中出现过，且该类型或其父类在表中登记过该 method_name，就能 resolve 到正确的 callee。

---

## 三、与建边的结合方式

两种常见方式：

1. **Build 时只建“可能 callee”的边（CHA 风格）**  
   用静态类型（或 CHA）先得到“可能的所有 callee”，对每个可能 callee 建 CallArg/CallRet；solver 照常沿这些边传播。精度略低，实现简单。

2. **Solve 时按 pts 动态解析再建边（Tai-e 风格）**  
   - 建 PAG 时，对 **虚调用** 不预先绑死 callee，只记录 call site（receiver_ptr_id, method_name 等）。
   - Solver 在传播时维护 `pts_map`；当 **receiver 的 pts** 发生变化时，用上面 2.2 的算法 **resolve 出当前 pts 下的 callee 集合**，对这些 callee 建 CallArg/CallRet（或等价地：把 actual → formal、formal_ret → actual_ret 的传播做一次），并把 **receiver 指向的每个对象** 作为 actual 传给各 callee 的 this。

当前 rcpta 已具备：

- **ClassPAG** 中的 Alloc/Assign/Cast/Load/Store 以及（可选）静态的 CallArg/CallRet；
- **ClassTypeSystem** 中的 `method_impls` 和 `dispatch(concrete_type, method_name)`；
- 建 PAG 时对 class method 调用的 **register_method_impl**。

要完成“基于 points-to 的多态解析”，需要在 **solver 或单独阶段**：

- 维护 **ptr_id → { obj_id }** 的 points-to；
- 在 **receiver 的 pts 变化** 时，对对应 call site 调用上述 2.2 的解析流程，得到 callee 集合；
- 对这些 callee 建 CallArg/CallRet 并做 propagation（或等价的一次性传递）。

---

## 四、小结

- **多态解析**：用 **receiver 的 points-to** 得到一组 **obj_id**，再对每个 obj 取 **class_type**，用 **dispatch(concrete_type, method_name)** 得到该 call site 下的所有 **callee**。
- **Dispatch**：由 ClassTypeSystem 的 **method_impls + parent 链** 实现，与 Tai-e 的 `hierarchy.dispatch(type, methodRef)` 对应；class/interface/mixin 统一用“具体类型 + 方法名”查表。
- **登记**：在识别到 class method 调用时 `register_method_impl`，保证 dispatch 表中有所有可能被调到的实现。

这样就把“根据 Points-to 把 method call 解析成 callee”的做法说清楚并落到 rcpta 的现有结构上了。
