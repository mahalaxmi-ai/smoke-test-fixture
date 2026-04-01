#!/usr/bin/env bash
set -euo pipefail

MANIFEST="S1-002-000-CIRCULAR.json"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FILE="$SCRIPT_DIR/$MANIFEST"
PASS=0
FAIL=0

check() {
    local label="$1"
    local result="$2"
    if [ "$result" = "true" ]; then
        echo "PASS: $label"
        PASS=$((PASS + 1))
    else
        echo "FAIL: $label"
        FAIL=$((FAIL + 1))
    fi
}

# Check 1: Valid JSON
if python3 -m json.tool "$FILE" > /dev/null 2>&1; then
    check "Valid JSON syntax" "true"
else
    echo "FAIL: Valid JSON syntax - file is not valid JSON"
    FAIL=$((FAIL + 1))
    echo ""
    echo "RESULT: $FAIL check(s) failed. Exiting."
    exit 1
fi

# Run all remaining checks via Python for reliable JSON parsing
RESULT=$(python3 -c "
import json, sys

with open('$FILE') as f:
    data = json.load(f)

# Check 2: manifest_id
mid = data.get('manifest_id', '')
print('manifest_id=' + ('true' if mid == 'S1-002-000-CIRCULAR' else 'false'))

# Check 3: sprint_id
sid = data.get('sprint_id', '')
print('sprint_id=' + ('true' if sid == 'S1-002' else 'false'))

# Check 4: version
ver = data.get('version', '')
print('version=' + ('true' if ver == '1.0.0' else 'false'))

# Check 5: exactly three items with correct IDs
items = data.get('items', [])
item_ids = sorted([i.get('id', '') for i in items])
expected_ids = ['S1-002-001', 'S1-002-002', 'S1-002-003']
print('items=' + ('true' if item_ids == expected_ids else 'false'))

# Check 6: dependencies form the expected cycle
deps = data.get('dependencies', [])
edges = set()
for d in deps:
    edges.add((d.get('from', ''), d.get('to', '')))
expected_edges = {
    ('S1-002-001', 'S1-002-002'),
    ('S1-002-002', 'S1-002-003'),
    ('S1-002-003', 'S1-002-001'),
}
print('deps_match=' + ('true' if edges == expected_edges and len(deps) == 3 else 'false'))

# Check 7: cycle detection via DFS
graph = {}
for d in deps:
    src = d.get('from', '')
    dst = d.get('to', '')
    graph.setdefault(src, []).append(dst)

all_nodes = set(graph.keys())
for targets in graph.values():
    all_nodes.update(targets)

WHITE, GRAY, BLACK = 0, 1, 2
color = {n: WHITE for n in all_nodes}
cycle_found = False

def dfs(node):
    global cycle_found
    color[node] = GRAY
    for neighbor in graph.get(node, []):
        if color[neighbor] == GRAY:
            cycle_found = True
            return
        if color[neighbor] == WHITE:
            dfs(neighbor)
            if cycle_found:
                return
    color[node] = BLACK

for node in all_nodes:
    if color[node] == WHITE:
        dfs(node)
    if cycle_found:
        break

print('cycle_detected=' + ('true' if cycle_found else 'false'))
")

for line in $RESULT; do
    key="${line%%=*}"
    val="${line#*=}"
    case "$key" in
        manifest_id)    check "manifest_id equals S1-002-000-CIRCULAR" "$val" ;;
        sprint_id)      check "sprint_id equals S1-002" "$val" ;;
        version)        check "version equals 1.0.0" "$val" ;;
        items)          check "Exactly three items with IDs S1-002-001, S1-002-002, S1-002-003" "$val" ;;
        deps_match)     check "Dependencies array has exactly three edges forming expected cycle" "$val" ;;
        cycle_detected) check "Circular dependency cycle detected by DFS algorithm" "$val" ;;
    esac
done

echo ""
echo "RESULT: $PASS passed, $FAIL failed"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
exit 0
