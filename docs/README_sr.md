<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Основна апликација за више платформи (Виндовс, мацОС, Линук), брза, лагана и вишејезична, за издвајање, дешифровање и враћање ваших резервних копија ВхатсАпп-а са Гоогле диска.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Језици-40_Подржано-success.svg)](#)
  [![License](https://img.shields.io/badge/Лиценца-MIT-green.svg)](#)
</div>

---

## 🌟 Карактеристике

* **Предиван УИ/УКС:** Модеран десктоп интерфејс са дизајном стакломорфизма, тамним режимом и водичем за рестаурацију корак по корак.
* **🌍 40 подржаних језика:** Квалитетна интернационализација природно интегрисана. Одмах промените језике.
* **Ултралаки и независни:** Поново изграђен коришћењем **Таури & Руст**, смањујући бинарну величину на само **~10МБ** са нула спољних зависности (нису потребна Питхон рунтимес или .НЕТ на систему хоста).
* **Безбедно складиштење кључева:** Чува ваше Гоогле ОАутх токене директно у изворни безбедни прстен за кључеве оперативног система (Управљач акредитивима у Виндовс-у, Кеицхаин у мацОС-у).
* **Локално Е2ЕЕ дешифровање (.црипт14 / .црипт15):** Брза, безбедна и потпуно локална АЕС-256-ГЦМ ВхатсАпп дешифровање базе података користећи Руст.
* **Директан пренос на телефон (АДБ):** Вратите своје дешифроване базе података директно на свој Андроид телефон једним кликом, позивајући систем АДБ изворно.

---

## 📋 Захтеви

Да бисте направили и покренули из извора, уверите се да имате:
1. **Руст & Царго** (v1.77 или надређени)
2. **Node.js & npm**
3. Андроид уређај са омогућеним УСБ отклањањем грешака (ако се враћа директно преко АДБ-а).

---

## 🚀 Извођење и развој

### Покретање апликације
Једноставно двапут кликните на [run.bat](../run.bat) фајл у корену пројекта.
Одмах ће отворити преведену продукцијску извршну датотеку (`апп.еке`) или се вратити у развојни режим ако није преведена.

### Покреће се у развојном режиму
Да бисте покренули развојни режим са врућим поновним учитавањем:
```bash
npx tauri dev
```

### Компајлирање и груписање
Да бисте генерисали самосталне, потписане производне инсталационе пакете (МСИ, ЕКСЕ подешавање):
```bash
npx tauri build
```
Добијени инсталатери ће се налазити у `срц-таури/таргет/релеасе/бундле/`.

Видите [DISTRIBUTION.md](../DISTRIBUTION.md) за више детаља о паковању и потписивању.
