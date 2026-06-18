<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Родно междуплатформено настолно приложение (Windows, macOS, Linux), бързо, леко и многоезично за извличане, декриптиране и възстановяване на вашите резервни копия на WhatsApp от Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Езици-40_Поддържа_се-success.svg)](#)
  [![License](https://img.shields.io/badge/Лиценз-MIT-green.svg)](#)
</div>

---

## 🌟 Характеристики

* **Красив UI/UX:** Модерен интерфейс за настолен компютър с дизайн на стъкломорфизъм, тъмен режим и ръководство за възстановяване стъпка по стъпка.
* **🌍 40 поддържани езика:** Интегрирана качествена интернационализация. Превключвайте езиците моментално.
* **Свръхлеки и независими:** Възстановен с помощта на **Tauri & Rust**, намалявайки двоичния размер до само **~10MB** с нулеви външни зависимости (не се изисква време за изпълнение на Python или .NET на хост системата).
* **Сигурно съхранение на ключодържател:** Записва вашите Google OAuth токени директно в родния защитен ключодържател на операционната система (Credential Manager в Windows, Keychain в macOS).
* **Локално E2EE декриптиране (.crypt14 / .crypt15):** Високоскоростно, сигурно и изцяло локално AES-256-GCM декриптиране на база данни на WhatsApp с помощта на Rust.
* **Директен трансфер към телефон (ADB):** Възстановете вашите декриптирани бази данни директно на вашия телефон с Android с едно щракване, извиквайки системата ADB първоначално.

---

## 📋 Изисквания

За да изградите и стартирате от източника, уверете се, че имате:
1. **Ръжда и товари** (v1.77 или превъзходен)
2. **Node.js & npm**
3. Устройство с Android с активирано USB отстраняване на грешки (ако се възстановява директно чрез ADB).

---

## 🚀 Изпълнение и развитие

### Стартиране на приложението
Просто щракнете двукратно върху [run.bat](../run.bat) файл в основата на проекта.
Той незабавно ще отвори компилирания производствен изпълним файл (`app.exe`) или ще се върне към режим на разработка, ако не е компилиран.

### Работи в режим на разработка
За да стартирате режим на разработка с горещо презареждане:
```bash
npx tauri dev
```

### Компилиране и групиране
За генериране на самостоятелни, подписани производствени инсталационни пакети (MSI, EXE настройка):
```bash
npx tauri build
```
Получените инсталатори ще бъдат разположени в `src-tauri/target/release/bundle/`.

Вижте [DISTRIBUTION.md](../DISTRIBUTION.md) за повече подробности относно опаковката и подписването.
