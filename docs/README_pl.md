<divlay="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Logo WhatsApp" szerokość="100"/>
  <h1>Ekstraktor Dysku Google WhatsApp 🚀</h1>
  <p><b>Nowoczesne, szybkie i wielojęzyczne narzędzie do wyodrębniania, odszyfrowywania i przywracania kopii zapasowych WhatsApp z Dysku Google.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Licencja](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Funkcje

* **Piękny internetowy interfejs użytkownika/UX:** nowoczesny, natywny interfejs w trybie ciemnym z elementami glassmorfizmu, który z łatwością poprowadzi Cię przez proces ekstrakcji.
* **🌍 Obsługiwanych 40 języków:** Wbudowane natywnie tłumaczenia w 100% o jakości ludzkiej. Zmień język w locie, bez przeładowywania!
* **Deszyfrowanie lokalne (.crypt14 / .crypt15):** W pełni lokalny moduł kryptograficzny umożliwiający bezpieczne odszyfrowywanie baz danych SQLite przy użyciu klucza skrótu E2EE bez przesyłania danych gdziekolwiek.
* **Bezpośredni transfer do Androida (ADB):** Prześlij pobraną i odszyfrowaną bazę danych bezpośrednio do pamięci telefonu z Androidem za pomocą jednego kliknięcia z poziomu pulpitu nawigacyjnego.
* **Bezpieczne uwierzytelnianie:** obsługuje zarówno tokeny Google OAuth, jak i hasła do aplikacji, zapewniając maksymalne bezpieczeństwo.

## 📋 Warunki wstępne

Zanim zaczniesz, upewnij się, że masz:
1. Zainstalowano **Python 3.x** lub **Docker**.
2. Urządzenie z systemem Android i zainstalowaną aplikacją WhatsApp oraz włączoną funkcją tworzenia kopii zapasowych na Dysku Google.
3. Dane logowania do Twojego konta Google (lub [Hasło do aplikacji](https://myaccount.google.com/apppasswords)).
4. *Opcjonalnie:* Identyfikator Androida Twojego urządzenia (aby zmniejszyć ryzyko wylogowania Cię przez Google).

## 🚀 Instalacja i użytkowanie

### Opcja 1: używanie języka Python (zalecane w przypadku interfejsu użytkownika)

1. Sklonuj repozytorium:
   ,,bicie
   klon git https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd ekstraktor WhatsApp-gdrive
   ```

2. Zainstaluj wymagane zależności:
   ,,bicie
   pip install -r wymagania.txt
   ```

3. Uruchom serwer WWW:
   ,,bicie
   serwer Pythona.py
   ```
4. Otwórz przeglądarkę i przejdź do `http://localhost:5000`, aby uzyskać dostęp do nowoczesnego panelu sterowania!

### Opcja 2: Korzystanie z Dockera

1. Sklonuj repozytorium i przejdź do niego.
2. Zbuduj obraz Dockera:
   ,,bicie
   kompilacja dokera. -t ekstraktor-Whatsapp-gdrive
   ```
3. Uruchom kontener Docker:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it WhatsApp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it WhatsApp-gdrive-extractor`

## 🔑 Przewodnik uwierzytelniania

Jeśli masz problemy ze standardowym adresem e-mail i hasłem Google, użyj metody **Tokenu OAuth** (krok 1 w interfejsie internetowym):
1. Przejdź do `https://getandroidapp.com/` lub dowolnego portalu logowania Google.
2. Zaloguj się na swoje konto Google.
3. Naciśnij `F12`, aby otworzyć Narzędzia programistyczne.
4. Przejdź do **Aplikacja** -> **Pliki cookie**.
5. Znajdź `oauth_token` (zwykle wygląda jak `oauth2_4/XXXXXXXXXXXXXXXXXX`).
6. Skopiuj i wklej go do interfejsu internetowego.

## 🤝 Uznania i podziękowania
* **Autor oryginalny:** TripCode
* **Główni współautorzy:** DrDeath1122 (wielowątkowy szkielet), YuriCosta (inżynieria wsteczna nowego systemu przywracania), macagua (poprawki SSL).
* **Modernizacja i interfejs użytkownika:** Przebudowany z nowoczesnym interfejsem internetowym, lokalnym modułem kryptograficznym, transferem telefonicznym ADB i w pełni zlokalizowany na 40 języków w celu zapewnienia globalnej dostępności.
