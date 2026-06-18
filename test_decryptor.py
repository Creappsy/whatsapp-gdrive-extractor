import os
import binascii
from Cryptodome.Cipher import AES
from decryptor import WhatsAppDecryptor

def generate_mock_crypt14():
    # 1. Generar una llave falsa de 32 bytes (64 hex)
    key = os.urandom(32)
    key_hex = binascii.hexlify(key).decode('utf-8')
    
    # 2. Generar un IV falso de 16 bytes
    iv = os.urandom(16)
    
    # 3. Crear un payload falso (una "base de datos" simulada)
    original_payload = b"SQLite format 3\x00 MOCK WHATSAPP DATABASE CONTENT"
    
    # 4. Encriptar el payload con AES-GCM (igual que WhatsApp)
    cipher = AES.new(key, AES.MODE_GCM, nonce=iv)
    encrypted_payload, tag = cipher.encrypt_and_digest(original_payload)
    
    # 5. Construir un header falso de 191 bytes.
    # El decryptor espera que los últimos 16 bytes del header sean el IV.
    # Así que ponemos 175 bytes de basura + 16 bytes de IV = 191 bytes.
    header = os.urandom(191 - 16) + iv
    
    # 6. Escribir el archivo .crypt14 simulado
    crypt14_path = "mock_backup.crypt14"
    with open(crypt14_path, 'wb') as f:
        f.write(header + encrypted_payload)
        # Nota: WhatsApp original también escribe el tag de autenticación al final, pero
        # nuestro decryptor actual ignora el tag por simplicidad en este prototipo, 
        # así que no lo añadiremos para ver si el decryptor básico funciona sin crashear.
        
    return key_hex, crypt14_path, original_payload

def test_decryption():
    print("--- INICIANDO PRUEBA DEL MÓDULO DE DESENCRIPTACIÓN ---")
    try:
        key_hex, crypt14_path, original_payload = generate_mock_crypt14()
        print(f"[+] Archivo mock '{crypt14_path}' generado.")
        print(f"[+] Llave E2EE generada: {key_hex}")
        
        output_db = "decrypted_mock.db"
        
        print("[*] Iniciando WhatsAppDecryptor...")
        decryptor = WhatsAppDecryptor(key_hex=key_hex)
        success, msg = decryptor.decrypt_crypt14_15(crypt14_path, output_db)
        
        if success:
            print(f"[+] ¡Éxito! Mensaje: {msg}")
            
            # Verificar si el contenido coincide
            with open(output_db, 'rb') as f:
                decrypted_content = f.read()
                
            if decrypted_content == original_payload:
                print("[+] VERIFICACIÓN PASADA: El contenido desencriptado es exactamente igual al original.")
            else:
                print("[-] FALLO: El contenido no coincide.")
        else:
            print(f"[-] FALLO en la función: {msg}")
            
    except Exception as e:
        print(f"[-] ERROR FATAL: {str(e)}")
    finally:
        # Limpieza
        for f in ["mock_backup.crypt14", "decrypted_mock.db"]:
            if os.path.exists(f):
                os.remove(f)

if __name__ == "__main__":
    test_decryption()
