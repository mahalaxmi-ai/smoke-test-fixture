#!/usr/bin/env bash
set -euo pipefail

FILE="S1-002-000-CIRCULAR.json"

if [ ! -f "$FILE" ]; then
    echo "ERROR: $FILE not found" >&2
    exit 1
fi

if ! python3 -c "
import json, sys

try:
    with open('$FILE') as f:
        data = json.load(f)
except json.JSONDecodeError as e:
    print(f'ERROR: Malformed JSON: {e}', file=sys.stderr)
    sys.exit(1)

errors = []

# Check required fields
if data.get('manifest_id') != 'S1-002-000-CIRCULAR':
    errors.append(f\"manifest_id: expected 'S1-002-000-CIRCULAR', got '{data.get('manifest_id')}'\")
if data.get('sprint_id') != 'S1-002':
    errors.append(f\"sprint_id: expected 'S1-002', got '{data.get('sprint_id')}'\")
if data.get('version') != '1.0.0':
    errors.append(f\"version: expected '1.0.0', got '{data.get('version')}'\")
if not data.get('title'):
    errors.append('title is missing or empty')

# Check items
items = data.get('items', [])
expected_ids = {'S1-002-001', 'S1-002-002', 'S1-002-003'}
actual_ids = {item.get('id') for item in items}
if actual_ids != expected_ids:
    errors.append(f'items: expected IDs {sorted(expected_ids)}, got {sorted(actual_ids)}')
if len(items) != 3:
    errors.append(f'items: expected 3, got {len(items)}')

# Check dependencies and cycle detection
deps = data.get('dependencies', [])
expected_deps = [
    ('S1-002-001', 'S1-002-002'),
    ('S1-002-002', 'S1-002-003'),
    ('S1-002-003', 'S1-002-001'),
]
actual_deps = [(d.get('from'), d.get('to')) for d in deps]
if sorted(actual_deps) != sorted(expected_deps):
    errors.append(f'dependencies: expected {expected_deps}, got {actual_deps}')

# Cycle detection via DFS
graph = {}
for d in deps:
    graph.setdefault(d.get('from'), []).append(d.get('to'))

def has_cycle(graph):
    visited = set()
    stack = set()
    def dfs(node):
        visited.add(node)
        stack.add(node)
        for neighbor in graph.get(node, []):
            if neighbor in stack:
                return True
            if neighbor not in visited:
                if dfs(neighbor):
                    return True
        stack.discard(node)
        return False
    for node in graph:
        if node not in visited:
            if dfs(node):
                return True
    return False

if not has_cycle(graph):
    errors.append('No circular dependency cycle detected in dependency graph')

if errors:
    for e in errors:
        print(f'ERROR: {e}', file=sys.stderr)
    sys.exit(1)

print('All checks passed: fields valid, cycle detected')
"; then
    exit 1
fi
