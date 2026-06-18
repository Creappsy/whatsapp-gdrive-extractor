<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp logotip" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Moderan, brz i višejezični alat za izdvajanje, dešifriranje i vraćanje vaših WhatsApp sigurnosnih kopija s Google diska.</b></p>

  <details>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licenca](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Značajke

* **Prekrasno web sučelje/UX:** Moderno izvorno sučelje tamnog načina s elementima staklomorfizma koji vas lako vode kroz proces ekstrakcije.
* **🌍 Podržano 40 jezika:** izvorno ugrađeni prijevodi 100% ljudske kvalitete. Promijenite jezik u hodu bez ponovnog učitavanja!
* **Lokalna dešifracija (.crypt14 / .crypt15):** Potpuno lokalni kriptografski modul za sigurno dešifriranje vaših SQLite baza podataka pomoću vašeg E2EE hash ključa bez učitavanja vaših podataka bilo gdje.
* **Direct Android Transfer (ADB):** Gurnite svoju preuzetu i dekriptiranu bazu podataka izravno u pohranu vašeg Android telefona jednim klikom s nadzorne ploče.
* **Sigurna autentifikacija:** Podržava i Google OAuth tokene i zaporke aplikacije za maksimalnu sigurnost.

## 📋 Preduvjeti

Prije nego počnete, provjerite imate li:
1. **Python 3.x** ili **Docker** instaliran.
2. Android uređaj s instaliranim WhatsAppom i omogućenim sigurnosnim kopijama na Google disku.
3. Vjerodajnice vašeg Google računa (ili [lozinka aplikacije](https://myaccount.google.com/apppasswords)).
4. *Izborno:* Android ID vašeg uređaja (kako biste smanjili rizik da vas Google odjavi).

## 🚀 Instalacija i upotreba

### Opcija 1: Korištenje Pythona (preporučeno za korisničko sučelje)

1. Klonirajte spremište:
   ```baš
   git klon https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Instalirajte potrebne ovisnosti:
   ```baš
   pip install -r zahtjevi.txt
   ```

3. Pokrenite web poslužitelj:
   ```baš
   python poslužitelj.py
   ```
4. Otvorite svoj preglednik i idite na `http://localhost:5000` za pristup modernoj nadzornoj ploči!

### Opcija 2: Korištenje Dockera

1. Klonirajte spremište i idite u njega.
2. Izgradite Docker sliku:
   ```baš
   docker build . -t WhatsApp-gdrive-extractor
   ```
3. Pokrenite Docker spremnik:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Vodič za autentifikaciju

Ako imate problema s korištenjem standardne Google e-pošte i lozinke, upotrijebite metodu **OAuth Token** (1. korak na web sučelju):
1. Idite na `https://getandroidapp.com/` ili bilo koji Google portal za prijavu.
2. Prijavite se svojim Google računom.
3. Pritisnite `F12` za otvaranje Alata za razvojne programere.
4. Idite na **Aplikacija** -> **Kolačići**.
5. Pronađite `oauth_token` (obično izgleda kao `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Kopirajte ga i zalijepite u web sučelje.

## 🤝 Zasluge i zahvale
* **Izvorni autor:** TripCode
* **Osnovni suradnici:** DrDeath1122 (Multi-threading okosnica), YuriCosta (Novi obrnuti inženjering sustava za vraćanje), macagua (SSL popravci).
* **Modernizacija i korisničko sučelje:** Obnovljeno s modernim web sučeljem, lokalnim kriptografskim modulom, ADB telefonskim prijenosom i potpuno lokalizirano na 40 jezika za globalnu dostupnost.