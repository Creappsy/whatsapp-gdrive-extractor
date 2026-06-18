<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Aplicativo de desktop nativo de plataforma cruzada (Windows, macOS, Linux), rápido, leve e multilíngue, para extrair, descriptografar e restaurar seus backups do WhatsApp do Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Idiomas-40_Suportado-success.svg)](#)
  [![License](https://img.shields.io/badge/Licença-MIT-green.svg)](#)
</div>

---

## 🌟 Características

* **Bela UI/UX:** Uma interface de desktop moderna com design glassmorphism, modo escuro e guia de restauração passo a passo.
* **🌍 40 idiomas suportados:** Internacionalização de qualidade nativamente integrada. Mude de idioma instantaneamente.
* **Ultraleve e independente:** Reconstruído usando **Tauri & Rust**, reduzindo o tamanho do binário para apenas **~10MB** com zero dependências externas (não são necessários tempos de execução Python ou .NET no sistema host).
* **Armazenamento seguro de chaveiro:** Salva seus tokens Google OAuth diretamente no chaveiro seguro nativo do sistema operacional (Credential Manager no Windows, Keychain no macOS).
* **Descriptografia E2EE local (.crypt14 / .crypt15):** Descriptografia de banco de dados WhatsApp AES-256-GCM de alta velocidade, segura e totalmente local usando Rust.
* **Transferência direta para telefone (ADB):** Restaure seus bancos de dados descriptografados diretamente para o seu telefone Android com um único clique, chamando o sistema ADB nativamente.

---

## 📋 Requisitos

Para compilar e executar a partir do código-fonte, certifique-se de ter:
1. **Ferrugem e Carga** (v1.77 ou superior)
2. **Node.js & npm**
3. Um dispositivo Android com depuração USB ativada (se estiver restaurando diretamente via ADB).

---

## 🚀 Execução e Desenvolvimento

### Iniciando o aplicativo
Basta clicar duas vezes no [run.bat](../run.bat) arquivo na raiz do projeto.
Ele abrirá instantaneamente o executável de produção compilado (`app.exe`) ou retornará ao modo de desenvolvimento se não for compilado.

### Executando em modo de desenvolvimento
Para iniciar o modo de desenvolvimento com recarregamento a quente:
```bash
npx tauri dev
```

### Compilando e agrupando
Para gerar pacotes de instalação de produção autocontidos e assinados (configuração MSI, EXE):
```bash
npx tauri build
```
Os instaladores resultantes estarão localizados em `src-tauri/target/release/bundle/`.

Ver [DISTRIBUTION.md](../DISTRIBUTION.md) para mais detalhes sobre embalagem e assinatura.
