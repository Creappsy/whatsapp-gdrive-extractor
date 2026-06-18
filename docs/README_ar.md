<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>تطبيق سطح مكتب أصلي متعدد الأنظمة الأساسية (Windows وmacOS وLinux)، سريع وخفيف الوزن ومتعدد اللغات، لاستخراج نسخ WhatsApp الاحتياطية وفك تشفيرها واستعادتها من Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/اللغات-40_المدعومة-success.svg)](#)
  [![License](https://img.shields.io/badge/رخصة-MIT-green.svg)](#)
</div>

---

## 🌟 سمات

* **واجهة المستخدم/تجربة المستخدم الجميلة:** واجهة سطح مكتب حديثة تتميز بتصميم على شكل زجاجي، ووضع داكن، ودليل استعادة خطوة بخطوة.
* **🌍 40 لغة مدعومة:** تدويل الجودة متكامل محليا. تبديل اللغات على الفور.
* **خفيفة ومستقلة:** تمت إعادة بنائها باستخدام **Tauri & Rust**، مما أدى إلى تقليل الحجم الثنائي إلى **~10 ميجابايت** فقط دون أي تبعيات خارجية (لا يلزم وجود أوقات تشغيل Python أو .NET على النظام المضيف).
* **تخزين كيرينغ آمن:** يحفظ رموز Google OAuth المميزة مباشرةً في حلقة المفاتيح الآمنة الأصلية لنظام التشغيل (مدير الاعتماد في نظام التشغيل Windows، وسلسلة المفاتيح في نظام التشغيل macOS).
* **فك تشفير E2EE المحلي (.crypt14 / .crypt15):** فك تشفير قاعدة بيانات WhatsApp عالية السرعة وآمنة ومحلية بالكامل AES-256-GCM باستخدام Rust.
* **التحويل المباشر إلى الهاتف (ADB):** قم باستعادة قواعد البيانات التي تم فك تشفيرها مباشرة إلى هاتف Android الخاص بك بنقرة واحدة، واستدعاء نظام ADB محليًا.

---

## 📋 متطلبات

للإنشاء والتشغيل من المصدر، تأكد من أن لديك:
1. **الصدأ والشحن** (v1.77 أو متفوقة)
2. **Node.js & npm**
3. جهاز Android مزود بميزة تصحيح أخطاء USB (في حالة الاستعادة مباشرة عبر ADB).

---

## 🚀 التنفيذ والتطوير

### إطلاق التطبيق
ما عليك سوى النقر نقرًا مزدوجًا فوق [run.bat](../run.bat) الملف في جذر المشروع.
سيفتح على الفور ملف الإنتاج المترجم القابل للتنفيذ (`app.exe`) أو الرجوع إلى وضع التطوير إذا لم يتم تجميعه.

### يعمل في وضع التطوير
لبدء وضع التطوير مع إعادة التحميل السريع:
```bash
npx tauri dev
```

### التجميع والتجميع
لإنشاء حزم تثبيت الإنتاج الموقعة والمكتفية ذاتيًا (إعداد MSI وEXE):
```bash
npx tauri build
```
سيتم تحديد موقع أدوات التثبيت الناتجة في `src-tauri/target/release/bundle/`.

يرى [DISTRIBUTION.md](../DISTRIBUTION.md) لمزيد من التفاصيل حول التعبئة والتغليف والتوقيع.
