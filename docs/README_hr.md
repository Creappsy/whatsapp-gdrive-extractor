<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Izvorna stolna aplikacija za više platformi (Windows, macOS, Linux), brza, lagana i višejezična za izdvajanje, dešifriranje i vraćanje vaših WhatsApp sigurnosnih kopija s Google diska.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/jezici-40_Podržano-success.svg)](#)
  [![License](https://img.shields.io/badge/Licenca-MIT-green.svg)](#)
</div>

---

## 🌟 Značajke

* **Prekrasan UI/UX:** Moderno sučelje za radnu površinu sa staklomorfnim dizajnom, tamnim načinom rada i vodičem za restauraciju korak po korak.
* **🌍 40 podržanih jezika:** Kvalitetna internacionalizacija izvorno integrirana. Trenutačno mijenjajte jezike.
* **Ultralaki i neovisni:** Ponovno izgrađen pomoću **Tauri & Rust**, smanjujući binarnu veličinu na samo **~10MB** s nula vanjskih ovisnosti (na glavnom sustavu nisu potrebna Python runtimes ili .NET).
* **Sigurna pohrana ključeva:** Sprema vaše Google OAuth tokene izravno u izvorni sigurnosni privjesak operativnog sustava (Credential Manager u Windowsima, Keychain u macOS-u).
* **Lokalno E2EE dešifriranje (.crypt14 / .crypt15):** Brzo, sigurno i potpuno lokalno AES-256-GCM dešifriranje WhatsApp baze podataka pomoću Rusta.
* **Izravni prijenos na telefon (ADB):** Vratite svoje dekriptirane baze podataka izravno na svoj Android telefon jednim klikom, izvorno pozivajući sustav ADB.

---

## 📋 Zahtjevi

Za izgradnju i pokretanje iz izvora, provjerite imate li:
1. **Hrđa i teret** (v1.77 ili nadređeni)
2. **Node.js & npm**
3. Android uređaj s omogućenim USB ispravljanjem pogrešaka (ako vraćate izravno putem ADB-a).

---

## 🚀 Izvođenje i razvoj

### Pokretanje aplikacije
Jednostavno dvaput kliknite na [run.bat](../run.bat) datoteku u korijenu projekta.
Odmah će otvoriti kompajliranu produkcijsku izvršnu datoteku (`app.exe`) ili se vratiti u razvojni mod ako nije kompilirana.

### Pokretanje u razvojnom načinu
Za pokretanje razvojnog moda s vrućim ponovnim učitavanjem:
```bash
npx tauri dev
```

### Sastavljanje i grupiranje
Za generiranje samostalnih, potpisanih proizvodnih instalacijskih paketa (MSI, EXE postava):
```bash
npx tauri build
```
Rezultirajući instalacijski programi bit će smješteni u `src-tauri/target/release/bundle/`.

Vidjeti [DISTRIBUTION.md](../DISTRIBUTION.md) za više detalja o pakiranju i potpisivanju.
