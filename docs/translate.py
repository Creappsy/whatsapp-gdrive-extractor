import os
import re
import sys
from pathlib import Path

# Try importing deep-translator, or instruct user/install it
try:
    from deep_translator import GoogleTranslator
except ImportError:
    print("deep-translator not found. Please install it with: pip install deep-translator")
    sys.exit(1)

# List of language files and their code in deep-translator
LANG_MAPPING = {
    'ar': 'ar',
    'bg': 'bg',
    'bn': 'bn',
    'cs': 'cs',
    'da': 'da',
    'de': 'de',
    'el': 'el',
    'fa': 'fa',
    'fi': 'fi',
    'fr': 'fr',
    'gu': 'gu',
    'hi': 'hi',
    'hr': 'hr',
    'hu': 'hu',
    'id': 'id',
    'it': 'it',
    'ja': 'ja',
    'ko': 'ko',
    'mr': 'mr',
    'nl': 'nl',
    'no': 'no',
    'pl': 'pl',
    'pt': 'pt',
    'ro': 'ro',
    'ru': 'ru',
    'sk': 'sk',
    'sr': 'sr',
    'sv': 'sv',
    'ta': 'ta',
    'te': 'te',
    'th': 'th',
    'tl': 'tl',
    'tr': 'tr',
    'uk': 'uk',
    'ur': 'ur',
    'vi': 'vi',
    'zh': 'zh-CN'
}

ROOT_DIR = Path(__file__).parent.parent
DOCS_DIR = ROOT_DIR / "docs"
ENGLISH_README = ROOT_DIR / "README.md"

def translate_text(text, target_lang):
    if not text.strip():
        return text
    
    # We want to preserve markdown syntax like **bold**, `code`, [link](url)
    # A simple way with GoogleTranslator is to translate the text.
    # GoogleTranslator handles standard punctuation and formatting reasonably well,
    # but we can try to translate line by line or block by block.
    try:
        translator = GoogleTranslator(source='en', target=target_lang)
        return translator.translate(text)
    except Exception as e:
        print(f"Error translating to {target_lang}: {e}")
        return text

def translate_markdown_content(english_content, target_lang):
    # We will reconstruct the file block by block to preserve code blocks and links
    lines = english_content.split('\n')
    translated_lines = []
    in_code_block = False
    
    for line in lines:
        stripped = line.strip()
        
        # Don't translate code blocks
        if stripped.startswith('```'):
            in_code_block = not in_code_block
            translated_lines.append(line)
            continue
            
        if in_code_block:
            translated_lines.append(line)
            continue
            
        # Don't translate HTML center/badges/images/links block at the beginning
        # Line 1 to 17 in English README has center and links:
        # We will handle the header manually.
        
        # If it's a heading
        if stripped.startswith('#'):
            # Translate only the text part of the heading
            m = re.match(r'^(#+\s*)(.*)$', line)
            if m:
                prefix, content = m.groups()
                translated_content = translate_text(content, target_lang)
                translated_lines.append(f"{prefix}{translated_content}")
            else:
                translated_lines.append(line)
            continue
            
        # If it's a list item
        if stripped.startswith('* ') or stripped.startswith('- ') or re.match(r'^\d+\.\s', stripped):
            # We want to translate but keep the list symbol
            m = re.match(r'^(\s*[\*\-\d\.]+\s*)(.*)$', line)
            if m:
                prefix, content = m.groups()
                # If content contains markdown formatting like **Title:** Text
                # Let's see if we can parse out the title in **...**
                bold_m = re.match(r'^(\*\*.*?\*\*\s*)(.*)$', content)
                if bold_m:
                    bold_prefix, rest = bold_m.groups()
                    # Translate bold title and rest separately or together
                    # Often translating together is better for grammar, but we must protect **
                    # Let's translate them together but clean up any spaces Google Translator adds around **
                    translated_content = translate_text(content, target_lang)
                    # Clean up: Google Translator sometimes translates "**" or adds spaces like "* * text * *"
                    # We will try to preserve the prefix structure:
                    # Translate bold title:
                    bold_text = bold_prefix.replace('*', '').strip()
                    translated_bold = translate_text(bold_text, target_lang)
                    translated_rest = translate_text(rest, target_lang)
                    translated_lines.append(f"{prefix}**{translated_bold}** {translated_rest}")
                else:
                    translated_content = translate_text(content, target_lang)
                    translated_lines.append(f"{prefix}{translated_content}")
            else:
                translated_lines.append(line)
            continue
            
        # Normal line
        if stripped:
            # Check if it's a link or reference
            if stripped.startswith('[') and ']:' in stripped:
                translated_lines.append(line)
            else:
                translated_lines.append(translate_text(line, target_lang))
        else:
            translated_lines.append(line)
            
    return '\n'.join(translated_lines)

def build_translated_readme(target_file, target_lang, lang_suffix):
    print(f"Generating {target_file.name} (Language: {target_lang})...")
    
    # 1. Base translation of description (line 4 of English README)
    english_desc = "Native cross-platform desktop application (Windows, macOS, Linux), fast, lightweight, and multi-language, to extract, decrypt, and restore your WhatsApp backups from Google Drive."
    translated_desc = translate_text(english_desc, target_lang)
    
    # 2. Translate Features section content
    features_en = """* **Beautiful UI/UX:** A modern desktop interface with glassmorphism design, dark mode, and step-by-step restoration guide.
* **🌍 40 Supported Languages:** Quality internationalization natively integrated. Switch languages instantly.
* **Ultralight & Independent:** Rebuilt using **Tauri & Rust**, reducing binary size to just **~10MB** with zero external dependencies (no Python runtimes or .NET required on the host system).
* **Secure Keyring Storage:** Saves your Google OAuth tokens directly to the operating system's native secure keyring (Credential Manager in Windows, Keychain in macOS).
* **Local E2EE Decryption (.crypt14 / .crypt15):** High-speed, secure, and entirely local AES-256-GCM WhatsApp database decryption using Rust.
* **Direct Transfer to Phone (ADB):** Restore your decrypted databases directly to your Android phone with a single click, calling system ADB natively."""
    
    features_tr = translate_markdown_content(features_en, target_lang)
    
    # 3. Translate Requirements section content
    reqs_intro_en = "To build and run from source, make sure you have:"
    reqs_intro_tr = translate_text(reqs_intro_en, target_lang)
    
    req_1_en = "**Rust & Cargo** (v1.77 or superior)"
    req_1_tr = f"**{translate_text('Rust & Cargo', target_lang)}** (v1.77 {translate_text('or superior', target_lang)})"
    
    req_2_en = "**Node.js & npm**"
    req_2_tr = f"**Node.js & npm**"
    
    req_3_en = "An Android device with USB Debugging enabled (if restoring directly via ADB)."
    req_3_tr = translate_text(req_3_en, target_lang)
    
    # 4. Translate Execution & Development section content
    exec_title_en = "Execution and Development"
    exec_title_tr = translate_text(exec_title_en, target_lang)
    
    launch_title_en = "Launching the Application"
    launch_title_tr = translate_text(launch_title_en, target_lang)
    
    launch_desc_en = "Simply double-click the [run.bat](../run.bat) file at the root of the project."
    # We will translate this but preserve the link to run.bat
    launch_desc_tr = translate_text("Simply double-click the", target_lang) + " [run.bat](../run.bat) " + translate_text("file at the root of the project.", target_lang)
    
    launch_fallback_en = "It will instantly open the compiled production executable (`app.exe`) or fallback to development mode if not compiled."
    launch_fallback_tr = translate_text(launch_fallback_en, target_lang)
    
    dev_title_en = "Running in Development Mode"
    dev_title_tr = translate_text(dev_title_en, target_lang)
    
    dev_desc_en = "To start development mode with hot-reloading:"
    dev_desc_tr = translate_text(dev_desc_en, target_lang)
    
    build_title_en = "Compiling and Bundling"
    build_title_tr = translate_text(build_title_en, target_lang)
    
    build_desc_en = "To generate self-contained, signed production installer packages (MSI, EXE setup):"
    build_desc_tr = translate_text(build_desc_en, target_lang)
    
    build_loc_en = "The resulting installers will be located in `src-tauri/target/release/bundle/`."
    build_loc_tr = translate_text(build_loc_en, target_lang)
    
    dist_desc_en = "See [DISTRIBUTION.md](../DISTRIBUTION.md) for more details on packaging and signing."
    dist_desc_tr = translate_text("See", target_lang) + " [DISTRIBUTION.md](../DISTRIBUTION.md) " + translate_text("for more details on packaging and signing.", target_lang)
    
    # Titles
    features_title_tr = translate_text("Features", target_lang)
    reqs_title_tr = translate_text("Requirements", target_lang)
    
    # Badges
    languages_badge_tr = translate_text("Languages", target_lang)
    supported_badge_tr = translate_text("Supported", target_lang)
    license_badge_tr = translate_text("License", target_lang)
    
    # Language Selector Link Correction:
    # Inside docs/README_*.md, the links should be:
    # <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> ...
    # We can read the line from the English README and modify it.
    with open(ENGLISH_README, "r", encoding="utf-8") as f:
        eng_lines = f.readlines()
        
    lang_selector_line = ""
    for line in eng_lines:
        if '<a href="docs/README_es.md">' in line or '<a href="README.md">🇺🇸 English</a>' in line:
            # Replace paths to fit relative layout from inside docs/
            line = line.replace('href="README.md"', 'href="../README.md"')
            line = line.replace('href="docs/', 'href="')
            lang_selector_line = line.strip()
            break
            
    # Assemble the README content
    content = f"""<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>{translated_desc}</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      {lang_selector_line}
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/{languages_badge_tr.replace(" ", "_")}-{40}_{supported_badge_tr.replace(" ", "_")}-success.svg)](#)
  [![License](https://img.shields.io/badge/{license_badge_tr.replace(" ", "_")}-MIT-green.svg)](#)
</div>

---

## 🌟 {features_title_tr}

{features_tr}

---

## 📋 {reqs_title_tr}

{reqs_intro_tr}
1. {req_1_tr}
2. {req_2_tr}
3. {req_3_tr}

---

## 🚀 {exec_title_tr}

### {launch_title_tr}
{launch_desc_tr}
{launch_fallback_tr}

### {dev_title_tr}
{dev_desc_tr}
```bash
npx tauri dev
```

### {build_title_tr}
{build_desc_tr}
```bash
npx tauri build
```
{build_loc_tr}

{dist_desc_tr}
"""
    
    with open(target_file, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"Finished {target_file.name}\n")

def main():
    for filename in os.listdir(DOCS_DIR):
        if filename.startswith("README_") and filename.endswith(".md"):
            if filename == "README_es.md":
                print("Skipping Spanish (README_es.md) as it is already updated.")
                continue
                
            target_file = DOCS_DIR / filename
            if target_file.exists():
                try:
                    with open(target_file, "r", encoding="utf-8") as f:
                        file_content = f.read().lower()
                    if "tauri" in file_content:
                        print(f"Skipping {filename} because it is already updated (contains 'tauri').")
                        continue
                except Exception as e:
                    print(f"Failed to check {filename}: {e}")

            lang_suffix = filename.replace("README_", "").replace(".md", "")
            target_lang = LANG_MAPPING.get(lang_suffix)
            if not target_lang:
                print(f"No mapping found for {filename}, skipping.")
                continue
                
            try:
                build_translated_readme(target_file, target_lang, lang_suffix)
            except Exception as e:
                print(f"Failed to translate {filename}: {e}")

if __name__ == "__main__":
    main()
