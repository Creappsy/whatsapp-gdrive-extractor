<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Herramienta moderna, rápida y multilingüe para extraer, desencriptar y restaurar tus copias de seguridad de WhatsApp desde Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


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
