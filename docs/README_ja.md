<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Google ドライブから WhatsApp バックアップを抽出、復号化、復元するための、高速、軽量、多言語対応のネイティブ クロスプラットフォーム デスクトップ アプリケーション (Windows、macOS、Linux)。</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/言語-40_サポートされています-success.svg)](#)
  [![License](https://img.shields.io/badge/ライセンス-MIT-green.svg)](#)
</div>

---

## 🌟 特徴

* **美しいUI/UX:** グラスモーフィズム デザイン、ダーク モード、ステップバイステップの復元ガイドを備えた最新のデスクトップ インターフェイス。
* **🌍 サポートされている 40 の言語:** 品質の国際化がネイティブに統合されています。言語を瞬時に切り替えます。
* **超軽量&独立型:** **Tauri と Rust** を使用して再構築され、バイナリ サイズがわずか **~10MB** に削減され、外部依存関係はありません (ホスト システムに Python ランタイムや .NET は必要ありません)。
* **安全なキーリングの保管:** Google OAuth トークンをオペレーティング システムのネイティブの安全なキーリング (Windows の認証情報マネージャー、macOS のキーチェーン) に直接保存します。
* **ローカル E2EE 復号化 (.crypt14 / .crypt15):** Rust を使用した、高速、安全、完全にローカルな AES-256-GCM WhatsApp データベースの復号化。
* **電話への直接転送 (ADB):** 復号化したデータベースをワンクリックで Android スマートフォンに直接復元し、システム ADB をネイティブに呼び出します。

---

## 📋 要件

ソースからビルドして実行するには、以下が揃っていることを確認してください。
1. **錆と貨物** (v1.77 またはそれ以上の)
2. **Node.js & npm**
3. USB デバッグが有効になっている Android デバイス (ADB 経由で直接復元する場合)。

---

## 🚀 実行と開発

### アプリケーションの起動
ダブルクリックするだけで、 [run.bat](../run.bat) プロジェクトのルートにあるファイル。
コンパイルされた実稼働実行可能ファイル (「app.exe」) を即座に開きます。コンパイルされていない場合は開発モードにフォールバックします。

### 開発モードで実行する
ホットリロードを使用して開発モードを開始するには:
```bash
npx tauri dev
```

### コンパイルとバンドル
自己完結型の署名付き実稼働インストーラー パッケージ (MSI、EXE セットアップ) を生成するには、次の手順を実行します。
```bash
npx tauri build
```
結果のインストーラーは `src-tauri/target/release/bundle/` に配置されます。

見る [DISTRIBUTION.md](../DISTRIBUTION.md) パッケージと署名の詳細については、こちらをご覧ください。
