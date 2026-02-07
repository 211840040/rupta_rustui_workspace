# rcpta 相关文档

## 规约：源码写法与 Class PAG 边的对应

- **rcpta_class_pag_spec.md** — rcpta Class PAG 规约：规定 Rust class 语法中**哪些写法必须建立哪种 Class PAG 边**（Alloc / Cast / Assign / Load / Store）。rcpta 按此规约实现可使输出**可解释、可验证**；可依据该规约和源码验证 rcpta 对某种边的构建是否正确。

---

## rcpta 分析结果目录与运行方式

## 如何以任意函数为入口跑 rcpta 并写入本目录

在**工作区根目录**（`rupta_rustdsl_workspace/`）执行：

```bash
./run_rcpta.sh <入口函数名> [测试二进制名] [结果目录名]
```

- **入口函数名**（必填）：Rust 里该函数的标识符，如 `prop_multilevel_upcast_preserves_identity`、`test_upcast_circle_to_shape`。
- **测试二进制名**（可选）：若入口在某个 integration test 里，填对应 **test 名**（即 `rustdsl/classes/tests/<name>/` 的 `<name>`）；不填则按 `--lib` 分析，入口为库中同名函数。
- **结果目录名**（可选）：指定后，结果写入 `analysis_results/rcpta/<结果目录名>/`；不填则自动为 `<测试二进制名>_<入口函数名>` 或 `<入口函数名>`。

所有结果统一落在 `analysis_results/rcpta/<结果目录>/` 下，包含：`class_pag.txt`（主结果）、`mir.txt`、`analysis.log`。

---

## 示例

```bash
# animal_hierarchy 测试里的 prop_multilevel_upcast_preserves_identity 为入口
# 结果目录默认: animal_hierarchy_prop_multilevel_upcast_preserves_identity
./run_rcpta.sh prop_multilevel_upcast_preserves_identity animal_hierarchy

# 同上，但结果目录指定为 animal_hierarchy（便于多个入口共用一个目录名）
./run_rcpta.sh prop_multilevel_upcast_preserves_identity animal_hierarchy animal_hierarchy

# shape_hierarchy 里的上转测试
./run_rcpta.sh test_upcast_circle_to_shape shape_hierarchy shape_upcast

# 库中某函数为入口（需在 classes 的 lib 里存在同名 fn）
./run_rcpta.sh my_entry
# 结果目录: my_entry
```

---

## 测试二进制与入口函数对应关系

| test 名（即 tests/ 下目录名） | 说明 | 含 alloc/cast 的示例入口函数 |
|------------------------------|------|-----------------------------|
| `animal_hierarchy`           | 动物层次（dog/cat/fish/bird/…） | `prop_multilevel_upcast_preserves_identity`, `test_downcast_animal_to_dog_success`, `test_eagle_multilevel_upcast`, `test_shark_multilevel_upcast` |
| `shape_hierarchy`            | 形状层次（circle/rectangle/triangle/…） | `test_upcast_circle_to_shape`, `test_upcast_rectangle_to_shape`, `prop_multilevel_polymorphic_consistency` |
| `vehicle_hierarchy`          | 车辆层次（car/motorcycle/…） | `test_drivable_interface_polymorphism`, `test_car_upcast_to_vehicle` |

入口函数必须是该 test 下实际存在的 `fn` 名（可从对应 `main.rs` 或子模块里查）。

---

## 批量分析多个入口函数

需要验证多个含 alloc/cast 的 fn 时，可循环调用脚本，并为每个入口指定一个结果目录：

```bash
# 示例：对 animal_hierarchy 下多个入口分别跑 rcpta，结果目录与入口一一对应
./run_rcpta.sh prop_multilevel_upcast_preserves_identity animal_hierarchy animal_multilevel_upcast
./run_rcpta.sh test_downcast_animal_to_dog_success animal_hierarchy animal_downcast_dog
./run_rcpta.sh test_eagle_multilevel_upcast animal_hierarchy animal_eagle_upcast
```

或使用默认结果目录（不传第三个参数）：

```bash
./run_rcpta.sh prop_multilevel_upcast_preserves_identity animal_hierarchy
./run_rcpta.sh test_upcast_circle_to_shape shape_hierarchy
```

每次运行都会在 `analysis_results/rcpta/` 下创建或覆盖对应子目录并写入 `class_pag.txt` 等文件。

---

## 输出文件说明

- **class_pag.txt** — 主结果：ClassPAG（ptrs, objs, assign/alloc/load/store/cast 等边），以 class_pag 为准。
- **mir.txt** — 可达函数 MIR
- **analysis.log** — 分析日志（含 tee 的终端输出）

---

## 注意

- 入口函数由 **item 名** 解析（`tcx.item_name`），若多个同名函数会取先找到的一个。
- 脚本会从 `rupta/` 调用 `cargo-pta`，manifest 默认为 `rustdsl/classes/Cargo.toml`。如需改 manifest 或 test，可修改 `run_rcpta.sh` 中的 `MANIFEST` 或增加参数。
- **若结果目录里只有 analysis.log、没有 class_pag.txt 等**：多半是 cargo 认为目标已是最新（"Fresh"）而未调用 rustc，PTA 未执行。当前脚本在分析 test 目标前会自动执行 `cargo clean --manifest-path ...`，强制重新编译以保证每次都会跑 PTA 并写出结果。若希望跳过 clean 以加快重复运行，可设置环境变量 `RCPTA_NO_CLEAN=1`（需自行保证目标已过期会重编）。
