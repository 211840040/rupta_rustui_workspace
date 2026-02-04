# rcpta 需要处理的 MIR 语句种类

本文档整理：**对类指向信息流动有影响的 MIR 语句/终止符/右值**。rcpta 基于 MIR 做类层面指针分析，必须识别并处理这些 MIR 构造，才能正确构建 ClassPAG 并求解 ClassPTS。

参考：animal_hierarchy / shape_hierarchy / vehicle_hierarchy 三组 class DSL 程序；rupta 的 `fpag_builder.rs`、`special_function_handler.rs`。

---

## 一、总览

| 层次 | MIR 种类 | 对类指向流动的影响 | rcpta 动作 |
|------|----------|--------------------|------------|
| **Statement** | Assign | 左侧为类类型时，右侧写入决定流动 | 见 Rvalue 分支 |
| **Statement** | Intrinsic(CopyNonOverlapping) | 类指针的 *dst = *src | Assign 或 Load+Store |
| **Rvalue** (Assign 的右值) | Use(Copy/Move) | dst = src | **Assign** 边 |
| **Rvalue** | Use(Constant) | dst = 常量（类类型少见） | 可能 Alloc（若为静态/常量对象） |
| **Rvalue** | Ref | dst = &src | 借位，可视为 src → dst 的 **Assign**（同一对象） |
| **Rvalue** | Cast | 类型转换（如 Unsize 到父类） | **Assign**（operand → lh_path） |
| **Rvalue** | Aggregate(Adt) | 结构体/枚举构造（含 class wrapper） | 字段级 **Assign** 或包装构造 |
| **Rvalue** | ShallowInitBox | Box/Rc 等分配 | 可能对应 **Alloc** 或内部实现 |
| **Rvalue** | CopyForDeref | 为 deref 做的拷贝 | **Load** 语义（*base → dst） |
| **Terminator** | Call | 函数调用 | 见下：构造/Getter/Setter/普通方法 |

---

## 二、StatementKind：需处理的语句

### 1. `StatementKind::Assign(place, rvalue)`

**含义**：将 `rvalue` 写入 `place`。若 `place` 的类型为 class 类型（或含 class 类型字段），则对类指向流动有影响。

**rcpta**：仅当左侧类型为 DSL class 类型（或含 class 指针投影）时处理；根据 `rvalue` 分支决定建何种 ClassPAG 边（见第三节 Rvalue）。

---

### 2. `StatementKind::Intrinsic(NonDivergingIntrinsic::CopyNonOverlapping(copy_info))`

**含义**：`copy_nonoverlapping(src, dst, size)`，语义近似 `*dst = *src`。若 `src`/`dst` 解引用后为 class 类型，则产生类指针的拷贝。

**rcpta**：若识别为 class 类型的指针拷贝，可建 **Assign** 边：`*src` 对应的“类指针” → `*dst` 对应的“类指针”。或拆成 Load + Store（先 *src → 临时，再 临时 → *dst）；与 rupta 当前处理方式一致即可。

---

### 3. 其他 StatementKind（一般不建类指向边）

- `FakeRead` / `SetDiscriminant` / `Deinit` / `StorageLive` / `StorageDead` / `Retag` / `PlaceMention` / `AscribeUserType` / `Coverage` / `Nop` 等：不直接表示“类指针的赋值或流动”，rcpta 可不建 ClassPAG 边。
- 若后续发现某一种在展开后的 class DSL 中有特殊用法（如某种 Retag 与 class 引用相关），再单独考虑。

---

## 三、Rvalue（Assign 的右值）：需处理的分支

以下均在 **Assign(place, rvalue)** 中、且左侧类型与 class 相关时处理。

### 1. `Rvalue::Use(operand)`

- **Operand::Copy(place) / Move(place)**  
  **含义**：`dst = src`（拷贝或移动）。  
  **rcpta**：**Assign** 边：`src` 对应的 ClassPtr → `dst` 对应的 ClassPtr。

- **Operand::Constant(...)**  
  **含义**：`dst = 常量`。  
  **rcpta**：类类型常量少见；若有（如静态单例），可按 **Alloc** 或单独规则处理；通常可忽略。

---

### 2. `Rvalue::Ref(borrow_kind, _, place)`

**含义**：`dst = &src` 或 `&mut src`。  
**rcpta**：借位不改变“指向哪个类对象”，仅产生引用。可视为同一对象：**Assign** 边 `src` → `dst`（或仅做类型标记，不建边，视精度需求而定）。

---

### 3. `Rvalue::Cast(cast_kind, operand, ty)`

**含义**：类型转换，如子类→父类（Unsize）、函数指针等。  
**rcpta**：当 `operand` 与 `lh_path` 均为 class 类型（或一类一父类）时，**Assign** 边：operand 对应的 ClassPtr → lh_path 对应的 ClassPtr。典型：`into_superclass`、`downgrade` 等展开后的 Cast。

---

### 4. `Rvalue::Aggregate(AggregateKind::Adt(def, variant_idx, args, _, _), operands)`

**含义**：结构体/枚举构造，如 `Point { x, y }`、class wrapper 的 `From::from(inner)` 等。  
**rcpta**：  
- 若为 class 类型的“包装”构造（如 `Point<T,V> { inner: T, ... }`），则各字段有 **Assign**：对应 operand → 对应 field path；若某字段为 class 类型，建 ClassPAG 的 **Assign**。  
- 若为“从 inner 构造 wrapper”（如 `_from_inner(rc)` 展开成 Aggregate），则 inner → 整个 wrapper 的 **Assign**。

---

### 5. `Rvalue::ShallowInitBox(operand, ty)`

**含义**：`Box::new` / Rc 等分配的第一步（分配堆空间）。  
**rcpta**：class DSL 的 `CRc`/`RcDyn` 展开后可能涉及此类；若该 Box/Rc 对应“类对象”的分配，则需建 **Alloc**（ptr → obj）。通常需与后面的 Call（如 `Rc::new`）或 Aggregate 一起识别“这是类实例分配”。

---

### 6. `Rvalue::CopyForDeref(place)`

**含义**：为后续 deref 做的拷贝，语义上类似 `tmp = *place`。  
**rcpta**：当 `place` 为 class 引用类型时，可视为从“base 指向的对象”读到 tmp：**Load** 语义（(base, 逻辑字段) → dst）。若 DSL 展开后 getter 内出现 CopyForDeref，需与 getter 的 Call 一起考虑，避免重复建边。

---

### 7. 其他 Rvalue（类指向影响小或间接）

- `Repeat` / `Len` / `BinaryOp` / `UnaryOp` / `Discriminant` / `ThreadLocalRef`：一般不直接表示类指针流动。  
- `NullaryOp`：可能表示常量；类类型少见。  
- `RawPtr` / `WrapUnsafeBinder`：按需扩展；当前可先不建 ClassPAG 边。

---

## 四、TerminatorKind：需处理的终止符

### 1. `TerminatorKind::Call { func, args, destination, .. }`

**含义**：函数调用，返回值写入 `destination`（若有）。  
**rcpta**：根据被调函数识别为以下几类之一，并建对应 ClassPAG 边 + ClassCG 边。

#### 1.1 类构造器（ClassName::new / data::new）

- **识别**：`identify_class_constructor(&func_ref)` 为 Some。  
- **动作**：  
  - **Alloc**：创建 ClassObj，建边 `destination` 对应的 ClassPtr → 该 ClassObj。  
  - 若构造器内部有“从 data 到 wrapper”的赋值，由 Assign 或 Call（from）处理。

#### 1.2 Getter（get_<field>）

- **识别**：`identify_getter_setter` 为 Some，且为 getter。  
- **动作**：**Load** 边：(`args[0]`（base）, field_name) → `destination`。

#### 1.3 Setter（set_<field>）

- **识别**：`identify_getter_setter` 为 Some，且为 setter。  
- **动作**：**Store** 边：(`args[0]`（base）, field_name) ← `args[1]`（value）。

#### 1.4 其他类方法（含 override、mixin 方法）

- **识别**：`identify_class_method(&func_ref)` 为 Some（且非构造、非 getter/setter）。  
- **动作**：  
  - **CallArg**：对每个类类型的实参，actual → formal。  
  - **CallRet**：若返回值为类类型，formal_ret → actual（destination）。  
  - ClassCG：caller → callee（类方法 → 类方法 或 entry → 类方法）。

#### 1.5 类型转换类调用（from / into_superclass / downgrade 等）

- **识别**：通过函数名或 def_path 识别（如 `from`、`into_superclass`、`downgrade_from`）。  
- **动作**：返回值仍指向同一对象，可建 **Assign** 边：被转换的 arg 或 self → destination。

#### 1.6 其他 Call（drop、vtable、Rc 内部等）

- **动作**：不建 ClassPAG 边；若为类类型的参数/返回值，可只做类型标记或与 1.4 统一按 CallArg/CallRet 处理（视实现简单度而定）。

---

### 2. 其他 TerminatorKind

- `Goto` / `SwitchInt` / `Resume` / `Abort` / `Return` / `Unreachable` / `Drop` / `Assert` / `Yield` / `GeneratorDrop` / `InlineAsm` 等：不直接表示“类指针的赋值或调用”，rcpta 不为其建 ClassPAG 边。

---

## 五、与三个 hierarchy 的对应关系（简要）

| 场景 | 典型 MIR | rcpta 处理 |
|------|----------|------------|
| 继承 (extends) | Cast(Unsize)、into_superclass 的 Call | Assign / CallRet |
| Mixin (with) | 多 impl、方法调用 | CallArg / CallRet、ClassCG |
| 抽象方法 (abstract) | 动态分发或单实现调用 | Call |
| Getter/Setter | get_* / set_* 的 Call | Load / Store |
| 构造 (new) | Call(new) + 可能 Aggregate/ShallowInitBox | Alloc |
| 字段为类类型 (late point: CRc<Point>) | set_point / get_point 的 Call | Store / Load |
| 方法返回类类型 | CallRet | CallRet |
| 多态 (Vec<CRc<Drivable>> 等) | Call 时 actual → formal | CallArg |

---

## 六、汇总：rcpta 必须处理的 MIR 种类列表

**StatementKind**

1. **Assign**（且左侧与 class 相关）— 再按 Rvalue 分支处理。  
2. **Intrinsic(CopyNonOverlapping)** — 可选；class 指针拷贝时建 Assign 或 Load+Store。

**Rvalue（均在 Assign 下）**

1. **Use(Copy/Move)** → Assign。  
2. **Use(Constant)** → 按需（少见）。  
3. **Ref** → Assign（或仅类型）。  
4. **Cast** → Assign。  
5. **Aggregate(Adt)** → 字段级 Assign / 包装构造 Assign。  
6. **ShallowInitBox** → 与 Alloc 配合（若为该类对象分配）。  
7. **CopyForDeref** → Load 语义。

**TerminatorKind**

1. **Call** → 按被调函数：Alloc / Load / Store / CallArg+CallRet / Assign（转换）/ ClassCG。

其他 Statement / Rvalue / Terminator 种类：当前可不建 ClassPAG 边，若在三组 hierarchy 的展开 MIR 中发现新的“类指向流动”模式，再补充到本文档与实现中。
