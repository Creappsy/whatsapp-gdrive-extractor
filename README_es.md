<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Aplicación nativa multiplataforma (Windows, macOS, Linux y Android), rápida y multilingüe para extraer, desencriptar y restaurar tus copias de seguridad de WhatsApp desde Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Podman](https://img.shields.io/badge/Podman-Supported-892CA0.svg)](https://podman.io/)
  [![i18n](https://img.shields.io/badge/Idiomas-40_Soportados-success.svg)](#)
  [![License](https://img.shields.io/badge/Licencia-MIT-green.svg)](#)
</div>

---

## 🌟 Características

* **Hermosa interfaz UI/UX:** Una interfaz nativa moderna en modo oscuro con elementos *glassmorphism* que te guiará fácilmente en el proceso de extracción.
* **🌍 40 Idiomas Soportados:** Traducciones de calidad 100% humana integradas de forma nativa. ¡Cambia de idioma al instante sin recargar!
* **Desencriptación Local (.crypt14 / .crypt15):** Un módulo de criptografía completamente local para desencriptar de forma segura tus bases de datos SQLite usando tu hash E2EE, sin subir tus datos a ningún lado.
* **Transferencia Directa a Android (ADB):** Envía tu base de datos descargada y desencriptada directamente al almacenamiento de tu teléfono Android con un solo clic desde el panel.
* **Modo Servidor Alternativo (Headless Fallback):** Soporta la ejecución automatizada en entornos de contenedor o terminales sin interfaz gráfica (catorceando el uso de `pywebview`), levantando en su lugar el servidor Flask convencional en el puerto `5000`.
* **Autenticación Segura:** Soporta tanto tokens de Google OAuth como Contraseñas de Aplicación para máxima seguridad.

## 📋 Requisitos Previos y Compatibilidad

Antes de comenzar, asegúrate de tener:
1. **Python 3.10 o superior** o bien **Podman** instalado (elegido sobre Docker por cumplimiento de licencias corporativas).
2. Un dispositivo Android con WhatsApp instalado y las copias de seguridad de Google Drive activadas.
3. Las credenciales de tu cuenta de Google (o una [Contraseña de Aplicación](https://myaccount.google.com/apppasswords)).
4. *Opcional:* El ID de Android de tu dispositivo (para reducir el riesgo de que Google cierre tu sesión).

### Versiones de Componentes Core:
* **Flask >= 3.0.0**
* **pywebview >= 4.0.0** (proveedor de ventana nativa en escritorio local)
* **BeeWare/Toga >= 0.4.0** (framework nativo para empaquetado)
* **gpsoauth == 2.0.0** (autenticación segura con Google)
* **pycryptodomex >= 3.0** (desencriptación local)
* **adbutils >= 2.0.0** (conexión directa por ADB con dispositivos Android)

## 🚀 Instalación y Uso

### Opción 1: Ejecución Rápida de Escritorio (Python + pywebview)

Esta opción ejecuta la aplicación directamente en una ventana de escritorio nativa utilizando el motor web integrado de tu sistema operativo.

1. Clona el repositorio:
   ```bash
   git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Crea y activa un entorno virtual (recomendado):
   * **Windows (PowerShell):**
     ```powershell
     python -m venv venv
     .\venv\Scripts\Activate.ps1
     ```
   * **macOS / Linux (Bash):**
     ```bash
     python3 -m venv venv
     source venv/bin/activate
     ```

3. Instala las dependencias necesarias:
   ```bash
   pip install -r requirements.txt
   ```

4. Ejecuta la aplicación:
   ```bash
   python app.py
   ```
   *(Esto iniciará una ventana nativa de escritorio independiente).*

---

### Opción 2: Compilación Nativa (Windows, macOS, Linux y Android) con BeeWare/Briefcase

Esta herramienta está configurada para compilarse de forma nativa en sistemas de escritorio y en dispositivos móviles Android utilizando **Briefcase**.

1. Instala `briefcase` en tu entorno:
   ```bash
   pip install briefcase
   ```

2. **Modo Desarrollo (Escritorio):**
   Para probar la aplicación en modo desarrollo en tu PC:
   ```bash
   briefcase dev
   ```

3. **Compilar y Ejecutar en Android 📱:**
   * Crea el andamiaje del proyecto para Android:
     ```bash
     briefcase create android
     ```
   * Compila el paquete de la aplicación (`.apk`/`.aab`):
     ```bash
     briefcase build android
     ```
   * Ejecuta la aplicación en un dispositivo Android conectado (con Depuración USB activada) o en un emulador:
     ```bash
     briefcase run android
     ```

4. **Compilar para otros Sistemas Operativos:**
   * **Windows:** `briefcase run windows`
   * **macOS:** `briefcase run macOS`
   * **Linux:** `briefcase run linux`

---

### Opción 3: Ejecución en Contenedor (Podman)

Si prefieres ejecutar la aplicación dentro de un contenedor aislado:

1. Construye la imagen del contenedor:
   ```bash
   podman build . -t whatsapp-gdrive-extractor -f Containerfile
   ```
2. Ejecuta el contenedor (montando el volumen actual para guardar las descargas en tu PC y con el flag `:Z` para temas de SELinux si aplica):
   * **Linux:** `podman run -v $(pwd):/app:Z -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `podman run -v .:/app:Z -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Guía de Autenticación

Si tienes problemas usando tu correo y contraseña habitual de Google, usa el método del **Token OAuth** (Paso 1 en la interfaz web):
1. Ve a `https://getandroidapp.com/` o cualquier portal de inicio de sesión de Google.
2. Inicia sesión con tu cuenta de Google.
3. Presiona `F12` para abrir las Herramientas de Desarrollador.
4. Ve a **Application** -> **Cookies**.
5. Encuentra el `oauth_token` (Generalmente se ve así: `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Cópialo y pégalo en la interfaz web.

## 📱 Guía de Restauración Manual en Android (El "Truco del Modo Avión")

Si ya has extraído los archivos de tu copia de seguridad con esta herramienta, sigue estos pasos para restaurarlos con éxito en tu dispositivo Android moderno (ej. Samsung Galaxy, Xiaomi, Pixel, etc.):

### Paso 1: Transferir los archivos al celular
* **Método Automático (Recomendado):** Conecta tu celular por USB a tu PC, activa la **Depuración USB** en las *Opciones de Desarrollador* de tu móvil y presiona el botón **"Transferir al celular"** en nuestra interfaz para que los archivos se copien automáticamente en la ruta correcta.
* **Método Manual:** Conecta tu celular mediante cable USB y copia la estructura de carpetas extraídas hacia el almacenamiento interno respetando esta estructura estricta:
  ```text
  Almacenamiento interno
  ┗ 📂 Android
     ┗ 📂 media
        ┗ 📂 com.whatsapp
           ┗ 📂 WhatsApp
              ┣ 📂 Databases  <-- Coloca aquí tu archivo msgstore.db.crypt14 / msgstore.db.crypt15
              ┣ 📂 Media      <-- Copia aquí tus carpetas de fotos, audios y videos
              ┗ 📂 Backups    <-- Copia aquí tus configuraciones y stickers
  ```

### Paso 2: Engañar a WhatsApp para forzar la restauración local
WhatsApp priorizará buscar un respaldo en Google Drive antes que la memoria local. Debemos forzar el escaneo del almacenamiento interno:
1. Instala WhatsApp desde Google Play Store (pero no lo abras aún). Si ya lo abriste, ve a Ajustes del sistema -> Aplicaciones -> WhatsApp y selecciona **"Forzar detención"**.
2. Activa el **Modo Avión** en tu celular y asegúrate de que el **Wi-Fi esté totalmente apagado** (el teléfono debe quedar sin conexión a internet).
3. Abre WhatsApp, acepta los términos e ingresa tu número de teléfono.
4. Como no tienes red, te dirá que no puede conectarse. **Desactiva el Modo Avión un momento** solo para recibir el SMS con el código de verificación de 6 dígitos.
5. Ingresa el código y, en cuanto la pantalla cambie a *"Buscando copia de seguridad..."*, **activa el Modo Avión de nuevo inmediatamente**.
6. Al no tener acceso a internet ni a Google Drive, la aplicación se verá obligada a escanear tu almacenamiento y dirá **"Copia de seguridad local encontrada"**.
7. Presiona **Restaurar**, espera a que llegue al 100% e introduce tu nombre de perfil.
8. Una vez dentro de tus chats, ya puedes desactivar el Modo Avión permanentemente. Los archivos multimedia de la carpeta `Media` se indexarán en segundo plano poco a poco.

## 🤝 Créditos y Agradecimientos
* **Autor Original:** TripCode
* **Colaboradores Principales:** DrDeath1122, YuriCosta, macagua.
* **Modernización y UI:** Reconstruido como una aplicación nativa de escritorio y móvil (Windows, macOS, Linux, Android) usando BeeWare/Briefcase y pywebview, con módulo de criptografía local, transferencia directa por ADB, y completamente localizado a 40 idiomas.
