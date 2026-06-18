<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp-Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Modernes, schnelles und mehrsprachiges Tool zum Extrahieren, Entschlüsseln und Wiederherstellen Ihrer WhatsApp-Backups aus Google Drive.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Lizenz](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Funktionen

* **Schöne Web-Benutzeroberfläche/UX:** Eine moderne, native Dark-Mode-Oberfläche mit Glassmorphismus-Elementen, die Sie einfach durch den Extraktionsprozess führt.
* **🌍 40 unterstützte Sprachen:** 100 % Übersetzungen in menschlicher Qualität sind nativ integriert. Ändern Sie die Sprache im Handumdrehen, ohne sie neu laden zu müssen!
* **Lokale Entschlüsselung (.crypt14 / .crypt15):** Ein vollständig lokales Kryptografiemodul zum sicheren Entschlüsseln Ihrer SQLite-Datenbanken mithilfe Ihres E2EE-Hash-Schlüssels, ohne Ihre Daten irgendwo hochzuladen.
* **Direct Android Transfer (ADB):** Übertragen Sie Ihre heruntergeladene und entschlüsselte Datenbank mit einem einzigen Klick über das Dashboard direkt in den Speicher Ihres Android-Telefons.
* **Sichere Authentifizierung:** Unterstützt sowohl Google OAuth-Tokens als auch App-Passwörter für maximale Sicherheit.

## 📋 Voraussetzungen

Bevor Sie beginnen, stellen Sie sicher, dass Sie Folgendes haben:
1. **Python 3.x** oder **Docker** installiert.
2. Ein Android-Gerät mit installiertem WhatsApp und aktivierten Google Drive-Backups.
3. Ihre Anmeldedaten für das Google-Konto (oder ein [App-Passwort](https://myaccount.google.com/apppasswords)).
4. *Optional:* Die Android-ID Ihres Geräts (um das Risiko zu verringern, dass Google Sie abmeldet).

## 🚀 Installation und Nutzung

### Option 1: Verwendung von Python (empfohlen für die Benutzeroberfläche)

1. Klonen Sie das Repository:
   „Bash
   Git-Klon https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   „

2. Installieren Sie die erforderlichen Abhängigkeiten:
   „Bash
   pip install -r Anforderungen.txt
   „

3. Führen Sie den Webserver aus:
   „Bash
   Python server.py
   „
4. Öffnen Sie Ihren Browser und gehen Sie zu „http://localhost:5000“, um auf das moderne Dashboard zuzugreifen!

### Option 2: Docker verwenden

1. Klonen Sie das Repository und navigieren Sie hinein.
2. Erstellen Sie das Docker-Image:
   „Bash
   Docker-Build. -t whatsapp-gdrive-extractor
   „
3. Führen Sie den Docker-Container aus:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Authentifizierungshandbuch

Wenn bei der Verwendung Ihrer standardmäßigen Google-E-Mail-Adresse und Ihres Standardkennworts Probleme auftreten, verwenden Sie die Methode **OAuth-Token** (Schritt 1 auf der Web-Benutzeroberfläche):
1. Gehen Sie zu „https://getandroidapp.com/“ oder einem beliebigen Google-Anmeldeportal.
2. Melden Sie sich mit Ihrem Google-Konto an.
3. Drücken Sie „F12“, um die Entwicklertools zu öffnen.
4. Gehen Sie zu **Anwendung** -> **Cookies**.
5. Suchen Sie das „oauth_token“ (es sieht normalerweise wie „oauth2_4/XXXXXXXXXXXXXXXXX“ aus).
6. Kopieren Sie es und fügen Sie es in die Web-Benutzeroberfläche ein.

## 🤝 Credits und Danksagungen
* **Originalautor:** TripCode
* **Hauptmitwirkende:** DrDeath1122 (Multi-Threading-Backbone), YuriCosta (Reverse Engineering des neuen Wiederherstellungssystems), Macagua (SSL-Korrekturen).
* **Modernisierung und Benutzeroberfläche:** Neu erstellt mit einer modernen Weboberfläche, lokalem Kryptografiemodul, ADB-Telefonübertragung und vollständig in 40 Sprachen lokalisiert für globale Zugänglichkeit.