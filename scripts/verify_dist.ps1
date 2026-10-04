$ErrorActionPreference = "Stop"
$root = $PSScriptRoot + "\.."
$dist = Join-Path $root "dist"

$html = Get-Content (Join-Path $dist "index.html") -Raw -Encoding UTF8
$m = [regex]::Match($html, '(?s)<script>(.*?)</script>')
if (-not $m.Success) {
    throw "No <script> tag found in dist/index.html"
}
$js = $m.Groups[1].Value

# Check bracket balances inside strings vs outside strings
$inSingle = $false
$inDouble = $false
$inBacktick = $false
$curly = 0
$paren = 0
$square = 0

for ($i = 0; $i -lt $js.Length; $i++) {
    $c = $js[$i]
    if ($c -eq '\' -and ($inSingle -or $inDouble -or $inBacktick)) {
        $i++
        continue
    }
    if ($c -eq "'" -and -not $inDouble -and -not $inBacktick) { $inSingle = -not $inSingle; continue }
    if ($c -eq '"' -and -not $inSingle -and -not $inBacktick) { $inDouble = -not $inDouble; continue }
    if ($c -eq '`' -and -not $inSingle -and -not $inDouble) { $inBacktick = -not $inBacktick; continue }
    
    if (-not $inSingle -and -not $inDouble -and -not $inBacktick) {
        if ($c -eq '{') { $curly++ }
        elseif ($c -eq '}') { $curly-- }
        elseif ($c -eq '(') { $paren++ }
        elseif ($c -eq ')') { $paren-- }
        elseif ($c -eq '[') { $square++ }
        elseif ($c -eq ']') { $square-- }
    }
}

Write-Host "Verification Results:" -ForegroundColor Cyan
Write-Host "Unterminated Strings: single=$inSingle, double=$inDouble, backtick=$inBacktick"
Write-Host "Brace balance: curly=$curly, paren=$paren, square=$square"

if ($curly -ne 0 -or $paren -ne 0 -or $square -ne 0 -or $inSingle -or $inDouble -or $inBacktick) {
    throw "Syntax balance verification failed!"
}

# Check for crucial identifiers and strings
$requiredPatterns = @(
    'Plus Jakarta Sans',
    'slide-in-right',
    'slide-in-left',
    'Control System Technology',
    'EMK22003 / EMK31103',
    'set_desktop_wallpaper',
    'update_tray_status',
    'timetable-v4'
)

foreach ($pat in $requiredPatterns) {
    if (-not $html.Contains($pat) -and -not (Get-Content (Join-Path $dist "sw.js") -Raw).Contains($pat)) {
        throw "Required pattern missing: $pat"
    }
    Write-Host "Found required pattern: $pat" -ForegroundColor Green
}

Write-Host "`nAll verification checks PASSED!" -ForegroundColor Green
