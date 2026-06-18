<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Логотип WhatsApp" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Сучасний, швидкий і багатомовний інструмент для видобування, розшифровки та відновлення резервних копій WhatsApp із Диска Google.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Ліцензія](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Особливості

* **Чудовий веб-інтерфейс/UX:** сучасний нативний інтерфейс у темному режимі з елементами скломорфізму, які легко допоможуть вам у процесі вилучення.
* **🌍 Підтримується 40 мов: ** 100% вбудований переклад людської якості. Змінюйте мову на льоту без перезавантаження!
* **Локальне розшифровування (.crypt14 / .crypt15):** Повністю локальний криптографічний модуль для безпечного розшифрування ваших баз даних SQLite за допомогою хеш-ключа E2EE без завантаження ваших даних куди-небудь.
* **Пряма передача Android (ADB):** Перемістіть завантажену та розшифровану базу даних безпосередньо в пам’ять телефону Android одним клацанням миші на інформаційній панелі.
* **Безпечна автентифікація:** підтримує маркери Google OAuth і паролі додатків для максимальної безпеки.

## 📋 Передумови

Перш ніж почати, переконайтеся, що у вас є:
1. Встановлено **Python 3.x** або **Docker**.
2. Пристрій Android із встановленим WhatsApp і ввімкненим резервним копіюванням Google Drive.
3. Облікові дані вашого облікового запису Google (або [пароль програми](https://myaccount.google.com/apppasswords)).
4. *Необов’язково:* Ідентифікатор Android вашого пристрою (щоб зменшити ризик виходу Google із системи).

## 🚀 Встановлення та використання

### Варіант 1: використання Python (рекомендовано для інтерфейсу користувача)

1. Клонуйте репозиторій:
   ```баш
   git клон https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd WhatsApp-gdrive-extractor
   ```

2. Встановіть необхідні залежності:
   ```баш
   pip install -r requirements.txt
   ```

3. Запустіть веб-сервер:
   ```баш
   python server.py
   ```
4. Відкрийте браузер і перейдіть до `http://localhost:5000`, щоб отримати доступ до сучасної інформаційної панелі!

### Варіант 2: використання Docker

1. Клонуйте репозиторій і перейдіть до нього.
2. Створіть образ Docker:
   ```баш
   складання докера. -t WhatsApp-gdrive-extractor
   ```
3. Запустіть контейнер Docker:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Керівництво з автентифікації

Якщо у вас виникли проблеми зі стандартною електронною поштою та паролем Google, скористайтеся методом **OAuth Token** (Крок 1 у веб-інтерфейсі):
1. Перейдіть на `https://getandroidapp.com/` або будь-який портал входу Google.
2. Увійдіть за допомогою свого облікового запису Google.
3. Натисніть `F12`, щоб відкрити інструменти розробника.
4. Перейдіть до **Додаток** -> **Файли cookie**.
5. Знайдіть `oauth_token` (зазвичай він виглядає як `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Скопіюйте та вставте його у веб-інтерфейс.

## 🤝 Подяки та подяки
* **Оригінальний автор:** TripCode
* **Основні учасники:** DrDeath1122 (багатопотокова магістраль), YuriCosta (нова реверсивна інженерія системи відновлення), macagua (виправлення SSL).
* **Модернізація та користувальницький інтерфейс:** перебудовано з сучасним веб-інтерфейсом, локальним модулем криптографії, пересиланням телефону ADB і повністю локалізовано на 40 мов для глобальної доступності.