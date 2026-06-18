#!/bin/bash
set -e

echo -e "\e[36mIniciando despliegue de WhatsApp Extractor Web...\e[0m"

# Comprobar si Podman está instalado
if command -v podman &> /dev/null; then
    echo -e "\e[32mPodman detectado. Construyendo contenedor...\e[0m"
    podman build -t whatsapp-extractor -f Containerfile .
    
    echo -e "\e[32mIniciando contenedor...\e[0m"
    # Abrir navegador en segundo plano (Linux)
    (sleep 3 && xdg-open "http://localhost:5000" 2>/dev/null) &
    
    # Ejecutar contenedor
    podman run -it --rm -p 5000:5000 -v "$(pwd):/app:Z" whatsapp-extractor

else
    echo -e "\e[33mPodman no está instalado. Usando entorno de Python local...\e[0m"
    
    if [ ! -d "venv" ]; then
        echo -e "\e[36mCreando entorno virtual (venv)...\e[0m"
        python3 -m venv venv
    fi

    echo -e "\e[36mActivando entorno virtual e instalando dependencias...\e[0m"
    source venv/bin/activate
    pip install -r requirements.txt

    echo -e "\e[32mAbriendo navegador...\e[0m"
    (sleep 3 && xdg-open "http://localhost:5000" 2>/dev/null) &

    echo -e "\e[32mIniciando servidor web...\e[0m"
    python app.py
fi
