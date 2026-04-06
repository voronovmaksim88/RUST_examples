# Задаёт пользовательские переменные ANDROID_HOME, JAVA_HOME, NDK_HOME (Windows).
# Запуск:  pwsh -File scripts\set-android-env.ps1
#         или из корня репозитория:  pwsh -ExecutionPolicy Bypass -File .\scripts\set-android-env.ps1
#
# После успешного выполнения закройте и снова откройте терминал (и при необходимости Cursor),
# затем проверьте:  $env:ANDROID_HOME; $env:NDK_HOME; Test-Path $env:NDK_HOME

$ErrorActionPreference = "Stop"

$androidHome = "$env:LOCALAPPDATA\Android\Sdk"
$javaHome    = "C:\Program Files\Android\Android Studio\jbr"

if (-not (Test-Path -LiteralPath $androidHome)) {
    Write-Error "Не найден Android SDK: $androidHome. Установите Android Studio и SDK."
}

if (-not (Test-Path -LiteralPath $javaHome)) {
    Write-Error "Не найден JAVA_HOME (jbr): $javaHome. Проверьте путь к Android Studio."
}

$ndkRoots = Get-ChildItem -LiteralPath "$androidHome\ndk" -Directory -ErrorAction SilentlyContinue
if (-not $ndkRoots) {
    Write-Error "Не найден NDK в $androidHome\ndk. В SDK Manager установите NDK (Side by side)."
}

$ndkHome = (
    $ndkRoots |
    Sort-Object { [version]($_.Name) } -Descending |
    Select-Object -First 1 -ExpandProperty FullName
)

Write-Host "ANDROID_HOME = $androidHome"
Write-Host "JAVA_HOME    = $javaHome"
Write-Host "NDK_HOME     = $ndkHome"
Write-Host ""

[Environment]::SetEnvironmentVariable("ANDROID_HOME", $androidHome, "User")
[Environment]::SetEnvironmentVariable("JAVA_HOME",    $javaHome,    "User")
[Environment]::SetEnvironmentVariable("NDK_HOME",     $ndkHome,     "User")

Write-Host "Переменные записаны для текущего пользователя."
Write-Host "Откройте новое окно PowerShell и проверьте: `$env:ANDROID_HOME; `$env:NDK_HOME"
