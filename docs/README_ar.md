<div محاذاة = "المركز">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="شعار WhatsApp" width="100"/>
  <h1>مستخرج الواتس اب جوجل درايف 🚀</h1>
  <p><b>أداة حديثة وسريعة ومتعددة اللغات لاستخراج وفك تشفير واستعادة النسخ الاحتياطية لتطبيق WhatsApp من Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![بايثون](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![الترخيص](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 المميزات

* **واجهة مستخدم/تجربة ويب جميلة:** واجهة أصلية حديثة ذات الوضع المظلم مع عناصر شكل الزجاج لإرشادك بسهولة خلال عملية الاستخراج.
* **🌍 40 لغة مدعومة:** ترجمات مدمجة محليًا بجودة بشرية بنسبة 100%. قم بتغيير اللغة أثناء التنقل دون إعادة التحميل!
* **فك التشفير المحلي (.crypt14 / .crypt15):** وحدة تشفير محلية بالكامل لفك تشفير قواعد بيانات SQLite بأمان باستخدام مفتاح تجزئة E2EE دون تحميل بياناتك في أي مكان.
* **النقل المباشر لنظام Android (ADB):** ادفع قاعدة البيانات التي تم تنزيلها وفك تشفيرها مباشرة إلى مساحة تخزين هاتف Android الخاص بك بنقرة واحدة من لوحة المعلومات.
* **المصادقة الآمنة:** تدعم كلاً من رموز Google OAuth وكلمات مرور التطبيقات لتحقيق أقصى قدر من الأمان.

## 📋 المتطلبات الأساسية

قبل البدء، تأكد من أن لديك:
1. تم تثبيت **Python 3.x** أو **Docker**.
2. جهاز Android مثبت عليه WhatsApp وتمكين النسخ الاحتياطية في Google Drive.
3. بيانات اعتماد حسابك على Google (أو [كلمة مرور التطبيق](https://myaccount.google.com/apppasswords)).
4. *اختياري:* معرف Android لجهازك (لتقليل خطر تسجيل خروجك من Google).

## 🚀 التثبيت والاستخدام

### الخيار 1: استخدام Python (موصى به لواجهة المستخدم)

1. استنساخ المستودع:
   ``` باش
   استنساخ بوابة https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive- extractor
   ```

2. قم بتثبيت التبعيات المطلوبة:
   ``` باش
   تثبيت النقطة -r متطلبات.txt
   ```

3. قم بتشغيل خادم الويب:
   ``` باش
   بيثون server.py
   ```
4. افتح متصفحك وانتقل إلى `http://localhost:5000` للوصول إلى لوحة التحكم الحديثة!

### الخيار 2: استخدام عامل الميناء

1. انسخ المستودع وانتقل إليه.
2. إنشاء صورة Docker:
   ``` باش
   بناء عامل ميناء . -t whatsapp-gdrive-نازع
   ```
3. قم بتشغيل حاوية Docker:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 دليل المصادقة

إذا واجهت مشكلات في استخدام البريد الإلكتروني وكلمة المرور القياسيين في Google، فاستخدم طريقة **OAuth Token** (الخطوة 1 في واجهة مستخدم الويب):
1. انتقل إلى https://getandroidapp.com/ أو أي بوابة لتسجيل الدخول إلى Google.
2. قم بتسجيل الدخول باستخدام حساب جوجل الخاص بك.
3. اضغط على "F12" لفتح أدوات المطور.
4. انتقل إلى **التطبيق** -> **ملفات تعريف الارتباط**.
5. ابحث عن `oauth_token` (يبدو عادةً مثل `oauth2_4/XXXXXXXXXXXXXXXXXX`).
6. انسخه والصقه في واجهة مستخدم الويب.

## 🤝 الاعتمادات والتقديرات
* **الكاتب الأصلي:** TripCode
* **المساهمون الأساسيون:** DrDeath1122 (العمود الفقري متعدد الخيوط)، YuriCosta (الهندسة العكسية لنظام الاستعادة الجديد)، macagua (إصلاحات SSL).
* **التحديث وواجهة المستخدم:** أعيد بناؤها بواجهة ويب حديثة، ووحدة تشفير محلية، ونقل هاتف ADB، ومترجمة بالكامل إلى 40 لغة لإمكانية الوصول العالمية.
