#!/usr/bin/env bash
set -euo pipefail

API_URL="${API_URL:-http://localhost:3000}"
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'
PASS=0
FAIL=0

pass() { echo -e "  ${GREEN}PASS${NC} $1"; ((PASS++)); }
fail() { echo -e "  ${RED}FAIL${NC} $1 — $2"; ((FAIL++)); }
section() { echo -e "\n${YELLOW}▸ $1${NC}"; }

wait_for_api() {
    echo -e "${YELLOW}Waiting for API at $API_URL ...${NC}"
    for i in $(seq 1 90); do
        if curl -sf "$API_URL/health" >/dev/null 2>&1; then
            echo -e "${GREEN}API is ready!${NC}\n"
            return
        fi
        sleep 1
    done
    echo -e "${RED}API did not respond after 90s${NC}"
    exit 1
}

assert_status() {
    local expected=$1 actual=$2 label=$3
    if [ "$actual" = "$expected" ]; then
        pass "$label (HTTP $actual)"
    else
        fail "$label" "expected HTTP $expected, got $actual"
    fi
}

assert_json_key() {
    local json=$1 key=$2 label=$3
    if echo "$json" | grep -q "\"$key\""; then
        pass "$label (has key '$key')"
    else
        fail "$label" "missing key '$key' in: $json"
    fi
}

assert_json_value() {
    local json=$1 key=$2 expected=$3 label=$4
    local actual
    actual=$(echo "$json" | grep -o "\"$key\":[^,}]*" | head -1 | sed 's/.*://;s/"//g;s/ //g')
    if [ "$actual" = "$expected" ]; then
        pass "$label ($key=$actual)"
    else
        fail "$label" "expected $key=$expected, got $key=$actual"
    fi
}

wait_for_api

# ──────────────────────────────────────────────
# 0. GET /test (verify routing works)
# ──────────────────────────────────────────────
section "GET /test (routing check)"
resp=$(curl -s -w "\n%{http_code}" "$API_URL/test")
status=$(echo "$resp" | tail -1)
body=$(echo "$resp" | sed '$d')
echo -e "  Response: $body"
assert_status 200 "$status" "GET /test"

# ──────────────────────────────────────────────
# 1. GET /user/1
# ──────────────────────────────────────────────
section "GET /user/1"
resp=$(curl -s -w "\n%{http_code}" "$API_URL/user/1")
status=$(echo "$resp" | tail -1)
body=$(echo "$resp" | sed '$d')
echo -e "  Response: $body"
assert_status 200 "$status" "GET /user/1"
assert_json_key "$body" "user_id" "response has user_id"
assert_json_key "$body" "name" "response has name"
assert_json_key "$body" "email" "response has email"
assert_json_value "$body" "user_id" "1" "user_id equals 1"

# ──────────────────────────────────────────────
# 2. GET /user/999 (not found)
# ──────────────────────────────────────────────
section "GET /user/999 (not found)"
status=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL/user/999")
assert_status 404 "$status" "GET /user/999 returns 404"

# ──────────────────────────────────────────────
# 3. GET /user/1/task
# ──────────────────────────────────────────────
section "GET /user/1/task"
resp=$(curl -s -w "\n%{http_code}" "$API_URL/user/1/task")
status=$(echo "$resp" | tail -1)
body=$(echo "$resp" | sed '$d')
assert_status 200 "$status" "GET /user/1/task"
assert_json_key "$body" "user_id" "response has user_id"
assert_json_key "$body" "tasks" "response has tasks"

# ──────────────────────────────────────────────
# 4. POST /task (create)
# ──────────────────────────────────────────────
section "POST /task (create)"
resp=$(curl -s -w "\n%{http_code}" \
    -X POST "$API_URL/task" \
    -d "title=Test task&description=Created by test script&status=1")
status=$(echo "$resp" | tail -1)
body=$(echo "$resp" | sed '$d')
assert_status 201 "$status" "POST /task returns 201"
assert_json_key "$body" "task_id" "response has task_id"
assert_json_key "$body" "title" "response has title"
assert_json_value "$body" "status" "1" "status equals 1"

TASK_ID=$(echo "$body" | grep -o '"task_id":[0-9]*' | head -1 | sed 's/.*://')
echo -e "  ${YELLOW}Created task_id=$TASK_ID${NC}"

# ──────────────────────────────────────────────
# 5. POST /task/{id} (update)
# ──────────────────────────────────────────────
section "POST /task/$TASK_ID (update)"
status=$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST "$API_URL/task/$TASK_ID" \
    -d "title=Updated title&description=Updated description&status=2")
assert_status 200 "$status" "POST /task/$TASK_ID returns 200"

# ──────────────────────────────────────────────
# 6. POST /user/1/task/{taskId} (associate)
# ──────────────────────────────────────────────
section "POST /user/1/task/$TASK_ID (associate)"
status=$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST "$API_URL/user/1/task/$TASK_ID")
assert_status 200 "$status" "POST /user/1/task/$TASK_ID returns 200"

# ──────────────────────────────────────────────
# 7. GET /user/1/task (verify association)
# ──────────────────────────────────────────────
section "GET /user/1/task (verify association)"
resp=$(curl -s "$API_URL/user/1/task")
if echo "$resp" | grep -q "\"$TASK_ID\""; then
    pass "task $TASK_ID found in user tasks"
else
    fail "task association" "task $TASK_ID not found in user tasks"
fi

# ──────────────────────────────────────────────
# 8. DELETE /user/1/task/{taskId} (remove association)
# ──────────────────────────────────────────────
section "DELETE /user/1/task/$TASK_ID (remove association)"
status=$(curl -s -o /dev/null -w "%{http_code}" \
    -X DELETE "$API_URL/user/1/task/$TASK_ID")
assert_status 200 "$status" "DELETE /user/1/task/$TASK_ID returns 200"

# ──────────────────────────────────────────────
# 9. DELETE /task/{id} (delete task)
# ──────────────────────────────────────────────
section "DELETE /task/$TASK_ID (delete task)"
status=$(curl -s -o /dev/null -w "%{http_code}" \
    -X DELETE "$API_URL/task/$TASK_ID")
assert_status 200 "$status" "DELETE /task/$TASK_ID returns 200"

# ──────────────────────────────────────────────
# 10. GET /task/{id} (verify deletion)
# ──────────────────────────────────────────────
section "GET /task/$TASK_ID (verify deletion)"
status=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL/task/$TASK_ID")
if [ "$status" = "404" ] || [ "$status" = "405" ]; then
    pass "task $TASK_ID no longer accessible (HTTP $status)"
else
    fail "task deletion verification" "expected 404/405, got $status"
fi

# ──────────────────────────────────────────────
# Summary
# ──────────────────────────────────────────────
echo -e "\n${YELLOW}═══════════════════════════════════════${NC}"
echo -e "  ${GREEN}Passed: $PASS${NC}  ${RED}Failed: $FAIL${NC}"
echo -e "${YELLOW}═══════════════════════════════════════${NC}"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
