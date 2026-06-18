<divlay="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Logo WhatsApp" szerokość="100"/>
  <h1>Ekstraktor Dysku Google WhatsApp 🚀</h1>
  <p><b>Nowoczesne, szybkie i wielojęzyczne narzędzie do wyodrębniania, odszyfrowywania i przywracania kopii zapasowych WhatsApp z Dysku Google.</b></p>

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