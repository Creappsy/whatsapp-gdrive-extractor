<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Modern, fast, and multi-language tool to extract, decrypt, and restore your WhatsApp backups from Google Drive.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![License](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Features

* **Beautiful Web UI/UX:** A modern, dark-mode native interface with glassmorphism elements to easily guide you through the extraction process.
* **🌍 40 Languages Supported:** 100% human-quality translations natively built-in. Change the language on-the-fly without reloading!
* **Local Decryption (.crypt14 / .crypt15):** A fully local cryptography module to safely decrypt your SQLite databases using your E2EE hash key without uploading your data anywhere.
* **Direct Android Transfer (ADB):** Push your downloaded and decrypted database directly into your Android phone's storage with a single click from the dashboard.
* **Safe Authentication:** Supports both Google OAuth tokens and App Passwords for maximum security.

## 📋 Prerequisites

Before you start, make sure you have:
1. **Python 3.x** or **Docker** installed.
2. An Android device with WhatsApp installed and Google Drive backups enabled.
3. Your Google account credentials (or an [App Password](https://myaccount.google.com/apppasswords)).
4. *Optional:* The Android ID of your device (to reduce the risk of Google logging you out).

## 🚀 Installation & Usage

### Option 1: Using Python (Recommended for UI)

1. Clone the repository:
   ```bash
   git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Install the required dependencies:
   ```bash
   pip install -r requirements.txt
   ```

3. Run the Web Server:
   ```bash
   python server.py
   ```
4. Open your browser and go to `http://localhost:5000` to access the modern dashboard!

### Option 2: Using Docker

1. Clone the repository and navigate into it.
2. Build the Docker image:
   ```bash
   docker build . -t whatsapp-gdrive-extractor
   ```
3. Run the Docker container:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Authentication Guide

If you experience issues using your standard Google Email and Password, use the **OAuth Token** method (Step 1 on the Web UI):
1. Go to `https://getandroidapp.com/` or any Google login portal.
2. Log in with your Google account.
3. Press `F12` to open Developer Tools.
4. Go to **Application** -> **Cookies**.
5. Find the `oauth_token` (It usually looks like `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Copy and paste it into the Web UI.

## 🤝 Credits & Acknowledgements
* **Original Author:** TripCode
* **Core Contributors:** DrDeath1122 (Multi-threading backbone), YuriCosta (New restore system reverse engineering), macagua (SSL fixes).
* **Modernization & UI:** Rebuilt with a modern web interface, local cryptography module, ADB phone transfer, and fully localized into 40 languages for global accessibility.