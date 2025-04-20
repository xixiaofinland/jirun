<#
.SYNOPSIS
  Installs 'jirun' CLI on Windows.

.DESCRIPTION
  1. Detects CPU architecture (x86_64 / aarch64).
  2. Fetches the latest release data from GitHub.
  3. Downloads the matching Windows ZIP asset.
  4. Extracts it, moves 'jirun.exe' to a user-level directory.
  5. (Optional) Updates PATH environment variable.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# --- CONFIGURATIONS ---
$Repo          = "xixiaofinland/jirun"
$ApiUrl        = "https://api.github.com/repos/$Repo/releases/latest"
$BinaryName    = "jirun.exe"
$InstallFolder = "$env:USERPROFILE\AppData\Local\Programs\jirun"
$TempDir       = Join-Path $env:TEMP "jirun_download"

# --- FUNCTIONS ---

Function Get-Architecture {
    switch ($env:PROCESSOR_ARCHITECTURE.ToLower()) {
        "amd64"   { return "x86_64" }
        "x86"     { return "x86_64" }
        "arm64"   { return "aarch64" }
        default   {
            Write-Host "Unsupported CPU architecture: $($env:PROCESSOR_ARCHITECTURE)"
            return $null
        }
    }
}

# --- MAIN SCRIPT ---

$arch = Get-Architecture
if (-not $arch) {
    exit 1
}

if (Test-Path $TempDir) {
    Remove-Item $TempDir -Recurse -Force
}
New-Item -ItemType Directory -Path $TempDir | Out-Null

Write-Host "Fetching latest release info from GitHub..."
try {
    $releaseData = Invoke-RestMethod -Uri $ApiUrl -UseBasicParsing
}
catch {
    Write-Host "Error: Unable to fetch release data."
    exit 1
}

$pattern = "windows-$arch"
Write-Host "Looking for an asset with pattern: $pattern"

$assets = $releaseData.assets | Where-Object {
    $_.browser_download_url -match $pattern -and $_.browser_download_url -like '*.zip'
}

if (-not $assets) {
    Write-Host "Error: No matching ZIP asset found for pattern '$pattern'."
    exit 1
}

$assetUrl = $assets[0].browser_download_url
Write-Host "Downloading asset from: $assetUrl"

$zipFile = Join-Path $TempDir (Split-Path $assetUrl -Leaf)
Invoke-WebRequest -Uri $assetUrl -OutFile $zipFile -UseBasicParsing

Write-Host "Extracting ZIP to: $TempDir"
Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::ExtractToDirectory($zipFile, $TempDir)

$jirunPath = Get-ChildItem -Path $TempDir -Filter $BinaryName -Recurse -File | Select-Object -First 1

if (-not $jirunPath) {
    Write-Host "Error: Could not find '$BinaryName' in the extracted files."
    exit 1
}

Write-Host "Found $($jirunPath.FullName). Moving to $InstallFolder..."

if (-not (Test-Path $InstallFolder)) {
    New-Item -ItemType Directory -Path $InstallFolder | Out-Null
}

Move-Item -Path $jirunPath.FullName -Destination (Join-Path $InstallFolder $BinaryName) -Force
Remove-Item $TempDir -Recurse -Force

# Prompt to add to PATH
if ($Env:PATH -notmatch [regex]::Escape($InstallFolder)) {
    Write-Host ""
    $response = Read-Host "Do you want to add '$InstallFolder' to your PATH? (y/n)"
    if ($response -match '^[Yy]') {
        try {
            [System.Environment]::SetEnvironmentVariable(
                "PATH",
                "$Env:PATH;$InstallFolder",
                [System.EnvironmentVariableTarget]::User
            )
            Write-Host "Successfully added '$InstallFolder' to your PATH."
            Write-Host "Please restart PowerShell or open a new terminal to apply the change."
        }
        catch {
            Write-Host "Error: Failed to update PATH. Please add '$InstallFolder' manually."
        }
    } else {
        Write-Host "Skipped adding to PATH. You can add it manually later if needed."
    }
}

# Color output
$Green = "Green"
$Yellow = "Yellow"
$Cyan = "Cyan"

Write-Host ""
Write-Host "========================== Complete! ==========================" -ForegroundColor $Cyan
Write-Host "jirun installed to: $InstallFolder\$BinaryName" -ForegroundColor $Green
Write-Host ""

if ($Env:PATH -notmatch [regex]::Escape($InstallFolder)) {
    Write-Host "Note: $InstallFolder is not in your PATH." -ForegroundColor $Yellow
    Write-Host "Run it with full path: '$InstallFolder\$BinaryName --help'" -ForegroundColor $Yellow
} else {
    Write-Host "Run 'jirun --help' to get started." -ForegroundColor $Green
}
Write-Host ""
