<div 정렬="중앙">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp 로고" width="100"/>
  <h1>WhatsApp Google 드라이브 추출기 🚀</h1>
  <p><b>Google 드라이브에서 WhatsApp 백업을 추출, 암호화 해제 및 복원할 수 있는 현대적이고 빠른 다국어 도구입니다.</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![라이선스](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 특징

* **아름다운 웹 UI/UX:** 추출 프로세스를 쉽게 안내할 수 있는 glassmorphism 요소가 포함된 현대적인 다크 모드 기본 인터페이스입니다.
* **🌍 40개 언어 지원:** 100% 인간 수준의 번역이 기본적으로 내장되어 있습니다. 다시 로드할 필요 없이 즉시 언어를 변경하세요!
* **로컬 복호화(.crypt14 / .crypt15):** 데이터를 어디에도 업로드하지 않고 E2EE 해시 키를 사용하여 SQLite 데이터베이스를 안전하게 복호화하는 완전 로컬 암호화 모듈입니다.
* **직접 Android 전송(ADB):** 대시보드에서 한 번의 클릭으로 다운로드하고 해독한 데이터베이스를 Android 휴대폰의 저장소로 직접 푸시할 수 있습니다.
* **안전한 인증:** 보안을 극대화하기 위해 Google OAuth 토큰과 앱 비밀번호를 모두 지원합니다.

## 📋 전제 조건

시작하기 전에 다음 사항을 확인하세요.
1. **Python 3.x** 또는 **Docker**가 설치되어 있습니다.
2. WhatsApp이 설치되어 있고 Google 드라이브 백업이 활성화된 Android 기기.
3. Google 계정 자격 증명(또는 [앱 비밀번호](https://myaccount.google.com/apppasswords)).
4. *선택 사항:* 기기의 Android ID(Google에서 로그아웃할 위험을 줄이기 위해).

## 🚀 설치 및 사용

### 옵션 1: Python 사용(UI에 권장)

1. 저장소를 복제합니다.
   ``배쉬
   자식 클론 https://github.com/daferferso/whatsapp-gdrive-extractor.git
   CD Whatsapp-gdrive-추출기
   ````

2. 필요한 종속성을 설치합니다.
   ``배쉬
   pip 설치 -r 요구사항.txt
   ````

3. 웹 서버를 실행합니다:
   ``배쉬
   파이썬 서버.py
   ````
4. 브라우저를 열고 'http://localhost:5000'으로 이동하여 최신 대시보드에 액세스하세요!

### 옵션 2: Docker 사용

1. 저장소를 복제하고 해당 저장소로 이동합니다.
2. Docker 이미지를 빌드합니다.
   ``배쉬
   도커 빌드 . -t whatsapp-gdrive-추출기
   ````
3. Docker 컨테이너를 실행합니다.
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows(PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 인증 안내

표준 Google 이메일 및 비밀번호를 사용하는 데 문제가 발생하는 경우 **OAuth 토큰** 방법(웹 UI의 1단계)을 사용하세요.
1. `https://getandroidapp.com/` 또는 Google 로그인 포털로 이동합니다.
2. 구글 계정으로 로그인하세요.
3. 'F12'를 눌러 개발자 도구를 엽니다.
4. **애플리케이션** -> **쿠키**로 이동합니다.
5. `oauth_token`을 찾습니다(보통 `oauth2_4/XXXXXXXXXXXXXXXXX`와 유사함).
6. 복사하여 웹 UI에 붙여넣습니다.

## 🤝 크레딧 및 감사의 말씀
* **원저자:** TripCode
* **핵심 기여자:** DrDeath1122(멀티스레딩 백본), YuriCosta(새로운 복원 시스템 리버스 엔지니어링), macagua(SSL 수정).
* **현대화 및 UI:** 최신 웹 인터페이스, 로컬 암호화 모듈, ADB 전화 전송으로 재구축되었으며 전 세계 접근성을 위해 40개 언어로 완전히 현지화되었습니다.