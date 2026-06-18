<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>本机跨平台桌面应用程序（Windows、macOS、Linux），快速、轻量级和多语言，用于从 Google 云端硬盘提取、解密和恢复 WhatsApp 备份。</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/语言-40_支持-success.svg)](#)
  [![License](https://img.shields.io/badge/执照-MIT-green.svg)](#)
</div>

---

## 🌟 特征

* **漂亮的用户界面/用户体验：** 现代桌面界面，采用玻璃态设计、深色模式和分步恢复指南。
* **🌍 40 种支持的语言：** 质量国际化原生集成。立即切换语言。
* **超轻且独立：** 使用 **Tauri 和 Rust** 进行重建，将二进制文件大小减少到仅 **~10MB**，外部依赖性为零（主机系统上不需要 Python 运行时或 .NET）。
* **安全密钥环存储：** 将您的 Google OAuth 令牌直接保存到操作系统的本机安全密钥环（Windows 中的凭据管理器，macOS 中的钥匙串）。
* **本地E2EE解密（.crypt14 / .crypt15）：** 使用 Rust 进行高速、安全且完全本地的 AES-256-GCM WhatsApp 数据库解密。
* **直接转接到电话 (ADB)：** 只需单击一下，本地调用系统 ADB，即可将解密的数据库直接恢复到 Android 手机。

---

## 📋 要求

要从源代码构建并运行，请确保您拥有：
1. **铁锈与货物** (v1.77 或上级)
2. **Node.js & npm**
3. 启用 USB 调试的 Android 设备（如果直接通过 ADB 恢复）。

---

## 🚀 执行与发展

### 启动应用程序
只需双击 [run.bat](../run.bat) 文件位于项目的根目录下。
它将立即打开已编译的生产可执行文件（“app.exe”），如果未编译，则回退到开发模式。

### 在开发模式下运行
要通过热重载启动开发模式：
```bash
npx tauri dev
```

### 编译和捆绑
要生成独立的、签名的生产安装程序包（MSI、EXE 安装程序）：
```bash
npx tauri build
```
生成的安装程序将位于“src-tauri/target/release/bundle/”中。

看 [DISTRIBUTION.md](../DISTRIBUTION.md) 有关包装和签名的更多详细信息。
