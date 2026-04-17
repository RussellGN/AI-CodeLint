#Requires -Version 5.1
$ErrorActionPreference = "Stop"

$Repo    = "RussellGN/AI-CodeLint"
$AppName = "ai-codelint"
$Target  = "x86_64-pc-windows-msvc"

# ── Color helpers ─────────────────────────────────────────────────────────────
function Write-Info    ($msg) { Write-Host "  -> " -NoNewline -ForegroundColor Cyan;    Write-Host $msg }
function Write-Success ($msg) { Write-Host "  v " -NoNewline -ForegroundColor Green;   Write-Host $msg }
function Write-Warn    ($msg) { Write-Host "  ! " -NoNewline -ForegroundColor Yellow;  Write-Host $msg }
function Write-Fail    ($msg) { Write-Host "  x " -NoNewline -ForegroundColor Red;     Write-Host $msg; exit 1 }
function Write-Step    ($msg) { Write-Host "`n$msg" -ForegroundColor White }

# ── Banner ────────────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "  AI-CodeLint Installer" -ForegroundColor Cyan
Write-Host "  ------------------------------------" -ForegroundColor DarkGray
Write-Host ""

# ── Fetch latest release tag ──────────────────────────────────────────────────
Write-Step "Fetching latest release..."

try {
    $Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
    $Tag = $Release.tag_name
} catch {
    Write-Fail "Could not fetch latest release. Check your internet connection.`n  $_"
}

if (-not $Tag) { Write-Fail "Release tag was empty." }
Write-Info "Latest version: $Tag"

# ── Construct download URL ────────────────────────────────────────────────────
Write-Step "Downloading binary..."

$File   = "$AppName-$Tag-$Target.exe"
$Url    = "https://github.com/$Repo/releases/download/$Tag/$File"

$InstallDir = "$env:LOCALAPPDATA\Programs\$AppName"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$OutPath = Join-Path $InstallDir "$AppName.exe"

Write-Info "Source : $Url"
Write-Info "Dest   : $OutPath"

try {
    # Use a web client that follows redirects properly
    $wc = New-Object System.Net.WebClient
    $wc.DownloadFile($Url, $OutPath)
} catch {
    Write-Fail "Download failed: $_"
}

$FileSize = (Get-Item $OutPath).Length
if ($FileSize -lt 10240) {
    Remove-Item $OutPath -Force
    Write-Fail "Downloaded file is suspiciously small ($FileSize bytes).`n  The asset '$File' may not exist for this release."
}

Write-Success "Downloaded $File ($([math]::Round($FileSize/1KB)) KB)"

# ── PATH setup ────────────────────────────────────────────────────────────────
Write-Step "Configuring PATH..."

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")

if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "PATH",
        "$UserPath;$InstallDir",
        "User"
    )
    Write-Success "Added $InstallDir to your user PATH"
    Write-Warn "Restart your terminal for the PATH change to take effect."
} else {
    Write-Info "$InstallDir is already in PATH"
}

# Also update the current session so the binary is usable immediately
$env:PATH = "$env:PATH;$InstallDir"

# ── Done ──────────────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "  ------------------------------------" -ForegroundColor DarkGray
Write-Host "  Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "  Run " -NoNewline
Write-Host "$AppName --help" -NoNewline -ForegroundColor Cyan
Write-Host " to get started."
Write-Host ""
