<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/6/6b/WhatsApp.svg" alt="WhatsApp Logo" width="100"/>
  <h1>WhatsApp Google Drive Extractor 🚀</h1>
  <p><b>Εγγενής εφαρμογή για υπολογιστές πολλαπλών πλατφορμών (Windows, macOS, Linux), γρήγορη, ελαφριά και πολυγλωσσική, για εξαγωγή, αποκρυπτογράφηση και επαναφορά των αντιγράφων ασφαλείας WhatsApp από το Google Drive.</b></p>

  <details open>
    <summary><b>🌍 Choose your language / Elige tu idioma</b></summary>
    <p align="center">
      <a href="../README.md">🇺🇸 English</a> • <a href="README_es.md">🇪🇸 Español</a> • <a href="README_zh.md">🇨🇳 中文</a> • <a href="README_hi.md">🇮🇳 हिन्दी</a> • <a href="README_fr.md">🇫🇷 Français</a> • <a href="README_ar.md">🇸🇦 العربية</a> • <a href="README_bn.md">🇧🇩 বাংলা</a> • <a href="README_ru.md">🇷🇺 Русский</a> • <a href="README_pt.md">🇵🇹 Português</a> • <a href="README_ur.md">🇵🇰 Urdu</a> • <a href="README_id.md">🇮🇩 Bahasa Indonesia</a> • <a href="README_de.md">🇩🇪 Deutsch</a> • <a href="README_ja.md">🇯🇵 日本語</a> • <a href="README_mr.md">🇮🇳 Marathi</a> • <a href="README_te.md">🇮🇳 Telugu</a> • <a href="README_tr.md">🇹🇷 Türkçe</a> • <a href="README_ta.md">🇮🇳 Tamil</a> • <a href="README_vi.md">🇻🇳 Tiếng Việt</a> • <a href="README_tl.md">🇵🇭 Tagalog</a> • <a href="README_ko.md">🇰🇷 한국어</a> • <a href="README_it.md">🇮🇹 Italiano</a> • <a href="README_pl.md">🇵🇱 Polski</a> • <a href="README_nl.md">🇳🇱 Nederlands</a> • <a href="README_th.md">🇹🇭 Thai</a> • <a href="README_fa.md">🇮🇷 Farsi</a> • <a href="README_gu.md">🇮🇳 Gujarati</a> • <a href="README_ro.md">🇷🇴 Română</a> • <a href="README_uk.md">🇺🇦 Ukrainian</a> • <a href="README_el.md">🇬🇷 Greek</a> • <a href="README_hu.md">🇭🇺 Hungarian</a> • <a href="README_cs.md">🇨🇿 Čeština</a> • <a href="README_sv.md">🇸🇪 Svenska</a> • <a href="README_da.md">🇩🇰 Dansk</a> • <a href="README_fi.md">🇫🇮 Suomi</a> • <a href="README_no.md">🇳🇴 Norsk</a> • <a href="README_sk.md">🇸🇰 Slovenčina</a> • <a href="README_bg.md">🇧🇬 Bulgarian</a> • <a href="README_hr.md">🇭🇷 Hrvatski</a> • <a href="README_sr.md">🇷🇸 Serbian</a>
    </p>
  </details>

  [![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
  [![i18n](https://img.shields.io/badge/Γλώσσες-40_Υποστηρίζεται-success.svg)](#)
  [![License](https://img.shields.io/badge/Αδεια-MIT-green.svg)](#)
</div>

---

## 🌟 Χαρακτηριστικά

* **Όμορφο UI/UX:** Μια σύγχρονη διεπαφή επιφάνειας εργασίας με σχεδιασμό γυαλομορφισμού, σκοτεινή λειτουργία και βήμα προς βήμα οδηγό αποκατάστασης.
* **🌍 40 υποστηριζόμενες γλώσσες:** Η διεθνοποίηση ποιότητας είναι εγγενώς ενσωματωμένη. Άμεση αλλαγή γλώσσας.
* **Υπερελαφρύ & Ανεξάρτητο:** Ανακατασκευάστηκε χρησιμοποιώντας **Tauri & Rust**, μειώνοντας το δυαδικό μέγεθος σε μόλις **~10MB** με μηδενικές εξωτερικές εξαρτήσεις (δεν απαιτούνται χρόνοι εκτέλεσης Python ή .NET στο κεντρικό σύστημα).
* **Ασφαλής χώρος αποθήκευσης μπρελόκ:** Αποθηκεύει τα διακριτικά σας Google OAuth απευθείας στο εγγενές ασφαλές κλειδί του λειτουργικού συστήματος (Credential Manager στα Windows, Keychain στο macOS).
* **Τοπική αποκρυπτογράφηση E2EE (.crypt14 / .crypt15):** Υψηλής ταχύτητας, ασφαλής και εξ ολοκλήρου τοπική αποκρυπτογράφηση βάσης δεδομένων WhatsApp AES-256-GCM με χρήση Rust.
* **Απευθείας μεταφορά στο τηλέφωνο (ADB):** Επαναφέρετε τις αποκρυπτογραφημένες βάσεις δεδομένων σας απευθείας στο τηλέφωνό σας Android με ένα μόνο κλικ, καλώντας το σύστημα ADB εγγενώς.

---

## 📋 Απαιτήσεις

Για να δημιουργήσετε και να εκτελέσετε από την πηγή, βεβαιωθείτε ότι έχετε:
1. **Rust & Cargo** (v1.77 ή ανώτερος)
2. **Node.js & npm**
3. Μια συσκευή Android με ενεργοποιημένη τη λειτουργία εντοπισμού σφαλμάτων USB (αν γίνεται απευθείας επαναφορά μέσω ADB).

---

## 🚀 Εκτέλεση και Ανάπτυξη

### Εκκίνηση της Εφαρμογής
Απλώς κάντε διπλό κλικ στο [run.bat](../run.bat) αρχείο στη ρίζα του έργου.
Θα ανοίξει αμέσως το μεταγλωττισμένο εκτελέσιμο αρχείο παραγωγής (`app.exe`) ή θα επιστρέψει στη λειτουργία ανάπτυξης εάν δεν έχει μεταγλωττιστεί.

### Λειτουργία σε λειτουργία ανάπτυξης
Για να ξεκινήσετε τη λειτουργία ανάπτυξης με επανάληψη φόρτωσης:
```bash
npx tauri dev
```

### Σύνταξη και ομαδοποίηση
Για να δημιουργήσετε αυτόνομα, υπογεγραμμένα πακέτα εγκατάστασης παραγωγής (Ρύθμιση MSI, EXE):
```bash
npx tauri build
```
Τα προγράμματα εγκατάστασης που θα προκύψουν θα βρίσκονται στο `src-tauri/target/release/bundle/`.

Βλέπω [DISTRIBUTION.md](../DISTRIBUTION.md) για περισσότερες λεπτομέρειες σχετικά με τη συσκευασία και την υπογραφή.
