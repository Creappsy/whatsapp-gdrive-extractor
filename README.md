# WhatsApp Google Drive Extractor 🟢

![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Python](https://img.shields.io/badge/Python-3.8%2B-green.svg)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Mac%20%7C%20Linux-purple.svg)

Una herramienta de código abierto y nativa de escritorio para extraer, descargar y **desencriptar** de forma segura tus copias de seguridad de WhatsApp desde la nube oculta de Google Drive hacia tu computadora.

> **Nuestra Visión:** Ser la alternativa libre, gratuita y transparente a herramientas comerciales cerradas como Dr.Fone. Diseñada con ingeniería de caja limpia, sin robar datos ni violar tu privacidad.

> **Nota Importante:** Este proyecto es un *Fork* modernizado de [daferferso/whatsapp-gdrive-extractor](https://github.com/daferferso/whatsapp-gdrive-extractor) (y del trabajo fundacional original de la comunidad como `B16f00t/whatsapp-gdrive-extractor`). Hemos tomado el robusto motor original de consola y lo hemos transformado en una Aplicación de Escritorio completa con Interfaz Gráfica (UI), descargas en tiempo real, auto-extracción de tokens y módulo de desencriptación local. Todo el crédito de la ingeniería inversa inicial a los servidores de Google pertenece a los autores originales.

![Login UI](assets/login_ui.png)
*Vista de Login con Extracción Automática.*

## ✨ Características Principales

- **Auto-Extracción Nativa:** Construida con `pywebview` para escritorio y `Toga` (WebView nativo) para Android. Abre una ventana oficial de Google, inicias sesión normalmente y la aplicación extrae la cookie `oauth_token` para descargar tus backups de forma manual e independiente.
- **Motor de Desencriptación Local:** ¿Descargaste un archivo `.crypt14` o `.crypt15`? Usa nuestro módulo integrado basado en AES-256 para convertirlo en una base de datos SQLite (.db) legible usando tu contraseña E2EE de 64 dígitos.
- **Directo al Celular (ADB USB):** Transfiere de manera directa las copias de seguridad descargadas al almacenamiento interno de tu dispositivo Android conectado por USB, en la ruta correspondiente de WhatsApp.
- **Acceso Rápido:** Abre la carpeta local de la copia de seguridad descargada en el Explorador de Archivos con un solo clic.
- **Interfaz Web Moderna y Responsiva:** Diseño premium de tipo Glassmorphism, con soporte para Modo Oscuro y adaptabilidad para pantallas de escritorio y teléfonos móviles.
- **Descargas en Tiempo Real:** Barra de progreso visual conectada al motor de descarga mediante SSE.
- **Seguro y Privado:** Tus credenciales nunca se guardan en el disco duro. Se mantienen en memoria y se borran al cerrar la app.

![Dashboard UI](assets/dashboard_ui.png)
*Panel de control interactivo con descargas, módulo de desencriptación y transferencias.*

---

## 🚀 Instalación y Uso (Escritorio - Windows)

Dado que la aplicación necesita ejecutarse localmente, puedes correrla en tu computadora de escritorio:

1. Haz clic derecho sobre el archivo `local_deploy.ps1` y selecciona **Ejecutar con PowerShell**.
2. El script creará un entorno virtual de Python, instalará las dependencias y abrirá la aplicación en una ventana de escritorio.

## 📚 ¿De dónde obtengo mis datos? (Guía Paso a Paso)

Para que la herramienta funcione y proteja tu privacidad, necesitas obtener dos datos de tu teléfono móvil.

### 1. ¿Cómo obtener el Android ID?
El Android ID es un código de 16 caracteres físicos de tu teléfono. Google lo pide para verificar que eres tú. Tienes 3 formas de obtenerlo:
- **Método 1 (El más fácil):** Descarga la aplicación gratuita **"Device ID"** (por Evozi o similares) desde la Google Play Store. Ábrela y copia el código que dice "Android Service Framework (GSF)".
- **Método 2 (Código USSD):** Abre la aplicación de llamadas en tu Android y marca `*#*#8255#*#*`. Busca en la pantalla de GTalk Service Monitor la línea que dice `Device ID: android-xxxxxxxxxxxxxxxx`.
- **Método 3 (Avanzado - ADB):** Conecta tu teléfono por USB con depuración activada y ejecuta en tu terminal: `adb shell settings get secure android_id`.

### 2. ¿Cómo obtener la Contraseña de 64 dígitos (E2EE)?
Esta contraseña es necesaria si quieres **desencriptar** tus archivos `.crypt14` o `.crypt15` a bases de datos legibles.
1. Abre WhatsApp en tu teléfono.
2. Ve a **Ajustes > Chats > Copia de seguridad**.
3. Toca en **Copia de seguridad cifrada de extremo a extremo**.
4. Dale a **Activar** y selecciona **"Usar clave de cifrado de 64 dígitos en su lugar"**.
5. WhatsApp generará una clave muy larga. **¡Cópiala, guárdala en un lugar seguro y úsala en nuestra aplicación para desencriptar!** (Si ya la tenías activada, puedes ir a la misma sección para ver o cambiar la clave).

## 📚 El Contexto Técnico (Cómo funciona la magia)

Google Drive almacena las copias de seguridad de WhatsApp en una partición oculta (App Data). No puedes simplemente entrar a `drive.google.com` y descargarlas.

1. **Ingeniería Inversa (gpsoauth):** Nuestra herramienta utiliza la librería `gpsoauth` para comunicarse con la API de Google imitando a un dispositivo Android real. Le hacemos creer a los servidores de Google que somos la aplicación oficial de WhatsApp solicitando acceso a sus propios datos.
2. **El Android ID:** Por motivos de seguridad, Google verifica la identidad del dispositivo que solicita el backup. Es por eso que necesitas proveer tu `Android ID` físico de 16 caracteres. Sin esto, Google bloquea la solicitud asumiendo que es un intento de hackeo.
3. **Criptografía (AES-256):** Los archivos descargados están fuertemente cifrados. WhatsApp utiliza el estándar militar AES-256-GCM. Si el usuario activó el cifrado E2EE (Extremo a Extremo), le proporcionamos un motor en Python (`pycryptodome`) para descifrarlo matemáticamente con la llave de 64 dígitos.

## 🖥️ Modo Consola Original (Legacy CLI / Docker)

Si prefieres usar la herramienta original de línea de comandos en lugar de la aplicación de escritorio nativa:

1. Modifica el archivo `settings.json` con tus datos.
2. **Contraseña de Google:** Si usas verificación en 2 pasos (2FA), debes crear una Contraseña de Aplicación en: [https://myaccount.google.com/apppasswords](https://myaccount.google.com/apppasswords).
3. Corre la imagen de Docker:
   ```powershell
   docker build . -t whatsapp-gdrive-extractor
   docker run -v .:/app -it whatsapp-gdrive-extractor
   ```

## 📱 Compilación para Android Nativo (APK)

La aplicación es completamente compatible con dispositivos Android utilizando el framework de empaquetado **BeeWare / Briefcase**.

### Requisitos previos:
- Python 3.10 o superior.
- Java JDK 17 (se descarga automáticamente si no se encuentra).
- Android SDK (se configura automáticamente en el primer build).

### Instrucciones para compilar:
1. Activa tu entorno virtual e instala `briefcase`:
   ```powershell
   .\venv\Scripts\python.exe -m pip install briefcase
   ```
2. Inicializa el proyecto Android:
   ```powershell
   .\venv\Scripts\briefcase.exe create android --no-input
   ```
3. Compila el APK ejecutable:
   ```powershell
   .\venv\Scripts\briefcase.exe build android --no-input
   ```
4. El archivo APK resultante se ubicará en la siguiente ruta de compilación lista para instalarse en tu celular:
   `build\whatsapp_extractor\android\gradle\app\build\outputs\apk\debug\app-debug.apk`

## 🤝 Contribuir
¡Las contribuciones son bienvenidas! Siéntete libre de abrir un *Issue* o enviar un *Pull Request* con mejoras.

## 📄 Licencia
Este proyecto está bajo la Licencia MIT. Consulta el archivo `LICENSE` para más detalles.