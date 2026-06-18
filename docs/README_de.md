<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Native plattformübergreifende Desktop-Anwendung (Windows, macOS, Linux), schnell, leicht und mehrsprachig, zum Extrahieren, Entschlüsseln und Wiederherstellen Ihrer WhatsApp-Backups von Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Sprachen-40_Unterstützt-success.svg)](#)
  [![License](https://img.shields.io/badge/Lizenz-MIT-green.svg)](#)
</div>

---

## 🌟 Merkmale

* **Schöne UI/UX:** Eine moderne Desktop-Oberfläche mit Glasmorphismus-Design, Dunkelmodus und Schritt-für-Schritt-Anleitung zur Wiederherstellung.
* **🌍 40 unterstützte Sprachen:** Qualitätsinternationalisierung nativ integriert. Wechseln Sie sofort die Sprache.
* **Ultraleicht und unabhängig:** Mit **Tauri & Rust** neu erstellt, wodurch die Binärgröße auf nur **~10 MB** reduziert wurde, ohne externe Abhängigkeiten (keine Python-Laufzeiten oder .NET auf dem Hostsystem erforderlich).
* **Sichere Schlüsselbundspeicherung:** Speichert Ihre Google OAuth-Tokens direkt im nativen sicheren Schlüsselbund des Betriebssystems (Anmeldeinformationsmanager in Windows, Schlüsselbund in macOS).
* **Lokale E2EE-Entschlüsselung (.crypt14 / .crypt15):** Schnelle, sichere und vollständig lokale AES-256-GCM WhatsApp-Datenbankentschlüsselung mit Rust.
* **Direkte Übertragung zum Telefon (ADB):** Stellen Sie Ihre entschlüsselten Datenbanken mit einem einzigen Klick direkt auf Ihrem Android-Telefon wieder her und rufen Sie das System ADB nativ auf.

---

## 📋 Anforderungen

Um aus dem Quellcode zu erstellen und auszuführen, stellen Sie sicher, dass Sie über Folgendes verfügen:
1. **Rost & Fracht** (v1.77 oder überlegen)
2. **Node.js & npm**
3. Ein Android-Gerät mit aktiviertem USB-Debugging (bei direkter Wiederherstellung über ADB).

---

## 🚀 Ausführung und Entwicklung

### Starten der Anwendung
Doppelklicken Sie einfach auf [run.bat](../run.bat) Datei im Stammverzeichnis des Projekts.
Die kompilierte ausführbare Produktionsdatei („app.exe“) wird sofort geöffnet oder in den Entwicklungsmodus zurückversetzt, wenn sie nicht kompiliert ist.

### Läuft im Entwicklungsmodus
So starten Sie den Entwicklungsmodus mit Hot-Reloading:
```bash
npx tauri dev
```

### Kompilieren und Bündeln
So generieren Sie eigenständige, signierte Produktionsinstallationspakete (MSI-, EXE-Setup):
```bash
npx tauri build
```
Die resultierenden Installationsprogramme befinden sich in „src-tauri/target/release/bundle/“.

Sehen [DISTRIBUTION.md](../DISTRIBUTION.md) Weitere Informationen zum Verpacken und Signieren finden Sie hier.
