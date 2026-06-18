: <<'BAT'
@echo off
title WhatsApp Extractor
echo ==========================================
echo       Iniciando WhatsApp Extractor
echo ==========================================
echo.

:: Instalar dependencias si no existe node_modules
if not exist "node_modules\" (
    echo [INFO] Instalando dependencias npm...
    npm install
    if errorlevel 1 (
        echo [ERROR] Fallo al instalar dependencias. Verifica que npm este instalado.
        pause
        exit /b 1
    )
    echo [INFO] Dependencias instaladas correctamente.
    echo.
)

if exist "src-tauri\target\release\app.exe" (
    echo [INFO] Iniciando version de produccion compilada...
    start "" "src-tauri\target\release\app.exe"
) else (
    echo [INFO] No se encontro compilacion previa. Compilando proyecto...
    npx tauri build
    if exist "src-tauri\target\release\app.exe" (
        echo [INFO] Compilacion completada. Iniciando version de produccion...
        start "" "src-tauri\target\release\app.exe"
    ) else (
        echo [ERROR] Compilacion fallo. Iniciando modo desarrollo...
        npx tauri dev
    )
)
exit /b
BAT

#!/bin/bash
echo "=========================================="
echo "      Iniciando WhatsApp Extractor"
echo "=========================================="
echo ""

# Instalar dependencias si no existe node_modules
if [ ! -d "node_modules" ]; then
    echo "[INFO] Instalando dependencias npm..."
    npm install
    if [ $? -ne 0 ]; then
        echo "[ERROR] Fallo al instalar dependencias. Verifica que npm este instalado."
        exit 1
    fi
    echo "[INFO] Dependencias instaladas correctamente."
    echo ""
fi

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
        echo "[INFO] No se encontro version compilada. Compilando proyecto..."
        npx tauri build
        if [ -d "src-tauri/target/release/bundle/macos/WhatsApp Extractor.app" ]; then
            echo "[INFO] Compilacion completada. Iniciando version de produccion..."
            open "src-tauri/target/release/bundle/macos/WhatsApp Extractor.app"
        else
            echo "[ERROR] Compilacion fallo. Iniciando modo desarrollo..."
            npx tauri dev
        fi
    fi
elif [ "$OS_NAME" = "Linux" ]; then
    # Linux
    if [ -f "src-tauri/target/release/app" ]; then
        echo "[INFO] Iniciando version de produccion compilada..."
        ./src-tauri/target/release/app &
    else
        echo "[INFO] No se encontro version compilada. Compilando proyecto..."
        npx tauri build
        if [ -f "src-tauri/target/release/app" ]; then
            echo "[INFO] Compilacion completada. Iniciando version de produccion..."
            ./src-tauri/target/release/app &
        else
            echo "[ERROR] Compilacion fallo. Iniciando modo desarrollo..."
            npx tauri dev
        fi
    fi
else
    echo "[ERROR] Sistema operativo no soportado por este script: $OS_NAME"
    exit 1
fi
