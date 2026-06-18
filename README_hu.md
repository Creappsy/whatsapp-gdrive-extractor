<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp logó" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Modern, gyors és többnyelvű eszköz a WhatsApp biztonsági másolatainak kibontására, visszafejtésére és visszaállítására a Google Drive-ból.</b></p>

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