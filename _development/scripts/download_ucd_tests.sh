#!/usr/bin/env bash
# Download Unicode conformance test data (same file set as download_ucd_tests.ps1).
# For Linux/macOS CI and local use. Pin version for reproducibility.
#
# Usage from repo root:
#   bash _development/scripts/download_ucd_tests.sh
#   UcdVersion=17.0.0 bash _development/scripts/download_ucd_tests.sh

set -euo pipefail

UcdVersion="${UcdVersion:-17.0.0}"
ScriptDir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DevDir="$(dirname "$ScriptDir")"
OutDir="${DevDir}/data/ucd"
Base="https://unicode.org/Public/${UcdVersion}"

mkdir -p "${OutDir}"

download_one() {
  local name="$1"
  local rel="$2"
  local url="${Base}/${rel}"
  local dest="${OutDir}/${name}"
  echo "Download ${name} ..."
  curl -fsSL "${url}" -o "${dest}"
  echo "  -> ${dest}"
}

echo "UCD version: ${UcdVersion}"
echo "Output: ${OutDir}"
echo "Base:   ${Base}"
echo ""

download_one "UnicodeData.txt" "ucd/UnicodeData.txt"
download_one "CompositionExclusions.txt" "ucd/CompositionExclusions.txt"
download_one "BidiBrackets.txt" "ucd/BidiBrackets.txt"
download_one "BidiTest.txt" "ucd/BidiTest.txt"
download_one "BidiCharacterTest.txt" "ucd/BidiCharacterTest.txt"
download_one "LineBreak.txt" "ucd/LineBreak.txt"
download_one "EastAsianWidth.txt" "ucd/EastAsianWidth.txt"
download_one "DerivedCoreProperties.txt" "ucd/DerivedCoreProperties.txt"
download_one "SpecialCasing.txt" "ucd/SpecialCasing.txt"
download_one "CaseFolding.txt" "ucd/CaseFolding.txt"
download_one "NormalizationTest.txt" "ucd/NormalizationTest.txt"
download_one "GraphemeBreakProperty.txt" "ucd/auxiliary/GraphemeBreakProperty.txt"
download_one "WordBreakProperty.txt" "ucd/auxiliary/WordBreakProperty.txt"
download_one "SentenceBreakProperty.txt" "ucd/auxiliary/SentenceBreakProperty.txt"
download_one "emoji-data.txt" "ucd/emoji/emoji-data.txt"
download_one "GraphemeBreakTest.txt" "ucd/auxiliary/GraphemeBreakTest.txt"
download_one "WordBreakTest.txt" "ucd/auxiliary/WordBreakTest.txt"
download_one "SentenceBreakTest.txt" "ucd/auxiliary/SentenceBreakTest.txt"
download_one "LineBreakTest.txt" "ucd/auxiliary/LineBreakTest.txt"

echo ""
echo "Done. Run table generators and tests as in STABLE_UCD_UPDATE.md / download_ucd_tests.ps1 comments."
