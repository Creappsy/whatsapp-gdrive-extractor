<div align="tengah">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Logo WhatsApp" width="100"/>
  <h1>Ekstrak Google Drive WhatsApp 🚀</h1>
  <p><b>Alat modern, cepat, dan multi-bahasa untuk mengekstrak, mendekripsi, dan memulihkan cadangan WhatsApp Anda dari Google Drive.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Lisensi](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Fitur

* **UI/UX Web yang Indah:** Antarmuka asli mode gelap yang modern dengan elemen morfisme kaca untuk memandu Anda dengan mudah melalui proses ekstraksi.
* **🌍 40 Bahasa yang Didukung:** 100% terjemahan berkualitas manusia sudah ada di dalamnya. Ubah bahasa dengan cepat tanpa memuat ulang!
* **Dekripsi Lokal (.crypt14 / .crypt15):** Modul kriptografi lokal sepenuhnya untuk mendekripsi database SQLite Anda dengan aman menggunakan kunci hash E2EE tanpa mengunggah data Anda di mana pun.
* **Transfer Android Langsung (ADB):** Dorong database Anda yang diunduh dan didekripsi langsung ke penyimpanan ponsel Android Anda dengan satu klik dari dasbor.
* **Otentikasi Aman:** Mendukung token Google OAuth dan Kata Sandi Aplikasi untuk keamanan maksimum.

## 📋 Prasyarat

Sebelum memulai, pastikan Anda memiliki:
1. **Python 3.x** atau **Docker** terpasang.
2. Perangkat Android dengan WhatsApp terinstal dan cadangan Google Drive diaktifkan.
3. Kredensial akun Google Anda (atau [Kata Sandi Aplikasi](https://myaccount.google.com/apppasswords)).
4. *Opsional:* ID Android perangkat Anda (untuk mengurangi risiko Google mengeluarkan Anda).

## 🚀 Instalasi & Penggunaan

### Opsi 1: Menggunakan Python (Direkomendasikan untuk UI)

1. Kloning repositori:
   ``` pesta
   git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-ekstraktor
   ```

2. Instal dependensi yang diperlukan:
   ``` pesta
   instalasi pip -r persyaratan.txt
   ```

3. Jalankan Server Web:
   ``` pesta
   server python.py
   ```
4. Buka browser Anda dan buka `http://localhost:5000` untuk mengakses dasbor modern!

### Opsi 2: Menggunakan Docker

1. Kloning repositori dan navigasikan ke dalamnya.
2. Bangun image Docker:
   ``` pesta
   membangun buruh pelabuhan. -t whatsapp-gdrive-ekstraktor
   ```
3. Jalankan wadah Docker:
   * **Linux:** `buruh pelabuhan menjalankan -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `buruh pelabuhan menjalankan -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Panduan Otentikasi

Jika Anda mengalami masalah saat menggunakan Email dan Kata Sandi Google standar, gunakan metode **OAuth Token** (Langkah 1 di UI Web):
1. Buka `https://getandroidapp.com/` atau portal masuk Google mana pun.
2. Masuk dengan akun Google Anda.
3. Tekan `F12` untuk membuka Alat Pengembang.
4. Buka **Aplikasi** -> **Cookie**.
5. Temukan `oauth_token` (Biasanya terlihat seperti `oauth2_4/XXXXXXXXXXXXXXXXX`).
6. Salin dan tempel ke UI Web.

## 🤝 Kredit & Ucapan Terima Kasih
* **Penulis Asli:** TripCode
* **Kontributor Inti:** DrDeath1122 (tulang punggung multi-threading), YuriCosta (Rekayasa balik sistem pemulihan baru), macagua (perbaikan SSL).
* **Modernisasi & UI:** Dibangun kembali dengan antarmuka web modern, modul kriptografi lokal, transfer telepon ADB, dan sepenuhnya dilokalkan ke dalam 40 bahasa untuk aksesibilitas global.