# Stage UniWorld into a folder named UniWorld (required by Publish-Module / PS Gallery),
# then publish with your API key.
#
# Usage (from repo root or this directory):
#   $env:PSGALLERY_API_KEY = '<key>'
#   .\Publish-ToGallery.ps1
#
# Prerequisite (once per machine if Publish-Module asks):
#   Install-PackageProvider -Name NuGet -MinimumVersion 2.8.5.201 -Force

[CmdletBinding()]
param(
    [switch] $SkipPublish
)

$ErrorActionPreference = "Stop"
$here = $PSScriptRoot
$stageRoot = Join-Path ([System.IO.Path]::GetTempPath()) "UniWorldGalleryPublish"
$stageModule = Join-Path $stageRoot "UniWorld"

Write-Host "Staging module from: $here"
Write-Host "Staging folder:      $stageModule"

if (-not (Test-Path (Join-Path $here "UniWorld.psd1"))) {
    Write-Error "UniWorld.psd1 not found under $here"
}

Remove-Item -LiteralPath $stageRoot -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $stageModule -Force | Out-Null

Copy-Item -LiteralPath (Join-Path $here "UniWorld.psd1") -Destination $stageModule
Copy-Item -LiteralPath (Join-Path $here "UniWorld.psm1") -Destination $stageModule

$nativeSrc = Join-Path $here "native"
if (Test-Path $nativeSrc) {
    Copy-Item -LiteralPath $nativeSrc -Destination (Join-Path $stageModule "native") -Recurse -Force
} else {
    Write-Warning "No native\ folder found. Build with: cargo build --release --features cffi (and copy into native\win-x64\ etc.)"
}

if (-not $SkipPublish) {
    if (-not $env:PSGALLERY_API_KEY) {
        Write-Error "Set PSGALLERY_API_KEY before publishing."
    }
    Publish-Module -Path $stageModule -NuGetApiKey $env:PSGALLERY_API_KEY
    Write-Host "Done. Check https://www.powershellgallery.com/packages/UniWorld"
} else {
    Write-Host "SkipPublish set; staged at: $stageModule"
}
