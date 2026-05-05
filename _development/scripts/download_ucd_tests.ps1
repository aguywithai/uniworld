# Download Unicode conformance test data and UCD source files for UniWorld.
# Defaults to a stable release tag (not "latest" and not beta) for reproducible builds.
#
# Run from repo root:
#   .\_development\scripts\download_ucd_tests.ps1
#   .\_development\scripts\download_ucd_tests.ps1 -UcdVersion "17.0.0" -Force
#
# Update policy: track formal Unicode releases only; skip alpha/beta UCD drops so
# public users and your test matrix stay aligned. See _development/scripts/STABLE_UCD_UPDATE.md

[CmdletBinding()]
param(
    [string] $UcdVersion = "17.0.0",
    [switch] $Force
)

$ErrorActionPreference = "Stop"
$devDir = Split-Path $PSScriptRoot -Parent
$outDir = Join-Path $devDir "data\ucd"
$base = "https://unicode.org/Public/$UcdVersion"

if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Path $outDir -Force | Out-Null
}

# Map: file name in _development/data/ucd/ -> path under Public/<version>/
# Unicode 16+ uses auxiliary/ for some break property files; emoji under ucd/emoji/.
$files = [ordered]@{
    "UnicodeData.txt"                = "ucd/UnicodeData.txt"
    "CompositionExclusions.txt"      = "ucd/CompositionExclusions.txt"
    "BidiBrackets.txt"               = "ucd/BidiBrackets.txt"
    "BidiTest.txt"                   = "ucd/BidiTest.txt"
    "BidiCharacterTest.txt"          = "ucd/BidiCharacterTest.txt"
    "LineBreak.txt"                  = "ucd/LineBreak.txt"
    "EastAsianWidth.txt"             = "ucd/EastAsianWidth.txt"
    "DerivedCoreProperties.txt"      = "ucd/DerivedCoreProperties.txt"
    "SpecialCasing.txt"              = "ucd/SpecialCasing.txt"
    "CaseFolding.txt"                = "ucd/CaseFolding.txt"
    "NormalizationTest.txt"          = "ucd/NormalizationTest.txt"
    "GraphemeBreakProperty.txt"      = "ucd/auxiliary/GraphemeBreakProperty.txt"
    "WordBreakProperty.txt"          = "ucd/auxiliary/WordBreakProperty.txt"
    "SentenceBreakProperty.txt"      = "ucd/auxiliary/SentenceBreakProperty.txt"
    "emoji-data.txt"                 = "ucd/emoji/emoji-data.txt"
    "GraphemeBreakTest.txt"         = "ucd/auxiliary/GraphemeBreakTest.txt"
    "WordBreakTest.txt"              = "ucd/auxiliary/WordBreakTest.txt"
    "SentenceBreakTest.txt"         = "ucd/auxiliary/SentenceBreakTest.txt"
    "LineBreakTest.txt"              = "ucd/auxiliary/LineBreakTest.txt"
}

Write-Host "UCD version: $UcdVersion"
Write-Host "Output: $outDir"
Write-Host "Base:   $base"
Write-Host ""

foreach ($entry in $files.GetEnumerator()) {
    $name = $entry.Key
    $rel = $entry.Value
    $dest = Join-Path $outDir $name
    $url = "$base/$rel"
    if ((Test-Path $dest) -and -not $Force) {
        Write-Host "Skip (exists): $name"
        continue
    }
    Write-Host "Download $name ..."
    try {
        Invoke-WebRequest -Uri $url -OutFile $dest -UseBasicParsing
    } catch {
        Write-Error "Failed: $url`n$($_.Exception.Message)"
    }
    Write-Host "  -> $dest"
}

# Optional: DerivedLineBreak is not used by current generators; download if we add LB rules.
# URL: ucd/extracted/DerivedLineBreak.txt (when present for this version)

Write-Host ""
Write-Host "Done. Run table generators from repo root, then cargo test / cargo test --features conformance"
Write-Host "  python _development\scripts\generate_gcb_tables.py"
Write-Host "  python _development\scripts\generate_bidi_tables.py"
Write-Host "  python _development\scripts\generate_linebreak_tables.py"
Write-Host "  python _development\scripts\generate_normalization_tables.py"
Write-Host "  python _development\scripts\generate_casemap_tables.py"
Write-Host "  (optional) python _development\scripts\generate_dictionary_data.py"
