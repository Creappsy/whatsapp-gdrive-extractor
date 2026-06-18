$ErrorActionPreference = "Stop"
Write-Host "Instalando entorno local..." -ForegroundColor Cyan

if (-Not (Test-Path "venv")) {
    Write-Host "Creando entorno virtual (venv)..." -ForegroundColor Cyan
    python -m venv venv
}

Write-Host "Activando entorno virtual e instalando dependencias..." -ForegroundColor Cyan
& ".\venv\Scripts\Activate.ps1"
pip install -r requirements.txt

Write-Host "Iniciando aplicación nativa..." -ForegroundColor Green
python app.py
