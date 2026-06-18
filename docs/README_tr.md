<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>WhatsApp yedeklerinizi Google Drive'dan çıkarmak, şifresini çözmek ve geri yüklemek için hızlı, hafif ve çok dilli, yerel platformlar arası masaüstü uygulaması (Windows, macOS, Linux).</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Diller-40_Destekleniyor-success.svg)](#)
  [![License](https://img.shields.io/badge/Lisans-MIT-green.svg)](#)
</div>

---

## 🌟 Özellikler

* **Güzel UI/UX:** Cammorfizm tasarımı, karanlık mod ve adım adım restorasyon kılavuzu içeren modern bir masaüstü arayüzü.
* **🌍 40 Desteklenen Dil:** Kalitenin uluslararası hale getirilmesi yerel olarak entegre edilmiştir. Dilleri anında değiştirin.
* **Ultra Hafif ve Bağımsız:** **Tauri ve Rust** kullanılarak yeniden oluşturuldu ve sıfır harici bağımlılıkla ikili dosya boyutu yalnızca **~10MB**'ye düşürüldü (ana sistemde Python çalışma zamanları veya .NET gerekmez).
* **Güvenli Anahtarlık Depolama:** Google OAuth jetonlarınızı doğrudan işletim sisteminin yerel güvenli anahtarlığına (Windows'ta Kimlik Bilgisi Yöneticisi, macOS'ta Anahtar Zinciri) kaydeder.
* **Yerel E2EE Şifre Çözme (.crypt14 / .crypt15):** Rust kullanarak yüksek hızlı, güvenli ve tamamen yerel AES-256-GCM WhatsApp veritabanı şifre çözme.
* **Telefona Doğrudan Transfer (ADB):** Şifresi çözülmüş veritabanlarınızı tek tıklamayla doğrudan Android telefonunuza geri yükleyin ve sistem ADB'yi yerel olarak çağırın.

---

## 📋 Gereksinimler

Derlemek ve kaynaktan çalıştırmak için aşağıdakilere sahip olduğunuzdan emin olun:
1. **Pas ve Kargo** (v1.77 veya üstün)
2. **Node.js & npm**
3. USB Hata Ayıklamanın etkin olduğu bir Android cihazı (doğrudan ADB aracılığıyla geri yükleniyorsa).

---

## 🚀 Uygulama ve Geliştirme

### Uygulamanın Başlatılması
Basitçe çift tıklayın [run.bat](../run.bat) projenin kökündeki dosya.
Derlenmiş üretim yürütülebilir dosyasını ('app.exe') anında açar veya derlenmemişse geliştirme moduna geri döner.

### Geliştirme Modunda Çalıştırma
Geliştirme modunu çalışırken yeniden yüklemeyle başlatmak için:
```bash
npx tauri dev
```

### Derleme ve Paketleme
Bağımsız, imzalı üretim yükleyici paketleri (MSI, EXE kurulumu) oluşturmak için:
```bash
npx tauri build
```
Sonuçta ortaya çıkan yükleyiciler "src-tauri/target/release/bundle/" konumunda bulunacaktır.

Görmek [DISTRIBUTION.md](../DISTRIBUTION.md) Paketleme ve imzalama hakkında daha fazla bilgi için.
