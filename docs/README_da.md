<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Native cross-platform desktop-applikation (Windows, macOS, Linux), hurtig, let og flersproget, til at udtrække, dekryptere og gendanne dine WhatsApp-sikkerhedskopier fra Google Drev.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Sprog-40_Understøttet-success.svg)](#)
  [![License](https://img.shields.io/badge/Licens-MIT-green.svg)](#)
</div>

---

## 🌟 Funktioner

* **Smuk UI/UX:** En moderne desktop-grænseflade med glasmorfi-design, mørk tilstand og trin-for-trin restaureringsvejledning.
* **🌍 40 understøttede sprog:** Kvalitetsinternationalisering integreret. Skift sprog med det samme.
* **Ultralet og uafhængig:** Genopbygget ved hjælp af **Tauri & Rust**, hvilket reducerer binær størrelse til kun **~10MB** med nul eksterne afhængigheder (ingen Python-runtime eller .NET påkrævet på værtssystemet).
* **Sikker opbevaring af nøglering:** Gemmer dine Google OAuth-tokens direkte i operativsystemets native sikre nøglering (Credential Manager i Windows, Keychain i macOS).
* **Lokal E2EE-dekryptering (.crypt14 / .crypt15):** Højhastigheds, sikker og helt lokal AES-256-GCM WhatsApp-databasedekryptering ved hjælp af Rust.
* **Direkte overførsel til telefon (ADB):** Gendan dine dekrypterede databaser direkte til din Android-telefon med et enkelt klik, og kalder systemet ADB indbygget.

---

## 📋 Krav

For at bygge og køre fra kilden skal du sørge for at have:
1. **Rust & last** (v1.77 eller overlegen)
2. **Node.js & npm**
3. En Android-enhed med USB-fejlretning aktiveret (hvis gendannelse direkte via ADB).

---

## 🚀 Udførelse og udvikling

### Start af applikationen
Du skal blot dobbeltklikke på [run.bat](../run.bat) fil i roden af ​​projektet.
Det vil øjeblikkeligt åbne den kompilerede produktionseksekverbare (`app.exe`) eller fallback til udviklingstilstand, hvis den ikke er kompileret.

### Kører i udviklingstilstand
Sådan starter du udviklingstilstand med hot-genindlæsning:
```bash
npx tauri dev
```

### Kompilering og bundling
For at generere selvstændige, signerede produktionsinstallationspakker (MSI, EXE-opsætning):
```bash
npx tauri build
```
De resulterende installationsprogrammer vil blive placeret i `src-tauri/target/release/bundle/`.

Se [DISTRIBUTION.md](../DISTRIBUTION.md) for flere detaljer om pakning og underskrift.
