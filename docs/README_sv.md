<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Inbyggd plattformsoberoende skrivbordsapplikation (Windows, macOS, Linux), snabb, lätt och flerspråkig, för att extrahera, dekryptera och återställa dina WhatsApp-säkerhetskopior från Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Språk-40_Stöds-success.svg)](#)
  [![License](https://img.shields.io/badge/Licens-MIT-green.svg)](#)
</div>

---

## 🌟 Drag

* **Vackert UI/UX:** Ett modernt skrivbordsgränssnitt med design av glasmorfism, mörkt läge och steg-för-steg-restaureringsguide.
* **🌍 40 språk som stöds:** Kvalitetsinternationalisering integrerat. Byt språk direkt.
* **Ultralätt och oberoende:** Ombyggd med **Tauri & Rust**, vilket minskar binär storlek till bara **~10MB** med noll externa beroenden (inga Python-körtider eller .NET krävs på värdsystemet).
* **Säker nyckelringsförvaring:** Sparar dina Google OAuth-tokens direkt till operativsystemets inbyggda säkra nyckelring (Credential Manager i Windows, Keychain i macOS).
* **Lokal E2EE-dekryptering (.crypt14 / .crypt15):** Höghastighets, säker och helt lokal AES-256-GCM WhatsApp-databasdekryptering med hjälp av Rust.
* **Direktöverföring till telefon (ADB):** Återställ dina dekrypterade databaser direkt till din Android-telefon med ett enda klick och ring systemet ADB inbyggt.

---

## 📋 Krav

För att bygga och köra från källan, se till att du har:
1. **Rust & last** (v1.77 eller överlägsen)
2. **Node.js & npm**
3. En Android-enhet med USB Debugging aktiverad (om återställning direkt via ADB).

---

## 🚀 Utförande och utveckling

### Starta applikationen
Dubbelklicka helt enkelt på [run.bat](../run.bat) filen i roten av projektet.
Den kommer omedelbart att öppna den kompilerade produktionskörbara filen (`app.exe`) eller fallback till utvecklingsläge om den inte kompileras.

### Körs i utvecklingsläge
Så här startar du utvecklingsläge med varmladdning:
```bash
npx tauri dev
```

### Sammanställning och buntning
Så här genererar du fristående, signerade produktionsinstallationspaket (MSI, EXE-installation):
```bash
npx tauri build
```
De resulterande installationsprogrammen kommer att finnas i `src-tauri/target/release/bundle/`.

Se [DISTRIBUTION.md](../DISTRIBUTION.md) för mer information om förpackning och signering.
