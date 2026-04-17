$ErrorActionPreference = "Stop"

$Repo = "RussellGN/AI-CodeLint"
$AppName = "ai-codelint.exe"

Write-Host "Installing ai-codelint..."

# Get latest release
$Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
$Tag = $Release.tag_name

$File = "ai-codelint-$Tag-x86_64-pc-windows-msvc.exe"
$Url = "https://github.com/$Repo/releases/download/$Tag/$File"

$InstallDir = "$env:LOCALAPPDATA\Programs\ai-codelint"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$Output = Join-Path $InstallDir $AppName

Write-Host "Downloading $File..."
Invoke-WebRequest $Url -OutFile $Output

# Add to PATH if needed
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")

if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "PATH",
        "$UserPath;$InstallDir",
        "User"
    )
    Write-Host "Added to PATH"
}

Write-Host "✔ Installed to $InstallDir"
Write-Host "Restart your terminal to use ai-codelint"
