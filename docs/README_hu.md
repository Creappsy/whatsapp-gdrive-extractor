<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Natív, többplatformos asztali alkalmazás (Windows, macOS, Linux), gyors, könnyű és többnyelvű a WhatsApp biztonsági másolatainak kibontásához, visszafejtéséhez és visszaállításához a Google Drive-ból.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Nyelvek-40_Támogatott-success.svg)](#)
  [![License](https://img.shields.io/badge/Engedély-MIT-green.svg)](#)
</div>

---

## 🌟 Jellemzők

* **Gyönyörű felhasználói felület/UX:** Modern asztali felület üvegmorfizmussal, sötét móddal és lépésről lépésre történő helyreállítási útmutatóval.
* **🌍 40 támogatott nyelv:** A minőségi nemzetközivé válás natívan integrálva. Nyelvváltás azonnal.
* **Ultrakönnyű és független:** A **Tauri & Rust** használatával újraépítve, mindössze **~10 MB**-ra csökkentve a bináris méretet, külső függőség nélkül (nincs szükség Python futtatókörnyezetre vagy .NET-re a gazdagépen).
* **Biztonságos kulcstartó tárolás:** A Google OAuth-jogkivonatokat közvetlenül az operációs rendszer natív biztonságos kulcstartójába menti (Windows rendszerben a Credential Manager, macOS rendszerben a Keychain).
* **Helyi E2EE visszafejtés (.crypt14 / .crypt15):** Nagy sebességű, biztonságos és teljesen helyi AES-256-GCM WhatsApp-adatbázis visszafejtése Rust használatával.
* **Közvetlen átvitel telefonra (ADB):** Állítsa vissza visszafejtett adatbázisait közvetlenül Android telefonjára egyetlen kattintással, natív módon hívva a rendszer ADB-t.

---

## 📋 Követelmények

A forrásból való építéshez és futtatáshoz győződjön meg arról, hogy rendelkezik:
1. **Rust & Cargo** (v1.77 vagy felsőbbrendű)
2. **Node.js & npm**
3. Android-eszköz, amelyen engedélyezve van az USB hibakeresés (ha közvetlenül az ADB-n keresztül történik visszaállítás).

---

## 🚀 Kivitelezés és fejlesztés

### Az Alkalmazás indítása
Egyszerűen kattintson duplán a [run.bat](../run.bat) fájlt a projekt gyökerében.
Azonnal megnyitja a lefordított éles futtatható fájlt ("app.exe"), vagy visszaáll a fejlesztési módba, ha nincs lefordítva.

### Fejlesztési módban fut
A fejlesztési mód elindítása forró újratöltéssel:
```bash
npx tauri dev
```

### Összeállítás és kötegelés
Önálló, aláírt éles telepítőcsomagok létrehozása (MSI, EXE beállítás):
```bash
npx tauri build
```
Az eredményül kapott telepítők az `src-tauri/target/release/bundle/` helyen lesznek.

Lásd [DISTRIBUTION.md](../DISTRIBUTION.md) a csomagolással és aláírással kapcsolatos további részletekért.
