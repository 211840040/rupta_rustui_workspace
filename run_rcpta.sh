#!/usr/bin/env bash
# rcpta: compile once, then analyze with any entry func (--analyze-only).
# You must build rcpta yourself (e.g. cd rupta && cargo build).
#
# Usage:
#   ./run_rcpta.sh compile <path_to_file>
#   ./run_rcpta.sh <path_to_file> <entry_func> <output_dir> [--analyze-only]
#
# Examples:
#   # One-time: compile the test program (all deps + target)
#   ./run_rcpta.sh compile rustdsl/classes/tests/animal_hierarchy/main.rs
#
#   # Analyze: run PTA with given entry, write class_pag to output_dir (does compile+analyze)
#   ./run_rcpta.sh rustdsl/classes/tests/animal_hierarchy/main.rs prop_multilevel_upcast_preserves_identity analysis_results/rcpta/out
#
#   # Analyze only: skip full compile, just recompile target crate with PTA (fast, use after "compile")
#   ./run_rcpta.sh rustdsl/classes/tests/animal_hierarchy/main.rs prop_multilevel_upcast_preserves_identity analysis_results/rcpta/out --analyze-only

set -e
set -o pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
MANIFEST="$SCRIPT_DIR/rustdsl/classes/Cargo.toml"
CLASSES_ROOT="$SCRIPT_DIR/rustdsl/classes"

# Subcommand: compile only
if [[ "${1:-}" == "compile" ]]; then
  FILE_PATH="${2:?Usage: $0 compile <path_to_file>}"
  ABS_FILE="$(cd "$SCRIPT_DIR" && realpath -e "$FILE_PATH")"
  [[ "$ABS_FILE" == "$CLASSES_ROOT"* ]] || { echo "Error: $FILE_PATH is not under $CLASSES_ROOT"; exit 1; }
  if [[ "$ABS_FILE" == "$CLASSES_ROOT/tests/"* ]]; then
    REL="${ABS_FILE#$CLASSES_ROOT/tests/}"
    TEST_BINARY="${REL%%/*}"
    echo "Compiling test $TEST_BINARY..."
    (unset RUSTC_WRAPPER; cargo test --no-run --test "$TEST_BINARY" --manifest-path "$MANIFEST" --quiet)
  else
    [[ "$ABS_FILE" == "$CLASSES_ROOT/src/"* ]] || { echo "Error: file must be under tests/ or src/"; exit 1; }
    echo "Compiling lib..."
    (unset RUSTC_WRAPPER; cargo check --lib --manifest-path "$MANIFEST" --quiet)
  fi
  echo "Done. You can now run with --analyze-only and different entry funcs."
  exit 0
fi

# Analyze: path_to_file, entry_func, output_dir [, --analyze-only]
FILE_PATH="${1:?Usage: $0 <path_to_file> <entry_func> <output_dir> [--analyze-only]}"
ENTRY_FUNC="${2:?Usage: $0 <path_to_file> <entry_func> <output_dir> [--analyze-only]}"
OUTPUT_DIR="${3:?Usage: $0 <path_to_file> <entry_func> <output_dir> [--analyze-only]}"
ANALYZE_ONLY=
[[ "${4:-}" == "--analyze-only" ]] && ANALYZE_ONLY=1

# Resolve output dir to absolute path so pta (running from cargo's cwd, often manifest dir) writes to the right place
ABS_OUTPUT_DIR="$(cd "$SCRIPT_DIR" && realpath -m "$OUTPUT_DIR")"

# Resolve file to absolute path; must be under rustdsl/classes (tests/ or src/)
ABS_FILE="$(cd "$SCRIPT_DIR" && realpath -e "$FILE_PATH")"
REQ_PREFIX="$CLASSES_ROOT"
[[ "$ABS_FILE" == "$REQ_PREFIX"* ]] || { echo "Error: $FILE_PATH is not under $CLASSES_ROOT"; exit 1; }

# Decide --test <name> or --lib from path
if [[ "$ABS_FILE" == "$CLASSES_ROOT/tests/"* ]]; then
  REL="${ABS_FILE#$CLASSES_ROOT/tests/}"
  TEST_BINARY="${REL%%/*}"
  USE_LIB=
else
  [[ "$ABS_FILE" == "$CLASSES_ROOT/src/"* ]] || { echo "Error: file must be under tests/ or src/"; exit 1; }
  USE_LIB=1
  TEST_BINARY=
fi

# rcpta binary
CARGO_PTA="${CARGO_PTA:-$SCRIPT_DIR/rupta/target/debug/cargo-pta}"
[[ -x "$CARGO_PTA" ]] || { echo "Error: rcpta not found at $CARGO_PTA (build with: cd rupta && cargo build)"; exit 1; }

mkdir -p "$ABS_OUTPUT_DIR"

echo "file:    $ABS_FILE"
echo "entry:   $ENTRY_FUNC"
echo "out:     $ABS_OUTPUT_DIR"
[[ -n "$ANALYZE_ONLY" ]] && echo "mode:    analyze-only (skip full compile)"
echo "----------------------------------------"

# Phase 1: full compile (skip if --analyze-only)
if [[ -z "$ANALYZE_ONLY" ]]; then
  echo "[1/2] Compiling..."
  if [[ -n "$USE_LIB" ]]; then
    (unset RUSTC_WRAPPER; cargo check --lib --manifest-path "$MANIFEST" --quiet)
  else
    (unset RUSTC_WRAPPER; cargo test --no-run --test "$TEST_BINARY" --manifest-path "$MANIFEST" --quiet)
  fi
  echo "[1/2] Done."
  echo ""
fi

# Phase 2: touch + cargo-pta (recompile target crate with PTA, write class_pag)
echo "[2/2] Analyzing (entry: $ENTRY_FUNC)..."
touch "$ABS_FILE"

# Avoid stack overflow when analyzing large crates (e.g. vehicle_hierarchy with deep type/call graphs).
# ulimit -s is in KiB; 1048576 = 1 GiB. All child processes (cargo, cargo-pta, pta) inherit this.
# Skip by setting RCPTA_SKIP_STACK_LIMIT=1 if you need the default limit.
if [[ -z "${RCPTA_SKIP_STACK_LIMIT:-}" ]]; then
  ulimit -s 1048576 2>/dev/null || ulimit -s 524288 2>/dev/null || ulimit -s 131072 2>/dev/null || true
fi
# Rust threads get stack from RUST_MIN_STACK (default 2 MiB); raise for deep recursion.
export RUST_MIN_STACK="${RUST_MIN_STACK:-67108864}"   # 64 MiB per thread

if [[ -n "$USE_LIB" ]]; then
  "$CARGO_PTA" pta \
    --manifest-path "$MANIFEST" \
    --lib \
    -- \
    --entry-func "$ENTRY_FUNC" \
    --pta-type cs \
    --context-depth 1 \
    --dump-class-pag "$ABS_OUTPUT_DIR/class_pag.txt" \
    --dump-class-pts "$ABS_OUTPUT_DIR/class_pts.txt" \
    --dump-class-call-graph "$ABS_OUTPUT_DIR/class_cg.txt" \
    --dump-mir "$ABS_OUTPUT_DIR/mir.txt" \
    2>&1 | tee "$ABS_OUTPUT_DIR/analysis.log"
else
  "$CARGO_PTA" pta \
    --manifest-path "$MANIFEST" \
    --test "$TEST_BINARY" \
    -- \
    --entry-func "$ENTRY_FUNC" \
    --pta-type cs \
    --context-depth 1 \
    --dump-class-pag "$ABS_OUTPUT_DIR/class_pag.txt" \
    --dump-class-pts "$ABS_OUTPUT_DIR/class_pts.txt" \
    --dump-class-call-graph "$ABS_OUTPUT_DIR/class_cg.txt" \
    --dump-mir "$ABS_OUTPUT_DIR/mir.txt" \
    2>&1 | tee "$ABS_OUTPUT_DIR/analysis.log"
fi
echo "[2/2] Done."
echo "----------------------------------------"
echo "class_pag:      $ABS_OUTPUT_DIR/class_pag.txt"
echo "class_pts:      $ABS_OUTPUT_DIR/class_pts.txt"
echo "class_cg:       $ABS_OUTPUT_DIR/class_cg.txt"
ls -la "$ABS_OUTPUT_DIR"
