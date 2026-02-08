# rcpta 输出验证报告：entry_complex_call_chain_demo

基于源码 `main.rs` 第 190–221 行的数据流与调用关系，对 CG / PAG / PTS 做一致性检查。

## 1. 源码摘要与预期

| 源码 | 含义 | 预期 |
|-----|------|------|
| `item_a = Item::new(10)` | 分配 Item | 1 个 Item 对象 (obj_0) |
| `keyed = KeyedItem::new(1,100)` | 分配 KeyedItem | 1 个 KeyedItem 对象 (obj_1) |
| `holder_1/2 = Holder::new()` | 分配两个 Holder | 2 个 Holder 对象 (obj_2, obj_3) |
| `holder_1.set_item(item_a)` | 写入 holder_1.item | Store: holder_1.item ← item_a |
| `holder_2.set_item(keyed as Item)` | 写入 holder_2.item | Store: holder_2.item ← keyed |
| `e1 = holder_1.get_and_wrap()` | Load + Cast + get_id，返回 Entity | e1 指向 item_a (Entity 视图) → obj_0 |
| `e2 = holder_2.get_and_wrap()` | 同上 | e2 指向 keyed → obj_1 |
| `chained = e1.chain_with(e2)` | chain_with 内部调 with_partner，返回 other | chained = e2 → obj_1 |
| `out = chained.apply_twice(e1,e2)` | apply_twice 调 with_partner(a,b)=b | out = e2 → obj_1 |
| `loaded_item = e1.process_holder(holder_1)` | process_holder 内 Load h.item | loaded_item = holder_1.item = item_a → obj_0 |
| `out.describe()`, `out.get_id()` | 对 out 调 describe/get_id | 调用边存在即可 |

对象预期：4 个堆对象 — Item(obj_0), KeyedItem(obj_1), Holder(obj_2), Holder(obj_3)。

---

## 2. Class Call Graph (CG) 检查

**预期调用边：**

- entry → `Holder::get_and_wrap`, `Entity::chain_with`, `Entity::apply_twice`, `Entity::process_holder`, `Entity::describe`, `Entity::get_id`
- `Holder::get_and_wrap` → `Entity::get_id`（get_and_wrap 内部调 entity.get_id()）
- `Entity::chain_with` → `Entity::with_partner`
- `Entity::apply_twice` → `Entity::with_partner`

**class_cg.txt 实际：**

- entry → get_and_wrap, chain_with, apply_twice, process_holder, describe, get_id ✓
- Holder::get_and_wrap → Entity::get_id ✓
- Entity::chain_with → Entity::with_partner ✓
- Entity::apply_twice → Entity::with_partner ✓

**结论：CG 与源码调用关系一致。**

---

## 3. Class PAG 检查

**Alloc：**

- 4 个对象：obj_0 (Item), obj_1 (KeyedItem), obj_2 (Holder), obj_3 (Holder) ✓  
- 对应 entry 中 local_1→obj_0, local_2→obj_1, local_3→obj_2, local_4→obj_3 ✓

**Store：**

- `local_3.item <- local_1`（holder_1.set_item(item_a)）✓
- `local_4.item <- local_10`（holder_2.set_item(keyed as Item)，local_10 为 cast 结果）✓

**get_and_wrap：**

- Load `param_1.item -> local_2` ✓
- Cast `local_2 -> local_3` ✓
- Assign `local_3 -> ret` ✓
- CallArg 到 Entity::get_id ✓

**process_holder：**

- Load `param_2.item -> ret` ✓

**with_partner / chain_with / apply_twice：**

- with_partner: param_2 → ret ✓
- chain_with/apply_twice 的 CallArg/CallRet 到 with_partner ✓

**entry：**

- 对 get_and_wrap、chain_with、apply_twice、process_holder、describe、get_id 的 CallArg/CallRet 边均存在 ✓

**结论：PAG 的 Alloc/Store/Load/Cast/Assign/CallArg/CallRet 与源码数据流一致。**

---

## 4. Class PTS 检查

**对象与指针对应关系：**

| 指针 | 预期可能指向 | PTS 输出 | 判定 |
|------|--------------|----------|------|
| local_1 (item_a) | obj_0 | obj_0 | ✓ |
| local_2 (keyed) | obj_1 | obj_1 | ✓ |
| local_3 (holder_1) | obj_2 | obj_2 | ✓ |
| local_4 (holder_2) | obj_3 | obj_3 | ✓ |
| obj_2.item | obj_0 | obj_0 | ✓ |
| obj_3.item | obj_1 | obj_1 | ✓ |
| get_and_wrap::param_1 | obj_2 或 obj_3 | obj_2, obj_3 | ✓ |
| get_and_wrap::ret (e1/e2) | obj_0 或 obj_1 | obj_0, obj_1 | ✓（流不敏感合并） |
| process_holder::ret (loaded_item) | obj_0 | obj_0 | ✓ |
| process_holder::param_2 | holder → obj_2 | obj_2 | ✓ |
| local_27 (loaded_item) | obj_0 | obj_0 | ✓ |
| local_29 (holder_1 的 clone) | obj_2 | obj_2 | ✓ |

**describe::local_9 / get_id::ret 为 (none)：** 表示 i32 等非类类型，不指向类对象，符合预期 ✓

**结论：PTS 与 PAG 传播结果一致；流不敏感下多分支合并为 {obj_0, obj_1} 等为正确过近似。**

---

## 5. Materialized Store/Load（PAG 节）检查

- Store: local_1 → obj_2.item, local_10 → obj_3.item ✓
- Load: obj_2.item / obj_3.item → get_and_wrap::local_2；obj_2.item → process_holder::ret；obj_0/obj_1.entity_id → describe::local_9, get_id::ret ✓

与 PTS 和源码语义一致。

---

## 6. 总结

| 产物 | 结论 |
|------|------|
| **class_cg.txt** | 与源码调用关系一致，边完整（含 Holder::get_and_wrap → Entity::get_id）。 |
| **class_pag.txt** | 与源码数据流一致，Alloc/Store/Load/Cast/Assign/CallArg/CallRet 及 Materialized 边正确。 |
| **class_pts.txt** | 与 PAG 传播结果一致，对象与指针对应关系正确；describe/get_id 的标量结果为 (none) 符合预期。 |

**整体判断：在 flow-insensitive 类指针分析前提下，CG、PAG、PTS 三者与源码语义一致，未见错误。**
