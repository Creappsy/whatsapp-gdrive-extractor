<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp-logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderne, hurtigt og flersproget værktøj til at udtrække, dekryptere og gendanne dine WhatsApp-sikkerhedskopier fra Google Drev.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licens](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Funktioner

* **Smuk web-UI/UX:** En moderne, mørk-mode indbygget grænseflade med glasmorfi-elementer, der nemt kan guide dig gennem udvindingsprocessen.
* **🌍 40 understøttede sprog:** 100 % oversættelser af menneskelig kvalitet, der er indbygget. Skift sprog på farten uden at genindlæse!
* **Lokal dekryptering (.crypt14 / .crypt15):** Et fuldt lokalt kryptografimodul til sikkert at dekryptere dine SQLite-databaser ved hjælp af din E2EE-hash-nøgle uden at uploade dine data nogen steder.
* **Direkte Android-overførsel (ADB):** Skub din downloadede og dekrypterede database direkte ind i din Android-telefons lager med et enkelt klik fra dashboardet.
* **Sikker godkendelse:** Understøtter både Google OAuth-tokens og app-adgangskoder for maksimal sikkerhed.

## 📋 Forudsætninger

Før du starter, skal du sørge for at have:
1. **Python 3.x** eller **Docker** installeret.
2. En Android-enhed med WhatsApp installeret og Google Drev-sikkerhedskopier aktiveret.
3. Dine Google-kontooplysninger (eller en [App-adgangskode](https://myaccount.google.com/apppasswords)).
4. *Valgfrit:* Android-id'et på din enhed (for at reducere risikoen for, at Google logger dig ud).

## 🚀 Installation og brug

### Mulighed 1: Brug af Python (anbefales til brugergrænseflade)

1. Klon depotet:
   ``` bash
   git klon https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Installer de nødvendige afhængigheder:
   ``` bash
   pip install -r requirements.txt
   ```

3. Kør webserveren:
   ``` bash
   python server.py
   ```
4. Åbn din browser og gå til `http://localhost:5000` for at få adgang til det moderne dashboard!

### Mulighed 2: Brug af Docker

1. Klon depotet, og naviger ind i det.
2. Byg Docker-billedet:
   ``` bash
   docker bygning. -t whatsapp-gdrive-extractor
   ```
3. Kør Docker-beholderen:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Godkendelsesvejledning

Hvis du oplever problemer med at bruge din standard Google-e-mail og adgangskode, skal du bruge **OAuth-token**-metoden (trin 1 på webbrugergrænsefladen):
1. Gå til `https://getandroidapp.com/` eller en hvilken som helst Google-loginportal.
2. Log ind med din Google-konto.
3. Tryk på `F12` for at åbne Udviklerværktøjer.
4. Gå til **Applikation** -> **Cookies**.
5. Find `oauth_token` (det ser normalt ud som `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Kopier og indsæt det i web-brugergrænsefladen.

## 🤝 Kreditter og anerkendelser
* **Original forfatter:** TripCode
* **Kernebidragydere:** DrDeath1122 (multi-threading-rygrad), YuriCosta (nyt gendannelsessystem omvendt konstruktion), macagua (SSL-rettelser).
* **Modernisering og brugergrænseflade:** Genopbygget med en moderne webgrænseflade, lokalt kryptografimodul, ADB-telefonoverførsel og fuldt lokaliseret til 40 sprog for global tilgængelighed.