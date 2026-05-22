# Keyboard Lock App Compression Script
# This script builds the app in release mode and uses UPX to compress the binary.

Write-Host "--- Step 1: Building optimized release binary ---" -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed. Aborting." -ForegroundColor Red
    exit 1
}

$binaryPath = "target\release\keyboard-lock.exe"

Write-Host "--- Step 2: Downloading portable UPX ---" -ForegroundColor Cyan
$url = "https://github.com/upx/upx/releases/download/v5.1.1/upx-5.1.1-win64.zip"
$zipFile = "upx_temp.zip"
$toolDir = "upx_tool"

Invoke-WebRequest -Uri $url -OutFile $zipFile
Expand-Archive -Path $zipFile -DestinationPath $toolDir

Write-Host "--- Step 3: Compressing binary with UPX ---" -ForegroundColor Cyan
$upxExe = Get-ChildItem -Path $toolDir -Filter "upx.exe" -Recurse | Select-Object -First 1
if ($upxExe) {
    & $upxExe.FullName --best --lzma $binaryPath
} else {
    Write-Host "Could not find upx.exe in the downloaded package." -ForegroundColor Red
}

Write-Host "--- Step 4: Cleaning up temporary files ---" -ForegroundColor Cyan
Remove-Item $zipFile -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force $toolDir -ErrorAction SilentlyContinue

Write-Host "Done! Your optimized and compressed binary is at: $binaryPath" -ForegroundColor Green
$size = (Get-Item $binaryPath).Length / 1KB
Write-Host "Final File Size: $('{0:N2}' -f $size) KB" -ForegroundColor Yellow
