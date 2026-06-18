<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Native platformonafhankelijke desktop-applicatie (Windows, macOS, Linux), snel, lichtgewicht en meertalig, om uw WhatsApp-back-ups uit Google Drive te extraheren, decoderen en herstellen.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Talen-40_Ondersteund-success.svg)](#)
  [![License](https://img.shields.io/badge/Licentie-MIT-green.svg)](#)
</div>

---

## 🌟 Functies

* **Prachtige gebruikersinterface/UX:** Een moderne desktopinterface met glasmorfisme-ontwerp, donkere modus en stapsgewijze restauratiegids.
* **🌍 40 ondersteunde talen:** Kwaliteitsinternationalisering van nature geïntegreerd. Schakel onmiddellijk van taal.
* **Ultralicht en onafhankelijk:** Herbouwd met behulp van **Tauri & Rust**, waardoor de binaire grootte wordt teruggebracht tot slechts **~10MB** zonder externe afhankelijkheden (geen Python-runtimes of .NET vereist op het hostsysteem).
* **Veilige sleutelhangeropslag:** Slaat uw Google OAuth-tokens rechtstreeks op in de eigen beveiligde sleutelhanger van het besturingssysteem (Credential Manager in Windows, Keychain in macOS).
* **Lokale E2EE-decodering (.crypt14 / .crypt15):** Snelle, veilige en volledig lokale AES-256-GCM WhatsApp-databasedecodering met behulp van Rust.
* **Directe overdracht naar telefoon (ADB):** Herstel uw gedecodeerde databases rechtstreeks naar uw Android-telefoon met een enkele klik, waarbij u systeemeigen ADB oproept.

---

## 📋 Vereisten

Als u vanaf de broncode wilt bouwen en uitvoeren, moet u ervoor zorgen dat u over het volgende beschikt:
1. **Roest en lading** (v1.77 of superieur)
2. **Node.js & npm**
3. Een Android-apparaat waarop USB-foutopsporing is ingeschakeld (bij rechtstreeks herstel via ADB).

---

## 🚀 Uitvoering en ontwikkeling

### Het starten van de applicatie
Dubbelklik eenvoudigweg op de [run.bat](../run.bat) bestand in de root van het project.
Het zal onmiddellijk het gecompileerde uitvoerbare productiebestand (`app.exe`) openen of terugvallen naar de ontwikkelingsmodus als het niet is gecompileerd.

### Wordt uitgevoerd in de ontwikkelingsmodus
Om de ontwikkelingsmodus te starten met hot-reloading:
```bash
npx tauri dev
```

### Samenstellen en bundelen
Op zichzelf staande, ondertekende productie-installatiepakketten genereren (MSI, EXE-installatie):
```bash
npx tauri build
```
De resulterende installatieprogramma's bevinden zich in `src-tauri/target/release/bundle/`.

Zien [DISTRIBUTION.md](../DISTRIBUTION.md) voor meer informatie over verpakking en ondertekening.
