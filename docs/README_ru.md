<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Логотип WhatsApp" width="100"/>
  <h1>Распаковщик WhatsApp Google Диска 🚀</h1>
  <p><b>Современный, быстрый и многоязычный инструмент для извлечения, расшифровки и восстановления резервных копий WhatsApp с Google Диска.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Лицензия](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Особенности

* **Красивый веб-интерфейс/UX:** Современный встроенный интерфейс в темном режиме с элементами гласморфизма, которые помогут вам легко пройти процесс извлечения.
* **🌍 Поддерживается 40 языков:** 100% встроенные переводы человеческого качества. Меняйте язык на лету без перезагрузки!
* **Локальное дешифрование (.crypt14 / .crypt15):** полностью локальный модуль шифрования для безопасного расшифрования ваших баз данных SQLite с использованием хеш-ключа E2EE без загрузки данных куда-либо.
* **Прямая передача Android (ADB):** Перенесите загруженную и расшифрованную базу данных прямо в хранилище вашего телефона Android одним щелчком мыши на панели управления.
* **Безопасная аутентификация:** поддерживает токены Google OAuth и пароли приложений для максимальной безопасности.

## 📋 Предварительные условия

Прежде чем начать, убедитесь, что у вас есть:
1. Установлен **Python 3.x** или **Docker**.
2. Устройство Android с установленным WhatsApp и включенным резервным копированием на Google Диск.
3. Учетные данные вашей учетной записи Google (или [Пароль приложения](https://myaccount.google.com/apppasswords)).
4. *Необязательно:* Android ID вашего устройства (чтобы снизить риск выхода из системы Google).

## 🚀 Установка и использование

### Вариант 1. Использование Python (рекомендуется для пользовательского интерфейса)

1. Клонируйте репозиторий:
   ``` баш
   git-клон https://github.com/daferferso/whatsapp-gdrive-extractor.git
   компакт-диск WhatsApp-GDrive-Extractor
   ```

2. Установите необходимые зависимости:
   ``` баш
   pip install -r требования.txt
   ```

3. Запустите веб-сервер:
   ``` баш
   python server.py
   ```
4. Откройте браузер и перейдите по адресу http://localhost:5000, чтобы получить доступ к современной панели управления!

### Вариант 2. Использование Docker

1. Клонируйте репозиторий и перейдите в него.
2. Создайте образ Docker:
   ``` баш
   сборка докера. -t WhatsApp-gdrive-extractor
   ```
3. Запустите Docker-контейнер:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it WhatsApp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it WhatsApp-gdrive-extractor`

## 🔑 Руководство по аутентификации

Если у вас возникли проблемы с использованием стандартного адреса электронной почты и пароля Google, используйте метод **Токен OAuth** (шаг 1 в веб-интерфейсе):
1. Перейдите на https://getandroidapp.com/ или на любой портал Google.
2. Войдите в свою учетную запись Google.
3. Нажмите «F12», чтобы открыть инструменты разработчика.
4. Откройте **Приложение** -> **Файлы cookie**.
5. Найдите `oauth_token` (обычно он выглядит как `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Скопируйте и вставьте его в веб-интерфейс.

## 🤝 Благодарности и благодарности
* **Автор оригинала:** TripCode
* **Основные участники:** DrDeath1122 (многопоточная магистральная сеть), ЮриКоста (реверс-инжиниринг новой системы восстановления), macagua (исправления SSL).
* **Модернизация и пользовательский интерфейс:** переработан с использованием современного веб-интерфейса, локального модуля шифрования, передачи данных по телефону ADB и полностью локализован на 40 языков для обеспечения глобальной доступности.
