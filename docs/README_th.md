<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>แอปพลิเคชันเดสก์ท็อปข้ามแพลตฟอร์มแบบเนทีฟ (Windows, macOS, Linux) รวดเร็ว น้ำหนักเบา และหลายภาษา เพื่อแยก ถอดรหัส และกู้คืนข้อมูลสำรอง WhatsApp ของคุณจาก Google Drive</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/ภาษา-40_รองรับ-success.svg)](#)
  [![License](https://img.shields.io/badge/ใบอนุญาต-MIT-green.svg)](#)
</div>

---

## 🌟 คุณสมบัติ

* **UI/UX ที่สวยงาม:** อินเทอร์เฟซเดสก์ท็อปสมัยใหม่พร้อมการออกแบบ Glassmorphism โหมดมืด และคำแนะนำในการคืนค่าทีละขั้นตอน
* **🌍 40 ภาษาที่รองรับ:** ความเป็นสากลที่มีคุณภาพบูรณาการโดยกำเนิด สลับภาษาได้ทันที
* **เบามากและเป็นอิสระ:** สร้างใหม่โดยใช้ **Tauri & Rust** ลดขนาดไบนารี่เหลือเพียง **~10MB** โดยไม่มีการพึ่งพาภายนอก (ไม่ต้องใช้ Python runtime หรือ .NET บนระบบโฮสต์)
* **การจัดเก็บพวงกุญแจที่ปลอดภัย:** บันทึกโทเค็น Google OAuth ของคุณโดยตรงไปยังคีย์ริงการรักษาความปลอดภัยดั้งเดิมของระบบปฏิบัติการ (ตัวจัดการข้อมูลรับรองใน Windows, พวงกุญแจใน macOS)
* **การถอดรหัส E2EE ในเครื่อง (.crypt14 / .crypt15):** การถอดรหัสฐานข้อมูล AES-256-GCM WhatsApp ความเร็วสูง ปลอดภัย และภายในเครื่องทั้งหมดโดยใช้ Rust
* **โอนโดยตรงไปยังโทรศัพท์ (ADB):** กู้คืนฐานข้อมูลที่ถอดรหัสแล้วของคุณโดยตรงไปยังโทรศัพท์ Android ของคุณด้วยการคลิกเพียงครั้งเดียว เรียกระบบ ADB แบบเนทีฟ

---

## 📋 ความต้องการ

หากต้องการสร้างและเรียกใช้จากแหล่งที่มา ตรวจสอบให้แน่ใจว่าคุณมี:
1. **สนิมและสินค้า** (v1.77 หรือเหนือกว่า)
2. **Node.js & npm**
3. อุปกรณ์ Android ที่เปิดใช้งานการแก้ไขจุดบกพร่อง USB (หากกู้คืนโดยตรงผ่าน ADB)

---

## 🚀 การดำเนินการและการพัฒนา

### การเปิดตัวแอปพลิเคชัน
เพียงดับเบิลคลิกที่ [run.bat](../run.bat) ไฟล์ที่รากของโครงการ
มันจะเปิดไฟล์ปฏิบัติการที่คอมไพล์แล้ว (`app.exe`) ทันทีหรือเปิดทางเลือกสู่โหมดการพัฒนาหากไม่ได้คอมไพล์

### ทำงานในโหมดการพัฒนา
ในการเริ่มโหมดการพัฒนาด้วยการโหลดซ้ำอย่างรวดเร็ว:
```bash
npx tauri dev
```

### การรวบรวมและการรวมกลุ่ม
หากต้องการสร้างแพ็คเกจตัวติดตั้งที่ใช้งานจริงและลงนามแล้ว (การตั้งค่า MSI, EXE):
```bash
npx tauri build
```
โปรแกรมติดตั้งที่ได้จะอยู่ใน `src-tauri/target/release/bundle/`

ดู [DISTRIBUTION.md](../DISTRIBUTION.md) สำหรับรายละเอียดเพิ่มเติมเกี่ยวกับบรรจุภัณฑ์และการลงนาม
