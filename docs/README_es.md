<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Aplicación nativa de escritorio (Windows, macOS, Linux) multiplataforma, rápida, ligera y multilingüe para extraer, desencriptar y restaurar tus copias de seguridad de WhatsApp desde Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Idiomas-40_Soportados-success.svg)](#)
  [![License](https://img.shields.io/badge/Licencia-MIT-green.svg)](#)
</div>

---

## 🌟 Características

* **Hermosa interfaz UI/UX:** Una interfaz de escritorio nativa moderna con diseño *glassmorphism*, modo oscuro y guía de restauración paso a paso.
* **🌍 40 Idiomas Soportados:** Localización de calidad integrada nativamente en el frontend. Cambia de idioma al instante.
* **Ultra-ligera e Independiente:** Rediseñada desde cero en **Tauri y Rust**, reduciendo el tamaño del binario a solo **~10MB** con cero dependencias externas (sin necesidad de intérpretes de Python ni .NET Core instalados en el host).
* **Almacenamiento Seguro de Credenciales:** Guarda los tokens de Google OAuth directamente en el llavero seguro nativo del sistema operativo (Credential Manager en Windows, Keychain en macOS).
* **Desencriptación Local Rápida (.crypt14 / .crypt15):** Módulo de criptografía local de alta velocidad implementado en Rust para desencriptar de forma segura tus bases de datos SQLite mediante AES-256-GCM.
* **Transferencia Directa al Teléfono (ADB):** Restaura tu base de datos descargada y desencriptada directamente en tu teléfono Android con un solo clic llamando a la utilidad ADB del sistema.

---

## 📋 Requisitos

Para compilar y ejecutar desde el código fuente, asegúrate de tener:
1. **Rust y Cargo** (v1.77 o superior)
2. **Node.js y npm**
3. Un dispositivo Android con la Depuración USB activada (si deseas restaurar mediante ADB).

---

## 🚀 Ejecución y Desarrollo

### Lanzamiento de la Aplicación
Simplemente haz doble clic en el archivo [run.bat](file:///d:/WhatsApp%20Google%20Drive%20Extractor/run.bat) en la raíz del proyecto.
El script abrirá al instante el ejecutable de producción compilado (`app.exe`) o iniciará en modo desarrollo si no se encuentra.

### Ejecución en Modo Desarrollo
Para iniciar el entorno de desarrollo con recarga en vivo (hot-reload):
```bash
npx tauri dev
```

### Compilación y Empaquetado
Para generar instaladores autocontenidos y firmados de producción (MSI, EXE setup):
```bash
npx tauri build
```
Los instaladores resultantes se guardarán en `src-tauri/target/release/bundle/`.

Consulta [DISTRIBUTION.md](file:///d:/WhatsApp%20Google%20Drive%20Extractor/DISTRIBUTION.md) para más detalles sobre empaquetado y firmado.
