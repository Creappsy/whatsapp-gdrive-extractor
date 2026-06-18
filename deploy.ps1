$ErrorActionPreference = "Stop"
Write-Host "Iniciando despliegue de WhatsApp Extractor Web..." -ForegroundColor Cyan

# Comprobar si Podman está instalado
$podmanInstalled = $false
try {
    $null = Get-Command podman -ErrorAction Stop
    $podmanInstalled = $true
} catch {}

if ($podmanInstalled) {
    Write-Host "Podman detectado. Construyendo contenedor..." -ForegroundColor Green
    podman build -t whatsapp-extractor -f Containerfile .
    
    Write-Host "Iniciando contenedor..." -ForegroundColor Green
    # Abrir navegador en segundo plano (espera un segundo para que levante el server)
    Start-Job -ScriptBlock { Start-Sleep -Seconds 3; Start-Process "http://localhost:5000" } | Out-Null
    
    # Montamos el volumen actual para que los archivos descargados se guarden en la PC
    podman run -it --rm -p 5000:5000 -v "${PWD}:/app:Z" whatsapp-extractor
} else {
    Write-Host "Podman no está instalado. Usando entorno de Python local..." -ForegroundColor Yellow
    
    if (-Not (Test-Path "venv")) {
        Write-Host "Creando entorno virtual (venv)..." -ForegroundColor Cyan
        python -m venv venv
    }

    Write-Host "Activando entorno virtual e instalando dependencias..." -ForegroundColor Cyan
    & ".\venv\Scripts\Activate.ps1"
    pip install -r requirements.txt

    Write-Host "Abriendo navegador..." -ForegroundColor Green
    Start-Job -ScriptBlock { Start-Sleep -Seconds 3; Start-Process "http://localhost:5000" } | Out-Null

    Write-Host "Iniciando servidor web..." -ForegroundColor Green
    python app.py
}
