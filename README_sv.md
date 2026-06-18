<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp-logotyp" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Modernt, snabbt och flerspråkigt verktyg för att extrahera, dekryptera och återställa dina WhatsApp-säkerhetskopior från Google Drive.</b></p>

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