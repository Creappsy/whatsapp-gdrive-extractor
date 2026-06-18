<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Herramienta moderna, rápida y multilingüe para extraer, desencriptar y restaurar tus copias de seguridad de WhatsApp desde Google Drive.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Idiomas-40_Soportados-success.svg)](#)
  [![License](https://img.shields.io/badge/Licencia-MIT-green.svg)](#)
</div>

---

## 🌟 Características

* **Hermosa interfaz UI/UX:** Una interfaz nativa moderna en modo oscuro con elementos *glassmorphism* que te guiará fácilmente en el proceso de extracción.
* **🌍 40 Idiomas Soportados:** Traducciones de calidad 100% humana integradas de forma nativa. ¡Cambia de idioma al instante sin recargar!
* **Desencriptación Local (.crypt14 / .crypt15):** Un módulo de criptografía completamente local para desencriptar de forma segura tus bases de datos SQLite usando tu hash E2EE, sin subir tus datos a ningún lado.
* **Transferencia Directa a Android (ADB):** Envía tu base de datos descargada y desencriptada directamente al almacenamiento de tu teléfono Android con un solo clic desde el panel.
* **Autenticación Segura:** Soporta tanto tokens de Google OAuth como Contraseñas de Aplicación para máxima seguridad.

## 📋 Requisitos Previos

Antes de comenzar, asegúrate de tener:
1. **Python 3.x** o **Docker** instalados.
2. Un dispositivo Android con WhatsApp instalado y las copias de seguridad de Google Drive activadas.
3. Las credenciales de tu cuenta de Google (o una [Contraseña de Aplicación](https://myaccount.google.com/apppasswords)).
4. *Opcional:* El ID de Android de tu dispositivo (para reducir el riesgo de que Google cierre tu sesión).

## 🚀 Instalación y Uso

### Opción 1: Usar Python (Recomendado para la UI)

1. Clona el repositorio:
   ```bash
   git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Instala las dependencias requeridas:
   ```bash
   pip install -r requirements.txt
   ```

3. Ejecuta el Servidor Web:
   ```bash
   python server.py
   ```
4. ¡Abre tu navegador y ve a `http://localhost:5000` para acceder al panel de control!

### Opción 2: Usar Docker

1. Clona el repositorio y entra en él.
2. Construye la imagen de Docker:
   ```bash
   docker build . -t whatsapp-gdrive-extractor
   ```
3. Ejecuta el contenedor:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Guía de Autenticación

Si tienes problemas usando tu correo y contraseña habitual de Google, usa el método del **Token OAuth** (Paso 1 en la interfaz web):
1. Ve a `https://getandroidapp.com/` o cualquier portal de inicio de sesión de Google.
2. Inicia sesión con tu cuenta de Google.
3. Presiona `F12` para abrir las Herramientas de Desarrollador.
4. Ve a **Application** -> **Cookies**.
5. Encuentra el `oauth_token` (Generalmente se ve así: `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Cópialo y pégalo en la interfaz web.

## 🤝 Créditos y Agradecimientos
* **Autor Original:** TripCode
* **Colaboradores Principales:** DrDeath1122, YuriCosta, macagua.
* **Modernización y UI:** Reconstruido con una moderna interfaz web, módulo de criptografía local, transferencia a teléfono por ADB, y completamente traducido a 40 idiomas para accesibilidad global.
