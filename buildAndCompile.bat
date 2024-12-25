@echo off
setlocal EnableDelayedExpansion

if "%~1"=="" (
    echo Usage: buildAndCompile.bat filename.qf
    exit /b 1
)

REM Get filename without extension
set "filename=%~n1"

REM Clean and build quickfall
make clean
make

REM Compile the specified file
cd examples
..\quickfall.exe compile %1 -o !filename!.exe
cd ..
