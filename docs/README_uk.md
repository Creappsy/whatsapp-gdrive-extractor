<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Власна міжплатформна настільна програма (Windows, macOS, Linux), швидка, легка та багатомовна для вилучення, розшифровки та відновлення резервних копій WhatsApp із Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Мови-40_Підтримується-success.svg)](#)
  [![License](https://img.shields.io/badge/Ліцензія-MIT-green.svg)](#)
</div>

---

## 🌟 особливості

* **Гарний UI/UX:** Сучасний інтерфейс настільного комп’ютера зі скломорфізмом, темним режимом і покроковим посібником із відновлення.
* **🌍 40 підтримуваних мов:** Інтегрована якісна інтернаціоналізація. Перемикайте мови миттєво.
* **Надлегкий і незалежний:** Перебудовано за допомогою **Tauri & Rust**, зменшуючи двійковий розмір лише до **~10 МБ** із нульовими зовнішніми залежностями (у хост-системі не потрібні середовища виконання Python або .NET).
* **Безпечне зберігання ключів:** Зберігає ваші токени Google OAuth безпосередньо у рідному захищеному брелоку операційної системи (Credential Manager у Windows, Keychain у macOS).
* **Локальна розшифровка E2EE (.crypt14 / .crypt15):** Високошвидкісне, безпечне та повністю локальне дешифрування бази даних WhatsApp AES-256-GCM за допомогою Rust.
* **Прямий переказ на телефон (ADB):** Відновіть розшифровані бази даних безпосередньо на телефоні Android одним клацанням миші, викликавши системний ADB.

---

## 📋 Вимоги

Щоб створити та запустити з джерела, переконайтеся, що у вас є:
1. **Іржа та вантажі** (v1.77 або вище)
2. **Node.js & npm**
3. Пристрій Android із увімкненим USB Debugging (якщо відновлення здійснюється безпосередньо через ADB).

---

## 🚀 Виконання та розробка

### Запуск програми
Просто двічі клацніть на [run.bat](../run.bat) файл у корені проекту.
Він миттєво відкриє скомпільований робочий виконуваний файл (`app.exe`) або повернеться до режиму розробки, якщо не скомпільовано.

### Запуск у режимі розробки
Щоб запустити режим розробки з гарячим перезавантаженням:
```bash
npx tauri dev
```

### Компіляція та об’єднання
Щоб створити самостійні підписані робочі пакети встановлення (MSI, налаштування EXE):
```bash
npx tauri build
```
Отримані інсталятори будуть розташовані в `src-tauri/target/release/bundle/`.

див [DISTRIBUTION.md](../DISTRIBUTION.md) детальніше про упаковку та підпис.
