<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Alkuperäinen monikäyttöinen työpöytäsovellus (Windows, macOS, Linux), nopea, kevyt ja monikielinen WhatsApp-varmuuskopioiden purkamiseen, salauksen purkamiseen ja palauttamiseen Google Drivesta.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Kielet-40_Tuettu-success.svg)](#)
  [![License](https://img.shields.io/badge/Lisenssi-MIT-green.svg)](#)
</div>

---

## 🌟 Ominaisuudet

* **Kaunis käyttöliittymä/UX:** Moderni työpöytäkäyttöliittymä, jossa on lasimorfismi, tumma tila ja vaiheittainen palautusopas.
* **🌍 40 tuettua kieltä:** Laadukas kansainvälistyminen integroituna natiivisti. Vaihda kieltä välittömästi.
* **Ultrakevyt ja itsenäinen:** Rakennettu uudelleen käyttämällä **Tauri & Rust** -tekniikkaa, pienentäen binaarikoon vain **~10 megatavuun** ilman ulkoisia riippuvuuksia (isäntäjärjestelmässä ei vaadita Python-ajoa tai .NETiä).
* **Turvallinen avaimenperäsäilytys:** Tallentaa Google OAuth -tunnuksesi suoraan käyttöjärjestelmän suojattuun avaimenperään (Windowsissa Credential Manager, macOS:ssä Keychain).
* **Paikallinen E2EE-salauksenpurku (.crypt14 / .crypt15):** Nopea, turvallinen ja täysin paikallinen AES-256-GCM WhatsApp -tietokannan salauksen purku Rustin avulla.
* **Suora siirto puhelimeen (ADB):** Palauta salatut tietokannat suoraan Android-puhelimeesi yhdellä napsautuksella kutsumalla järjestelmän ADB:tä natiivisti.

---

## 📋 Vaatimukset

Jotta voit rakentaa ja käyttää lähdekoodia, varmista, että sinulla on:
1. **Rust & Cargo** (v1.77 tai parempi)
2. **Node.js & npm**
3. Android-laite, jossa on käytössä USB-virheenkorjaus (jos palautetaan suoraan ADB:n kautta).

---

## 🚀 Toteutus ja kehittäminen

### Sovelluksen käynnistäminen
Yksinkertaisesti kaksoisnapsauta [run.bat](../run.bat) tiedosto projektin juuressa.
Se avaa välittömästi käännetyn tuotantosuoritettavan tiedoston ("app.exe") tai palaa kehitystilaan, jos sitä ei ole käännetty.

### Toimii kehitystilassa
Kehitystilan käynnistäminen kuumalla uudelleenlatauksella:
```bash
npx tauri dev
```

### Kääntäminen ja niputtaminen
Itsenäisten, allekirjoitettujen tuotantoasennuspakettien luominen (MSI, EXE-asennus):
```bash
npx tauri build
```
Tuloksena olevat asennusohjelmat sijaitsevat hakemistossa "src-tauri/target/release/bundle/".

Katso [DISTRIBUTION.md](../DISTRIBUTION.md) lisätietoja pakkauksesta ja allekirjoittamisesta.
