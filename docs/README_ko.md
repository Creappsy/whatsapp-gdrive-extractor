<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Google Drive에서 WhatsApp 백업을 추출, 해독 및 복원하기 위한 빠르고 가벼운 다중 언어 기본 크로스 플랫폼 데스크톱 애플리케이션(Windows, macOS, Linux)입니다.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/언어-40_지원됨-success.svg)](#)
  [![License](https://img.shields.io/badge/특허-MIT-green.svg)](#)
</div>

---

## 🌟 특징

* **아름다운 UI/UX:** Glassmorphism 디자인, 다크 모드, 단계별 복원 가이드를 갖춘 최신 데스크탑 인터페이스입니다.
* **🌍 40개 지원 언어:** 고품질의 국제화가 기본적으로 통합되었습니다. 즉시 언어를 전환하세요.
* **초경량 및 독립성:** **Tauri & Rust**를 사용하여 재구축하여 외부 종속성이 전혀 없는 바이너리 크기를 **~10MB**로 줄였습니다(호스트 시스템에 Python 런타임이나 .NET이 필요하지 않음).
* **보안 키링 저장소:** Google OAuth 토큰을 운영 체제의 기본 보안 키링(Windows의 Credential Manager, macOS의 키체인)에 직접 저장합니다.
* **로컬 E2EE 암호 해독(.crypt14 / .crypt15):** Rust를 사용하여 빠르고 안전하며 완전히 로컬인 AES-256-GCM WhatsApp 데이터베이스 암호를 해독합니다.
* **전화로 직접 전송(ADB):** 한 번의 클릭으로 시스템 ADB를 기본적으로 호출하여 해독된 데이터베이스를 Android 휴대폰에 직접 복원합니다.

---

## 📋 요구사항

소스에서 빌드하고 실행하려면 다음 사항을 확인하세요.
1. **러스트 앤 카고** (v1.77 또는 우수한)
2. **Node.js & npm**
3. USB 디버깅이 활성화된 Android 장치(ADB를 통해 직접 복원하는 경우)

---

## 🚀 실행 및 개발

### 애플리케이션 실행
간단히 두 번 클릭하면 [run.bat](../run.bat) 프로젝트 루트에 있는 파일입니다.
컴파일된 프로덕션 실행 파일(`app.exe`)을 즉시 열거나 컴파일되지 않은 경우 개발 모드로 대체합니다.

### 개발 모드에서 실행
핫 리로딩으로 개발 모드를 시작하려면 다음 안내를 따르세요.
```bash
npx tauri dev
```

### 컴파일 및 번들링
독립적이고 서명된 프로덕션 설치 프로그램 패키지(MSI, EXE 설정)를 생성하려면 다음을 수행하십시오.
```bash
npx tauri build
```
결과 설치 프로그램은 `src-tauri/target/release/bundle/`에 위치하게 됩니다.

보다 [DISTRIBUTION.md](../DISTRIBUTION.md) 포장 및 서명에 대한 자세한 내용은
