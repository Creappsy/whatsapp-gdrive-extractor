<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Aplikasi desktop lintas platform asli (Windows, macOS, Linux), cepat, ringan, dan multi-bahasa, untuk mengekstrak, mendekripsi, dan memulihkan cadangan WhatsApp Anda dari Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Bahasa-40_Didukung-success.svg)](#)
  [![License](https://img.shields.io/badge/Lisensi-MIT-green.svg)](#)
</div>

---

## 🌟 Fitur

* **UI/UX yang indah:** Antarmuka desktop modern dengan desain morfisme kaca, mode gelap, dan panduan restorasi langkah demi langkah.
* **🌍 40 Bahasa yang Didukung:** Internasionalisasi kualitas terintegrasi secara alami. Beralih bahasa secara instan.
* **Ultralight & Independen:** Dibangun kembali menggunakan **Tauri & Rust**, mengurangi ukuran biner menjadi hanya **~10MB** tanpa ketergantungan eksternal (tidak diperlukan runtime Python atau .NET pada sistem host).
* **Penyimpanan Gantungan Kunci Aman:** Menyimpan token Google OAuth Anda langsung ke keyring aman asli sistem operasi (Credential Manager di Windows, Keychain di macOS).
* **Dekripsi E2EE Lokal (.crypt14 / .crypt15):** Dekripsi database WhatsApp AES-256-GCM berkecepatan tinggi, aman, dan sepenuhnya lokal menggunakan Rust.
* **Transfer Langsung ke Telepon (ADB):** Pulihkan database Anda yang didekripsi langsung ke ponsel Android Anda dengan satu klik, memanggil sistem ADB secara asli.

---

## 📋 Persyaratan

Untuk membangun dan menjalankan dari sumber, pastikan Anda memiliki:
1. **Karat & Kargo** (v1.77 atau unggul)
2. **Node.js & npm**
3. Perangkat Android dengan USB Debugging diaktifkan (jika memulihkan langsung melalui ADB).

---

## 🚀 Eksekusi dan Pengembangan

### Meluncurkan Aplikasi
Cukup klik dua kali [run.bat](../run.bat) file di root proyek.
Ini akan langsung membuka produksi terkompilasi yang dapat dieksekusi (`app.exe`) atau mundur ke mode pengembangan jika tidak dikompilasi.

### Berjalan dalam Mode Pengembangan
Untuk memulai mode pengembangan dengan hot-reload:
```bash
npx tauri dev
```

### Kompilasi dan Bundling
Untuk menghasilkan paket penginstal produksi mandiri yang ditandatangani (pengaturan MSI, EXE):
```bash
npx tauri build
```
Pemasang yang dihasilkan akan berlokasi di `src-tauri/target/release/bundle/`.

Melihat [DISTRIBUTION.md](../DISTRIBUTION.md) untuk detail lebih lanjut tentang pengemasan dan penandatanganan.
