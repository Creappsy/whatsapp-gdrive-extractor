import os
from base64 import b64decode
from multiprocessing.pool import ThreadPool
import gpsoauth
import hashlib
import json
import requests

def human_size(size):
    for s in ["B", "kiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"]:
        if abs(size) < 1024:
            break
        size = int(size / 1024)
    return f"{size}{s}"

def have_file(file, size, md5):
    if not os.path.exists(file) or size != os.path.getsize(file):
        return False

    digest = hashlib.md5()
    with open(file, "br") as input:
        while True:
            b = input.read(8 * 1024)
            if not b:
                break
            digest.update(b)

    return md5 == digest.digest()

def download_file(file, stream):
    os.makedirs(os.path.dirname(file), exist_ok=True)
    with open(file, "bw") as dest:
        for chunk in stream.iter_content(chunk_size=None):
            dest.write(chunk)

class WaBackup:
    def __init__(self, android_id, gmail, password=None, oauth_token=None):
        if oauth_token and oauth_token.strip():
            token = gpsoauth.exchange_token(gmail.strip().lower(), oauth_token.strip(), android_id)
        elif password:
            # Eliminar espacios (Google muestra las contraseñas de aplicación con espacios, ej: 'abcd efgh ijkl mnop')
            sanitized_password = password.strip().replace(" ", "")
            token = gpsoauth.perform_master_login(gmail.strip().lower(), sanitized_password, android_id)
        else:
            raise ValueError("No password or oauth_token provided")

        if "Token" not in token:
            raise Exception(f"Authentication failed: {token}")
            
        self.auth = gpsoauth.perform_oauth(
            gmail,
            token["Token"],
            android_id,
            "oauth2:https://www.googleapis.com/auth/drive.appdata",
            "com.whatsapp",
            "38a0f7d505fe18fec64fbf343ecaaaf310dbd799",
        )

    def get(self, path, params=None, **kwargs):
        try:
            response = requests.get(
                f"https://backup.googleapis.com/v1/{path}",
                headers={"Authorization": f"Bearer {self.auth['Auth']}"},
                params=params,
                **kwargs,
            )
            response.raise_for_status()
        except requests.exceptions.RequestException as err:
            print ("\nError API:",err)
        return response

    def get_page(self, path, page_token=None):
        return self.get(
            path,
            None if page_token is None else {"pageToken": page_token},
        ).json()

    def list_path(self, path):
        last_component = path.split("/")[-1]
        page_token = None
        while True:
            page = self.get_page(path, page_token)
            if last_component in page:
                for item in page[last_component]:
                    yield item
            if "nextPageToken" not in page:
                break
            page_token = page["nextPageToken"]

    def backups(self):
        return self.list_path("clients/wa/backups")

    def backup_files(self, backup):
        return self.list_path(f"{backup['name']}/files")

    def fetch(self, file):
        name = os.path.sep.join(file["name"].split("/")[3:])
        md5Hash = b64decode(file["md5Hash"], validate=True)
        size = int(file.get("sizeBytes", 0))
        if not have_file(name, size, md5Hash):
            download_file(
                name,
                self.get(file["name"].replace("%", "%25").replace("+", "%2B"), {"alt": "media"}, stream=True)
            )
        return name, size, md5Hash

    def fetch_all(self, backup, cksums, progress_callback=None):
        files = list(self.backup_files(backup))
        num_files = 0
        total_size = 0

        with ThreadPool(10) as pool:
            downloads = pool.imap_unordered(
                lambda file: self.fetch(file),
                files
            )
            for name, size, md5Hash in downloads:
                num_files += 1
                total_size += size
                if progress_callback:
                    progress_callback(num_files, len(files), size)
                cksums.write(f"{md5Hash.hex()} *{name}\n")
        return num_files, total_size

def get_backup_info(backup):
    metadata_str = backup.get("metadata", "{}")
    metadata = json.loads(metadata_str)
    
    return {
        "id": backup['name'].split('/')[-1],
        "backupSize": metadata.get("backupSize", "0"),
        "backupSizeHuman": human_size(int(metadata.get("backupSize", 0))),
        "uploadTime": backup.get('updateTime', 'Unknown'),
        "whatsappVersion": metadata.get("versionOfAppWhenBackup", "Unknown"),
        "messages": metadata.get("numOfMessages", "0"),
        "mediaFiles": metadata.get("numOfMediaFiles", "0"),
        "photos": metadata.get("numOfPhotos", "0")
    }
