#!/usr/bin/env bash
set -euo pipefail

PASS=0
FAIL=0
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

check() {
    local desc="$1"
    local result="$2"
    if [ "$result" = "true" ]; then
        echo "PASS: $desc"
        PASS=$((PASS + 1))
    else
        echo "FAIL: $desc"
        FAIL=$((FAIL + 1))
    fi
}

json_field() {
    python3 -c "
import json, sys
with open('$1') as f:
    data = json.load(f)
keys = '$2'.split('.')
val = data
for k in keys:
    if isinstance(val, dict) and k in val:
        val = val[k]
    else:
        sys.exit(1)
print(json.dumps(val) if isinstance(val, (dict, list)) else val)
" 2>/dev/null
}

json_array_len() {
    python3 -c "
import json
with open('$1') as f:
    data = json.load(f)
keys = '$2'.split('.')
val = data
for k in keys:
    val = val[k]
if not isinstance(val, list):
    print(-1)
else:
    print(len(val))
" 2>/dev/null
}

json_array_contains() {
    python3 -c "
import json
with open('$1') as f:
    data = json.load(f)
keys = '$2'.split('.')
val = data
for k in keys:
    val = val[k]
print('true' if '$3' in val else 'false')
" 2>/dev/null
}

ROADMAP="$SCRIPT_DIR/S1-003-000-ROADMAP.json"
PHASE1="$SCRIPT_DIR/S1-003-001-PHASE1.json"
PHASE2="$SCRIPT_DIR/S1-003-002-PHASE2.json"

# Check files exist
for f in "$ROADMAP" "$PHASE1" "$PHASE2"; do
    if [ ! -f "$f" ]; then
        echo "FAIL: File not found: $f"
        exit 1
    fi
done

# Validate JSON syntax
for f in "$ROADMAP" "$PHASE1" "$PHASE2"; do
    if ! python3 -c "import json; json.load(open('$f'))" 2>/dev/null; then
        echo "FAIL: Invalid JSON: $f"
        exit 1
    fi
done

# --- ROADMAP checks ---
check "ROADMAP manifest_id exists" "$([ -n "$(json_field "$ROADMAP" manifest_id)" ] && echo true || echo false)"
check "ROADMAP sprint_id is S1-003" "$([ "$(json_field "$ROADMAP" sprint_id)" = "S1-003" ] && echo true || echo false)"
check "ROADMAP title exists" "$([ -n "$(json_field "$ROADMAP" title)" ] && echo true || echo false)"
check "ROADMAP version is 1.0.0" "$([ "$(json_field "$ROADMAP" version)" = "1.0.0" ] && echo true || echo false)"
check "ROADMAP has exactly 2 items" "$([ "$(json_array_len "$ROADMAP" items)" = "2" ] && echo true || echo false)"
check "ROADMAP has exactly 1 dependency" "$([ "$(json_array_len "$ROADMAP" dependencies)" = "1" ] && echo true || echo false)"

# Check dependency direction: from S1-003-002 to S1-003-001
DEP_FROM=$(python3 -c "
import json
with open('$ROADMAP') as f:
    data = json.load(f)
print(data['dependencies'][0]['from'])
" 2>/dev/null)
DEP_TO=$(python3 -c "
import json
with open('$ROADMAP') as f:
    data = json.load(f)
print(data['dependencies'][0]['to'])
" 2>/dev/null)
check "ROADMAP dependency from S1-003-002 to S1-003-001" "$([ "$DEP_FROM" = "S1-003-002" ] && [ "$DEP_TO" = "S1-003-001" ] && echo true || echo false)"

# --- PHASE1 checks ---
check "PHASE1 id is S1-003-001" "$([ "$(json_field "$PHASE1" id)" = "S1-003-001" ] && echo true || echo false)"
check "PHASE1 title exists" "$([ -n "$(json_field "$PHASE1" title)" ] && echo true || echo false)"
check "PHASE1 branch exists" "$([ -n "$(json_field "$PHASE1" branch)" ] && echo true || echo false)"
check "PHASE1 repo_url exists" "$([ -n "$(json_field "$PHASE1" repo_url)" ] && echo true || echo false)"
check "PHASE1 requirements exists" "$([ -n "$(json_field "$PHASE1" requirements)" ] && echo true || echo false)"
check "PHASE1 project_root exists" "$([ -n "$(json_field "$PHASE1" project_root)" ] && echo true || echo false)"
check "PHASE1 domain_id exists" "$([ -n "$(json_field "$PHASE1" domain_id)" ] && echo true || echo false)"

# --- PHASE2 checks ---
check "PHASE2 id is S1-003-002" "$([ "$(json_field "$PHASE2" id)" = "S1-003-002" ] && echo true || echo false)"
check "PHASE2 title exists" "$([ -n "$(json_field "$PHASE2" title)" ] && echo true || echo false)"
check "PHASE2 branch exists" "$([ -n "$(json_field "$PHASE2" branch)" ] && echo true || echo false)"
check "PHASE2 repo_url exists" "$([ -n "$(json_field "$PHASE2" repo_url)" ] && echo true || echo false)"
check "PHASE2 requirements exists" "$([ -n "$(json_field "$PHASE2" requirements)" ] && echo true || echo false)"
check "PHASE2 project_root exists" "$([ -n "$(json_field "$PHASE2" project_root)" ] && echo true || echo false)"
check "PHASE2 domain_id exists" "$([ -n "$(json_field "$PHASE2" domain_id)" ] && echo true || echo false)"
check "PHASE2 dependencies contains S1-003-001" "$(json_array_contains "$PHASE2" dependencies "S1-003-001")"

# --- Acyclic dependency check ---
# Phase 2 depends on Phase 1. Verify Phase 1 has no dependency on Phase 2.
PHASE1_HAS_DEPS=$(python3 -c "
import json
with open('$PHASE1') as f:
    data = json.load(f)
deps = data.get('dependencies', [])
print('true' if 'S1-003-002' in deps else 'false')
" 2>/dev/null)
check "Dependency graph is acyclic (Phase 1 does not depend on Phase 2)" "$([ "$PHASE1_HAS_DEPS" = "false" ] && echo true || echo false)"

# --- Summary ---
echo ""
echo "Results: $PASS passed, $FAIL failed"
if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
echo "All manifest verification checks passed."
exit 0
