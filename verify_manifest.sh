#!/usr/bin/env bash
set -euo pipefail

MANIFEST="S1-001-000-ROADMAP.json"
PASS_COUNT=0
FAIL_COUNT=0

check() {
  local label="$1"
  local result="$2"
  if [ "$result" = "true" ]; then
    echo "PASS: $label"
    PASS_COUNT=$((PASS_COUNT + 1))
  else
    echo "FAIL: $label"
    FAIL_COUNT=$((FAIL_COUNT + 1))
  fi
}

# Verify file exists and is valid JSON
if ! python3 -c "import json, sys; json.load(open(sys.argv[1]))" "$MANIFEST" 2>/dev/null; then
  echo "FAIL: $MANIFEST is not valid JSON"
  exit 1
fi

# Read values using python3 (more portable than requiring jq)
read_json() {
  python3 -c "
import json, sys, re

with open('$MANIFEST') as f:
    m = json.load(f)

field = sys.argv[1]

if field == 'manifest_id_pattern':
    print('true' if re.match(r'^[A-Z0-9-]+\$', m.get('manifest_id', '')) else 'false')
elif field == 'sprint_id_pattern':
    print('true' if re.match(r'^S[0-9]+-[0-9]{3}\$', m.get('sprint_id', '')) else 'false')
elif field == 'title_nonempty':
    print('true' if isinstance(m.get('title'), str) and len(m['title'].strip()) > 0 else 'false')
elif field == 'version_semver':
    print('true' if re.match(r'^[0-9]+\.[0-9]+\.[0-9]+\$', m.get('version', '')) else 'false')
elif field == 'items_nonempty':
    print('true' if isinstance(m.get('items'), list) and len(m['items']) > 0 else 'false')
elif field == 'item_ids_valid':
    items = m.get('items', [])
    print('true' if all(re.match(r'^S[0-9]+-[0-9]{3}-[0-9]{3}\$', it.get('id', '')) for it in items) else 'false')
elif field == 'item_titles_valid':
    items = m.get('items', [])
    print('true' if all(isinstance(it.get('title'), str) and len(it['title'].strip()) > 0 for it in items) else 'false')
elif field == 'dependencies_array':
    print('true' if isinstance(m.get('dependencies'), list) else 'false')
" "$1"
}

check "1. manifest_id matches ^[A-Z0-9-]+$" "$(read_json manifest_id_pattern)"
check "2. sprint_id matches ^S[0-9]+-[0-9]{3}$" "$(read_json sprint_id_pattern)"
check "3. title is non-empty" "$(read_json title_nonempty)"
check "4. version is semantic format X.Y.Z" "$(read_json version_semver)"
check "5. items array has at least one item" "$(read_json items_nonempty)"
check "6. Each item id matches ^S[0-9]+-[0-9]{3}-[0-9]{3}$" "$(read_json item_ids_valid)"
check "7. Each item has a non-empty title" "$(read_json item_titles_valid)"
check "8. dependencies is an array" "$(read_json dependencies_array)"

echo ""
echo "Results: $PASS_COUNT passed, $FAIL_COUNT failed"

if [ "$FAIL_COUNT" -gt 0 ]; then
  exit 1
fi

exit 0
