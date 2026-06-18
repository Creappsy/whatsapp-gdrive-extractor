<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp-logotyp" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Modernt, snabbt och flerspråkigt verktyg för att extrahera, dekryptera och återställa dina WhatsApp-säkerhetskopior från Google Drive.</b></p>

  <details>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![License](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Funktioner

* **Vackert webbgränssnitt/UX:** Ett modernt inbyggt gränssnitt i mörkt läge med element av glasmorfism för att enkelt guida dig genom utvinningsprocessen.
* **🌍 40 språk som stöds:** 100 % mänskliga översättningar som är inbyggda. Ändra språk i farten utan att ladda om!
* **Lokal dekryptering (.crypt14 / .crypt15):** En helt lokal kryptografimodul för att säkert dekryptera dina SQLite-databaser med din E2EE-hashnyckel utan att ladda upp dina data någonstans.
* **Direkt Android-överföring (ADB):** Skjut din nedladdade och dekrypterade databas direkt till din Android-telefons lagring med ett enda klick från instrumentpanelen.
* **Säker autentisering:** Stöder både Google OAuth-tokens och applösenord för maximal säkerhet.

## 📋 Förutsättningar

Innan du börjar, se till att du har:
1. **Python 3.x** eller **Docker** installerat.
2. En Android-enhet med WhatsApp installerat och Google Drive-säkerhetskopior aktiverade.
3. Dina Google-kontouppgifter (eller ett [applösenord](https://myaccount.google.com/apppasswords)).
4. *Valfritt:* Android-ID:t för din enhet (för att minska risken för att Google loggar ut dig).

## 🚀 Installation och användning

### Alternativ 1: Använda Python (rekommenderas för UI)

1. Klona förvaret:
   ``` bash
   git-klon https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Installera nödvändiga beroenden:
   ``` bash
   pip installation -r requirements.txt
   ```

3. Kör webbservern:
   ``` bash
   python server.py
   ```
4. Öppna din webbläsare och gå till `http://localhost:5000` för att komma åt den moderna instrumentpanelen!

### Alternativ 2: Använda Docker

1. Klona förvaret och navigera in i det.
2. Bygg Docker-bilden:
   ``` bash
   hamnarbyggare. -t whatsapp-gdrive-extractor
   ```
3. Kör Docker-behållaren:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Autentiseringsguide

Om du upplever problem med att använda din vanliga e-post och lösenord för Google, använd metoden **OAuth Token** (steg 1 i webbgränssnittet):
1. Gå till `https://getandroidapp.com/` eller valfri Google-inloggningsportal.
2. Logga in med ditt Google-konto.
3. Tryck på `F12` för att öppna utvecklarverktyg.
4. Gå till **Applikation** -> **Cookies**.
5. Hitta `oauth_token` (Det ser vanligtvis ut som `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Kopiera och klistra in det i webbgränssnittet.

## 🤝 Tack och tack
* **Original författare:** TripCode
* **Core Contributors:** DrDeath1122 (Multi-threading backbone), YuriCosta (New Restore System Reverse Engineering), macagua (SSL-fixar).
* **Modernisering och användargränssnitt:** Ombyggd med ett modernt webbgränssnitt, lokal kryptografimodul, ADB-telefonöverföring och helt lokaliserad till 40 språk för global tillgänglighet.