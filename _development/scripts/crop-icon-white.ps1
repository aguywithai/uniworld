# Crop icon2.png to a centered square at ~92% of size; output icon8.png.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

$dir = Join-Path $PSScriptRoot '..\..\extensions\vscode'
$inPath  = Join-Path $dir 'icon2.png'
$outPath = Join-Path $dir 'icon8.png'

if (-not (Test-Path $inPath)) { Write-Error "Not found: $inPath"; exit 1 }

$img = [System.Drawing.Bitmap]::FromFile((Resolve-Path $inPath))
$w = $img.Width
$h = $img.Height
$side = [Math]::Min($w, $h)
$cropSide = [Math]::Round($side * 0.90)
$srcX = [Math]::Round(($w - $cropSide) / 2)
$srcY = [Math]::Round(($h - $cropSide) / 2)

$cropped = New-Object System.Drawing.Bitmap([int]$cropSide, [int]$cropSide)
$g = [System.Drawing.Graphics]::FromImage($cropped)
$g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$g.CompositingQuality = [System.Drawing.Drawing2D.CompositingQuality]::HighQuality
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
$srcRect = [System.Drawing.Rectangle]::new($srcX, $srcY, $cropSide, $cropSide)
$dstRect = [System.Drawing.Rectangle]::new(0, 0, $cropSide, $cropSide)
$g.DrawImage($img, $dstRect, $srcRect, [System.Drawing.GraphicsUnit]::Pixel)
$g.Dispose()
$cropped.Save($outPath, [System.Drawing.Imaging.ImageFormat]::Png)
$img.Dispose()
$cropped.Dispose()

Write-Host "Cropped to $cropSide x $cropSide (from $w x $h, ~92%). Saved: $outPath"
