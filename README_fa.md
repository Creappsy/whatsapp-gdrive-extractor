<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="لوگوی واتساپ" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>ابزاری مدرن، سریع و چند زبانه برای استخراج، رمزگشایی و بازیابی نسخه‌های پشتیبان WhatsApp شما از Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![License](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 ویژگی ها

* ** UI/UX وب زیبا:** یک رابط بومی مدرن با حالت تاریک با عناصر شیشه‌ای که به راحتی شما را در فرآیند استخراج راهنمایی می‌کند.
* **🌍 40 زبان پشتیبانی شده: ** ترجمه های 100٪ با کیفیت انسانی به صورت بومی ساخته شده است. تغییر زبان در حین پرواز بدون بارگیری مجدد!
* **رمزگشایی محلی (crypt14 / .crypt15):** یک ماژول رمزنگاری کاملاً محلی برای رمزگشایی ایمن پایگاه داده های SQLite شما با استفاده از کلید هش E2EE بدون آپلود داده های شما در هر مکانی.
* **انتقال مستقیم اندروید (ADB):** پایگاه داده دانلود شده و رمزگشایی شده خود را مستقیماً با یک کلیک از داشبورد به حافظه تلفن Android خود فشار دهید.
* **تأیید هویت ایمن:** از توکن های Google OAuth و رمزهای عبور برنامه برای حداکثر امنیت پشتیبانی می کند.

## 📋 پیش نیازها

قبل از شروع، مطمئن شوید که:
1. **Python 3.x** یا **Docker** نصب شده است.
2. یک دستگاه اندرویدی که واتس اپ نصب شده و پشتیبان گیری گوگل درایو فعال است.
3. اطلاعات کاربری حساب Google شما (یا یک [گذرواژه برنامه] (https://myaccount.google.com/apppasswords)).
4. *اختیاری:* شناسه اندروید دستگاه شما (برای کاهش خطر خروج گوگل از شما).

## 🚀 نصب و استفاده

### گزینه 1: استفاده از پایتون (توصیه شده برای UI)

1. مخزن را شبیه سازی کنید:
   ``باش
   کلون git https://github.com/daferferso/whatsapp-gdrive-extractor.git
   سی دی whatsapp-gdrive-extractor
   ```

2. وابستگی های مورد نیاز را نصب کنید:
   ``باش
   pip install -r requires.txt
   ```

3. وب سرور را اجرا کنید:
   ``باش
   python server.py
   ```
4. مرورگر خود را باز کنید و به «http://localhost:5000» بروید تا به داشبورد مدرن دسترسی پیدا کنید!

### گزینه 2: استفاده از Docker

1. مخزن را کلون کنید و در آن حرکت کنید.
2. تصویر داکر را بسازید:
   ``باش
   ساخت داکر . -t whatsapp-gdrive-extractor
   ```
3. ظرف Docker را اجرا کنید:
   * **لینوکس:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **ویندوز (PowerShell):** 'docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor'

## 🔑 راهنمای احراز هویت

اگر در استفاده از ایمیل و رمز عبور استاندارد Google خود با مشکل مواجه شدید، از روش **OAuth Token** استفاده کنید (مرحله 1 در رابط کاربری وب):
1. به «https://getandroidapp.com/» یا هر پورتال ورود به سیستم Google بروید.
2. با اکانت گوگل خود وارد شوید.
3. «F12» را فشار دهید تا Developer Tools باز شود.
4. به **Application** -> **Cookies** بروید.
5. «oauth_token» را پیدا کنید (معمولاً شبیه «oauth2_4/XXXXXXXXXXXXXXXXX» است).
6. آن را کپی و در رابط کاربری وب قرار دهید.

## 🤝 اعتبار و قدردانی
* **نویسنده اصلی:** TripCode
* **مشارکت کنندگان اصلی:** DrDeath1122 (مهندسی چند رشته ای)، YuriCosta (مهندسی معکوس سیستم بازیابی جدید)، macagua (اصلاحات SSL).
* **مدرن سازی و رابط کاربری:** با یک رابط وب مدرن، ماژول رمزنگاری محلی، انتقال تلفن ADB، بازسازی شده و به طور کامل به 40 زبان برای دسترسی جهانی ترجمه شده است.