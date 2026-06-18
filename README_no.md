<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp-logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderne, raske og flerspråklige verktøy for å trekke ut, dekryptere og gjenopprette WhatsApp-sikkerhetskopier fra Google Disk.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Lisens](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Funksjoner

* **Vakker nettgrensesnitt/UX:** Et moderne, mørkt modus native grensesnitt med glassmorfisme-elementer for å enkelt veilede deg gjennom utvinningsprosessen.
* **🌍 40 språk som støttes:** 100 % oversettelser av menneskelig kvalitet innebygd. Endre språket mens du er på farten uten å laste på nytt!
* **Lokal dekryptering (.crypt14 / .crypt15):** En fullstendig lokal kryptografimodul for å trygt dekryptere SQLite-databasene dine ved å bruke E2EE-hash-nøkkelen uten å laste opp dataene dine hvor som helst.
* **Direct Android Transfer (ADB):** Skyv den nedlastede og dekrypterte databasen din direkte inn i Android-telefonens lagring med ett enkelt klikk fra dashbordet.
* **Sikker autentisering:** Støtter både Google OAuth-tokens og app-passord for maksimal sikkerhet.

## 📋 Forutsetninger

Før du begynner, sørg for at du har:
1. **Python 3.x** eller **Docker** installert.
2. En Android-enhet med WhatsApp installert og Google Drive-sikkerhetskopier aktivert.
3. Google-kontolegitimasjonen din (eller et [apppassord](https://myaccount.google.com/apppasswords)).
4. *Valgfritt:* Android-ID-en til enheten din (for å redusere risikoen for at Google logger deg ut).

## 🚀 Installasjon og bruk

### Alternativ 1: Bruke Python (anbefalt for brukergrensesnitt)

1. Klon depotet:
   ``` bash
   git-klone https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Installer de nødvendige avhengighetene:
   ``` bash
   pip install -r requirements.txt
   ```

3. Kjør nettserveren:
   ``` bash
   python server.py
   ```
4. Åpne nettleseren din og gå til `http://localhost:5000` for å få tilgang til det moderne dashbordet!

### Alternativ 2: Bruke Docker

1. Klon depotet og naviger inn i det.
2. Bygg Docker-bildet:
   ``` bash
   docker bygge. -t whatsapp-gdrive-extractor
   ```
3. Kjør Docker-beholderen:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Autentiseringsveiledning

Hvis du opplever problemer med å bruke standard Google-e-post og passord, bruk **OAuth-token**-metoden (trinn 1 på nettgrensesnittet):
1. Gå til `https://getandroidapp.com/` eller en hvilken som helst Google-påloggingsportal.
2. Logg på med Google-kontoen din.
3. Trykk "F12" for å åpne utviklerverktøy.
4. Gå til **Applikasjon** -> **Informasjonskapsler**.
5. Finn `oauth_token` (det ser vanligvis ut som `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Kopier og lim den inn i nettgrensesnittet.

## 🤝 Kreditter og anerkjennelser
* **Original forfatter:** TripCode
* **Kjernebidragsytere:** DrDeath1122 (Multi-threading backbone), YuriCosta (Reverse engineering for nytt system for gjenoppretting), macagua (SSL-fikser).
* **Modernisering og brukergrensesnitt:** Gjenoppbygd med et moderne nettgrensesnitt, lokal kryptografimodul, ADB-telefonoverføring og fullstendig lokalisert til 40 språk for global tilgjengelighet.