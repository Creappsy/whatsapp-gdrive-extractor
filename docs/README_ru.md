<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Собственное кроссплатформенное настольное приложение (Windows, macOS, Linux), быстрое, легкое и многоязычное для извлечения, расшифровки и восстановления резервных копий WhatsApp с Google Диска.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Языки-40_Поддерживается-success.svg)](#)
  [![License](https://img.shields.io/badge/Лицензия-MIT-green.svg)](#)
</div>

---

## 🌟 Функции

* **Красивый UI/UX:** Современный интерфейс рабочего стола с дизайном Glassmorphism, темным режимом и пошаговым руководством по восстановлению.
* **🌍 40 поддерживаемых языков:** Интернационализация качества изначально интегрирована. Переключайте языки мгновенно.
* **Сверхлегкий и независимый:** Пересобран с использованием **Tauri & Rust**, размер двоичного файла уменьшен до **~10 МБ** с нулевыми внешними зависимостями (в хост-системе не требуется среда выполнения Python или .NET).
* **Безопасное хранение ключей:** Сохраняет ваши токены Google OAuth непосредственно в собственный безопасный набор ключей операционной системы (Диспетчер учетных данных в Windows, Связка ключей в macOS).
* **Локальное расшифрование E2EE (.crypt14/.crypt15):** Высокоскоростное, безопасное и полностью локальное расшифрование базы данных WhatsApp AES-256-GCM с использованием Rust.
* **Прямой перевод на телефон (ADB):** Восстановите свои расшифрованные базы данных прямо на свой телефон Android одним щелчком мыши, вызывая систему ADB.

---

## 📋 Требования

Для сборки и запуска из исходного кода убедитесь, что у вас есть:
1. **Ржавчина и груз** (v1.77 или выше)
2. **Node.js & npm**
3. Устройство Android с включенной отладкой по USB (при восстановлении напрямую через ADB).

---

## 🚀 Исполнение и развитие

### Запуск приложения
Просто дважды щелкните значок [run.bat](../run.bat) файл в корне проекта.
Он мгновенно откроет скомпилированный рабочий исполняемый файл («app.exe») или вернется в режим разработки, если он не скомпилирован.

### Запуск в режиме разработки
Чтобы запустить режим разработки с горячей перезагрузкой:
```bash
npx tauri dev
```

### Компиляция и объединение
Чтобы создать автономные подписанные пакеты производственного установщика (установка MSI, EXE):
```bash
npx tauri build
```
Полученные установщики будут расположены в `src-tauri/target/release/bundle/`.

Видеть [DISTRIBUTION.md](../DISTRIBUTION.md) для получения более подробной информации об упаковке и подписании.
