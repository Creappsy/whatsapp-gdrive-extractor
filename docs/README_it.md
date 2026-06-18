<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Applicazione desktop nativa multipiattaforma (Windows, macOS, Linux), veloce, leggera e multilingue, per estrarre, decrittografare e ripristinare i backup di WhatsApp da Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Lingue-40_Supportato-success.svg)](#)
  [![License](https://img.shields.io/badge/Licenza-MIT-green.svg)](#)
</div>

---

## 🌟 Caratteristiche

* **Bellissima interfaccia utente/UX:** Un'interfaccia desktop moderna con design glassmorphism, modalità oscura e guida al ripristino passo passo.
* **🌍 40 lingue supportate:** Internazionalizzazione di qualità nativamente integrata. Cambia lingua istantaneamente.
* **Ultraleggero e indipendente:** Ricostruito utilizzando **Tauri e Rust**, riducendo la dimensione binaria a soli **~10 MB** con zero dipendenze esterne (non sono richiesti runtime Python o .NET sul sistema host).
* **Archiviazione sicura dei portachiavi:** Salva i tuoi token Google OAuth direttamente nel portachiavi sicuro nativo del sistema operativo (Credential Manager in Windows, Portachiavi in ​​macOS).
* **Decrittografia E2EE locale (.crypt14 / .crypt15):** Decrittazione del database WhatsApp AES-256-GCM ad alta velocità, sicura e interamente locale tramite Rust.
* **Trasferimento diretto al telefono (ADB):** Ripristina i tuoi database decrittografati direttamente sul tuo telefono Android con un solo clic, chiamando il sistema ADB in modo nativo.

---

## 📋 Requisiti

Per compilare ed eseguire dal sorgente, assicurati di avere:
1. **Ruggine e carico** (v1.77 o superiore)
2. **Node.js & npm**
3. Un dispositivo Android con debug USB abilitato (se si ripristina direttamente tramite ADB).

---

## 🚀 Esecuzione e sviluppo

### Avvio dell'applicazione
Basta fare doppio clic su [run.bat](../run.bat) file nella radice del progetto.
Si aprirà immediatamente l'eseguibile di produzione compilato (`app.exe`) o tornerà alla modalità di sviluppo se non compilato.

### In esecuzione in modalità di sviluppo
Per avviare la modalità di sviluppo con ricaricamento a caldo:
```bash
npx tauri dev
```

### Compilazione e raggruppamento
Per generare pacchetti di installazione di produzione autonomi e firmati (installazione MSI, EXE):
```bash
npx tauri build
```
I programmi di installazione risultanti si troveranno in `src-tauri/target/release/bundle/`.

Vedere [DISTRIBUTION.md](../DISTRIBUTION.md) per maggiori dettagli sulla confezione e sulla firma.
