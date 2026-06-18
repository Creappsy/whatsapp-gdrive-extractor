<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Ứng dụng máy tính để bàn đa nền tảng gốc (Windows, macOS, Linux), nhanh, nhẹ và đa ngôn ngữ, để trích xuất, giải mã và khôi phục bản sao lưu WhatsApp của bạn từ Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Ngôn_ngữ-40_Được_hỗ_trợ-success.svg)](#)
  [![License](https://img.shields.io/badge/Giấy_phép-MIT-green.svg)](#)
</div>

---

## 🌟 Đặc trưng

* **Giao diện người dùng/UX đẹp mắt:** Giao diện máy tính để bàn hiện đại với thiết kế dạng thủy tinh, chế độ tối và hướng dẫn khôi phục từng bước.
* **🌍 40 ngôn ngữ được hỗ trợ:** Chất lượng quốc tế hóa được tích hợp ngay từ đầu. Chuyển đổi ngôn ngữ ngay lập tức.
* **Siêu nhẹ & Độc lập:** Được xây dựng lại bằng cách sử dụng **Tauri & Rust**, giảm kích thước nhị phân xuống chỉ **~10 MB** mà không cần phụ thuộc bên ngoài (không cần thời gian chạy Python hoặc .NET trên hệ thống máy chủ).
* **Lưu trữ khóa an toàn:** Lưu mã thông báo Google OAuth của bạn trực tiếp vào khóa bảo mật gốc của hệ điều hành (Trình quản lý thông tin xác thực trong Windows, Chuỗi khóa trong macOS).
* **Giải mã E2EE cục bộ (.crypt14 / .crypt15):** Giải mã cơ sở dữ liệu WhatsApp AES-256-GCM tốc độ cao, an toàn và hoàn toàn cục bộ bằng Rust.
* **Chuyển khoản trực tiếp tới điện thoại (ADB):** Khôi phục cơ sở dữ liệu đã được giải mã trực tiếp vào điện thoại Android của bạn chỉ bằng một cú nhấp chuột, gọi hệ thống ADB nguyên bản.

---

## 📋 Yêu cầu

Để xây dựng và chạy từ nguồn, hãy đảm bảo bạn có:
1. **Rỉ Sét & Hàng Hóa** (v1.77 hoặc cấp trên)
2. **Node.js & npm**
3. Thiết bị Android đã bật Gỡ lỗi USB (nếu khôi phục trực tiếp qua ADB).

---

## 🚀 Thực thi và phát triển

### Khởi chạy ứng dụng
Chỉ cần nhấp đúp vào [run.bat](../run.bat) tập tin ở thư mục gốc của dự án.
Nó sẽ ngay lập tức mở tệp thực thi sản xuất đã biên dịch (`app.exe`) hoặc dự phòng sang chế độ phát triển nếu không được biên dịch.

### Chạy ở chế độ phát triển
Để bắt đầu chế độ phát triển với tính năng tải lại nóng:
```bash
npx tauri dev
```

### Biên dịch và đóng gói
Để tạo các gói cài đặt sản xuất độc lập, có chữ ký (thiết lập MSI, EXE):
```bash
npx tauri build
```
Các trình cài đặt kết quả sẽ được đặt trong `src-tauri/target/release/bundle/`.

Nhìn thấy [DISTRIBUTION.md](../DISTRIBUTION.md) để biết thêm chi tiết về đóng gói và ký kết.
