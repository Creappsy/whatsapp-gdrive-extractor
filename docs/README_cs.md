<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Nativní multiplatformní desktopová aplikace (Windows, macOS, Linux), rychlá, lehká a vícejazyčná, k extrahování, dešifrování a obnově vašich záloh WhatsApp z Disku Google.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Jazyky-40_Podporováno-success.svg)](#)
  [![License](https://img.shields.io/badge/Licence-MIT-green.svg)](#)
</div>

---

## 🌟 Vlastnosti

* **Krásné UI/UX:** Moderní desktopové rozhraní s designem glassmorphism, tmavým režimem a průvodcem obnovení krok za krokem.
* **🌍 40 podporovaných jazyků:** Kvalitní internacionalizace nativně integrovaná. Okamžitě přepínejte jazyky.
* **Ultralehký a nezávislý:** Přestavěno pomocí **Tauri & Rust**, zmenšení binární velikosti na pouhých **~10 MB** s nulovými externími závislostmi (na hostitelském systému nejsou vyžadovány žádné běhové moduly Python nebo .NET).
* **Zabezpečené úložiště klíčů:** Uloží vaše tokeny Google OAuth přímo do nativního zabezpečeného svazku klíčů operačního systému (Credential Manager ve Windows, Keychain v macOS).
* **Místní dešifrování E2EE (.crypt14 / .crypt15):** Vysokorychlostní, bezpečné a zcela lokální dešifrování databáze WhatsApp AES-256-GCM pomocí Rust.
* **Přímý přenos do telefonu (ADB):** Obnovte své dešifrované databáze přímo do telefonu Android jediným kliknutím a nativním voláním systému ADB.

---

## 📋 Požadavky

Chcete-li sestavit a spustit ze zdroje, ujistěte se, že máte:
1. **Rust & Cargo** (v1.77 nebo nadřízený)
2. **Node.js & npm**
3. Zařízení Android s povoleným laděním USB (pokud provádíte obnovu přímo přes ADB).

---

## 🚀 Provedení a vývoj

### Spuštění aplikace
Jednoduše dvakrát klikněte na [run.bat](../run.bat) soubor v kořenovém adresáři projektu.
Okamžitě otevře zkompilovaný produkční spustitelný soubor (`app.exe`) nebo se vrátí do vývojového režimu, pokud není zkompilován.

### Běží ve vývojovém režimu
Chcete-li spustit vývojový režim s hot-reloading:
```bash
npx tauri dev
```

### Kompilace a sdružování
Generování samostatných, podepsaných produkčních instalačních balíčků (MSI, EXE nastavení):
```bash
npx tauri build
```
Výsledné instalační programy budou umístěny v `src-tauri/target/release/bundle/`.

Vidět [DISTRIBUTION.md](../DISTRIBUTION.md) pro více podrobností o balení a podepisování.
