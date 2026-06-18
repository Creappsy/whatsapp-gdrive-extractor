<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Google ड्राइव से आपके व्हाट्सएप बैकअप को निकालने, डिक्रिप्ट करने और पुनर्स्थापित करने के लिए नेटिव क्रॉस-प्लेटफ़ॉर्म डेस्कटॉप एप्लिकेशन (विंडोज़, मैकओएस, लिनक्स), तेज़, हल्का और बहु-भाषा।</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/बोली-40_का_समर्थन_किया-success.svg)](#)
  [![License](https://img.shields.io/badge/लाइसेंस-MIT-green.svg)](#)
</div>

---

## 🌟 विशेषताएँ

* **सुंदर यूआई/यूएक्स:** ग्लासमॉर्फ़िज़्म डिज़ाइन, डार्क मोड और चरण-दर-चरण पुनर्स्थापना मार्गदर्शिका के साथ एक आधुनिक डेस्कटॉप इंटरफ़ेस।
* **🌍 40 समर्थित भाषाएँ:** गुणवत्ता अंतर्राष्ट्रीयकरण मूल रूप से एकीकृत। तुरंत भाषाएँ बदलें.
* **अल्ट्रालाइट और स्वतंत्र:** **टौरी और रस्ट** का उपयोग करके पुनर्निर्माण किया गया, शून्य बाहरी निर्भरता (मेजबान सिस्टम पर कोई पायथन रनटाइम या .NET की आवश्यकता नहीं) के साथ बाइनरी आकार को घटाकर केवल **~10MB** कर दिया गया।
* **सुरक्षित कीरिंग भंडारण:** आपके Google OAuth टोकन को सीधे ऑपरेटिंग सिस्टम के मूल सुरक्षित कीरिंग (Windows में क्रेडेंशियल मैनेजर, macOS में कीचेन) में सहेजता है।
* **स्थानीय E2EE डिक्रिप्शन (.crypt14 / .crypt15):** रस्ट का उपयोग करके उच्च गति, सुरक्षित और पूरी तरह से स्थानीय AES-256-GCM व्हाट्सएप डेटाबेस डिक्रिप्शन।
* **फ़ोन पर सीधा स्थानांतरण (एडीबी):** अपने डिक्रिप्ट किए गए डेटाबेस को एक क्लिक से सीधे अपने एंड्रॉइड फोन पर पुनर्स्थापित करें, सिस्टम एडीबी को मूल रूप से कॉल करें।

---

## 📋 आवश्यकताएं

स्रोत से निर्माण और चलाने के लिए, सुनिश्चित करें कि आपके पास:
1. **जंग और माल** (v1.77 या श्रेष्ठ)
2. **Node.js & npm**
3. यूएसबी डिबगिंग सक्षम एक एंड्रॉइड डिवाइस (यदि एडीबी के माध्यम से सीधे पुनर्स्थापित किया जा रहा है)।

---

## 🚀 निष्पादन एवं विकास

### एप्लिकेशन लॉन्च करना
बस डबल-क्लिक करें [run.bat](../run.bat) प्रोजेक्ट के मूल में फ़ाइल.
यह संकलित उत्पादन निष्पादन योग्य (`app.exe`) को तुरंत खोल देगा या संकलित नहीं होने पर विकास मोड में वापस आ जाएगा।

### विकास मोड में चल रहा है
हॉट-रीलोडिंग के साथ विकास मोड शुरू करने के लिए:
```bash
npx tauri dev
```

### संकलन एवं बंडलिंग
स्व-निहित, हस्ताक्षरित उत्पादन इंस्टॉलर पैकेज (MSI, EXE सेटअप) उत्पन्न करने के लिए:
```bash
npx tauri build
```
परिणामी इंस्टॉलर `src-tauri/target/release/bundle/` में स्थित होंगे।

देखना [DISTRIBUTION.md](../DISTRIBUTION.md) पैकेजिंग और हस्ताक्षर के बारे में अधिक जानकारी के लिए।
