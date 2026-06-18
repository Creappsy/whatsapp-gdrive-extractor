<div align="centro">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Logo WhatsApp" width="100"/>
  <h1>Estrattore WhatsApp Google Drive 🚀</h1>
  <p><b>Strumento moderno, veloce e multilingue per estrarre, decrittografare e ripristinare i backup di WhatsApp da Google Drive.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licenza](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟Caratteristiche

* **Bella interfaccia utente/UX Web:** una moderna interfaccia nativa in modalità oscura con elementi glassmorphism per guidarti facilmente attraverso il processo di estrazione.
* **🌍 40 lingue supportate:** Traduzioni di qualità umana al 100% integrate in modo nativo. Cambia la lingua al volo senza ricaricare!
* **Decrittografia locale (.crypt14 / .crypt15):** Un modulo di crittografia completamente locale per decrittografare in modo sicuro i tuoi database SQLite utilizzando la chiave hash E2EE senza caricare i tuoi dati ovunque.
* **Trasferimento diretto Android (ADB):** Invia il database scaricato e decrittografato direttamente nella memoria del tuo telefono Android con un solo clic dalla dashboard.
* **Autenticazione sicura:** Supporta sia i token OAuth di Google che le password delle app per la massima sicurezza.

## 📋Prerequisiti

Prima di iniziare, assicurati di avere:
1. **Python 3.x** o **Docker** installato.
2. Un dispositivo Android con WhatsApp installato e backup di Google Drive abilitati.
3. Le credenziali del tuo account Google (o una [Password per l'app](https://myaccount.google.com/apppasswords)).
4. *Facoltativo:* L'ID Android del tuo dispositivo (per ridurre il rischio che Google ti disconnetta).

## 🚀 Installazione e utilizzo

### Opzione 1: utilizzo di Python (consigliato per l'interfaccia utente)

1. Clona il repository:
   "bash."
   git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-estrattore
   ```

2. Installa le dipendenze richieste:
   "bash."
   pip install -r requisiti.txt
   ```

3. Eseguire il server Web:
   "bash."
   server python.py
   ```
4. Apri il browser e vai su "http://localhost:5000" per accedere alla dashboard moderna!

### Opzione 2: utilizzo di Docker

1. Clona il repository e naviga al suo interno.
2. Costruisci l'immagine Docker:
   "bash."
   creazione della finestra mobile. -t whatsapp-gdrive-estrattore
   ```
3. Esegui il contenitore Docker:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Guida all'autenticazione

Se riscontri problemi con l'utilizzo dell'e-mail e della password standard di Google, utilizza il metodo **Token OAuth** (Passaggio 1 nell'interfaccia utente Web):
1. Vai su "https://getandroidapp.com/" o su qualsiasi portale di accesso di Google.
2. Accedi con il tuo account Google.
3. Premi "F12" per aprire gli Strumenti per sviluppatori.
4. Vai su **Applicazione** -> **Cookie**.
5. Trova `oauth_token` (di solito assomiglia a `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Copialo e incollalo nell'interfaccia utente Web.

## 🤝 Crediti e riconoscimenti
* **Autore originale:** TripCode
* **Contributori principali:** DrDeath1122 (backbone multi-threading), YuriCosta (reverse engineering del nuovo sistema di ripristino), macagua (correzioni SSL).
* **Modernizzazione e interfaccia utente:** Ricostruito con un'interfaccia web moderna, modulo di crittografia locale, trasferimento telefonico ADB e completamente localizzato in 40 lingue per l'accessibilità globale.