<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Sigla WhatsApp" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Instrument modern, rapid și în mai multe limbi pentru extragerea, decriptarea și restaurarea backup-urilor WhatsApp din Google Drive.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licență](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Caracteristici

* **Beautiful Web UI/UX:** O interfață nativă modernă, în mod întunecat, cu elemente de sticlămorfism pentru a vă ghida cu ușurință prin procesul de extracție.
* **🌍 40 de limbi acceptate:** traduceri 100% de calitate umană încorporate nativ. Schimbați limba din mers fără reîncărcare!
* **Decriptare locală (.crypt14 / .crypt15):** Un modul de criptare complet local pentru a decripta în siguranță bazele de date SQLite folosind cheia hash E2EE fără a încărca datele oriunde.
* **Transfer direct Android (ADB):** Împingeți baza de date descărcată și decriptată direct în spațiul de stocare al telefonului Android cu un singur clic din tabloul de bord.
* **Autentificare sigură:** acceptă atât jetoane OAuth Google, cât și parole de aplicație pentru securitate maximă.

## 📋 Condiții preliminare

Înainte de a începe, asigurați-vă că aveți:
1. **Python 3.x** sau **Docker** instalat.
2. Un dispozitiv Android cu WhatsApp instalat și backup-uri Google Drive activate.
3. Acreditările contului dvs. Google (sau o [Parola aplicației](https://myaccount.google.com/apppasswords)).
4. *Opțional:* ID-ul Android al dispozitivului dvs. (pentru a reduce riscul ca Google să vă deconecteze).

## 🚀 Instalare și utilizare

### Opțiunea 1: Utilizarea Python (recomandat pentru UI)

1. Clonează depozitul:
   ```bash
   clona git https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Instalați dependențele necesare:
   ```bash
   pip install -r requirements.txt
   ```

3. Rulați serverul web:
   ```bash
   python server.py
   ```
4. Deschideți browserul și accesați `http://localhost:5000` pentru a accesa tabloul de bord modern!

### Opțiunea 2: Utilizarea Docker

1. Clonați depozitul și navigați în el.
2. Construiți imaginea Docker:
   ```bash
   docker build . -t whatsapp-gdrive-extractor
   ```
3. Rulați containerul Docker:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Ghid de autentificare

Dacă întâmpinați probleme la utilizarea e-mailului și a parolei Google standard, utilizați metoda **OAuth Token** (pasul 1 pe interfața de utilizare web):
1. Accesați `https://getandroidapp.com/` sau orice portal de conectare Google.
2. Conectați-vă cu contul dvs. Google.
3. Apăsați „F12” pentru a deschide Instrumente pentru dezvoltatori.
4. Accesați **Aplicație** -> **Cookie-uri**.
5. Găsiți `oauth_token` (de obicei arată ca `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Copiați și inserați-l în interfața de utilizare web.

## 🤝 Credite și mulțumiri
* **Autor original:** TripCode
* **Colaboratori de bază:** DrDeath1122 (coloana vertebrală multi-threading), YuriCosta (inginerie inversă a noului sistem de restaurare), macagua (remedieri SSL).
* **Modernizare și interfață de utilizare:** Reconstruit cu o interfață web modernă, modul de criptare locală, transfer telefonic ADB și complet localizat în 40 de limbi pentru accesibilitate globală.