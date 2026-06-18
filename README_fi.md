<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp-logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderni, nopea ja monikielinen työkalu WhatsApp-varmuuskopioiden purkamiseen, salauksen purkamiseen ja palauttamiseen Google Drivesta.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Lisenssi](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Ominaisuudet

* **Kaunis verkkokäyttöliittymä/UX:** Moderni pimeän tilan natiivi käyttöliittymä, jossa on lasimorfismielementtejä, jotka opastavat sinut helposti poimintaprosessin läpi.
* **🌍 40 tuettua kieltä:** 100 % ihmislaatuiset käännökset sisäänrakennettuna. Vaihda kieli lennossa ilman uudelleenlatausta!
* **Paikallinen salauksenpurku (.crypt14 / .crypt15):** Täysin paikallinen salausmoduuli, jonka avulla voit purkaa turvallisesti SQLite-tietokantojesi salauksen E2EE-hajautusavaimella lataamatta tietojasi minnekään.
* **Direct Android Transfer (ADB):** Työnnä ladattu ja salattu tietokanta suoraan Android-puhelimesi tallennustilaan yhdellä napsautuksella kojelautasta.
* **Turvallinen todennus:** Tukee sekä Google OAuth -tunnuksia että sovellusten salasanoja maksimaalisen turvallisuuden takaamiseksi.

## 📋 Edellytykset

Ennen kuin aloitat, varmista, että sinulla on:
1. **Python 3.x** tai **Docker** asennettuna.
2. Android-laite, johon on asennettu WhatsApp ja Google Drive -varmuuskopiot käytössä.
3. Google-tilisi kirjautumistiedot (tai [sovelluksen salasana](https://myaccount.google.com/apppasswords)).
4. *Valinnainen:* Laitteesi Android-tunnus (jotta vähennät riskiä, ​​että Google kirjaa sinut ulos).

## 🚀 Asennus ja käyttö

### Vaihtoehto 1: Pythonin käyttö (suositus käyttöliittymälle)

1. Kloonaa arkisto:
   ```bash
   git-klooni https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Asenna tarvittavat riippuvuudet:
   ```bash
   pip install -r vaatimukset.txt
   ```

3. Suorita Web-palvelin:
   ```bash
   python server.py
   ```
4. Avaa selaimesi ja siirry osoitteeseen `http://localhost:5000` päästäksesi nykyaikaiseen kojelautaan!

### Vaihtoehto 2: Dockerin käyttö

1. Kloonaa arkisto ja siirry siihen.
2. Luo Docker-kuva:
   ```bash
   telakkarakennelma. -t whatsapp-gdrive-extractor
   ```
3. Suorita Docker-säilö:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Todennusopas

Jos sinulla on ongelmia tavallisen Google-sähköpostin ja salasanan käytössä, käytä **OAuth Token** -menetelmää (vaihe 1 verkkokäyttöliittymässä):
1. Siirry osoitteeseen https://getandroidapp.com/ tai mihin tahansa Googlen kirjautumisportaaliin.
2. Kirjaudu sisään Google-tililläsi.
3. Avaa Kehittäjätyökalut painamalla F12-näppäintä.
4. Siirry kohtaan **Sovellus** -> **Evästeet**.
5. Etsi "oauth_token" (se näyttää yleensä tältä "oauth2_4/XXXXXXXXXXXXXXXXXX").
6. Kopioi ja liitä se verkkokäyttöliittymään.

## 🤝 Kiitokset ja kiitokset
* **Alkuperäinen kirjoittaja:** TripCode
* **Ydin avustajat:** DrDeath1122 (monisäikeinen runkoverkko), YuriCosta (uuden palautusjärjestelmän käänteinen suunnittelu), macagua (SSL-korjaukset).
* **Modernisointi ja käyttöliittymä:** Uudelleen rakennettu nykyaikaisella verkkoliittymällä, paikallisella salausmoduulilla, ADB-puhelinsiirrolla ja lokalisoitu täysin 40 kielelle maailmanlaajuista saatavuutta varten.