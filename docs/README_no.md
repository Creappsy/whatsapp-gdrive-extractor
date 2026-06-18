<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Innebygd skrivebordsapplikasjon på tvers av plattformer (Windows, macOS, Linux), rask, lett og flerspråklig, for å trekke ut, dekryptere og gjenopprette WhatsApp-sikkerhetskopiene dine fra Google Disk.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Språk-40_Støttes-success.svg)](#)
  [![License](https://img.shields.io/badge/Tillatelse-MIT-green.svg)](#)
</div>

---

## 🌟 Funksjoner

* **Vakkert UI/UX:** Et moderne skrivebordsgrensesnitt med glassmorfisme-design, mørk modus og trinn-for-trinn restaureringsveiledning.
* **🌍 40 støttede språk:** Kvalitet internasjonalisering naturlig integrert. Bytt språk umiddelbart.
* **Ultralett og uavhengig:** Gjenoppbygget ved hjelp av **Tauri & Rust**, reduserer binær størrelse til bare **~10MB** med null eksterne avhengigheter (ingen Python-kjøretider eller .NET kreves på vertssystemet).
* **Sikker oppbevaring av nøkkelring:** Lagrer Google OAuth-tokenene dine direkte til operativsystemets innebygde sikre nøkkelring (Credential Manager i Windows, nøkkelring i macOS).
* **Lokal E2EE-dekryptering (.crypt14 / .crypt15):** Høyhastighets, sikker og helt lokal AES-256-GCM WhatsApp-databasedekryptering ved hjelp av Rust.
* **Direkte overføring til telefon (ADB):** Gjenopprett de dekrypterte databasene dine direkte til Android-telefonen din med et enkelt klikk, og ring systemet ADB naturlig.

---

## 📋 Krav

For å bygge og kjøre fra kilden, sørg for at du har:
1. **Rust og last** (v1.77 eller overlegen)
2. **Node.js & npm**
3. En Android-enhet med USB-feilsøking aktivert (hvis gjenoppretting direkte via ADB).

---

## 🚀 Gjennomføring og utvikling

### Starte applikasjonen
Bare dobbeltklikk på [run.bat](../run.bat) filen i roten av prosjektet.
Den vil umiddelbart åpne den kompilerte produksjonskjørbare filen (`app.exe`) eller fallback til utviklingsmodus hvis den ikke er kompilert.

### Kjører i utviklingsmodus
Slik starter du utviklingsmodus med hot-reloading:
```bash
npx tauri dev
```

### Kompilering og bunting
For å generere selvstendige, signerte produksjonsinstallasjonspakker (MSI, EXE-oppsett):
```bash
npx tauri build
```
De resulterende installasjonsprogrammene vil være plassert i `src-tauri/target/release/bundle/`.

Se [DISTRIBUTION.md](../DISTRIBUTION.md) for mer informasjon om pakking og signering.
