<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Native cross-platform desktop application (Windows, macOS, Linux), mabilis, magaan, at multi-language, upang i-extract, i-decrypt, at i-restore ang iyong mga backup sa WhatsApp mula sa Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Mga_wika-40_Sinusuportahan-success.svg)](#)
  [![License](https://img.shields.io/badge/Lisensya-MIT-green.svg)](#)
</div>

---

## 🌟 Mga tampok

* **Magandang UI/UX:** Isang modernong desktop interface na may disenyong glassmorphism, dark mode, at step-by-step na gabay sa pagpapanumbalik.
* **🌍 40 Mga Sinusuportahang Wika:** Ang kalidad na internasyonalisasyon ay katutubong isinama. Agad na lumipat ng mga wika.
* **Ultralight at Independent:** Muling itinayo gamit ang **Tauri & Rust**, na binabawasan ang laki ng binary sa **~10MB** lang na walang mga external na dependency (walang Python runtime o .NET na kinakailangan sa host system).
* **Secure na Keyring Storage:** Direktang sine-save ang iyong mga Google OAuth token sa native secure na keyring ng operating system (Credential Manager sa Windows, Keychain sa macOS).
* **Lokal na E2EE Decryption (.crypt14 / .crypt15):** High-speed, secure, at ganap na lokal na AES-256-GCM WhatsApp database decryption gamit ang Rust.
* **Direktang Paglipat sa Telepono (ADB):** Ibalik ang iyong mga na-decrypt na database nang direkta sa iyong Android phone sa isang pag-click, ang sistema ng pagtawag sa ADB nang katutubong.

---

## 📋 Mga kinakailangan

Upang bumuo at tumakbo mula sa pinagmulan, tiyaking mayroon kang:
1. **kalawang at Cargo** (v1.77 o superior)
2. **Node.js & npm**
3. Isang Android device na may naka-enable na USB Debugging (kung direktang nagre-restore sa pamamagitan ng ADB).

---

## 🚀 Pagpapatupad at Pagpapaunlad

### Paglulunsad ng Application
I-double click lang ang [run.bat](../run.bat) file sa ugat ng proyekto.
Agad nitong bubuksan ang pinagsama-samang production executable (`app.exe`) o fallback sa development mode kung hindi na-compile.

### Tumatakbo sa Development Mode
Upang simulan ang development mode gamit ang hot-reloading:
```bash
npx tauri dev
```

### Pag-iipon at Pag-bundle
Upang bumuo ng mga self-contained, nilagdaang production installer packages (MSI, EXE setup):
```bash
npx tauri build
```
Ang mga resultang installer ay makikita sa `src-tauri/target/release/bundle/`.

Tingnan mo [DISTRIBUTION.md](../DISTRIBUTION.md) para sa higit pang mga detalye sa packaging at pagpirma.
