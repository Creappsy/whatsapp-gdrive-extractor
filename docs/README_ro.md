<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Aplicație desktop nativă multiplatformă (Windows, macOS, Linux), rapidă, ușoară și multilingvă, pentru a extrage, decripta și restabili backup-urile WhatsApp din Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Limbi-40_Sprijinit-success.svg)](#)
  [![License](https://img.shields.io/badge/Licenţă-MIT-green.svg)](#)
</div>

---

## 🌟 Caracteristici

* **UI/UX frumos:** O interfață desktop modernă cu design de sticlă, mod întunecat și ghid pas cu pas pentru restaurare.
* **🌍 40 de limbi acceptate:** Internaționalizare de calitate integrată nativ. Schimbați limbile instantaneu.
* **Ultraușoară și independentă:** Reconstruit folosind **Tauri & Rust**, reducând dimensiunea binară la doar **~10MB** cu zero dependențe externe (nu sunt necesare runtime Python sau .NET pe sistemul gazdă).
* **Depozitare securizată pentru chei:** Salvează jetoanele dvs. Google OAuth direct în breloul de chei securizat nativ al sistemului de operare (Credential Manager în Windows, Keychain în macOS).
* **Decriptare locală E2EE (.crypt14 / .crypt15):** Decriptare de mare viteză, sigură și în întregime locală AES-256-GCM a bazei de date WhatsApp folosind Rust.
* **Transfer direct la telefon (ADB):** Restaurați-vă bazele de date decriptate direct pe telefonul dvs. Android cu un singur clic, apelând sistemul ADB în mod nativ.

---

## 📋 Cerințe

Pentru a construi și a rula de la sursă, asigurați-vă că aveți:
1. **Rugina și încărcătură** (v1.77 sau superior)
2. **Node.js & npm**
3. Un dispozitiv Android cu depanare USB activată (dacă se restaurează direct prin ADB).

---

## 🚀 Execuție și Dezvoltare

### Lansarea aplicației
Pur și simplu faceți dublu clic pe [run.bat](../run.bat) fișier la rădăcina proiectului.
Acesta va deschide instantaneu executabilul de producție compilat (`app.exe`) sau va reveni la modul de dezvoltare dacă nu este compilat.

### Rulează în modul de dezvoltare
Pentru a porni modul de dezvoltare cu reîncărcare la cald:
```bash
npx tauri dev
```

### Compilarea și gruparea
Pentru a genera pachete de instalare de producție autonome și semnate (configurare MSI, EXE):
```bash
npx tauri build
```
Programele de instalare rezultate vor fi localizate în `src-tauri/target/release/bundle/`.

Vedea [DISTRIBUTION.md](../DISTRIBUTION.md) pentru mai multe detalii despre ambalare și semnare.
