<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Logo ng WhatsApp" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderno, mabilis, at multi-language na tool upang i-extract, i-decrypt, at i-restore ang iyong mga backup sa WhatsApp mula sa Google Drive.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![License](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Mga Tampok

* **Magandang Web UI/UX:** Isang moderno, dark-mode na katutubong interface na may mga elemento ng glassmorphism upang madaling gabayan ka sa proseso ng pagkuha.
* **🌍 40 Mga Suportadong Wika:** 100% na mga pagsasalin sa kalidad ng tao na native na built-in. Baguhin ang wika on-the-fly nang hindi nagre-reload!
* **Local Decryption (.crypt14 / .crypt15):** Isang ganap na lokal na cryptography module upang ligtas na i-decrypt ang iyong mga SQLite database gamit ang iyong E2EE hash key nang hindi ina-upload ang iyong data kahit saan.
* **Direct Android Transfer (ADB):** Itulak ang iyong na-download at na-decrypt na database nang direkta sa storage ng iyong Android phone sa isang pag-click mula sa dashboard.
* **Ligtas na Pagpapatotoo:** Sinusuportahan ang parehong mga token ng Google OAuth at Mga Password ng App para sa maximum na seguridad.

## 📋 Mga kinakailangan

Bago ka magsimula, tiyaking mayroon kang:
1. Naka-install ang **Python 3.x** o **Docker**.
2. Isang Android device na may naka-install na WhatsApp at pinagana ang mga backup ng Google Drive.
3. Mga kredensyal ng iyong Google account (o isang [App Password](https://myaccount.google.com/apppasswords)).
4. *Opsyonal:* Ang Android ID ng iyong device (upang bawasan ang panganib na ma-log out ka ng Google).

## 🚀 Pag-install at Paggamit

### Opsyon 1: Paggamit ng Python (Inirerekomenda para sa UI)

1. I-clone ang repositoryo:
   ```bash
   git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. I-install ang mga kinakailangang dependencies:
   ```bash
   pip install -r requirements.txt
   ```

3. Patakbuhin ang Web Server:
   ```bash
   python server.py
   ```
4. Buksan ang iyong browser at pumunta sa `http://localhost:5000` upang ma-access ang modernong dashboard!

### Opsyon 2: Paggamit ng Docker

1. I-clone ang repository at mag-navigate papunta dito.
2. Buuin ang imahe ng Docker:
   ```bash
   build ng docker. -t whatsapp-gdrive-extractor
   ```
3. Patakbuhin ang lalagyan ng Docker:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Gabay sa Pagpapatunay

Kung nakakaranas ka ng mga isyu sa paggamit ng iyong karaniwang Google Email at Password, gamitin ang **OAuth Token** na paraan (Hakbang 1 sa Web UI):
1. Pumunta sa `https://getandroidapp.com/` o anumang portal sa pag-login sa Google.
2. Mag-log in gamit ang iyong Google account.
3. Pindutin ang `F12` upang buksan ang Mga Tool ng Developer.
4. Pumunta sa **Application** -> **Cookies**.
5. Hanapin ang `oauth_token` (Karaniwan itong mukhang `oauth2_4/XXXXXXXXXXXXXXXXXX`).
6. Kopyahin at i-paste ito sa Web UI.

## 🤝 Mga Kredito at Pasasalamat
* **Orihinal na May-akda:** TripCode
* **Mga Core Contributor:** DrDeath1122 (Multi-threading backbone), YuriCosta (Bagong restore system reverse engineering), macagua (SSL fixes).
* **Modernisasyon at UI:** Muling itinayo gamit ang modernong web interface, lokal na cryptography module, paglilipat ng telepono ng ADB, at ganap na na-localize sa 40 wika para sa pandaigdigang accessibility.