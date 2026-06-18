<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp-logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderne, snelle en meertalige tool om je WhatsApp-back-ups uit Google Drive te extraheren, decoderen en herstellen.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licentie](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Kenmerken

* **Mooie web-UI/UX:** Een moderne, native interface in de donkere modus met glasmorfisme-elementen om u eenvoudig door het extractieproces te leiden.
* **🌍 40 ondersteunde talen:** 100% native ingebouwde vertalingen van menselijke kwaliteit. Verander de taal direct zonder opnieuw te laden!
* **Lokale decodering (.crypt14 / .crypt15):** Een volledig lokale cryptografiemodule om uw SQLite-databases veilig te decoderen met behulp van uw E2EE-hashsleutel zonder uw gegevens ergens te hoeven uploaden.
* **Directe Android-overdracht (ADB):** Push uw gedownloade en gedecodeerde database rechtstreeks naar de opslag van uw Android-telefoon met een enkele klik vanaf het dashboard.
* **Veilige authenticatie:** Ondersteunt zowel Google OAuth-tokens als app-wachtwoorden voor maximale veiligheid.

## 📋 Vereisten

Zorg ervoor dat u, voordat u begint, beschikt over:
1. **Python 3.x** of **Docker** geïnstalleerd.
2. Een Android-apparaat waarop WhatsApp is geïnstalleerd en Google Drive-back-ups zijn ingeschakeld.
3. De inloggegevens van uw Google-account (of een [App-wachtwoord] (https://myaccount.google.com/apppasswords)).
4. *Optioneel:* De Android-ID van uw apparaat (om het risico te verkleinen dat Google u uitlogt).

## 🚀 Installatie en gebruik

### Optie 1: Python gebruiken (aanbevolen voor UI)

1. Kloon de repository:
   ``` bash
   git kloon https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Installeer de vereiste afhankelijkheden:
   ``` bash
   pip install -r vereisten.txt
   ```

3. Voer de webserver uit:
   ``` bash
   Python-server.py
   ```
4. Open uw browser en ga naar `http://localhost:5000` om toegang te krijgen tot het moderne dashboard!

### Optie 2: Docker gebruiken

1. Kloon de repository en navigeer ernaartoe.
2. Bouw de Docker-image:
   ``` bash
   havenarbeider gebouwd. -t whatsapp-gdrive-extractor
   ```
3. Voer de Docker-container uit:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Authenticatiegids

Als u problemen ondervindt bij het gebruik van uw standaard e-mailadres en wachtwoord van Google, gebruikt u de **OAuth Token**-methode (stap 1 in de webinterface):
1. Ga naar `https://getandroidapp.com/` of een Google-inlogportaal.
2. Log in met uw Google-account.
3. Druk op `F12` om Ontwikkelaarstools te openen.
4. Ga naar **Applicatie** -> **Cookies**.
5. Zoek de `oauth_token` (deze ziet er meestal uit als `oauth2_4/XXXXXXXXXXXXXXXX`).
6. Kopieer en plak het in de webinterface.

## 🤝 Credits en dankbetuigingen
* **Originele auteur:** TripCode
* **Kernbijdragers:** DrDeath1122 (Multi-threading backbone), YuriCosta (Nieuwe reverse engineering voor herstelsysteem), macagua (SSL-fixes).
* **Modernisering en gebruikersinterface:** Herbouwd met een moderne webinterface, lokale cryptografiemodule, ADB-telefoonoverdracht en volledig gelokaliseerd in 40 talen voor wereldwijde toegankelijkheid.