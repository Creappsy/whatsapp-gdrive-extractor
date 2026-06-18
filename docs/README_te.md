<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Google డిస్క్ నుండి మీ WhatsApp బ్యాకప్‌లను సంగ్రహించడానికి, డీక్రిప్ట్ చేయడానికి మరియు పునరుద్ధరించడానికి స్థానిక క్రాస్-ప్లాట్‌ఫారమ్ డెస్క్‌టాప్ అప్లికేషన్ (Windows, macOS, Linux), వేగవంతమైన, తేలికైన మరియు బహుళ-భాష.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/భాషలు-40_మద్దతు_ఇచ్చారు-success.svg)](#)
  [![License](https://img.shields.io/badge/లైసెన్స్-MIT-green.svg)](#)
</div>

---

## 🌟 ఫీచర్లు

* **అందమైన UI/UX:** గ్లాస్‌మార్ఫిజం డిజైన్, డార్క్ మోడ్ మరియు స్టెప్-బై-స్టెప్ రిస్టోరేషన్ గైడ్‌తో కూడిన ఆధునిక డెస్క్‌టాప్ ఇంటర్‌ఫేస్.
* **🌍 40 మద్దతు ఉన్న భాషలు:** నాణ్యమైన అంతర్జాతీయీకరణ స్థానికంగా ఏకీకృతం చేయబడింది. తక్షణమే భాషలను మార్చండి.
* **అల్ట్రాలైట్ & ఇండిపెండెంట్:** సున్నా బాహ్య డిపెండెన్సీలతో బైనరీ పరిమాణాన్ని కేవలం **~10MB**కి తగ్గించడం ద్వారా **టౌరీ & రస్ట్**ని ఉపయోగించి పునర్నిర్మించబడింది (హోస్ట్ సిస్టమ్‌లో పైథాన్ రన్‌టైమ్‌లు లేదా .NET అవసరం లేదు).
* **సురక్షిత కీరింగ్ నిల్వ:** మీ Google OAuth టోకెన్‌లను నేరుగా ఆపరేటింగ్ సిస్టమ్ యొక్క స్థానిక సురక్షిత కీరింగ్‌కి సేవ్ చేస్తుంది (Windowsలో క్రెడెన్షియల్ మేనేజర్, macOSలో కీచైన్).
* **స్థానిక E2EE డిక్రిప్షన్ (.crypt14 / .crypt15):** రస్ట్‌ని ఉపయోగించి హై-స్పీడ్, సురక్షితమైన మరియు పూర్తిగా స్థానిక AES-256-GCM వాట్సాప్ డేటాబేస్ డిక్రిప్షన్.
* **నేరుగా ఫోన్‌కి బదిలీ (ADB):** మీ డిక్రిప్ట్ చేయబడిన డేటాబేస్‌లను ఒకే క్లిక్‌తో నేరుగా మీ Android ఫోన్‌కు పునరుద్ధరించండి, సిస్టమ్ ADBకి స్థానికంగా కాల్ చేయండి.

---

## 📋 అవసరాలు

మూలం నుండి నిర్మించడానికి మరియు అమలు చేయడానికి, మీరు వీటిని కలిగి ఉన్నారని నిర్ధారించుకోండి:
1. **రస్ట్ & కార్గో** (v1.77 లేదా ఉన్నతమైనది)
2. **Node.js & npm**
3. USB డీబగ్గింగ్ ప్రారంభించబడిన Android పరికరం (నేరుగా ADB ద్వారా పునరుద్ధరించినట్లయితే).

---

## 🚀 అమలు మరియు అభివృద్ధి

### అప్లికేషన్‌ను ప్రారంభించడం
కేవలం డబుల్ క్లిక్ చేయండి [run.bat](../run.bat) ప్రాజెక్ట్ యొక్క మూలంలో ఫైల్.
ఇది కంపైల్ చేయబడిన ప్రొడక్షన్ ఎక్జిక్యూటబుల్ (`app.exe`)ని తక్షణమే తెరుస్తుంది లేదా కంపైల్ చేయకపోతే డెవలప్‌మెంట్ మోడ్‌కి ఫాల్‌బ్యాక్ చేస్తుంది.

### డెవలప్‌మెంట్ మోడ్‌లో అమలవుతోంది
హాట్-రీలోడింగ్‌తో డెవలప్‌మెంట్ మోడ్‌ను ప్రారంభించడానికి:
```bash
npx tauri dev
```

### కంపైలింగ్ మరియు బండ్లింగ్
స్వీయ-నియంత్రణ, సంతకం చేయబడిన ఉత్పత్తి ఇన్‌స్టాలర్ ప్యాకేజీలను (MSI, EXE సెటప్) రూపొందించడానికి:
```bash
npx tauri build
```
ఫలితంగా ఇన్‌స్టాలర్‌లు `src-tauri/target/release/bundle/`లో ఉంటాయి.

చూడండి [DISTRIBUTION.md](../DISTRIBUTION.md) ప్యాకేజింగ్ మరియు సంతకంపై మరిన్ని వివరాల కోసం.
