<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Logo WhatsApp" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderní, rychlý a vícejazyčný nástroj pro extrahování, dešifrování a obnovení vašich záloh WhatsApp z Disku Google.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licence](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Funkce

* **Krásné webové uživatelské rozhraní/UX:** Moderní nativní rozhraní v tmavém režimu s prvky glassmorphism, které vás snadno provede procesem extrakce.
* **🌍 40 podporovaných jazyků:** 100% nativně zabudované překlady v lidské kvalitě. Změňte jazyk za běhu bez opětovného načítání!
* **Místní dešifrování (.crypt14 / .crypt15):** Plně lokální kryptografický modul pro bezpečné dešifrování vašich databází SQLite pomocí vašeho hash klíče E2EE, aniž byste svá data kamkoli nahrávali.
* **Direct Android Transfer (ADB):** Přeneste staženou a dešifrovanou databázi přímo do úložiště telefonu Android jediným kliknutím z ovládacího panelu.
* **Bezpečné ověřování:** Podporuje jak tokeny Google OAuth, tak hesla aplikací pro maximální zabezpečení.

## 📋 Předpoklady

Než začnete, ujistěte se, že máte:
1. Nainstalovaný **Python 3.x** nebo **Docker**.
2. Zařízení Android s nainstalovanou aplikací WhatsApp a povolenými zálohami na Disk Google.
3. Vaše přihlašovací údaje k účtu Google (nebo [heslo aplikace](https://myaccount.google.com/apppasswords)).
4. *Volitelné:* ID Android vašeho zařízení (pro snížení rizika, že vás Google odhlásí).

## 🚀 Instalace a použití

### Možnost 1: Použití Pythonu (doporučeno pro uživatelské rozhraní)

1. Klonujte úložiště:
   ``` bash
   git klon https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Nainstalujte požadované závislosti:
   ``` bash
   pip install -r požadavky.txt
   ```

3. Spusťte webový server:
   ``` bash
   python server.py
   ```
4. Otevřete prohlížeč a přejděte na `http://localhost:5000`, abyste získali přístup k modernímu řídicímu panelu!

### Možnost 2: Použití Dockeru

1. Klonujte úložiště a přejděte do něj.
2. Vytvořte obraz Docker:
   ``` bash
   sestavení dockeru. -t whatsapp-gdrive-extractor
   ```
3. Spusťte kontejner Docker:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Průvodce ověřením

Pokud zaznamenáte problémy s používáním standardního e-mailu a hesla Google, použijte metodu **OAuth Token** (krok 1 ve webovém rozhraní):
1. Přejděte na `https://getandroidapp.com/` nebo jakýkoli přihlašovací portál Google.
2. Přihlaste se pomocí svého účtu Google.
3. Stisknutím `F12` otevřete Nástroje pro vývojáře.
4. Přejděte na **Aplikace** -> **Soubory cookie**.
5. Najděte `oauth_token` (obvykle vypadá jako `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Zkopírujte jej a vložte do webového uživatelského rozhraní.

## 🤝 Kredity a poděkování
* **Původní autor:** TripCode
* **Základní přispěvatelé:** DrDeath1122 (multivláknová páteř), YuriCosta (reverzní inženýrství nového systému obnovení), macagua (opravy SSL).
* **Modernizace a uživatelské rozhraní:** Přestavěno s moderním webovým rozhraním, lokálním kryptografickým modulem, ADB telefonním přenosem a plně lokalizováno do 40 jazyků pro globální dostupnost.