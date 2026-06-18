<div align="centre">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Logo WhatsApp" width="100"/>
  <h1>Extracteur WhatsApp Google Drive 🚀</h1>
  <p><b>Outil moderne, rapide et multilingue pour extraire, décrypter et restaurer vos sauvegardes WhatsApp à partir de Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licence](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Fonctionnalités

* **Belle interface utilisateur/UX Web :** Une interface native moderne en mode sombre avec des éléments de morphisme de verre pour vous guider facilement tout au long du processus d'extraction.
* **🌍 40 langues prises en charge :** traductions 100 % de qualité humaine intégrées nativement. Changez la langue à la volée sans recharger !
* **Déchiffrement local (.crypt14 / .crypt15) :** Un module de cryptographie entièrement local pour décrypter en toute sécurité vos bases de données SQLite à l'aide de votre clé de hachage E2EE sans télécharger vos données n'importe où.
* **Direct Android Transfer (ADB) :** Poussez votre base de données téléchargée et déchiffrée directement dans le stockage de votre téléphone Android en un seul clic depuis le tableau de bord.
* **Authentification sécurisée :** Prend en charge les jetons Google OAuth et les mots de passe d'application pour une sécurité maximale.

## 📋 Prérequis

Avant de commencer, assurez-vous d'avoir :
1. **Python 3.x** ou **Docker** installé.
2. Un appareil Android sur lequel WhatsApp est installé et les sauvegardes Google Drive activées.
3. Les identifiants de votre compte Google (ou un [Mot de passe d'application](https://myaccount.google.com/apppasswords)).
4. *Facultatif :* L'identifiant Android de votre appareil (pour réduire le risque de déconnexion de Google).

## 🚀Installation et utilisation

### Option 1 : Utilisation de Python (recommandé pour l'interface utilisateur)

1. Clonez le référentiel :
   ```bash
   clone git https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd WhatsApp-gdrive-extracteur
   ```

2. Installez les dépendances requises :
   ```bash
   pip install -r exigences.txt
   ```

3. Exécutez le serveur Web :
   ```bash
   serveur python.py
   ```
4. Ouvrez votre navigateur et accédez à « http://localhost:5000 » pour accéder au tableau de bord moderne !

### Option 2 : Utiliser Docker

1. Clonez le référentiel et accédez-y.
2. Créez l'image Docker :
   ```bash
   construction de docker. -t WhatsApp-gdrive-extracteur
   ```
3. Exécutez le conteneur Docker :
   * **Linux :** `docker run -v $(pwd):/app -p 5000:5000 -it WhatsApp-gdrive-extractor`
   * **Windows (PowerShell) :** `docker run -v .:/app -p 5000:5000 -it WhatsApp-gdrive-extractor`

## 🔑 Guide d'authentification

Si vous rencontrez des problèmes lors de l'utilisation de votre adresse e-mail et de votre mot de passe Google standard, utilisez la méthode **OAuth Token** (étape 1 sur l'interface utilisateur Web) :
1. Accédez à « https://getandroidapp.com/ » ou à n'importe quel portail de connexion Google.
2. Connectez-vous avec votre compte Google.
3. Appuyez sur « F12 » pour ouvrir les outils de développement.
4. Accédez à **Application** -> **Cookies**.
5. Recherchez le « oauth_token » (il ressemble généralement à « oauth2_4/XXXXXXXXXXXXXXXXXX »).
6. Copiez-le et collez-le dans l'interface utilisateur Web.

## 🤝 Crédits et remerciements
* **Auteur original :** TripCode
* **Contributeurs principaux :** DrDeath1122 (épine dorsale multithreading), YuriCosta (ingénierie inverse du nouveau système de restauration), macagua (correctifs SSL).
* **Modernisation et interface utilisateur :** Reconstruit avec une interface Web moderne, un module de cryptographie locale, un transfert téléphonique ADB et entièrement localisé en 40 langues pour une accessibilité mondiale.