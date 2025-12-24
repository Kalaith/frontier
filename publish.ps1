# Frontier Kingdom - Itch.io Publish Script
# Creates a distributable package for itch.io upload

param(
    [switch]$SkipBuild = $false,
    [string]$OutputName = "frontier_kingdom_windows"
)

$ErrorActionPreference = "Stop"
$ProjectRoot = $PSScriptRoot
$DistDir = Join-Path $ProjectRoot "dist"
$PackageDir = Join-Path $DistDir "frontier_kingdom"

Write-Host "=== Frontier Kingdom Publisher ===" -ForegroundColor Cyan
Write-Host ""

# Step 1: Build release
if (-not $SkipBuild) {
    Write-Host "[1/4] Building release..." -ForegroundColor Yellow
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed!"
        exit 1
    }
    Write-Host "Build complete!" -ForegroundColor Green
} else {
    Write-Host "[1/4] Skipping build (using existing)" -ForegroundColor Gray
}

# Step 2: Clean and create dist folder
Write-Host "[2/4] Preparing dist folder..." -ForegroundColor Yellow
if (Test-Path $DistDir) {
    Remove-Item $DistDir -Recurse -Force
}
New-Item -ItemType Directory -Path $PackageDir -Force | Out-Null

# Step 3: Copy files
Write-Host "[3/4] Copying files..." -ForegroundColor Yellow

# Copy executable
$ExePath = Join-Path $ProjectRoot "target\release\frontier_kingdom.exe"
if (-not (Test-Path $ExePath)) {
    Write-Error "Executable not found at: $ExePath"
    exit 1
}
Copy-Item $ExePath $PackageDir

# Copy assets folder
$AssetsPath = Join-Path $ProjectRoot "assets"
Copy-Item $AssetsPath -Destination $PackageDir -Recurse

# Copy README
$ReadmePath = Join-Path $ProjectRoot "README.md"
if (Test-Path $ReadmePath) {
    Copy-Item $ReadmePath $PackageDir
}

Write-Host "Files copied!" -ForegroundColor Green

# Step 4: Create zip
Write-Host "[4/4] Creating zip archive..." -ForegroundColor Yellow
$ZipPath = Join-Path $DistDir "$OutputName.zip"
if (Test-Path $ZipPath) {
    Remove-Item $ZipPath -Force
}

Compress-Archive -Path $PackageDir -DestinationPath $ZipPath -CompressionLevel Optimal

# Summary
$ZipSize = [math]::Round((Get-Item $ZipPath).Length / 1MB, 2)
Write-Host ""
Write-Host "=== Package Complete ===" -ForegroundColor Cyan
Write-Host "Location: $ZipPath" -ForegroundColor Green
Write-Host "Size: ${ZipSize} MB" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Go to https://itch.io/dashboard" -ForegroundColor White
Write-Host "  2. Create new project or edit existing" -ForegroundColor White
Write-Host "  3. Upload: $ZipPath" -ForegroundColor White
Write-Host "  4. Mark as Windows executable" -ForegroundColor White
Write-Host ""

# Open dist folder
explorer $DistDir
