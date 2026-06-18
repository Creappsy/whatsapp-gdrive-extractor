<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>برنامه دسکتاپ چند پلتفرمی بومی (ویندوز، macOS، لینوکس)، سریع، سبک و چند زبانه، برای استخراج، رمزگشایی و بازیابی نسخه‌های پشتیبان WhatsApp شما از Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/زبان_ها-40_پشتیبانی_می_شود-success.svg)](#)
  [![License](https://img.shields.io/badge/مجوز-MIT-green.svg)](#)
</div>

---

## 🌟 ویژگی ها

* **UI/UX زیبا:** یک رابط دسکتاپ مدرن با طراحی شیشه‌مورفیسم، حالت تاریک و راهنمای بازیابی گام به گام.
* **🌍 40 زبان پشتیبانی شده:** بین المللی سازی کیفیت به صورت بومی یکپارچه شده است. فوراً زبان ها را تغییر دهید.
* **فوق سبک و مستقل:** بازسازی شده با استفاده از **Tauri & Rust**، کاهش اندازه باینری فقط به **~10MB** با هیچ وابستگی خارجی (بدون نیاز به زمان اجرا پایتون یا دات نت در سیستم میزبان).
* **محل ذخیره کلید ایمن:** توکن‌های Google OAuth شما را مستقیماً در کلیدهای امن بومی سیستم عامل (مدیر اعتبار در Windows، Keychain در macOS) ذخیره می‌کند.
* **رمزگشایی محلی E2EE (.crypt14 / .crypt15):** رمزگشایی پایگاه داده واتساپ AES-256-GCM با سرعت بالا، ایمن و کاملاً محلی با استفاده از Rust.
* **انتقال مستقیم به تلفن (ADB):** پایگاه داده های رمزگشایی شده خود را مستقیماً با یک کلیک به تلفن اندرویدی خود بازیابی کنید و به صورت بومی با سیستم ADB تماس بگیرید.

---

## 📋 الزامات

برای ساخت و اجرا از منبع، مطمئن شوید که:
1. **زنگ و محموله** (v1.77 یا برتر)
2. **Node.js & npm**
3. یک دستگاه Android با USB Debugging فعال (در صورت بازیابی مستقیم از طریق ADB).

---

## 🚀 اجرا و توسعه

### راه اندازی برنامه
به سادگی روی آن دوبار کلیک کنید [run.bat](../run.bat) فایل در ریشه پروژه
فورا فایل اجرایی تولید کامپایل شده (`app.exe`) را باز می کند یا اگر کامپایل نشده باشد به حالت توسعه باز می گردد.

### در حال اجرا در حالت توسعه
برای شروع حالت توسعه با بارگذاری مجدد داغ:
```bash
npx tauri dev
```

### کامپایل و بسته بندی
برای تولید بسته های نصب کننده تولید امضا شده (MSI، EXE راه اندازی):
```bash
npx tauri build
```
نصب کننده های حاصل در «src-tauri/target/release/bundle/» قرار خواهند گرفت.

ببینید [DISTRIBUTION.md](../DISTRIBUTION.md) برای جزئیات بیشتر در مورد بسته بندی و امضا.
