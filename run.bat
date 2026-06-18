: <<'BAT'
@echo off
title WhatsApp Extractor
echo ==========================================
echo       Iniciando WhatsApp Extractor
echo ==========================================
echo.

if exist "src-tauri\target\release\app.exe" (
    echo [INFO] Iniciando version de produccion compilada...
    start "" "src-tauri\target\release\app.exe"
) else (
    echo [INFO] No se encontro compilacion previa. Iniciando modo desarrollo...
    npx tauri dev
)
exit /b
BAT

#!/bin/bash
echo "=========================================="
echo "      Iniciando WhatsApp Extractor"
echo "=========================================="
echo ""

OS_NAME=$(uname -s)

if [ "$OS_NAME" = "Darwin" ]; then
    # macOS
    if [ -d "src-tauri/target/release/WhatsApp Extractor.app" ]; then
        echo "[INFO] Iniciando version de produccion en macOS..."
        open "src-tauri/target/release/WhatsApp Extractor.app"
    elif [ -d "src-tauri/target/release/bundle/macos/WhatsApp Extractor.app" ]; then
        echo "[INFO] Iniciando version de produccion en macOS..."
        open "src-tauri/target/release/bundle/macos/WhatsApp Extractor.app"
    else
        echo "[INFO] No se encontro version compilada. Iniciando modo desarrollo..."
        npx tauri dev
    fi
elif [ "$OS_NAME" = "Linux" ]; then
    # Linux
    if [ -f "src-tauri/target/release/app" ]; then
        echo "[INFO] Iniciando version de produccion compilada..."
        ./src-tauri/target/release/app &
    else
        echo "[INFO] No se encontro version compilada. Iniciando modo desarrollo..."
        npx tauri dev
    fi
else
    echo "[ERROR] Sistema operativo no soportado por este script: $OS_NAME"
    exit 1
fi
