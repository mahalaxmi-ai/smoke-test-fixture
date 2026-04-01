#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ERRORS=0

err() {
    echo "ERROR: $1" >&2
    ERRORS=$((ERRORS + 1))
}

ok() {
    echo "OK: $1"
}

# Determine JSON query tool: prefer jq, fall back to python3
if command -v jq >/dev/null 2>&1; then
    USE_JQ=1
else
    if command -v python3 >/dev/null 2>&1; then
        USE_JQ=0
    else
        echo "FATAL: Neither jq nor python3 is available" >&2
        exit 1
    fi
fi

# Helper: extract a value from JSON using a jq-style path
json_get() {
    local file="$1" query="$2"
    if [ "$USE_JQ" -eq 1 ]; then
        jq -r "$query" "$file" 2>/dev/null
    else
        python3 -c "
import json, sys
with open('$file') as f:
    data = json.load(f)
path = '''$query'''
# Simple jq path translation
if path == '.':
    print(json.dumps(data))
elif '|' in path:
    parts = path.split('|')
    obj = data
    for p in parts:
        p = p.strip()
        if p == 'length':
            obj = len(obj)
        elif p.startswith('.'):
            for key in p.lstrip('.').split('.'):
                if key.endswith(']'):
                    k, idx = key.split('[')
                    idx = int(idx.rstrip(']'))
                    if k:
                        obj = obj[k]
                    obj = obj[idx]
                else:
                    obj = obj[key]
    print(obj)
elif path.startswith('.'):
    obj = data
    for key in path.lstrip('.').split('.'):
        if key.endswith(']'):
            k, idx = key.split('[')
            idx = int(idx.rstrip(']'))
            if k:
                obj = obj[k]
            obj = obj[idx]
        else:
            obj = obj[key]
    if isinstance(obj, (dict, list)):
        print(json.dumps(obj))
    else:
        print(obj)
" 2>/dev/null
    fi
}

# Helper: validate JSON syntax
validate_json() {
    local file="$1"
    if [ ! -f "$file" ]; then
        err "File not found: $file"
        return 1
    fi
    if [ "$USE_JQ" -eq 1 ]; then
        if jq empty "$file" 2>/dev/null; then
            ok "Valid JSON: $file"
            return 0
        else
            err "Invalid JSON: $file"
            return 1
        fi
    else
        if python3 -c "import json; json.load(open('$file'))" 2>/dev/null; then
            ok "Valid JSON: $file"
            return 0
        else
            err "Invalid JSON: $file"
            return 1
        fi
    fi
}

# Helper: assert a field equals expected value
assert_eq() {
    local file="$1" path="$2" expected="$3" label="$4"
    local actual
    actual="$(json_get "$file" "$path")"
    if [ "$actual" = "$expected" ]; then
        ok "$label = '$actual'"
    else
        err "$label: expected '$expected', got '$actual'"
    fi
}

# Helper: assert a field is a non-empty string
assert_nonempty() {
    local file="$1" path="$2" label="$3"
    local actual
    actual="$(json_get "$file" "$path")"
    if [ -n "$actual" ] && [ "$actual" != "null" ]; then
        ok "$label is non-empty"
    else
        err "$label is empty or null"
    fi
}

ROADMAP="$SCRIPT_DIR/S1-003-000-ROADMAP.json"
PHASE1="$SCRIPT_DIR/S1-003-001-PHASE1.json"
PHASE2="$SCRIPT_DIR/S1-003-002-PHASE2.json"

echo "=== Step 1: Validate JSON syntax ==="
validate_json "$ROADMAP"
validate_json "$PHASE1"
validate_json "$PHASE2"

echo ""
echo "=== Step 2: Validate ROADMAP structure ==="
assert_eq "$ROADMAP" ".manifest_id" "S1-003-000" "roadmap manifest_id"
assert_eq "$ROADMAP" ".sprint_id" "S1-003" "roadmap sprint_id"
assert_nonempty "$ROADMAP" ".title" "roadmap title"
assert_eq "$ROADMAP" ".version" "1.0.0" "roadmap version"

ITEMS_LEN="$(json_get "$ROADMAP" ".items | length")"
if [ "$ITEMS_LEN" -eq 2 ]; then
    ok "roadmap items count = 2"
else
    err "roadmap items count: expected 2, got $ITEMS_LEN"
fi

ITEM0_ID="$(json_get "$ROADMAP" ".items[0].id")"
ITEM1_ID="$(json_get "$ROADMAP" ".items[1].id")"
if [ "$ITEM0_ID" = "S1-003-001" ] && [ "$ITEM1_ID" = "S1-003-002" ]; then
    ok "roadmap item ids are S1-003-001 and S1-003-002"
else
    err "roadmap item ids: expected S1-003-001,S1-003-002 got $ITEM0_ID,$ITEM1_ID"
fi

DEPS_LEN="$(json_get "$ROADMAP" ".dependencies | length")"
if [ "$DEPS_LEN" -eq 1 ]; then
    ok "roadmap dependencies count = 1"
else
    err "roadmap dependencies count: expected 1, got $DEPS_LEN"
fi

DEP_FROM="$(json_get "$ROADMAP" ".dependencies[0].from")"
DEP_TO="$(json_get "$ROADMAP" ".dependencies[0].to")"
if [ "$DEP_FROM" = "S1-003-002" ] && [ "$DEP_TO" = "S1-003-001" ]; then
    ok "roadmap dependency: S1-003-002 -> S1-003-001"
else
    err "roadmap dependency: expected from=S1-003-002,to=S1-003-001 got from=$DEP_FROM,to=$DEP_TO"
fi

echo ""
echo "=== Step 3: Validate PHASE1 structure ==="
assert_eq "$PHASE1" ".id" "S1-003-001" "phase1 id"
assert_nonempty "$PHASE1" ".title" "phase1 title"
assert_nonempty "$PHASE1" ".branch" "phase1 branch"
assert_nonempty "$PHASE1" ".repo_url" "phase1 repo_url"
assert_nonempty "$PHASE1" ".requirements" "phase1 requirements"
assert_eq "$PHASE1" ".project_root" "." "phase1 project_root"
assert_nonempty "$PHASE1" ".domain_id" "phase1 domain_id"

# Phase 1 should NOT have dependencies (it's the root phase)
P1_DEPS="$(json_get "$PHASE1" ".dependencies" 2>/dev/null || echo "null")"
if [ "$P1_DEPS" = "null" ] || [ -z "$P1_DEPS" ]; then
    ok "phase1 has no dependencies (root phase)"
else
    err "phase1 should not have dependencies, got: $P1_DEPS"
fi

echo ""
echo "=== Step 4: Validate PHASE2 structure ==="
assert_eq "$PHASE2" ".id" "S1-003-002" "phase2 id"
assert_nonempty "$PHASE2" ".title" "phase2 title"
assert_nonempty "$PHASE2" ".branch" "phase2 branch"
assert_nonempty "$PHASE2" ".repo_url" "phase2 repo_url"
assert_nonempty "$PHASE2" ".requirements" "phase2 requirements"
assert_eq "$PHASE2" ".project_root" "." "phase2 project_root"
assert_nonempty "$PHASE2" ".domain_id" "phase2 domain_id"

P2_DEP0="$(json_get "$PHASE2" ".dependencies[0]")"
if [ "$P2_DEP0" = "S1-003-001" ]; then
    ok "phase2 depends on S1-003-001"
else
    err "phase2 dependencies[0]: expected S1-003-001, got $P2_DEP0"
fi

echo ""
echo "=== Step 5: Cross-referential integrity ==="

# Every item in roadmap must have a corresponding file
for item_id in "$ITEM0_ID" "$ITEM1_ID"; do
    case "$item_id" in
        S1-003-001)
            file_id="$(json_get "$PHASE1" ".id")"
            ;;
        S1-003-002)
            file_id="$(json_get "$PHASE2" ".id")"
            ;;
        *)
            err "Unknown roadmap item id: $item_id"
            continue
            ;;
    esac
    if [ "$file_id" = "$item_id" ]; then
        ok "Roadmap item $item_id matches individual file id"
    else
        err "Roadmap item $item_id does not match individual file id ($file_id)"
    fi
done

# Roadmap dependency (S1-003-002 -> S1-003-001) must be reflected in Phase 2's dependencies
if [ "$DEP_FROM" = "S1-003-002" ] && [ "$DEP_TO" = "S1-003-001" ] && [ "$P2_DEP0" = "S1-003-001" ]; then
    ok "Cross-ref: roadmap dependency consistent with phase2 dependencies"
else
    err "Cross-ref: roadmap dependency inconsistent with individual file declarations"
fi

echo ""
echo "================================"
if [ "$ERRORS" -eq 0 ]; then
    echo "ALL CHECKS PASSED"
    exit 0
else
    echo "FAILED: $ERRORS error(s) found"
    exit 1
fi
