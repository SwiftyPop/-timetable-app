# Automated Web Optimization, Minification & Pre-compression Script

$ErrorActionPreference = "Stop"

$root = $PSScriptRoot + "\.."
Set-Location $root

Write-Host "=== Starting Web Optimization Pipeline ===" -ForegroundColor Cyan

# 1. Clean and initialize dist directory
$dist = Join-Path $root "dist"
if (Test-Path $dist) {
    Remove-Item $dist -Recurse -Force
}
New-Item -ItemType Directory -Path $dist -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $dist "fonts") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $dist "icons") -Force | Out-Null

# 2. Copy optimized font files (400, 600, 800)
$fontsToKeep = @("pjs-400.woff2", "pjs-600.woff2", "pjs-800.woff2")
foreach ($f in $fontsToKeep) {
    $srcFont = Join-Path (Join-Path $root "fonts") $f
    if (Test-Path $srcFont) {
        Copy-Item $srcFont (Join-Path $dist "fonts") -Force
    }
}

# 3. Copy icons
Copy-Item (Join-Path (Join-Path $root "icons") "*") (Join-Path $dist "icons") -Force

# 4. Read source index.html
$srcHtml = Get-Content (Join-Path $root "index.html") -Raw -Encoding UTF8
$origLength = [System.Text.Encoding]::UTF8.GetByteCount($srcHtml)

# 5. Minify HTML, inlined CSS, and inlined JS
$minHtml = $srcHtml

# A. Strip HTML comments (except conditionals)
$minHtml = [System.Text.RegularExpressions.Regex]::Replace($minHtml, '<!--(?!\[if).*?-->', '', [System.Text.RegularExpressions.RegexOptions]::Singleline)

# B. Minify CSS inside <style> blocks
$minHtml = [System.Text.RegularExpressions.Regex]::Replace($minHtml, '(?s)<style>(.*?)</style>', {
    param($match)
    $css = $match.Groups[1].Value
    # Remove CSS comments
    $css = [System.Text.RegularExpressions.Regex]::Replace($css, '/\*.*?\*/', '', [System.Text.RegularExpressions.RegexOptions]::Singleline)
    # Remove space around delimiters
    $css = [System.Text.RegularExpressions.Regex]::Replace($css, '\s*([\{\}:;,>])\s*', '$1')
    $css = [System.Text.RegularExpressions.Regex]::Replace($css, ';\}', '}')
    $css = [System.Text.RegularExpressions.Regex]::Replace($css, '\s+', ' ')
    return "<style>" + $css.Trim() + "</style>"
})

# C. Minify JavaScript inside <script> blocks (Token-aware to preserve string literals)
function Minify-JS([string]$code) {
    $sb = New-Object System.Text.StringBuilder
    $len = $code.Length
    $i = 0
    $prev = [char]0

    while ($i -lt $len) {
        $c = $code[$i]

        # Single-line comment
        if ($c -eq '/' -and $i + 1 -lt $len -and $code[$i + 1] -eq '/') {
            $i += 2
            while ($i -lt $len -and $code[$i] -ne "`n" -and $code[$i] -ne "`r") { $i++ }
            continue
        }

        # Multi-line comment
        if ($c -eq '/' -and $i + 1 -lt $len -and $code[$i + 1] -eq '*') {
            $i += 2
            while ($i + 1 -lt $len -and -not ($code[$i] -eq '*' -and $code[$i + 1] -eq '/')) { $i++ }
            $i += 2
            continue
        }

        # String literals: ', ", ` (preserve verbatim!)
        if ($c -eq "'" -or $c -eq '"' -or $c -eq '`') {
            $quote = $c
            [void]$sb.Append($c)
            $i++
            while ($i -lt $len) {
                $sc = $code[$i]
                [void]$sb.Append($sc)
                if ($sc -eq '\') {
                    $i++
                    if ($i -lt $len) { [void]$sb.Append($code[$i]) }
                } elseif ($sc -eq $quote) {
                    $prev = $sc
                    $i++
                    break
                }
                $i++
            }
            continue
        }

        # Whitespace
        if ([char]::IsWhiteSpace($c)) {
            $hasNewline = ($c -eq "`n" -or $c -eq "`r")
            $j = $i + 1
            while ($j -lt $len -and [char]::IsWhiteSpace($code[$j])) {
                if ($code[$j] -eq "`n" -or $code[$j] -eq "`r") { $hasNewline = $true }
                $j++
            }
            $next = if ($j -lt $len) { $code[$j] } else { [char]0 }

            $delims = "(){}[];,:=<>!&|?~^%/*"
            $isPlusMinus = ($prev -eq '+' -and $next -eq '+') -or ($prev -eq '-' -and $next -eq '-')
            $isWordPrev = [char]::IsLetterOrDigit($prev) -or $prev -eq '_' -or $prev -eq '$'
            $isWordNext = [char]::IsLetterOrDigit($next) -or $next -eq '_' -or $next -eq '$'

            if ($isPlusMinus -or ($isWordPrev -and $isWordNext)) {
                [void]$sb.Append(' ')
                $prev = ' '
            } elseif ($hasNewline -and $prev -ne [char]0 -and $prev -ne ';' -and $prev -ne '{' -and $prev -ne '}' -and -not ($delims.IndexOf($next) -ge 0)) {
                # Safe statement boundary
                [void]$sb.Append(';')
                $prev = ';'
            }
            $i = $j
            continue
        }

        [void]$sb.Append($c)
        $prev = $c
        $i++
    }
    return $sb.ToString().Trim()
}

$minHtml = [System.Text.RegularExpressions.Regex]::Replace($minHtml, '(?s)<script>(.*?)</script>', {
    param($match)
    $js = $match.Groups[1].Value
    $minJs = Minify-JS $js
    return "<script>" + $minJs + "</script>"
})

# D. Strip intra-tag formatting whitespace
$minHtml = [System.Text.RegularExpressions.Regex]::Replace($minHtml, '>\s+<', '><')

# Write minified index.html to dist
$distHtmlPath = Join-Path $dist "index.html"
[System.IO.File]::WriteAllText($distHtmlPath, $minHtml, [System.Text.Encoding]::UTF8)
$minLength = (Get-Item $distHtmlPath).Length

# 6. Minify and copy sw.js and manifest.webmanifest
$rawSw = Get-Content (Join-Path $root "sw.js") -Raw -Encoding UTF8
$minSw = Minify-JS $rawSw
$distSwPath = Join-Path $dist "sw.js"
[System.IO.File]::WriteAllText($distSwPath, $minSw, [System.Text.Encoding]::UTF8)

$rawManifest = Get-Content (Join-Path $root "manifest.webmanifest") -Raw -Encoding UTF8
# Compact JSON whitespace
$minManifest = [System.Text.RegularExpressions.Regex]::Replace($rawManifest, '\s+', ' ')
$minManifest = [System.Text.RegularExpressions.Regex]::Replace($minManifest, '\s*([\{\}\[\]:,])\s*', '$1')
$distManifestPath = Join-Path $dist "manifest.webmanifest"
[System.IO.File]::WriteAllText($distManifestPath, $minManifest.Trim(), [System.Text.Encoding]::UTF8)

# 7. Pre-compress text assets with Gzip (Level 9)
function Pre-Compress-GZip([string]$filePath) {
    $bytes = [System.IO.File]::ReadAllBytes($filePath)
    $gzPath = $filePath + ".gz"
    $outStream = [System.IO.File]::Create($gzPath)
    $gzStream = New-Object System.IO.Compression.GZipStream($outStream, [System.IO.Compression.CompressionLevel]::Optimal)
    $gzStream.Write($bytes, 0, $bytes.Length)
    $gzStream.Close()
    $outStream.Close()
}

Pre-Compress-GZip $distHtmlPath
Pre-Compress-GZip (Join-Path $dist "sw.js")
Pre-Compress-GZip (Join-Path $dist "manifest.webmanifest")

$gzLength = (Get-Item ($distHtmlPath + ".gz")).Length

# 8. Calculate total asset weight comparison
$origFiles = Get-ChildItem -Path $root -Include *.html,*.js,*.webmanifest,*.woff2,*.png -Recurse -File | Where-Object { $_.FullName -notmatch '[\\/](target|dist|\.cargo)[\\/]' }
$origTotal = ($origFiles | Measure-Object -Property Length -Sum).Sum
$distTotal = (Get-ChildItem -Path $dist -Recurse -File -Exclude *.gz | Measure-Object -Property Length -Sum).Sum

Write-Host "`n=== Optimization Summary ===" -ForegroundColor Green
Write-Host ("Original index.html : {0,8:N0} bytes ({1:N2} KB)" -f $origLength, ($origLength / 1KB))
Write-Host ("Minified index.html : {0,8:N0} bytes ({1:N2} KB) [Saved {2:P1}]" -f $minLength, ($minLength / 1KB), (1 - $minLength/$origLength))
Write-Host ("Gzipped  index.html : {0,8:N0} bytes ({1:N2} KB) [Saved {2:P1}]" -f $gzLength, ($gzLength / 1KB), (1 - $gzLength/$origLength))
Write-Host ("----------------------------------------------------")
Write-Host ("Total App Weight (Original)   : {0,8:N0} bytes ({1:N2} KB)" -f $origTotal, ($origTotal / 1KB))
Write-Host ("Total App Weight (Optimized)  : {0,8:N0} bytes ({1:N2} KB)" -f $distTotal, ($distTotal / 1KB))
Write-Host ("Total Storage Reduction       : {0:P1}" -f (1 - $distTotal/$origTotal)) -ForegroundColor Yellow
