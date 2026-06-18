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
