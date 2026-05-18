param(
    [string]$ApiUrl = "http://localhost:3000"
)

$ErrorActionPreference = "Stop"
$Pass = 0
$Fail = 0

function Pass { param($msg) Write-Host "  PASS $msg" -ForegroundColor Green; $script:Pass++ }
function Fail { param($msg, $detail) Write-Host "  FAIL $msg - $detail" -ForegroundColor Red; $script:Fail++ }
function Section { param($title) Write-Host "`n> $title" -ForegroundColor Yellow }

function Wait-ForApi {
    Write-Host "Waiting for API at $ApiUrl ..." -ForegroundColor Yellow
    for ($i = 0; $i -lt 30; $i++) {
        try {
            $r = Invoke-WebRequest -Uri "$ApiUrl/user/1" -Method Get -TimeoutSec 2 -UseBasicParsing
            Write-Host "API is ready!" -ForegroundColor Green
            Write-Host ""
            return
        } catch {}
        Start-Sleep -Seconds 1
    }
    Write-Host "API did not respond after 30s" -ForegroundColor Red
    exit 1
}

function Assert-Status {
    param($Expected, $Actual, $Label)
    if ($Actual -eq $Expected) {
        Pass "$Label (HTTP $Actual)"
    } else {
        Fail $Label "expected HTTP $Expected, got $Actual"
    }
}

function Assert-JsonKey {
    param($Json, $Key, $Label)
    if ($Json -match "`"$Key`"") {
        Pass "$Label (has key '$Key')"
    } else {
        Fail $Label "missing key '$Key'"
    }
}

Wait-ForApi

# 1. GET /user/1
Section "GET /user/1"
try {
    $r = Invoke-WebRequest -Uri "$ApiUrl/user/1" -Method Get -UseBasicParsing
    Assert-Status 200 $r.StatusCode "GET /user/1"
    Assert-JsonKey $r.Content "user_id" "response has user_id"
    Assert-JsonKey $r.Content "name" "response has name"
    Assert-JsonKey $r.Content "email" "response has email"
} catch {
    Fail "GET /user/1" $_.Exception.Message
}

# 2. GET /user/999
Section "GET /user/999 (not found)"
try {
    Invoke-WebRequest -Uri "$ApiUrl/user/999" -Method Get -UseBasicParsing | Out-Null
    Fail "GET /user/999" "expected 404 but got 200"
} catch {
    $status = $_.Exception.Response.StatusCode.value__
    Assert-Status 404 $status "GET /user/999 returns 404"
}

# 3. GET /user/1/task
Section "GET /user/1/task"
try {
    $r = Invoke-WebRequest -Uri "$ApiUrl/user/1/task" -Method Get -UseBasicParsing
    Assert-Status 200 $r.StatusCode "GET /user/1/task"
    Assert-JsonKey $r.Content "user_id" "response has user_id"
    Assert-JsonKey $r.Content "tasks" "response has tasks"
} catch {
    Fail "GET /user/1/task" $_.Exception.Message
}

# 4. POST /task
Section "POST /task (create)"
try {
    $body = "title=Test task&description=Created by test script&status=1"
    $r = Invoke-WebRequest -Uri "$ApiUrl/task" -Method Post -Body $body -ContentType "application/x-www-form-urlencoded" -UseBasicParsing
    Assert-Status 201 $r.StatusCode "POST /task returns 201"
    Assert-JsonKey $r.Content "task_id" "response has task_id"
    Assert-JsonKey $r.Content "title" "response has title"
    $taskId = ($r.Content | ConvertFrom-Json).task_id
    Write-Host "  Created task_id=$taskId" -ForegroundColor Cyan
} catch {
    Fail "POST /task" $_.Exception.Message
    $taskId = $null
}

if ($taskId) {
    # 5. POST /task/{id} (update)
    Section "POST /task/$taskId (update)"
    try {
        $body = "title=Updated title&description=Updated desc&status=2"
        $r = Invoke-WebRequest -Uri "$ApiUrl/task/$taskId" -Method Post -Body $body -ContentType "application/x-www-form-urlencoded" -UseBasicParsing
        Assert-Status 200 $r.StatusCode "POST /task/$taskId returns 200"
    } catch {
        Fail "POST /task/$taskId" $_.Exception.Message
    }

    # 6. POST /user/1/task/{taskId}
    Section "POST /user/1/task/$taskId (associate)"
    try {
        $r = Invoke-WebRequest -Uri "$ApiUrl/user/1/task/$taskId" -Method Post -UseBasicParsing
        Assert-Status 200 $r.StatusCode "POST /user/1/task/$taskId returns 200"
    } catch {
        Fail "POST /user/1/task/$taskId" $_.Exception.Message
    }

    # 7. GET /user/1/task (verify)
    Section "GET /user/1/task (verify association)"
    try {
        $r = Invoke-WebRequest -Uri "$ApiUrl/user/1/task" -Method Get -UseBasicParsing
        if ($r.Content -match "`"$taskId`"") {
            Pass "task $taskId found in user tasks"
        } else {
            Fail "task association" "task $taskId not found"
        }
    } catch {
        Fail "GET /user/1/task verify" $_.Exception.Message
    }

    # 8. DELETE /user/1/task/{taskId}
    Section "DELETE /user/1/task/$taskId (remove association)"
    try {
        $r = Invoke-WebRequest -Uri "$ApiUrl/user/1/task/$taskId" -Method Delete -UseBasicParsing
        Assert-Status 200 $r.StatusCode "DELETE /user/1/task/$taskId returns 200"
    } catch {
        Fail "DELETE /user/1/task/$taskId" $_.Exception.Message
    }

    # 9. DELETE /task/{id}
    Section "DELETE /task/$taskId (delete task)"
    try {
        $r = Invoke-WebRequest -Uri "$ApiUrl/task/$taskId" -Method Delete -UseBasicParsing
        Assert-Status 200 $r.StatusCode "DELETE /task/$taskId returns 200"
    } catch {
        Fail "DELETE /task/$taskId" $_.Exception.Message
    }
}

# Summary
Write-Host "`n===========================================" -ForegroundColor Yellow
Write-Host "  Passed: $Pass  Failed: $Fail" -ForegroundColor $(if ($Fail -gt 0) { "Red" } else { "Green" })
Write-Host "===========================================" -ForegroundColor Yellow

if ($Fail -gt 0) { exit 1 }
