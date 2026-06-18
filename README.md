<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Native cross-platform desktop application (Windows, macOS, Linux), fast, lightweight, and multi-language, to extract, decrypt, and restore your WhatsApp backups from Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="docs/README_es.md">🇪🇸 Español</a> • <a href="docs/README_zh.md">🇨🇳 中文</a> • <a href="docs/README_hi.md">🇮🇳 हिन्दी</a> • <a href="docs/README_fr.md">🇫🇷 Français</a> • <a href="docs/README_ar.md">🇸🇦 العربية</a> • <a href="docs/README_bn.md">🇧🇩 বাংলা</a> • <a href="docs/README_ru.md">🇷🇺 Русский</a> • <a href="docs/README_pt.md">🇵🇹 Português</a> • <a href="docs/README_ur.md">🇵🇰 Urdu</a> • <a href="docs/README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="docs/README_de.md">🇩🇪 Deutsch</a> • <a href="docs/README_ja.md">🇯🇵 日本語</a> • <a href="docs/README_mr.md">🇮🇳 Marathi</a> • <a href="docs/README_te.md">🇮🇳 Telugu</a> • <a href="docs/README_tr.md">🇹🇷 Türkçe</a> • <a href="docs/README_ta.md">🇮🇳 Tamil</a> • <a href="docs/README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="docs/README_tl.md">🇵🇭 Tagalog</a> • <a href="docs/README_ko.md">🇰🇷 한국어</a> • <a href="docs/README_it.md">🇮🇹 Italiano</a> • <a href="docs/README_pl.md">🇵🇱 Polski</a> • <a href="docs/README_nl.md">🇳🇱 Nederlands</a> • <a href="docs/README_th.md">🇹🇭 Thai</a> • <a href="docs/README_fa.md">🇮🇷 Farsi</a> • <a href="docs/README_gu.md">🇮🇳 Gujarati</a> • <a href="docs/README_ro.md">🇷🇴 Română</a> • <a href="docs/README_uk.md">🇺🇦 Ukrainian</a> • <a href="docs/README_el.md">🇬🇷 Greek</a> • <a href="docs/README_hu.md">🇭🇺 Hungarian</a> • <a href="docs/README_cs.md">🇨🇿 Čeština</a> • <a href="docs/README_sv.md">🇸🇪 Svenska</a> • <a href="docs/README_da.md">🇩🇰 Dansk</a> • <a href="docs/README_fi.md">🇫🇮 Suomi</a> • <a href="docs/README_no.md">🇳🇴 Norsk</a> • <a href="docs/README_sk.md">🇸🇰 Slovenčina</a> • <a href="docs/README_bg.md">🇧🇬 Bulgarian</a> • <a href="docs/README_hr.md">🇭🇷 Hrvatski</a> • <a href="docs/README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![License](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Features

* **Beautiful UI/UX:** A modern desktop interface with glassmorphism design, dark mode, and step-by-step restoration guide.
* **🌍 40 Supported Languages:** Quality internationalization natively integrated. Switch languages instantly.
* **Ultralight & Independent:** Rebuilt using **Tauri & Rust**, reducing binary size to just **~10MB** with zero external dependencies (no Python runtimes or .NET required on the host system).
* **Secure Keyring Storage:** Saves your Google OAuth tokens directly to the operating system's native secure keyring (Credential Manager in Windows, Keychain in macOS).
* **Local E2EE Decryption (.crypt14 / .crypt15):** High-speed, secure, and entirely local AES-256-GCM WhatsApp database decryption using Rust.
* **Direct Transfer to Phone (ADB):** Restore your decrypted databases directly to your Android phone with a single click, calling system ADB natively.

---

## 📋 Requirements

To build and run from source, make sure you have:
1. **Rust & Cargo** (v1.77 or superior)
2. **Node.js & npm**
3. An Android device with USB Debugging enabled (if restoring directly via ADB).

---

## 🚀 Execution and Development

### Launching the Application
Simply double-click the [run.bat](file:///d:/WhatsApp%20Google%20Drive%20Extractor/run.bat) file at the root of the project.
It will instantly open the compiled production executable (`app.exe`) or fallback to development mode if not compiled.

### Running in Development Mode
To start development mode with hot-reloading:
```bash
npx tauri dev
```

### Compiling and Bundling
To generate self-contained, signed production installer packages (MSI, EXE setup):
```bash
npx tauri build
```
The resulting installers will be located in `src-tauri/target/release/bundle/`.

See [DISTRIBUTION.md](file:///d:/WhatsApp%20Google%20Drive%20Extractor/DISTRIBUTION.md) for more details on packaging and signing.
