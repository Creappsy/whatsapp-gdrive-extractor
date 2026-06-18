<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Natywna, wieloplatformowa aplikacja komputerowa (Windows, macOS, Linux), szybka, lekka i wielojęzyczna, do wyodrębniania, odszyfrowywania i przywracania kopii zapasowych WhatsApp z Dysku Google.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Języki-40_Utrzymany-success.svg)](#)
  [![License](https://img.shields.io/badge/Licencja-MIT-green.svg)](#)
</div>

---

## 🌟 Cechy

* **Piękny interfejs użytkownika/UX:** Nowoczesny interfejs pulpitu z projektem glassmorphism, trybem ciemnym i przewodnikiem krok po kroku przywracania.
* **🌍 40 obsługiwanych języków:** Umiędzynarodowienie jakości natywnie zintegrowane. Natychmiast przełączaj języki.
* **Ultralekki i niezależny:** Przebudowany przy użyciu **Tauri & Rust**, redukując rozmiar pliku binarnego do zaledwie **~10MB** przy zerowych zależnościach zewnętrznych (nie jest wymagane środowisko wykonawcze Pythona ani .NET w systemie hosta).
* **Bezpieczne przechowywanie breloków:** Zapisuje tokeny Google OAuth bezpośrednio w natywnym bezpiecznym pliku kluczy systemu operacyjnego (Menedżer poświadczeń w systemie Windows, pęk kluczy w systemie macOS).
* **Lokalne deszyfrowanie E2EE (.crypt14 / .crypt15):** Szybkie, bezpieczne i całkowicie lokalne deszyfrowanie bazy danych WhatsApp AES-256-GCM przy użyciu Rusta.
* **Bezpośredni transfer na telefon (ADB):** Przywróć odszyfrowane bazy danych bezpośrednio na telefon z Androidem za pomocą jednego kliknięcia, wywołując natywnie system ADB.

---

## 📋 Wymagania

Aby skompilować i uruchomić ze źródła, upewnij się, że masz:
1. **Rdza i ładunek** (v1.77 lub wyższy)
2. **Node.js & npm**
3. Urządzenie z systemem Android z włączonym debugowaniem USB (w przypadku przywracania bezpośrednio przez ADB).

---

## 🚀 Wykonanie i rozwój

### Uruchomienie aplikacji
Po prostu kliknij dwukrotnie plik [run.bat](../run.bat) plik w katalogu głównym projektu.
Natychmiast otworzy skompilowany plik wykonywalny produkcyjny (`app.exe`) lub powróci do trybu programistycznego, jeśli nie został skompilowany.

### Działa w trybie programistycznym
Aby rozpocząć tryb programistyczny z ponownym ładowaniem na gorąco:
```bash
npx tauri dev
```

### Kompilacja i łączenie
Aby wygenerować samodzielne, podpisane pakiety instalatora produkcyjnego (konfiguracja MSI, EXE):
```bash
npx tauri build
```
Powstałe instalatory będą zlokalizowane w `src-tauri/target/release/bundle/`.

Widzieć [DISTRIBUTION.md](../DISTRIBUTION.md) aby uzyskać więcej informacji na temat pakowania i podpisywania.
