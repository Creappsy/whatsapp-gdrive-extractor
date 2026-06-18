<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp-logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderni, nopea ja monikielinen työkalu WhatsApp-varmuuskopioiden purkamiseen, salauksen purkamiseen ja palauttamiseen Google Drivesta.</b></p>

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