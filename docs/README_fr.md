<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Application de bureau multiplateforme native (Windows, macOS, Linux), rapide, légère et multilingue, pour extraire, décrypter et restaurer vos sauvegardes WhatsApp depuis Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Langues-40_Soutenu-success.svg)](#)
  [![License](https://img.shields.io/badge/Licence-MIT-green.svg)](#)
</div>

---

## 🌟 Caractéristiques

* **Belle UI/UX :** Une interface de bureau moderne avec un design glassmorphism, un mode sombre et un guide de restauration étape par étape.
* **🌍 40 langues prises en charge :** Internationalisation de qualité nativement intégrée. Changez de langue instantanément.
* **Ultraléger et indépendant :** Reconstruit à l'aide de **Tauri & Rust**, réduisant la taille du binaire à seulement **~10 Mo** sans aucune dépendance externe (aucun environnement d'exécution Python ou .NET requis sur le système hôte).
* **Stockage sécurisé du porte-clés :** Enregistre vos jetons Google OAuth directement dans le trousseau sécurisé natif du système d'exploitation (Credential Manager sous Windows, trousseau sous macOS).
* **Décryptage E2EE local (.crypt14 / .crypt15) :** Décryptage de base de données WhatsApp AES-256-GCM haut débit, sécurisé et entièrement local à l'aide de Rust.
* **Transfert direct vers le téléphone (ADB) :** Restaurez vos bases de données décryptées directement sur votre téléphone Android en un seul clic, en appelant le système ADB de manière native.

---

## 📋 Exigences

Pour créer et exécuter à partir des sources, assurez-vous d'avoir :
1. **Rouille et cargaison** (v1.77 ou supérieur)
2. **Node.js & npm**
3. Un appareil Android avec le débogage USB activé (en cas de restauration directement via ADB).

---

## 🚀 Exécution et développement

### Lancement de l'application
Double-cliquez simplement sur le [run.bat](../run.bat) fichier à la racine du projet.
Il ouvrira instantanément l'exécutable de production compilé (`app.exe`) ou reviendra en mode développement s'il n'est pas compilé.

### Exécution en mode développement
Pour démarrer le mode développement avec rechargement à chaud :
```bash
npx tauri dev
```

### Compilation et regroupement
Pour générer des packages d'installation de production autonomes et signés (configuration MSI, EXE) :
```bash
npx tauri build
```
Les installateurs résultants seront situés dans `src-tauri/target/release/bundle/`.

Voir [DISTRIBUTION.md](../DISTRIBUTION.md) pour plus de détails sur l’emballage et la signature.
