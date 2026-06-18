# WhatsApp Google Drive Extractor
Allows WhatsApp users on Android to extract their backed-up WhatsApp data from Google Drive.

## Prerequisites
* Docker
* Android device with WhatsApp installed and the Google Drive backup feature enabled.
* The device's Android ID (if you want to reduce the risk of being logged out of Google). Run `adb shell settings get secure android_id` or search Google Play for "device ID" apps.
* Google account login credentials (username and password). If using 2-factor authentication, create and use an App password: https://myaccount.google.com/apppasswords

## Instructions
Clone the repository:

```bash
git clone https://github.com/daferferso/whatsapp-gdrive-extractor.git
```

Add your Gmail, password, and Android ID to the `settings.json` file before running the container.

Build the Docker image:

```bash
cd whatsapp-gdrive-extractor/
docker build . -t whatsapp-gdrive-extractor
```

Run the Docker container:

**Linux:**
```bash
cd whatsapp-gdrive-extractor/
docker run -v $(pwd):/app -it whatsapp-gdrive-extractor
```

**Windows:**
```powershell
cd .\whatsapp-gdrive-extractor\
docker run -v .:/app -it whatsapp-gdrive-extractor
```

If downloading is interrupted, the files that were received successfully won't be re-downloaded when running the tool one more time. After downloading, you may verify the integrity of the downloaded files using `md5sum --check md5sum.txt` on Linux or `md5summer` on Windows.

## Alternative authentication method
Instead of email and password, you can use an OAuth token to auth with Google. To obtain the value of the token, follow the instructions here. The token value should be in the format `oauth2_4/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`. Put this into the `settings.json` file, and re-build and re-run the docker container. Note that this token expires quickly, and can be used only once. If this happend, you can just obtain a fresh token, replace the value in `settings.json` and re-build the docker container.

## Troubleshooting
* If you have `Error:Need Browser`, go to this url to solve the issue: https://accounts.google.com/b/0/DisplayUnlockCaptcha (Link no longer works 2025-12-11)
* If you experience issues authenticating (`BadAuthentication` errors) with Google using you email and password, you can try using the oauth token cookie instead of the password. See Alternative authentication method.

## Agradecimientos / Credits
* **Author:** TripCode
* **Contributors:** DrDeath1122 from XDA for the multi-threading backbone part, YuriCosta for reverse engineering the new restore system, and macagua for the solution to the SSL problem. 
* **Special thanks:** to YuriCosta, as I forked and improved his repository, adapting it to work with Docker along with macagua's solution.
* **Modernización y UI:** Gracias a las recientes contribuciones para añadir una interfaz gráfica (UI), desencriptación local y la compilación para Android, expandiendo las capacidades originales del proyecto.