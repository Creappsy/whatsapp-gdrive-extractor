<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>नेटिव्ह क्रॉस-प्लॅटफॉर्म डेस्कटॉप ऍप्लिकेशन (Windows, macOS, Linux), जलद, हलके आणि बहु-भाषा, Google ड्राइव्हवरून तुमचे WhatsApp बॅकअप काढण्यासाठी, डिक्रिप्ट करण्यासाठी आणि पुनर्संचयित करण्यासाठी.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/भाषा-40_समर्थित-success.svg)](#)
  [![License](https://img.shields.io/badge/परवाना-MIT-green.svg)](#)
</div>

---

## 🌟 वैशिष्ट्ये

* **सुंदर UI/UX:** ग्लासमॉर्फिझम डिझाइन, गडद मोड आणि चरण-दर-चरण पुनर्संचयित मार्गदर्शकासह आधुनिक डेस्कटॉप इंटरफेस.
* **🌍 40 समर्थित भाषा:** दर्जेदार आंतरराष्ट्रीयीकरण नेटिव्ह समाकलित. झटपट भाषा बदला.
* **अल्ट्रालाइट आणि स्वतंत्र:** **टौरी आणि रस्ट** वापरून पुनर्निर्मित, शून्य बाह्य अवलंबनांसह बायनरी आकार फक्त **~10MB** पर्यंत कमी केला (होस्ट सिस्टमवर पायथन रनटाइम किंवा .NET आवश्यक नाही).
* **सुरक्षित कीरिंग स्टोरेज:** तुमची Google OAuth टोकन थेट ऑपरेटिंग सिस्टमच्या मूळ सुरक्षित कीरिंगमध्ये सेव्ह करते (विंडोजमधील क्रेडेन्शियल मॅनेजर, मॅकओएसमधील कीचेन).
* **स्थानिक E2EE डिक्रिप्शन (.crypt14 / .crypt15):** रस्ट वापरून हाय-स्पीड, सुरक्षित आणि संपूर्णपणे स्थानिक AES-256-GCM WhatsApp डेटाबेस डिक्रिप्शन.
* **फोनवर थेट हस्तांतरण (ADB):** तुमचा डिक्रिप्ट केलेला डेटाबेस थेट तुमच्या Android फोनवर एका क्लिकने पुनर्संचयित करा, ADB ला स्थानिकरित्या कॉल करा.

---

## 📋 आवश्यकता

स्रोत तयार करण्यासाठी आणि चालवण्यासाठी, तुमच्याकडे हे असल्याची खात्री करा:
1. **गंज आणि कार्गो** (v1.77 किंवा श्रेष्ठ)
2. **Node.js & npm**
3. USB डीबगिंग सक्षम असलेले Android डिव्हाइस (एडीबीद्वारे थेट पुनर्संचयित केल्यास).

---

## 🚀 अंमलबजावणी आणि विकास

### ऍप्लिकेशन लाँच करत आहे
फक्त डबल-क्लिक करा [run.bat](../run.bat) प्रकल्पाच्या मुळाशी फाइल.
संकलित न केल्यास ते त्वरित संकलित उत्पादन एक्झिक्युटेबल (`app.exe`) उघडेल किंवा विकास मोडवर फॉलबॅक करेल.

### विकास मोडमध्ये चालत आहे
हॉट-रीलोडिंगसह विकास मोड सुरू करण्यासाठी:
```bash
npx tauri dev
```

### संकलित आणि बंडलिंग
स्वयंपूर्ण, स्वाक्षरी केलेले उत्पादन इंस्टॉलर पॅकेजेस (MSI, EXE सेटअप) व्युत्पन्न करण्यासाठी:
```bash
npx tauri build
```
परिणामी इंस्टॉलर `src-tauri/target/release/bundle/` मध्ये स्थित असतील.

पहा [DISTRIBUTION.md](../DISTRIBUTION.md) पॅकेजिंग आणि स्वाक्षरीबद्दल अधिक तपशीलांसाठी.
