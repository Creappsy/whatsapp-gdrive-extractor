# Guía de Distribución y Firmado (Tauri & Rust)

Esta guía explica el proceso para empaquetar, compilar y firmar digitalmente la aplicación **WhatsApp Extractor** utilizando **Tauri (v2)**.

Al haber migrado a Tauri + Rust, los instaladores se generan de manera nativa sin necesidad de Python ni frameworks pesados en el cliente final.

---

## 🚀 Compilación de Producción (Release)

Para generar los instaladores de producción para tu sistema operativo actual, ejecuta:
```bash
npx tauri build
```

Este comando descargará automáticamente los compiladores requeridos y generará los paquetes finales de distribución en la ruta:
`src-tauri/target/release/bundle/`

---

## 💻 1. Windows (MSI y EXE)

Tauri genera dos tipos de instaladores de forma automática:
- **MSI:** Un instalador estándar de Windows.
- **EXE:** Un instalador autoejecutable ligero (NSIS).

Rutas de salida:
- `src-tauri/target/release/bundle/msi/WhatsApp Extractor_0.1.0_x64_en-US.msi`
- `src-tauri/target/release/bundle/nsis/WhatsApp Extractor_0.1.0_x64-setup.exe`

### Firmado Digital (Para evitar Windows SmartScreen)
Para que los usuarios no reciban la alerta de "Editor no verificado" o "SmartScreen protegió su PC":

1. **Requisitos:**
   - Descarga Windows SDK para obtener `signtool.exe`.
   - Obtén un certificado de firma de código en formato `.pfx`.

2. **Comando de Firmado:**
   ```powershell
   signtool sign /f "ruta\al\certificado.pfx" /p "TuContraseña" /tr http://timestamp.digicert.com /td sha256 /fd sha256 "src-tauri\target\release\bundle\nsis\WhatsApp Extractor_0.1.0_x64-setup.exe"
   ```

---

## 🍎 2. macOS (DMG y App)

Para empaquetar en macOS, debes ejecutar el comando en una Mac.

Ruta de salida:
- `src-tauri/target/release/bundle/dmg/WhatsApp Extractor_0.1.0_x64.dmg`

### Firmado y Notarización
1. **Configuración en `tauri.conf.json`:**
   Configura el perfil de firma en la sección de bundle de macOS con tu cuenta de Apple Developer.
2. **Compilar y Firmar:**
   ```bash
   npx tauri build
   ```
   Tauri firmará el archivo automáticamente si detecta tus certificados en tu Llavero (Keychain).

3. **Notarizar con Apple:**
   ```bash
   xcrun notarytool submit "src-tauri/target/release/bundle/dmg/WhatsApp Extractor_0.1.0_x64.dmg" --apple-id "tu-email@apple.com" --password "abcd-efgh-ijkl-mnop" --team-id "TU_TEAM_ID" --wait
   ```

4. **Grapado (Stapling):**
   ```bash
   xcrun stapler staple "src-tauri/target/release/bundle/dmg/WhatsApp Extractor_0.1.0_x64.dmg"
   ```

---

## 🤖 3. Android (APK / AAB)

Tauri v2 soporta compilar para Android usando Rust nativo y Kotlin.

1. **Inicializar soporte de Android:**
   ```bash
   npx tauri android init
   ```
2. **Compilar APK/AAB:**
   ```bash
   npx tauri android build
   ```
   Esto generará un APK optimizado en la carpeta `src-tauri/gen/android/`.
