#!/usr/bin/env bash
# Run rcpta for every entry function in each of the four test main.rs files.
# Output: analysis_results/rcpta/<suite>/<entry_func>/ (class_pag.txt, class_pts.txt, class_cg.txt, etc.)
#
# Usage:
#   ./run_rcpta_all_entries.sh [suite] [--analyze-only]
#
# Examples:
#   ./run_rcpta_all_entries.sh                    # all four suites
#   ./run_rcpta_all_entries.sh vehicle_hierarchy # only vehicle_hierarchy
#   ./run_rcpta_all_entries.sh vehicle_hierarchy --analyze-only
#   ./run_rcpta_all_entries.sh --analyze-only     # all suites, analyze-only
#
# Without --analyze-only: compile each test binary once, then run rcpta for each entry.
# With --analyze-only: skip compile; use when binaries are already built (faster for re-runs).

set -e
set -o pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RESULTS_BASE="$SCRIPT_DIR/analysis_results/rcpta"

# Optional: only run this suite (e.g. vehicle_hierarchy)
SUITE_FILTER=""
ANALYZE_ONLY=""
for arg in "$@"; do
  if [[ "$arg" == "--analyze-only" ]]; then
    ANALYZE_ONLY=1
  elif [[ -n "$arg" && "$arg" != "--analyze-only" ]]; then
    SUITE_FILTER="$arg"
  fi
done

# Paths relative to workspace (under rustdsl/classes/tests/)
FILES=(
  "rustdsl/classes/tests/animal_hierarchy/main.rs"
  "rustdsl/classes/tests/rcpta_full_hierarchy/main.rs"
  "rustdsl/classes/tests/shape_hierarchy/main.rs"
  "rustdsl/classes/tests/vehicle_hierarchy/main.rs"
)

# Extract suite name from path: .../tests/<suite>/main.rs
suite_from_path() {
  local path="$1"
  local rest="${path#*tests/}"
  echo "${rest%%/*}"
}

# List entry functions for rcpta_full_hierarchy: top-level "pub fn entry_*"
entries_rcpta_full_hierarchy() {
  grep -E '^pub fn entry_[a-z0-9_]+' "$1" | sed -n 's/^pub fn \([a-z0-9_]*\).*/\1/p'
}

# List entry functions for test binaries: every #[test] fn <name>
entries_tests() {
  grep -A1 '#\[test\]' "$1" | grep -oE 'fn [a-z_][a-z0-9_]*' | awk '{print $2}' | sort -u
}

mkdir -p "$RESULTS_BASE"

for MAIN_RS in "${FILES[@]}"; do
  ABS_MAIN="$SCRIPT_DIR/$MAIN_RS"
  [[ -f "$ABS_MAIN" ]] || { echo "Skip (not found): $MAIN_RS"; continue; }

  SUITE="$(suite_from_path "$MAIN_RS")"
  if [[ -n "$SUITE_FILTER" && "$SUITE" != "$SUITE_FILTER" ]]; then
    echo "Skip suite: $SUITE (filter: $SUITE_FILTER)"
    continue
  fi

  echo "=========================================="
  echo "Suite: $SUITE ($MAIN_RS)"
  echo "=========================================="

  if [[ "$SUITE" == "rcpta_full_hierarchy" ]]; then
    ENTRIES=($(entries_rcpta_full_hierarchy "$ABS_MAIN"))
  else
    ENTRIES=($(entries_tests "$ABS_MAIN"))
  fi

  if [[ ${#ENTRIES[@]} -eq 0 ]]; then
    echo "  No entry functions found."
    continue
  fi

  echo "  Entries (${#ENTRIES[@]}): ${ENTRIES[*]}"

  # Compile once per suite (unless --analyze-only)
  if [[ -z "$ANALYZE_ONLY" ]]; then
    echo "  [Compile] ..."
    "$SCRIPT_DIR/run_rcpta.sh" compile "$MAIN_RS" || { echo "  Compile failed."; exit 1; }
  fi

  for ENTRY in "${ENTRIES[@]}"; do
    OUT_DIR="$RESULTS_BASE/$SUITE/$ENTRY"
    echo "  [Analyze] entry=$ENTRY -> $OUT_DIR"
    # Use --analyze-only when we already compiled this suite (faster)
    "$SCRIPT_DIR/run_rcpta.sh" "$MAIN_RS" "$ENTRY" "$OUT_DIR" --analyze-only
  done
  echo ""
done

echo "Done. Results under $RESULTS_BASE"
