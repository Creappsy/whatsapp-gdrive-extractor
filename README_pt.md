<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Ferramenta moderna, rápida e multilíngue para extrair, descriptografar e restaurar seus backups do WhatsApp do Google Drive.</b></p>
</div>

---

## 🌟 Recursos

* **Bela Interface UI/UX:** Uma interface nativa moderna em modo escuro com elementos *glassmorphism*.
* **🌍 40 Idiomas Suportados:** Traduções de qualidade 100% humana integradas nativamente.
* **Descriptografia Local (.crypt14 / .crypt15):** Um módulo de criptografia totalmente local para descriptografar seus bancos de dados SQLite usando seu hash E2EE.
* **Transferência Direta para Android (ADB):** Envie seu banco de dados diretamente para o armazenamento do seu telefone Android com um clique.
* **Autenticação Segura:** Suporta tokens OAuth do Google e Senhas de Aplicativo.

## 📋 Pré-requisitos

1. **Python 3.x** ou **Docker**.
2. Um dispositivo Android com backups do Google Drive ativados.
3. Credenciais da conta Google.

## 🚀 Instalação e Uso

### Opção 1: Python (Recomendado)
```bash
git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
cd whatsapp-gdrive-extractor
pip install -r requirements.txt
python server.py
```
Abra `http://localhost:5000` no seu navegador!

## 🔑 Guia de Autenticação
Se tiver problemas com a senha, use o método do **Token OAuth** copiando o valor de `oauth_token` da guia de cookies do seu navegador após fazer login no Google.
