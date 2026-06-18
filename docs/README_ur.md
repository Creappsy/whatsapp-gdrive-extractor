<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>مقامی کراس پلیٹ فارم ڈیسک ٹاپ ایپلیکیشن (Windows, macOS, Linux)، تیز، ہلکا پھلکا، اور کثیر زبان، Google Drive سے اپنے WhatsApp بیک اپ کو نکالنے، ڈکرپٹ کرنے اور بحال کرنے کے لیے۔</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/زبانیں-40_حمایت_کی-success.svg)](#)
  [![License](https://img.shields.io/badge/لائسنس-MIT-green.svg)](#)
</div>

---

## 🌟 خصوصیات

* **خوبصورت UI/UX:** گلاسمورفزم ڈیزائن، ڈارک موڈ، اور مرحلہ وار بحالی گائیڈ کے ساتھ ایک جدید ڈیسک ٹاپ انٹرفیس۔
* **🌍 40 معاون زبانیں:** کوالٹی انٹرنیشنلائزیشن مقامی طور پر مربوط ہے۔ فوری طور پر زبانیں تبدیل کریں۔
* **انتہائی ہلکا اور آزاد:** **Tauri اور Rust** کا استعمال کرتے ہوئے دوبارہ بنایا گیا، صفر بیرونی انحصار کے ساتھ بائنری سائز کو صرف **~10MB** تک کم کیا گیا (میزبان سسٹم پر Python کے رن ٹائمز یا .NET کی ضرورت نہیں ہے)۔
* **محفوظ کیرنگ اسٹوریج:** آپ کے Google OAuth ٹوکنز کو براہ راست آپریٹنگ سسٹم کی مقامی محفوظ کیرنگ میں محفوظ کرتا ہے (ونڈوز میں کریڈینشل مینیجر، میک او ایس میں کیچین)۔
* **مقامی E2EE ڈکرپشن (.crypt14 / .crypt15):** تیز رفتار، محفوظ، اور مکمل طور پر مقامی AES-256-GCM WhatsApp ڈیٹا بیس ڈکرپشن Rust کا استعمال کرتے ہوئے۔
* **فون پر براہ راست منتقلی (ADB):** اپنے ڈیکرپٹ شدہ ڈیٹا بیسز کو براہ راست اپنے اینڈرائیڈ فون پر ایک ہی کلک سے بحال کریں، ADB کو مقامی طور پر کال کریں۔

---

## 📋 تقاضے

ماخذ سے بنانے اور چلانے کے لیے، یقینی بنائیں کہ آپ کے پاس ہے:
1. **زنگ اور کارگو** (v1.77 یا اعلی)
2. **Node.js & npm**
3. ایک اینڈرائیڈ ڈیوائس جس میں USB ڈیبگنگ فعال ہے (اگر براہ راست ADB کے ذریعے بحال ہو رہی ہو)۔

---

## 🚀 پھانسی اور ترقی

### ایپلیکیشن لانچ کرنا
بس پر ڈبل کلک کریں۔ [run.bat](../run.bat) پروجیکٹ کی جڑ میں فائل۔
یہ مرتب شدہ پروڈکشن ایگزیکیوٹیبل (`app.exe`) کو فوری طور پر کھول دے گا یا اگر مرتب نہ کیا گیا ہو تو ڈویلپمنٹ موڈ میں فال بیک۔

### ترقی کے موڈ میں چل رہا ہے۔
ہاٹ ری لوڈنگ کے ساتھ ڈیولپمنٹ موڈ شروع کرنے کے لیے:
```bash
npx tauri dev
```

### مرتب کرنا اور بنڈلنگ کرنا
خود ساختہ، دستخط شدہ پروڈکشن انسٹالر پیکجز (MSI، EXE سیٹ اپ) بنانے کے لیے:
```bash
npx tauri build
```
نتیجے میں انسٹالرز `src-tauri/target/release/bundle/` میں واقع ہوں گے۔

دیکھیں [DISTRIBUTION.md](../DISTRIBUTION.md) پیکیجنگ اور دستخط کے بارے میں مزید تفصیلات کے لیے۔
