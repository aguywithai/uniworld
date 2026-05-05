<#
Render the UniWorld Unicode showcase markdown document to PDF.

Uses Python: markdown -> HTML; PDF via Edge/Chrome headless (system browser, full Unicode fonts).

Usage (from repo root):
  pwsh _development/scripts/render_unicode_showcase.ps1

Requirements:
- Python: pip install -r requirements.txt (markdown only)
- Edge or Chrome installed (Windows/macOS/Linux)
- PowerShell (Windows PowerShell or pwsh)

Output:
- docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.pdf
- docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.html
#>

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
$scriptPath = Join-Path $PSScriptRoot "render_unicode_showcase_to_pdf.py"

if (-not (Test-Path $scriptPath)) {
    Write-Error "Python script not found: $scriptPath"
}

Push-Location $repoRoot
try {
    & python $scriptPath
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
} finally {
    Pop-Location
}
