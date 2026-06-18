<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp ロゴ" width="100"/>
  <h1>WhatsApp Google ドライブ抽出ツール 🚀</h1>
  <p><b>Google ドライブから WhatsApp のバックアップを抽出、復号化、復元するための、最新の高速多言語ツールです。</b></p>

  [![Python](https://img.shields.io/badge/Python-3.x-blue.svg)](https://www.python.org/)
  [![Docker](https://img.shields.io/badge/Docker-Supported-2496ED.svg)](https://www.docker.com/)
  [![i18n](https://img.shields.io/badge/Languages-40_Supported-success.svg)](#)
  [![ライセンス](https://img.shields.io/badge/License-MIT-green.svg)](#)
</div>

---

## 🌟 特徴

* **美しい Web UI/UX:** グラスモーフィズム要素を備えた最新のダークモード ネイティブ インターフェイスにより、抽出プロセスを簡単にガイドできます。
* **🌍 40 言語をサポート:** 100% 人間による品質の翻訳がネイティブに組み込まれています。リロードせずにその場で言語を変更できます。
* **ローカル復号化 (.crypt14 / .crypt15):** データをどこにもアップロードせずに、E2EE ハッシュ キーを使用して SQLite データベースを安全に復号化する完全ローカル暗号化モジュール。
* **Direct Android Transfer (ADB):** ダッシュボードから 1 回クリックするだけで、ダウンロードして復号化したデータベースを Android スマートフォンのストレージに直接プッシュします。
* **安全な認証:** セキュリティを最大限に高めるために、Google OAuth トークンとアプリ パスワードの両方をサポートします。

## 📋 前提条件

始める前に、次のものが揃っていることを確認してください。
1. **Python 3.x** または **Docker** がインストールされている。
2. WhatsApp がインストールされ、Google ドライブのバックアップが有効になっている Android デバイス。
3. Google アカウントの認証情報 (または [アプリ パスワード](https://myaccount.google.com/apppasswords))。
4. *オプション:* デバイスの Android ID (Google によってログアウトされるリスクを軽減するため)。

## 🚀 インストールと使用方法

### オプション 1: Python の使用 (UI に推奨)

1. リポジトリのクローンを作成します。
   「」バッシュ
   git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
   cd whatsapp-gdrive-extractor
   「」

2. 必要な依存関係をインストールします。
   「」バッシュ
   pip install -r 要件.txt
   「」

3. Web サーバーを実行します。
   「」バッシュ
   Pythonサーバー.py
   「」
4. ブラウザを開いて「http://localhost:5000」に移動し、最新のダッシュボードにアクセスします。

### オプション 2: Docker の使用

1. リポジトリのクローンを作成し、そこに移動します。
2. Docker イメージをビルドします。
   「」バッシュ
   ドッカービルド 。 -t whatsapp-gdrive-extractor
   「」
3. Docker コンテナを実行します。
   * **Linux:** `docker run -v $(pwd):/app -p 5000:5000 -it whatsapp-gdrive-extractor`
   * **Windows (PowerShell):** `docker run -v .:/app -p 5000:5000 -it whatsapp-gdrive-extractor`

## 🔑 認証ガイド

標準の Google メールとパスワードの使用で問題が発生した場合は、**OAuth トークン** 方法を使用してください (Web UI のステップ 1)。
1. 「https://getandroidapp.com/」または任意の Google ログイン ポータルに移動します。
2. Google アカウントでログインします。
3. 「F12」を押して開発者ツールを開きます。
4. **アプリケーション** -> **Cookie** に移動します。
5. 「oauth_token」を見つけます (通常は「oauth2_4/XXXXXXXXXXXXXXXXX」のようになります)。
6. これをコピーして Web UI に貼り付けます。

## 🤝 クレジットと謝辞
* **原著者:** TripCode
* **中心的な貢献者:** DrDeath1122 (マルチスレッド バックボーン)、YuriCosta (新しい復元システム リバース エンジニアリング)、macagua (SSL 修正)。
* **最新化と UI:** 最新の Web インターフェイス、ローカル暗号化モジュール、ADB 電話転送を使用して再構築され、グローバル アクセシビリティのために 40 言語に完全にローカライズされています。