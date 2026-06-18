<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Лого на WhatsApp" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Модерен, бърз и многоезичен инструмент за извличане, декриптиране и възстановяване на вашите резервни копия на WhatsApp от Google Диск.</b></p>

  <details>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Лиценз](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Характеристики

* **Красив уеб UI/UX:** Модерен естествен интерфейс в тъмен режим с елементи на стъкломорфизма, които лесно да ви водят през процеса на извличане.
* **🌍 Поддържани 40 езика: ** 100% вградени преводи с човешко качество. Променете езика в движение без презареждане!
* **Локално декриптиране (.crypt14 / .crypt15):** Напълно локален криптографски модул за безопасно декриптиране на вашите SQLite бази данни с помощта на вашия E2EE хеш ключ, без да качвате вашите данни навсякъде.
* **Директно Android Transfer (ADB):** Прехвърлете вашата изтеглена и декриптирана база данни директно в хранилището на вашия Android телефон с едно щракване от таблото за управление.
* **Безопасна автентификация:** Поддържа както Google OAuth токени, така и пароли за приложения за максимална сигурност.

## 📋 Предпоставки

Преди да започнете, уверете се, че имате:
1. Инсталиран **Python 3.x** или **Docker**.
2. Устройство с Android с инсталиран WhatsApp и активирано архивиране на Google Drive.
3. Идентификационните данни за вашия акаунт в Google (или [парола за приложение](https://myaccount.google.com/apppasswords)).
4. *По избор:* Идентификационният номер на Android на вашето устройство (за да намалите риска Google да ви изведе).

## 🚀 Инсталиране и използване

### Вариант 1: Използване на Python (препоръчва се за потребителски интерфейс)

1. Клонирайте хранилището:
   ``` баш
   git клонинг https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Инсталирайте необходимите зависимости:
   ``` баш
   pip install -r requirements.txt
   ```

3. Стартирайте уеб сървъра:
   ``` баш
   python server.py
   ```
4. Отворете браузъра си и отидете на `http://localhost:5000` за достъп до модерното табло за управление!

### Вариант 2: Използване на Docker

1. Клонирайте хранилището и навигирайте в него.
2. Създайте изображението на Docker:
   ``` баш
   изграждане на докер. -t whatsapp-gdrive-екстрактор
   ```
3. Стартирайте Docker контейнера:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Ръководство за удостоверяване

Ако имате проблеми при използването на стандартния ви имейл и парола в Google, използвайте метода **OAuth Token** (стъпка 1 в уеб интерфейса):
1. Отидете на `https://getandroidapp.com/` или който и да е портал за влизане в Google.
2. Влезте с вашия Google акаунт.
3. Натиснете „F12“, за да отворите Инструменти за разработчици.
4. Отидете на **Приложение** -> **Бисквитки**.
5. Намерете `oauth_token` (обикновено изглежда като `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Копирайте и го поставете в уеб интерфейса.

## 🤝 Кредити и признания
* **Оригинален автор:** TripCode
* **Основни участници:** DrDeath1122 (многонишков гръбнак), YuriCosta (ново инженерство за възстановяване на системата), macagua (поправки на SSL).
* **Модернизация и потребителски интерфейс:** Възстановен с модерен уеб интерфейс, локален модул за криптография, ADB телефонен трансфер и напълно локализиран на 40 езика за глобална достъпност.