<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Native cross-platform application (Windows, macOS, Linux, and Android), fast and multi-language, to extract, decrypt, and restore your WhatsApp backups from Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Podman](https://img.shields.io/badge/Podman-Supported-892CA0.svg)](https://podman.io/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![License](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Features

* **Beautiful Web UI/UX:** A modern, dark-mode native interface with glassmorphism elements to easily guide you through the extraction process.
* **🌍 40 Languages Supported:** 100% human-quality translations natively built-in. Change the language on-the-fly without reloading!
* **Local Decryption (.crypt14 / .crypt15):** A fully local cryptography module to safely decrypt your SQLite databases using your E2EE hash key without uploading your data anywhere.
* **Direct Android Transfer (ADB):** Push your downloaded and decrypted database directly into your Android phone's storage with a single click from the dashboard.
* **Headless Server Mode Fallback:** Automatically detects running in headless container environments (like Podman) and falls back to a standard Flask server on port `5000` instead of failing due to missing graphical environments (by skipping `pywebview`).
* **Safe Authentication:** Supports both Google OAuth tokens and App Passwords for maximum security.

## 📋 Prerequisites & Compatibility

Before you start, make sure you have:
1. **Python 3.10 or higher** or **Podman** installed (chosen over Docker for corporate licensing compliance).
2. An Android device with WhatsApp installed and Google Drive backups enabled.
3. Your Google account credentials (or an [App Password](https://myaccount.google.com/apppasswords)).
4. *Optional:* The Android ID of your device (to reduce the risk of Google logging you out).

### Core Dependency Versions:
* **Flask >= 3.0.0**
* **pywebview >= 4.0.0** (native local desktop window provider)
* **BeeWare/Toga >= 0.4.0** (cross-platform native app framework)
* **gpsoauth == 2.0.0** (secure Google authentication library)
* **pycryptodomex >= 3.0** (local database decryption)
* **adbutils >= 2.0.0** (direct connection to Android devices via ADB)

## 🚀 Installation & Usage

### Option 1: Quick Desktop Execution (Python + pywebview)

This option runs the application directly in a native desktop window using your operating system's built-in web rendering engine.

1. Clone the repository:
   ```bash
   git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Create and activate a virtual environment (recommended):
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

3. Install the required dependencies:
   ```bash
   pip install -r requirements.txt
   ```

4. Run the application:
   ```bash
   python app.py
   ```
   *(This will launch an independent, native desktop window).*

---

### Option 2: Native Compilation (Windows, macOS, Linux, and Android) with BeeWare/Briefcase

This tool is configured to compile natively on desktop systems and Android mobile devices using **Briefcase**.

1. Install `briefcase` in your environment:
   ```bash
   pip install briefcase
   ```

2. **Development Mode (Desktop):**
   To test the application in development mode on your PC:
   ```bash
   briefcase dev
   ```

3. **Compile and Run on Android 📱:**
   * Create the Android project scaffolding:
     ```bash
     briefcase create android
     ```
   * Build the application package (`.apk`/`.aab`):
     ```bash
     briefcase build android
     ```
   * Run the app on a connected Android device (with USB Debugging enabled) or an emulator:
     ```bash
     briefcase run android
     ```

4. **Compile for other Operating Systems:**
   * **Windows:** `briefcase run windows`
   * **macOS:** `briefcase run macOS`
   * **Linux:** `briefcase run linux`

---

### Option 3: Containerized Execution (Podman)

If you prefer to run the application within an isolated container:

1. Build the container image:
   ```bash
   podman build . -t whatsapp-gdrive-extractor -f Containerfile
   ```
2. Run the Podman container (mounting the current directory to save downloads to your PC and using the `:Z` flag for SELinux security settings if applicable):
   * **Linux:** `podman run -v $(pwd):/app:Z -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `podman run -v .:/app:Z -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Authentication Guide

If you experience issues using your standard Google Email and Password, use the **OAuth Token** method (Step 1 on the Web UI):
1. Go to `https://getandroidapp.com/` or any Google login portal.
2. Log in with your Google account.
3. Press `F12` to open Developer Tools.
4. Go to **Application** -> **Cookies**.
5. Find the `oauth_token` (It usually looks like `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Copy and paste it into the Web UI.

## 📱 Manual Restoration Guide on Android (The "Airplane Mode Trick")

If you have already extracted your backup files using this tool, follow these steps to successfully restore them to your modern Android device (e.g., Samsung Galaxy, Xiaomi, Pixel, etc.):

### Step 1: Transfer Files to Your Phone
* **Automatic Method (Recommended):** Connect your phone to your PC via USB, enable **USB Debugging** in your phone's *Developer Options*, and click the **"Transfer to phone"** button on our interface to copy the files to the correct path automatically.
* **Manual Method:** Connect your phone via USB cable and manually copy the extracted folders to the internal storage following this strict directory layout:
  ```text
  Internal storage
  ┗ 📂 Android
     ┗ 📂 media
        ┗ 📂 com.whatsapp
           ┗ 📂 WhatsApp
              ┣ 📂 Databases  <-- Place your msgstore.db.crypt14 / msgstore.db.crypt15 file here
              ┣ 📂 Media      <-- Copy your photos, audio, and video folders here
              ┗ 📂 Backups    <-- Copy your settings and stickers folders here
  ```

### Step 2: Force Local Restoration by Bypassing Google Drive
WhatsApp will always prioritize searching for backups on Google Drive over local storage. You must force the app to scan the internal memory:
1. Install WhatsApp from the Google Play Store (do not open it yet). If you already opened it, go to System Settings -> Apps -> WhatsApp and select **"Force Stop"**.
2. Turn on **Airplane Mode** on your phone and ensure **Wi-Fi is completely disabled** (the device must have no internet connection).
3. Open WhatsApp, accept the terms, and enter your phone number.
4. Since you have no connection, it will say it cannot connect. **Disable Airplane Mode temporarily** just to receive the 6-digit SMS verification code.
5. Enter the code, and as soon as the screen transitions to *"Looking for backups..."*, **immediately enable Airplane Mode again**.
6. Having no internet or Google Drive access forces the app to scan local storage, showing **"Local backup found"**.
7. Tap **Restore**, wait for it to reach 100%, and enter your profile name.
8. Once inside your chats, you can turn off Airplane Mode permanently. The media files in the `Media` folder will index in the background gradually.

## 🤝 Credits & Acknowledgements
* **Original Author:** TripCode
* **Core Contributors:** DrDeath1122 (Multi-threading backbone), YuriCosta (New restore system reverse engineering), macagua (SSL fixes).
* **Modernization & UI:** Rebuilt as a native desktop and mobile application (Windows, macOS, Linux, Android) using BeeWare/Briefcase and pywebview, featuring a local cryptography module, direct ADB phone transfer, and fully localized into 40 languages.