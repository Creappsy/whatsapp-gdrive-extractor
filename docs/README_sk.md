<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Natívna multiplatformová aplikácia pre stolné počítače (Windows, macOS, Linux), rýchla, ľahká a viacjazyčná na extrahovanie, dešifrovanie a obnovenie vašich záloh WhatsApp z Disku Google.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Jazyky-40_Podporované-success.svg)](#)
  [![License](https://img.shields.io/badge/Licencia-MIT-green.svg)](#)
</div>

---

## 🌟 Vlastnosti

* **Krásne UI/UX:** Moderné desktopové rozhranie s dizajnom glassmorphism, tmavým režimom a podrobným sprievodcom obnovy.
* **🌍 40 podporovaných jazykov:** Natívne integrovaná internacionalizácia kvality. Okamžite prepínajte jazyky.
* **Ultraľahký a nezávislý:** Prestavané pomocou **Tauri & Rust**, čím sa zmenšila binárna veľkosť len na **~10 MB** s nulovými externými závislosťami (na hostiteľskom systéme nie sú potrebné žiadne runtime Python alebo .NET).
* **Bezpečné úložisko kľúčov:** Uloží vaše tokeny Google OAuth priamo do natívneho bezpečného zväzku kľúčov operačného systému (Správca poverení v systéme Windows, Kľúčenka v systéme MacOS).
* **Lokálne E2EE dešifrovanie (.crypt14 / .crypt15):** Vysokorýchlostné, bezpečné a úplne lokálne dešifrovanie databázy WhatsApp AES-256-GCM pomocou Rust.
* **Priamy prenos do telefónu (ADB):** Obnovte svoje dešifrované databázy priamo do telefónu s Androidom jediným kliknutím a natívne zavolajte systém ADB.

---

## 📋 Požiadavky

Ak chcete zostaviť a spustiť zo zdroja, uistite sa, že máte:
1. **Hrdza a náklad** (v1.77 alebo nadriadený)
2. **Node.js & npm**
3. Zariadenie so systémom Android s povoleným ladením USB (ak sa obnovuje priamo cez ADB).

---

## 🚀 Realizácia a rozvoj

### Spustenie aplikácie
Jednoducho dvakrát kliknite na [run.bat](../run.bat) súbor v koreňovom adresári projektu.
Okamžite otvorí kompilovaný produkčný spustiteľný súbor (`app.exe`) alebo sa vráti do vývojového režimu, ak nie je skompilovaný.

### Beží v režime vývoja
Ak chcete spustiť vývojový režim s hot-reloading:
```bash
npx tauri dev
```

### Zostavovanie a viazanie
Ak chcete vygenerovať samostatné, podpísané produkčné inštalačné balíky (MSI, EXE nastavenie):
```bash
npx tauri build
```
Výsledné inštalačné programy budú umiestnené v `src-tauri/target/release/bundle/`.

Pozri [DISTRIBUTION.md](../DISTRIBUTION.md) pre viac podrobností o balení a podpisovaní.
