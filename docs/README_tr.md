<div align = "merkez">
  <img src = "https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt = "WhatsApp Logosu" width = "100"/>
  <h1>WhatsApp Google Drive Çıkarıcı 🚀</h1>
  <p><b>WhatsApp yedeklerinizi Google Drive'dan çıkarmak, şifresini çözmek ve geri yüklemek için modern, hızlı ve çok dilli bir araç.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Lisans](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 Özellikler

* **Güzel Web Kullanıcı Arayüzü/UX:** Çıkarma sürecinde size kolayca rehberlik edecek cammorfizm öğelerine sahip modern, karanlık modlu yerel bir arayüz.
* **🌍 Desteklenen 40 Dil:** Yerel olarak yerleşik %100 insan kalitesinde çeviriler. Yeniden yüklemeye gerek kalmadan anında dili değiştirin!
* **Yerel Şifre Çözme (.crypt14 / .crypt15):** Verilerinizi herhangi bir yere yüklemeden, E2EE karma anahtarınızı kullanarak SQLite veritabanlarınızın şifresini güvenli bir şekilde çözmek için tamamen yerel bir şifreleme modülü.
* **Doğrudan Android Aktarımı (ADB):** İndirdiğiniz ve şifresi çözülen veritabanınızı kontrol panelinden tek bir tıklamayla doğrudan Android telefonunuzun depolama alanına aktarın.
* **Güvenli Kimlik Doğrulama:** Maksimum güvenlik için hem Google OAuth belirteçlerini hem de Uygulama Şifrelerini destekler.

## 📋 Önkoşullar

Başlamadan önce aşağıdakilere sahip olduğunuzdan emin olun:
1. **Python 3.x** veya **Docker** yüklü.
2. WhatsApp'ın yüklü olduğu ve Google Drive yedeklemelerinin etkin olduğu bir Android cihaz.
3. Google hesabı kimlik bilgileriniz (veya [Uygulama Şifresi](https://myaccount.google.com/apppasswords)).
4. *İsteğe bağlı:* Cihazınızın Android Kimliği (Google'ın oturumunuzu kapatması riskini azaltmak için).

## 🚀 Kurulum ve Kullanım

### Seçenek 1: Python'u Kullanma (Kullanıcı Arayüzü için Önerilen)

1. Depoyu klonlayın:
   ``` bash
   git klonu https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-çıkarıcı
   ''''

2. Gerekli bağımlılıkları yükleyin:
   ``` bash
   pip kurulumu -r gereksinimleri.txt
   ''''

3. Web Sunucusunu çalıştırın:
   ``` bash
   piton sunucusu.py
   ''''
4. Tarayıcınızı açın ve modern kontrol paneline erişmek için "http://localhost:5000" adresine gidin!

### Seçenek 2: Docker'ı Kullanma

1. Depoyu klonlayın ve içine gidin.
2. Docker görüntüsünü oluşturun:
   ``` bash
   liman işçisi yapısı. -t whatsapp-gdrive-çıkarıcı
   ''''
3. Docker kapsayıcısını çalıştırın:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Kimlik Doğrulama Kılavuzu

Standart Google E-postanızı ve Şifrenizi kullanırken sorun yaşıyorsanız **OAuth Token** yöntemini kullanın (Web Kullanıcı Arayüzünde 1. Adım):
1. 'https://getandroidapp.com/' adresine veya herhangi bir Google giriş portalına gidin.
2. Google hesabınızla oturum açın.
3. Geliştirici Araçlarını açmak için 'F12' tuşuna basın.
4. **Uygulama** -> **Çerezler**'e gidin.
5. 'Oauth_token'ı bulun (Genellikle 'oauth2_4/XXXXXXXXXXXXXXXXXX' şeklinde görünür).
6. Kopyalayıp Web kullanıcı arayüzüne yapıştırın.

## 🤝 Teşekkür ve Teşekkür
* **Orijinal Yazar:** TripCode
* **Temel Katkıda Bulunanlar:** DrDeath1122 (Çok iş parçacıklı omurga), YuriCosta (Yeni geri yükleme sistemi tersine mühendislik), macagua (SSL düzeltmeleri).
* **Modernizasyon ve Kullanıcı Arayüzü:** Modern bir web arayüzü, yerel şifreleme modülü, ADB telefon aktarımı ile yeniden oluşturuldu ve küresel erişilebilirlik için 40 dile tamamen yerelleştirildi.
