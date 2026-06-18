<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Λογότυπο WhatsApp" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Σύγχρονο, γρήγορο και πολυγλωσσικό εργαλείο για εξαγωγή, αποκρυπτογράφηση και επαναφορά των αντιγράφων ασφαλείας WhatsApp από το Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Άδεια χρήσης](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Features

* **Beautiful Web UI/UX:** A modern, dark-mode native interface with glassmorphism elements to easily guide you through the extraction process.
* **🌍 Υποστηριζόμενες 40 γλώσσες:** 100% μεταφράσεις ανθρώπινης ποιότητας εγγενώς ενσωματωμένες. Αλλάξτε τη γλώσσα εν κινήσει χωρίς επαναφόρτωση!
* **Local Decryption (.crypt14 / .crypt15):** A fully local cryptography module to safely decrypt your SQLite databases using your E2EE hash key without uploading your data anywhere.
* **Direct Android Transfer (ADB):** Push your downloaded and decrypted database directly into your Android phone's storage with a single click from the dashboard.
* **Safe Authentication:** Supports both Google OAuth tokens and App Passwords for maximum security.

## 📋 Prerequisites

Πριν ξεκινήσετε, βεβαιωθείτε ότι έχετε:
1. Εγκατεστημένο **Python 3.x** ή **Docker**.
2. Μια συσκευή Android με εγκατεστημένο το WhatsApp και ενεργοποιημένα τα αντίγραφα ασφαλείας του Google Drive.
3. Τα διαπιστευτήρια του λογαριασμού σας Google (ή ένας [Κωδικός εφαρμογής](https://myaccount.google.com/apppasswords)).
4. *Προαιρετικό:* Το αναγνωριστικό Android της συσκευής σας (για να μειωθεί ο κίνδυνος αποσύνδεσης της Google).

## 🚀 Installation & Usage

### Option 1: Using Python (Recommended for UI)

1. Clone the repository:
   ```bash
   κλώνος git https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Install the required dependencies:
   ```bash
   pip install -r απαιτήσεις.txt
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

Εάν αντιμετωπίζετε προβλήματα με τη χρήση του τυπικού email και του κωδικού πρόσβασης Google, χρησιμοποιήστε τη μέθοδο **OAuth Token** (Βήμα 1 στη διεπαφή χρήστη Web):
1. Μεταβείτε στη διεύθυνση `https://getandroidapp.com/` ή σε οποιαδήποτε πύλη σύνδεσης της Google.
2. Log in with your Google account.
3. Press `F12` to open Developer Tools.
4. Μεταβείτε στην ενότητα **Εφαρμογή** -> **Cookies**.
5. Find the `oauth_token` (It usually looks like `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Copy and paste it into the Web UI.

## 🤝 Credits & Acknowledgements
* **Original Author:** TripCode
* **Βασικοί Συνεισφέροντες:** DrDeath1122 (Πολλαπλή ραχοκοκαλιά), YuriCosta (Αντίστροφη μηχανική νέου συστήματος επαναφοράς), macagua (διορθώσεις SSL).
* **Modernization & UI:** Rebuilt with a modern web interface, local cryptography module, ADB phone transfer, and fully localized into 40 languages for global accessibility.