<div căn chỉnh="trung tâm">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="Logo WhatsApp" width="100"/>
  <h1>Trình trích xuất Google Drive của WhatsApp 🚀</h1>
  <p><b>Công cụ hiện đại, nhanh chóng và đa ngôn ngữ để trích xuất, giải mã và khôi phục bản sao lưu WhatsApp của bạn từ Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 اردو</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 मराठी</a> • <a href="README_te.md">🇮🇳 తెలుగు</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 தமிழ்</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 ไทย</a> • <a href="README_fa.md">🇮🇷 فارسی</a> • <a href="README_gu.md">🇮🇳 ગુજરાતી</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Українська</a> • <a href="README_el.md">🇬🇷 Ελληνικά</a> • <a href="README_hu.md">🇭🇺 Magyar</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Български</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Српски</a>
    </p>
  </details>


  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![Giấy phép](https://img.shields.io/badge/Lince-MIT-green.svg)](#)
</div>

---

## 🌟 Tính năng

* **Giao diện người dùng/UX web đẹp mắt:** Giao diện gốc ở chế độ tối, hiện đại với các thành phần hình thái thủy tinh để dễ dàng hướng dẫn bạn trong quá trình trích xuất.
* **🌍 40 ngôn ngữ được hỗ trợ:** Bản dịch gốc 100% do con người thực hiện được tích hợp sẵn. Thay đổi ngôn ngữ nhanh chóng mà không cần tải lại!
* **Giải mã cục bộ (.crypt14 / .crypt15):** Một mô-đun mật mã cục bộ hoàn toàn để giải mã cơ sở dữ liệu SQLite một cách an toàn bằng cách sử dụng khóa băm E2EE mà không cần tải dữ liệu của bạn lên bất cứ đâu.
* **Chuyển Android trực tiếp (ADB):** Đẩy cơ sở dữ liệu đã tải xuống và giải mã của bạn trực tiếp vào bộ nhớ điện thoại Android của bạn chỉ bằng một cú nhấp chuột từ trang tổng quan.
* **Xác thực an toàn:** Hỗ trợ cả mã thông báo OAuth của Google và Mật khẩu ứng dụng để bảo mật tối đa.

## 📋 Điều kiện tiên quyết

Trước khi bắt đầu, hãy đảm bảo bạn có:
1. **Đã cài đặt **Python 3.x** hoặc **Docker**.
2. Thiết bị Android đã cài đặt WhatsApp và bật sao lưu Google Drive.
3. Thông tin đăng nhập tài khoản Google của bạn (hoặc [Mật khẩu ứng dụng](https://myaccount.google.com/apppasswords)).
4. *Tùy chọn:* ID Android của thiết bị của bạn (để giảm nguy cơ Google đăng xuất bạn).

## 🚀 Cài đặt & Sử dụng

### Tùy chọn 1: Sử dụng Python (Được khuyến nghị cho UI)

1. Sao chép kho lưu trữ:
   ``` bash
   bản sao git https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   ```

2. Cài đặt các phụ thuộc cần thiết:
   ``` bash
   cài đặt pip -r require.txt
   ```

3. Chạy máy chủ Web:
   ``` bash
   máy chủ python.py
   ```
4. Mở trình duyệt của bạn và truy cập `http://localhost:5000` để truy cập bảng điều khiển hiện đại!

### Cách 2: Sử dụng Docker

1. Sao chép kho lưu trữ và điều hướng vào đó.
2. Xây dựng hình ảnh Docker:
   ``` bash
   xây dựng docker. -t whatsapp-gdrive-extractor
   ```
3. Chạy vùng chứa Docker:
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 Hướng dẫn xác thực

Nếu bạn gặp sự cố khi sử dụng Email và Mật khẩu Google tiêu chuẩn của mình, hãy sử dụng phương thức **Mã thông báo OAuth** (Bước 1 trên Giao diện người dùng web):
1. Truy cập `https://getandroidapp.com/` hoặc bất kỳ cổng đăng nhập Google nào.
2. Đăng nhập bằng tài khoản Google của bạn.
3. Nhấn `F12` để mở Công cụ dành cho nhà phát triển.
4. Đi tới **Ứng dụng** -> **Cookie**.
5. Tìm `oauth_token` (Nó thường trông giống như `oauth2_4/XXXXXXXXXXXXXXXXXX`).
6. Sao chép và dán nó vào Giao diện người dùng Web.

## 🤝 Tín dụng & Lời cảm ơn
* **Tác giả gốc:** TripCode
* **Người đóng góp cốt lõi:** DrDeath1122 (Xương sống đa luồng), YuriCosta (Kỹ thuật đảo ngược hệ thống khôi phục mới), macagua (sửa lỗi SSL).
* **Hiện đại hóa & giao diện người dùng:** Được xây dựng lại với giao diện web hiện đại, mô-đun mật mã cục bộ, chuyển điện thoại ADB và được bản địa hóa hoàn toàn sang 40 ngôn ngữ để có thể truy cập toàn cầu.
