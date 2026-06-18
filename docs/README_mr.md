<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp लोगो" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Google ड्राइव्हवरून तुमचे WhatsApp बॅकअप काढण्यासाठी, डिक्रिप्ट करण्यासाठी आणि पुनर्संचयित करण्यासाठी आधुनिक, जलद आणि बहु-भाषा साधन.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![डॉकर](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![परवाना](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 वैशिष्ट्ये

* **सुंदर वेब UI/UX:** काढण्याच्या प्रक्रियेत तुम्हाला सहज मार्गदर्शन करण्यासाठी ग्लासमॉर्फिजम घटकांसह आधुनिक, गडद-मोड नेटिव्ह इंटरफेस.
* **🌍 40 भाषा समर्थित:** 100% मानवी-गुणवत्तेची भाषांतरे मूळतः अंगभूत. रीलोड न करता ऑन-द-फ्लाय भाषा बदला!
* **स्थानिक डिक्रिप्शन (.crypt14 / .crypt15):** तुमचा डेटा कोठेही अपलोड न करता तुमची E2EE हॅश की वापरून तुमचा SQLite डेटाबेस सुरक्षितपणे डिक्रिप्ट करण्यासाठी पूर्णतः स्थानिक क्रिप्टोग्राफी मॉड्यूल.
* **डायरेक्ट अँड्रॉइड ट्रान्सफर (ADB):** डॅशबोर्डवरून एका क्लिकने तुमचा डाउनलोड केलेला आणि डिक्रिप्ट केलेला डेटाबेस थेट तुमच्या Android फोनच्या स्टोरेजमध्ये पुश करा.
* **सुरक्षित प्रमाणीकरण:** कमाल सुरक्षिततेसाठी Google OAuth टोकन आणि ॲप पासवर्ड या दोन्हींना सपोर्ट करते.

## 📋 पूर्वतयारी

तुम्ही सुरू करण्यापूर्वी, तुमच्याकडे हे असल्याची खात्री करा:
1. **Python 3.x** किंवा **डॉकर** स्थापित.
2. WhatsApp स्थापित केलेले आणि Google ड्राइव्ह बॅकअप सक्षम केलेले Android डिव्हाइस.
३. तुमचे Google खाते क्रेडेंशियल (किंवा [ॲप पासवर्ड](https://myaccount.google.com/apppasswords)).
4. *पर्यायी:* तुमच्या डिव्हाइसचा Android आयडी (Google तुम्हाला लॉग आउट करण्याचा धोका कमी करण्यासाठी).

## 🚀 स्थापना आणि वापर

### पर्याय १: पायथन वापरणे (UI साठी शिफारस केलेले)

1. रेपॉजिटरी क्लोन करा:
   ``बाश
   git क्लोन https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. आवश्यक अवलंबित्व स्थापित करा:
   ``बाश
   pip install -r requirements.txt
   ```

3. वेब सर्व्हर चालवा:
   ``बाश
   python server.py
   ```
4. आधुनिक डॅशबोर्डवर प्रवेश करण्यासाठी तुमचा ब्राउझर उघडा आणि `http://localhost:5000` वर जा!

### पर्याय २: डॉकर वापरणे

1. रेपॉजिटरी क्लोन करा आणि त्यात नेव्हिगेट करा.
2. डॉकर प्रतिमा तयार करा:
   ``बाश
   डॉकर बिल्ड. -t whatsapp-gdrive-extractor
   ```
3. डॉकर कंटेनर चालवा:
   * **लिनक्स:** `डॉकर रन -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `डॉकर रन -v.:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 प्रमाणीकरण मार्गदर्शक

तुम्हाला तुमचा मानक Google ईमेल आणि पासवर्ड वापरताना समस्या येत असल्यास, **OAuth टोकन** पद्धत वापरा (वेब UI वर पायरी 1):
1. `https://getandroidapp.com/` किंवा कोणत्याही Google लॉगिन पोर्टलवर जा.
2. तुमच्या Google खात्याने लॉग इन करा.
3. विकसक साधने उघडण्यासाठी `F12` दाबा.
4. **Application** -> **कुकीज** वर जा.
5. `oauth_token` शोधा (हे सहसा `oauth2_4/XXXXXXXXXXXXXXXXXXX` सारखे दिसते).
6. वेब UI मध्ये कॉपी आणि पेस्ट करा.

## 🤝 श्रेय आणि पावती
* **मूळ लेखक:** ट्रिपकोड
* **कोर कंट्रिब्युटर:** DrDeath1122 (मल्टी-थ्रेडिंग बॅकबोन), युरीकोस्टा (नवीन रिस्टोर सिस्टम रिव्हर्स इंजिनिअरिंग), मॅकागुआ (SSL फिक्सेस).
* **आधुनिकीकरण आणि UI:** आधुनिक वेब इंटरफेस, स्थानिक क्रिप्टोग्राफी मॉड्यूल, ADB फोन हस्तांतरण आणि जागतिक प्रवेशयोग्यतेसाठी 40 भाषांमध्ये पूर्णपणे स्थानिकीकरणासह पुनर्निर्मित.
