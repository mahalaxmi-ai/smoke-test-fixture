#!/usr/bin/env bash
set -euo pipefail

MANIFEST="S1-001-000-ROADMAP.json"
FAIL=0

check() {
  local label="$1" result="$2"
  if [ "$result" = "PASS" ]; then
    echo "PASS: $label"
  else
    echo "FAIL: $label"
    FAIL=1
  fi
}

if ! python3 -c "import json, sys; json.load(open(sys.argv[1]))" "$MANIFEST" 2>/dev/null; then
  echo "FAIL: $MANIFEST is not valid JSON"
  exit 1
fi

result=$(python3 -c "
import json, re, sys

with open(sys.argv[1]) as f:
    m = json.load(f)

checks = []

# 1. manifest_id matches ^[A-Z0-9-]+$
mid = m.get('manifest_id', '')
checks.append(('manifest_id matches ^[A-Z0-9-]+\$', 'PASS' if re.fullmatch(r'[A-Z0-9-]+', mid) else 'FAIL'))

# 2. sprint_id matches ^S[0-9]+-[0-9]{3}$
sid = m.get('sprint_id', '')
checks.append(('sprint_id matches ^S[0-9]+-[0-9]{3}\$', 'PASS' if re.fullmatch(r'S[0-9]+-[0-9]{3}', sid) else 'FAIL'))

# 3. title is non-empty
title = m.get('title', '')
checks.append(('title is non-empty', 'PASS' if isinstance(title, str) and len(title) > 0 else 'FAIL'))

# 4. version is semantic X.Y.Z
ver = m.get('version', '')
checks.append(('version matches X.Y.Z', 'PASS' if re.fullmatch(r'[0-9]+\.[0-9]+\.[0-9]+', ver) else 'FAIL'))

# 5. items array has at least one valid item
items = m.get('items', [])
items_ok = False
if isinstance(items, list) and len(items) > 0:
    items_ok = all(
        re.fullmatch(r'S[0-9]+-[0-9]{3}-[0-9]{3}', item.get('id', '')) and
        isinstance(item.get('title', ''), str) and len(item.get('title', '')) > 0
        for item in items
    )
checks.append(('items array valid (>=1 item, ids and titles valid)', 'PASS' if items_ok else 'FAIL'))

# 6. dependencies is an array
deps = m.get('dependencies', None)
checks.append(('dependencies is an array', 'PASS' if isinstance(deps, list) else 'FAIL'))

for label, status in checks:
    print(f'{status}:{label}')
" "$MANIFEST")

while IFS= read -r line; do
  status="${line%%:*}"
  label="${line#*:}"
  check "$label" "$status"
done <<< "$result"

exit $FAIL
