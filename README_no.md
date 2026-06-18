<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp-logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderne, raske og flerspråklige verktøy for å trekke ut, dekryptere og gjenopprette WhatsApp-sikkerhetskopier fra Google Disk.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


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