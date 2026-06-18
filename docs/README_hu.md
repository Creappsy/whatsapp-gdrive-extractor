<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp logó" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Modern, gyors és többnyelvű eszköz a WhatsApp biztonsági másolatainak kibontására, visszafejtésére és visszaállítására a Google Drive-ból.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licenc](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Jellemzők

* **Gyönyörű webes felhasználói felület/UX:** Modern, sötét módú natív felület üvegmorfizmus elemekkel, amelyek könnyedén végigvezetik Önt a kivonási folyamaton.
* **🌍 40 támogatott nyelv:** 100%-ban emberi minőségű fordítások natívan beépítve. Változtassa meg a nyelvet menet közben, újratöltés nélkül!
* **Helyi visszafejtés (.crypt14 / .crypt15):** Teljesen helyi kriptográfiai modul az SQLite adatbázisok biztonságos visszafejtéséhez az E2EE hash kulcs használatával anélkül, hogy bárhova feltöltené adatait.
* **Közvetlen Android Transfer (ADB):** A letöltött és visszafejtett adatbázist közvetlenül az Android telefon tárhelyére tolhatja egyetlen kattintással az irányítópulton.
* **Biztonságos hitelesítés:** A maximális biztonság érdekében támogatja a Google OAuth tokeneket és az alkalmazásjelszavakat is.

## 📋 Előfeltételek

Mielőtt elkezdené, győződjön meg arról, hogy rendelkezik:
1. **Python 3.x** vagy **Docker** telepítve.
2. Android-eszköz, amelyen telepítve van a WhatsApp, és engedélyezve van a Google Drive biztonsági mentése.
3. Google-fiókjának hitelesítő adatai (vagy egy [alkalmazásjelszó](https://myaccount.google.com/apppasswords)).
4. *Opcionális:* Az eszköz Android-azonosítója (a Google kijelentkezési kockázatának csökkentése érdekében).

## 🚀 Telepítés és használat

### 1. lehetőség: Python használata (a felhasználói felülethez ajánlott)

1. A tár klónozása:
   ``` bash
   git klón https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Telepítse a szükséges függőségeket:
   ``` bash
   pip install -r követelmények.txt
   ```

3. Futtassa a webszervert:
   ``` bash
   python server.py
   ```
4. Nyissa meg böngészőjét, és lépjen a `http://localhost:5000` címre a modern irányítópult eléréséhez!

### 2. lehetőség: Docker használata

1. Klónozza a tárat, és navigáljon bele.
2. A Docker-kép létrehozása:
   ``` bash
   dokkoló épít. -t whatsapp-gdrive-extractor
   ```
3. Futtassa a Docker-tárolót:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Hitelesítési útmutató

Ha problémákat tapasztal normál Google e-mail címe és jelszava használatával, használja az **OAuth Token** módszert (1. lépés a webes felhasználói felületen):
1. Nyissa meg a „https://getandroidapp.com/” webhelyet vagy bármely Google bejelentkezési portált.
2. Jelentkezzen be Google-fiókjával.
3. Nyomja meg az F12 billentyűt a Fejlesztői eszközök megnyitásához.
4. Lépjen az **Alkalmazás** -> **Cookie-k** elemre.
5. Keresse meg az "oauth_token" (általában így néz ki: "oauth2_4/XXXXXXXXXXXXXXXXXX").
6. Másolja ki és illessze be a webes felhasználói felületre.

## 🤝 Köszönetnyilvánítás és köszönet
* **Eredeti szerző:** TripCode
* **Alapvető közreműködők:** DrDeath1122 (többszálas gerinc), YuriCosta (új visszaállítási rendszer reverse engineering), macagua (SSL-javítások).
* **Modernizálás és felhasználói felület:** Újraépítve modern webes felülettel, helyi kriptográfiai modullal, ADB telefonos átvitellel, és 40 nyelvre teljesen lokalizálva a globális hozzáférhetőség érdekében.
