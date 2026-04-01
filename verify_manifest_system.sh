#!/bin/sh
# verify_manifest_system.sh — Validates sprint manifest system integrity
# Checks structural correctness and cross-referential integrity of all manifest files.

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROADMAP="$SCRIPT_DIR/S1-003-000-ROADMAP.json"
PHASE1="$SCRIPT_DIR/S1-003-001-PHASE1.json"
PHASE2="$SCRIPT_DIR/S1-003-002-PHASE2.json"

PASS_COUNT=0
FAIL_COUNT=0
ERRORS=""

pass() {
    PASS_COUNT=$((PASS_COUNT + 1))
    printf "  PASS: %s\n" "$1"
}

fail() {
    FAIL_COUNT=$((FAIL_COUNT + 1))
    ERRORS="${ERRORS}  FAIL: $1\n"
    printf "  FAIL: %s\n" "$1"
}

# Determine JSON query method: jq or python3 fallback
if command -v jq >/dev/null 2>&1; then
    JSON_TOOL="jq"
elif command -v python3 >/dev/null 2>&1; then
    JSON_TOOL="python3"
else
    printf "ERROR: Neither jq nor python3 is available. Cannot parse JSON.\n" >&2
    exit 1
fi

# json_get FILE EXPRESSION
# For jq: EXPRESSION is a jq filter (e.g. '.manifest_id')
# For python3: EXPRESSION is a Python expression where 'd' is the parsed dict
json_get() {
    _file="$1"
    _jq_expr="$2"
    _py_expr="$3"
    if [ "$JSON_TOOL" = "jq" ]; then
        jq -r "$_jq_expr" "$_file" 2>/dev/null
    else
        python3 -c "
import json, sys
try:
    with open('$_file') as f:
        d = json.load(f)
    result = $_py_expr
    if result is None:
        print('null')
    elif isinstance(result, bool):
        print(str(result).lower())
    elif isinstance(result, list):
        print(json.dumps(result))
    else:
        print(result)
except (json.JSONDecodeError, KeyError, IndexError, TypeError) as e:
    print('ERROR:' + str(e), file=sys.stderr)
    sys.exit(1)
" 2>/dev/null
    fi
}

# --- Check file existence ---
printf "\n=== File Existence ===\n"
ALL_FILES_EXIST=true
for f in "$ROADMAP" "$PHASE1" "$PHASE2"; do
    if [ -f "$f" ]; then
        pass "$(basename "$f") exists"
    else
        fail "$(basename "$f") is missing"
        ALL_FILES_EXIST=false
    fi
done

if [ "$ALL_FILES_EXIST" = false ]; then
    printf "\nERROR: Required manifest files are missing. Cannot continue validation.\n" >&2
    exit 1
fi

# --- Validate JSON is parseable ---
printf "\n=== JSON Parse Check ===\n"
ALL_VALID_JSON=true
for f in "$ROADMAP" "$PHASE1" "$PHASE2"; do
    if [ "$JSON_TOOL" = "jq" ]; then
        if jq empty "$f" 2>/dev/null; then
            pass "$(basename "$f") is valid JSON"
        else
            fail "$(basename "$f") is malformed JSON"
            ALL_VALID_JSON=false
        fi
    else
        if python3 -c "import json; json.load(open('$f'))" 2>/dev/null; then
            pass "$(basename "$f") is valid JSON"
        else
            fail "$(basename "$f") is malformed JSON"
            ALL_VALID_JSON=false
        fi
    fi
done

if [ "$ALL_VALID_JSON" = false ]; then
    printf "\nERROR: Malformed JSON detected. Cannot continue validation.\n" >&2
    exit 1
fi

# --- Validate ROADMAP ---
printf "\n=== S1-003-000-ROADMAP.json ===\n"

val=$(json_get "$ROADMAP" '.manifest_id' "d['manifest_id']")
if [ "$val" = "S1-003-000" ]; then pass "manifest_id = S1-003-000"; else fail "manifest_id expected 'S1-003-000', got '$val'"; fi

val=$(json_get "$ROADMAP" '.sprint_id' "d['sprint_id']")
if [ "$val" = "S1-003" ]; then pass "sprint_id = S1-003"; else fail "sprint_id expected 'S1-003', got '$val'"; fi

val=$(json_get "$ROADMAP" '.title' "d['title']")
if [ -n "$val" ] && [ "$val" != "null" ]; then pass "title is non-empty"; else fail "title is empty or missing"; fi

val=$(json_get "$ROADMAP" '.version' "d['version']")
if [ "$val" = "1.0.0" ]; then pass "version = 1.0.0"; else fail "version expected '1.0.0', got '$val'"; fi

val=$(json_get "$ROADMAP" '.items | length' "len(d['items'])")
if [ "$val" = "2" ]; then pass "items count = 2"; else fail "items count expected 2, got '$val'"; fi

val=$(json_get "$ROADMAP" '.dependencies | length' "len(d['dependencies'])")
if [ "$val" = "1" ]; then pass "dependencies count = 1"; else fail "dependencies count expected 1, got '$val'"; fi

dep_from=$(json_get "$ROADMAP" '.dependencies[0].from' "d['dependencies'][0]['from']")
dep_to=$(json_get "$ROADMAP" '.dependencies[0].to' "d['dependencies'][0]['to']")
if [ "$dep_from" = "S1-003-002" ] && [ "$dep_to" = "S1-003-001" ]; then
    pass "dependency from=S1-003-002, to=S1-003-001"
else
    fail "dependency expected from=S1-003-002/to=S1-003-001, got from=$dep_from/to=$dep_to"
fi

# --- Validate PHASE1 ---
printf "\n=== S1-003-001-PHASE1.json ===\n"

val=$(json_get "$PHASE1" '.id' "d['id']")
if [ "$val" = "S1-003-001" ]; then pass "id = S1-003-001"; else fail "id expected 'S1-003-001', got '$val'"; fi

for field in title branch repo_url requirements; do
    val=$(json_get "$PHASE1" ".$field" "d['$field']")
    if [ -n "$val" ] && [ "$val" != "null" ]; then pass "$field is non-empty"; else fail "$field is empty or missing"; fi
done

val=$(json_get "$PHASE1" '.project_root' "d['project_root']")
if [ "$val" = "." ]; then pass "project_root = '.'"; else fail "project_root expected '.', got '$val'"; fi

val=$(json_get "$PHASE1" '.domain_id' "d['domain_id']")
if [ "$val" = "infrastructure" ]; then pass "domain_id = infrastructure"; else fail "domain_id expected 'infrastructure', got '$val'"; fi

# --- Validate PHASE2 ---
printf "\n=== S1-003-002-PHASE2.json ===\n"

val=$(json_get "$PHASE2" '.id' "d['id']")
if [ "$val" = "S1-003-002" ]; then pass "id = S1-003-002"; else fail "id expected 'S1-003-002', got '$val'"; fi

for field in title branch repo_url requirements; do
    val=$(json_get "$PHASE2" ".$field" "d['$field']")
    if [ -n "$val" ] && [ "$val" != "null" ]; then pass "$field is non-empty"; else fail "$field is empty or missing"; fi
done

val=$(json_get "$PHASE2" '.project_root' "d['project_root']")
if [ "$val" = "." ]; then pass "project_root = '.'"; else fail "project_root expected '.', got '$val'"; fi

val=$(json_get "$PHASE2" '.domain_id' "d['domain_id']")
if [ "$val" = "features" ]; then pass "domain_id = features"; else fail "domain_id expected 'features', got '$val'"; fi

# Check PHASE2 dependencies contains S1-003-001
if [ "$JSON_TOOL" = "jq" ]; then
    has_dep=$(jq -r '.dependencies | index("S1-003-001") != null' "$PHASE2" 2>/dev/null)
else
    has_dep=$(python3 -c "
import json
with open('$PHASE2') as f:
    d = json.load(f)
print('true' if 'S1-003-001' in d.get('dependencies', []) else 'false')
" 2>/dev/null)
fi
if [ "$has_dep" = "true" ]; then pass "dependencies contains S1-003-001"; else fail "dependencies does not contain S1-003-001"; fi

# --- Cross-referential integrity ---
printf "\n=== Cross-Referential Integrity ===\n"

# Check that every ROADMAP item id has a corresponding file
if [ "$JSON_TOOL" = "jq" ]; then
    item_ids=$(jq -r '.items[].id' "$ROADMAP" 2>/dev/null)
else
    item_ids=$(python3 -c "
import json
with open('$ROADMAP') as f:
    d = json.load(f)
for item in d['items']:
    print(item['id'])
" 2>/dev/null)
fi

for item_id in $item_ids; do
    matched_file=$(find "$SCRIPT_DIR" -maxdepth 1 -name "${item_id}-*.json" 2>/dev/null | head -1)
    if [ -n "$matched_file" ]; then
        pass "ROADMAP item $item_id has matching file $(basename "$matched_file")"
    else
        fail "ROADMAP item $item_id has no corresponding requirement file"
    fi
done

# Validate PHASE2 dependencies match ROADMAP dependency graph
# ROADMAP says S1-003-002 depends on S1-003-001, so PHASE2.dependencies must contain S1-003-001
if [ "$JSON_TOOL" = "jq" ]; then
    roadmap_dep_targets=$(jq -r '.dependencies[] | select(.from == "S1-003-002") | .to' "$ROADMAP" 2>/dev/null)
    phase2_deps=$(jq -r '.dependencies[]' "$PHASE2" 2>/dev/null)
else
    roadmap_dep_targets=$(python3 -c "
import json
with open('$ROADMAP') as f:
    d = json.load(f)
for dep in d['dependencies']:
    if dep['from'] == 'S1-003-002':
        print(dep['to'])
" 2>/dev/null)
    phase2_deps=$(python3 -c "
import json
with open('$PHASE2') as f:
    d = json.load(f)
for dep in d.get('dependencies', []):
    print(dep)
" 2>/dev/null)
fi

cross_ref_ok=true
for target in $roadmap_dep_targets; do
    found=false
    for pdep in $phase2_deps; do
        if [ "$pdep" = "$target" ]; then
            found=true
            break
        fi
    done
    if [ "$found" = true ]; then
        pass "PHASE2 dependency '$target' matches ROADMAP dependency graph"
    else
        fail "ROADMAP declares S1-003-002 depends on '$target' but PHASE2 does not list it"
        cross_ref_ok=false
    fi
done

# --- Summary ---
printf "\n=== Summary ===\n"
TOTAL=$((PASS_COUNT + FAIL_COUNT))
printf "  Total checks: %d\n" "$TOTAL"
printf "  Passed: %d\n" "$PASS_COUNT"
printf "  Failed: %d\n" "$FAIL_COUNT"

if [ "$FAIL_COUNT" -gt 0 ]; then
    printf "\nVERIFICATION FAILED — %d check(s) did not pass.\n" "$FAIL_COUNT"
    exit 1
fi

printf "\nVERIFICATION PASSED — All %d checks passed. Manifest system is valid.\n" "$PASS_COUNT"
exit 0
