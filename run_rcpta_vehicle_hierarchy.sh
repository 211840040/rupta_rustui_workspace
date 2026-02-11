#!/usr/bin/env bash
# Run rcpta for every entry (test) in vehicle_hierarchy. On stack overflow, skip that entry
# and continue with the rest. Output: analysis_results/rcpta/vehicle_hierarchy/<entry>/
#
# Usage:
#   ./run_rcpta_vehicle_hierarchy.sh [--analyze-only]
#
# With --analyze-only: skip compile; use when the test binary is already built.

set -o pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RESULTS_BASE="$SCRIPT_DIR/analysis_results/rcpta/vehicle_hierarchy"
MAIN_RS="rustdsl/classes/tests/vehicle_hierarchy/main.rs"
ABS_MAIN="$SCRIPT_DIR/$MAIN_RS"
SUITE="vehicle_hierarchy"

ANALYZE_ONLY=""
[[ "${1:-}" == "--analyze-only" ]] && ANALYZE_ONLY=1

# List entry functions: every #[test] fn <name>
entries_tests() {
  grep -A1 '#\[test\]' "$1" | grep -oE 'fn [a-z_][a-z0-9_]*' | awk '{print $2}' | sort -u
}

# Detect stack overflow in analysis log (exit 255 or log content)
is_stack_overflow() {
  local out_dir="$1"
  local code="${2:-0}"
  [[ "$code" -eq 255 ]] && return 0
  [[ -f "$out_dir/analysis.log" ]] && grep -q "stack overflow\|overflowed its stack" "$out_dir/analysis.log" 2>/dev/null && return 0
  return 1
}

[[ -f "$ABS_MAIN" ]] || { echo "Error: $MAIN_RS not found."; exit 1; }

ENTRIES=($(entries_tests "$ABS_MAIN"))
[[ ${#ENTRIES[@]} -eq 0 ]] && { echo "No entry functions found."; exit 1; }

echo "=========================================="
echo "Suite: $SUITE ($MAIN_RS)"
echo "Entries (${#ENTRIES[@]}): ${ENTRIES[*]}"
echo "=========================================="

mkdir -p "$RESULTS_BASE"

if [[ -z "$ANALYZE_ONLY" ]]; then
  echo "[Compile] ..."
  "$SCRIPT_DIR/run_rcpta.sh" compile "$MAIN_RS" || { echo "Compile failed."; exit 1; }
fi

SKIPPED=()
OK=()
FAILED=()

for ENTRY in "${ENTRIES[@]}"; do
  OUT_DIR="$RESULTS_BASE/$ENTRY"
  echo "  [Analyze] entry=$ENTRY -> $OUT_DIR"
  if "$SCRIPT_DIR/run_rcpta.sh" "$MAIN_RS" "$ENTRY" "$OUT_DIR" --analyze-only; then
    OK+=("$ENTRY")
  else
    EXIT=$?
    if is_stack_overflow "$OUT_DIR" "$EXIT"; then
      echo "  -> Skip (stack overflow), continuing with next entry."
      SKIPPED+=("$ENTRY")
    else
      echo "  -> Failed (exit $EXIT), not a stack overflow."
      FAILED+=("$ENTRY")
    fi
  fi
done

echo ""
echo "=========================================="
echo "Done. OK: ${#OK[@]}  Skipped (stack overflow): ${#SKIPPED[@]}  Failed: ${#FAILED[@]}"
[[ ${#SKIPPED[@]} -gt 0 ]] && echo "  Skipped: ${SKIPPED[*]}"
[[ ${#FAILED[@]} -gt 0 ]] && echo "  Failed:  ${FAILED[*]}"
echo "Results under $RESULTS_BASE"
echo "=========================================="

[[ ${#FAILED[@]} -gt 0 ]] && exit 1
exit 0
