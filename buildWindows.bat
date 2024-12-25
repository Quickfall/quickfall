@echo off
setlocal EnableDelayedExpansion

:: Check if ANSI colors are supported (Windows 10+ or ConEmu/Cmder)
for /f "tokens=2 delims=[]" %%a in ('ver') do for /f "tokens=2,3 delims=. " %%b in ("%%a") do set "version.major=%%b"

:: Initialize color variables
set "GREEN="
set "RED="
set "NC="

:: Enable ANSI escape sequences if supported
if %version.major% geq 10 (
    :: Windows 10 or later - enable VT100
    reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1 /f >nul 2>&1
    set "GREEN=[92m"
    set "RED=[91m"
    set "NC=[0m"
) else (
    :: Check for ConEmu/Cmder
    if defined ConEmuPID (
        set "GREEN=[92m"
        set "RED=[91m"
        set "NC=[0m"
    )
)

echo %GREEN%Building Quickfall...%NC%

:: Run make and capture the exit code
make
if %ERRORLEVEL% EQU 0 (
    if defined GREEN (
        echo %GREEN%Build successful!%NC%
    ) else (
        echo Build successful!
    )
    exit /b 0
) else (
    if defined RED (
        echo %RED%Build failed!%NC%
    ) else (
        echo Build failed!
    )
    exit /b 1
)
