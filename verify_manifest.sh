#!/usr/bin/env bash
set -euo pipefail

MANIFEST="S1-001-000-ROADMAP.json"
PASS=0
FAIL=0

check() {
  local num="$1" desc="$2" result="$3"
  if [ "$result" = "true" ]; then
    echo "CHECK $num PASS: $desc"
    PASS=$((PASS + 1))
  else
    echo "CHECK $num FAIL: $desc"
    FAIL=$((FAIL + 1))
  fi
}

if ! python3 -c "import json, sys; json.load(open(sys.argv[1]))" "$MANIFEST" 2>/dev/null; then
  echo "CHECK 9 FAIL: File is not valid JSON"
  exit 1
fi
check 9 "File is valid JSON with no syntax errors" "true"

MID=$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['manifest_id'])" "$MANIFEST")
SID=$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['sprint_id'])" "$MANIFEST")
TITLE=$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['title'])" "$MANIFEST")
VER=$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['version'])" "$MANIFEST")
ITEMS_COUNT=$(python3 -c "import json,sys; print(len(json.load(open(sys.argv[1]))['items']))" "$MANIFEST")
DEPS_IS_ARRAY=$(python3 -c "import json,sys; print(isinstance(json.load(open(sys.argv[1]))['dependencies'], list))" "$MANIFEST")

check 1 "manifest_id matches ^[A-Z0-9-]+\$" "$(echo "$MID" | grep -qE '^[A-Z0-9-]+$' && echo true || echo false)"
check 2 "sprint_id matches ^S[0-9]+-[0-9]{3}\$" "$(echo "$SID" | grep -qE '^S[0-9]+-[0-9]{3}$' && echo true || echo false)"
check 3 "title is non-empty" "$([ -n "$TITLE" ] && echo true || echo false)"
check 4 "version is semantic X.Y.Z" "$(echo "$VER" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$' && echo true || echo false)"
check 5 "items array has at least one item" "$([ "$ITEMS_COUNT" -ge 1 ] && echo true || echo false)"

ITEM_IDS_OK=$(python3 -c "
import json, sys, re
data = json.load(open(sys.argv[1]))
ok = all(re.match(r'^S[0-9]+-[0-9]{3}-[0-9]{3}\$', item['id']) for item in data['items'])
print(str(ok))
" "$MANIFEST")
check 6 "Each item id matches ^S[0-9]+-[0-9]{3}-[0-9]{3}\$" "$([ "$ITEM_IDS_OK" = "True" ] && echo true || echo false)"

ITEM_TITLES_OK=$(python3 -c "
import json, sys
data = json.load(open(sys.argv[1]))
ok = all(item.get('title', '') != '' for item in data['items'])
print(str(ok))
" "$MANIFEST")
check 7 "Each item has a non-empty title" "$([ "$ITEM_TITLES_OK" = "True" ] && echo true || echo false)"

check 8 "dependencies is an array" "$([ "$DEPS_IS_ARRAY" = "True" ] && echo true || echo false)"

echo ""
echo "Results: $PASS passed, $FAIL failed out of $((PASS + FAIL)) checks"

if [ "$FAIL" -gt 0 ]; then
  exit 1
fi
exit 0
