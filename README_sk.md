<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Logo WhatsApp" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderný, rýchly a viacjazyčný nástroj na extrahovanie, dešifrovanie a obnovenie vašich záloh WhatsApp z Disku Google.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licencia](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Funkcie

* **Krásne webové používateľské rozhranie/UX:** Moderné natívne rozhranie v tmavom režime s prvkami glassmorphism, ktoré vás jednoducho prevedie procesom extrakcie.
* **🌍 40 podporovaných jazykov:** 100% natívne vstavané preklady v ľudskej kvalite. Zmeňte jazyk za behu bez opätovného načítania!
* **Lokálne dešifrovanie (.crypt14 / .crypt15):** Plne lokálny kryptografický modul na bezpečné dešifrovanie vašich SQLite databáz pomocou vášho E2EE hash kľúča bez toho, aby ste vaše dáta kamkoľvek nahrávali.
* **Priamy prenos cez Android (ADB):** Preneste stiahnutú a dešifrovanú databázu priamo do úložiska telefónu s Androidom jediným kliknutím z ovládacieho panela.
* **Bezpečné overenie:** Podporuje tokeny Google OAuth aj heslá aplikácií pre maximálnu bezpečnosť.

## 📋 Predpoklady

Skôr ako začnete, uistite sa, že máte:
1. Nainštalovaný **Python 3.x** alebo **Docker**.
2. Zariadenie Android s nainštalovanou aplikáciou WhatsApp a povolenými zálohami na Disk Google.
3. Vaše poverenia účtu Google (alebo [heslo aplikácie](https://myaccount.google.com/apppasswords)).
4. *Voliteľné:* ​​ID Android vášho zariadenia (na zníženie rizika, že vás Google odhlási).

## 🚀 Inštalácia a použitie

### Možnosť 1: Použitie Pythonu (odporúčané pre používateľské rozhranie)

1. Klonujte úložisko:
   ``` bash
   git klon https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Nainštalujte požadované závislosti:
   ``` bash
   pip install -r requirements.txt
   ```

3. Spustite webový server:
   ``` bash
   python server.py
   ```
4. Otvorte svoj prehliadač a prejdite na `http://localhost:5000`, aby ste získali prístup k modernému ovládaciemu panelu!

### Možnosť 2: Použitie Docker

1. Naklonujte úložisko a prejdite doň.
2. Vytvorte obraz Docker:
   ``` bash
   zostava dockera. -t whatsapp-gdrive-extractor
   ```
3. Spustite kontajner Docker:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Sprievodca overením

Ak sa pri používaní štandardného e-mailu a hesla Google vyskytnú problémy, použite metódu **OAuth Token** (krok 1 vo webovom používateľskom rozhraní):
1. Prejdite na stránku `https://getandroidapp.com/` alebo akýkoľvek prihlasovací portál Google.
2. Prihláste sa pomocou svojho účtu Google.
3. Stlačením `F12` otvorte Nástroje pre vývojárov.
4. Prejdite na **Aplikácia** -> **Súbory cookie**.
5. Nájdite `oauth_token` (zvyčajne vyzerá ako `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Skopírujte ho a vložte do webového používateľského rozhrania.

## 🤝 Kredity a poďakovanie
* **Pôvodný autor:** TripCode
* **Hlavní prispievatelia:** DrDeath1122 (multivláknová chrbtica), YuriCosta (reverzné inžinierstvo nového systému obnovenia), macagua (opravy SSL).
* **Modernizácia a používateľské rozhranie:** Prestavané s moderným webovým rozhraním, lokálnym kryptografickým modulom, ADB telefónnym prenosom a plne lokalizované do 40 jazykov pre globálnu dostupnosť.