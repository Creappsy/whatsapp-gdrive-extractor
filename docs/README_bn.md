<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>নেটিভ ক্রস-প্ল্যাটফর্ম ডেস্কটপ অ্যাপ্লিকেশন (Windows, macOS, Linux), দ্রুত, লাইটওয়েট এবং বহু-ভাষা, Google ড্রাইভ থেকে আপনার হোয়াটসঅ্যাপ ব্যাকআপগুলি নিষ্কাশন, ডিক্রিপ্ট এবং পুনরুদ্ধার করতে।</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/ভাষা-40_সমর্থিত-success.svg)](#)
  [![License](https://img.shields.io/badge/লাইসেন্স-MIT-green.svg)](#)
</div>

---

## 🌟 বৈশিষ্ট্য

* **সুন্দর UI/UX:** গ্লাসমর্ফিজম ডিজাইন, ডার্ক মোড এবং ধাপে ধাপে পুনরুদ্ধার গাইড সহ একটি আধুনিক ডেস্কটপ ইন্টারফেস।
* **🌍 40টি সমর্থিত ভাষা:** গুণমান আন্তর্জাতিকীকরণ নেটিভলি একত্রিত. অবিলম্বে ভাষা পরিবর্তন করুন.
* **আল্ট্রালাইট এবং স্বাধীন:** **টাউরি এবং রাস্ট** ব্যবহার করে পুনঃনির্মিত, শূন্য বাহ্যিক নির্ভরতা সহ বাইনারি আকার হ্রাস করে মাত্র **~10MB** করে (হোস্ট সিস্টেমে পাইথন রানটাইম বা .NET এর প্রয়োজন নেই)।
* **সুরক্ষিত কীরিং স্টোরেজ:** আপনার Google OAuth টোকেনগুলি সরাসরি অপারেটিং সিস্টেমের নেটিভ সুরক্ষিত কীরিংয়ে সংরক্ষণ করে (Windows-এ শংসাপত্র ম্যানেজার, macOS-এ Keychain)।
* **স্থানীয় E2EE ডিক্রিপশন (.crypt14 / .crypt15):** মরিচা ব্যবহার করে উচ্চ-গতি, সুরক্ষিত এবং সম্পূর্ণ স্থানীয় AES-256-GCM WhatsApp ডাটাবেস ডিক্রিপশন।
* **ফোনে সরাসরি স্থানান্তর (ADB):** আপনার ডিক্রিপ্ট করা ডাটাবেসগুলিকে সরাসরি আপনার অ্যান্ড্রয়েড ফোনে একটি একক ক্লিকে পুনরুদ্ধার করুন, স্থানীয়ভাবে ADB সিস্টেমকে কল করুন৷

---

## 📋 প্রয়োজনীয়তা

উত্স থেকে তৈরি এবং চালানোর জন্য, নিশ্চিত করুন যে আপনার আছে:
1. **মরিচা এবং কার্গো** (v1.77 বা উচ্চতর)
2. **Node.js & npm**
3. USB ডিবাগিং সক্ষম সহ একটি Android ডিভাইস (যদি সরাসরি ADB এর মাধ্যমে পুনরুদ্ধার করা হয়)।

---

## 🚀 এক্সিকিউশন এবং ডেভেলপমেন্ট

### অ্যাপ্লিকেশন চালু করা হচ্ছে
শুধু ডাবল ক্লিক করুন [run.bat](../run.bat) প্রকল্পের মূলে ফাইল।
এটি তাৎক্ষণিকভাবে কম্পাইল করা প্রোডাকশন এক্সিকিউটেবল (`app.exe`) খুলবে বা কম্পাইল না হলে ডেভেলপমেন্ট মোডে ফলব্যাক করবে।

### উন্নয়ন মোডে চলছে
হট-রিলোডিংয়ের সাথে বিকাশ মোড শুরু করতে:
```bash
npx tauri dev
```

### কম্পাইলিং এবং বান্ডলিং
স্বয়ংসম্পূর্ণ, স্বাক্ষরিত উৎপাদন ইনস্টলার প্যাকেজ (MSI, EXE সেটআপ) তৈরি করতে:
```bash
npx tauri build
```
ফলস্বরূপ ইনস্টলারগুলি `src-tauri/target/release/bundle/`-এ অবস্থিত হবে।

দেখুন [DISTRIBUTION.md](../DISTRIBUTION.md) প্যাকেজিং এবং সাইনিং সম্পর্কে আরো বিস্তারিত জানার জন্য।
