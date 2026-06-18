import os
import uuid
import threading
import queue
import json
import time
from flask import Flask, render_template, request, jsonify, session, Response

from extractor import WaBackup, get_backup_info

app = Flask(__name__)
app.secret_key = os.urandom(24)

# In-memory storage for active sessions to avoid saving credentials to disk
active_sessions = {}
# Queues for SSE progress updates
progress_queues = {}

@app.route('/')
def index():
    if 'session_id' not in session or session['session_id'] not in active_sessions:
        return render_template('login.html')
    return render_template('dashboard.html')

@app.route('/api/login', methods=['POST'])
def login():
    data = request.json
    gmail = data.get('gmail')
    password = data.get('password')
    android_id = data.get('android_id')
    oauth_token = data.get('oauth_token')

    if not android_id or not android_id.strip():
        import random
        android_id = ''.join(random.choices('0123456789abcdef', k=16))

    try:
        wa_backup = WaBackup(android_id, gmail, password=password, oauth_token=oauth_token)
        
        session_id = str(uuid.uuid4())
        session['session_id'] = session_id
        active_sessions[session_id] = {
            'wa_backup': wa_backup,
            'backups_cache': []
        }
        return jsonify({"success": True})
    except Exception as e:
        return jsonify({"success": False, "error": str(e)}), 401



@app.route('/api/logout', methods=['POST'])
def logout():
    session_id = session.get('session_id')
    if session_id in active_sessions:
        del active_sessions[session_id]
    session.pop('session_id', None)
    return jsonify({"success": True})

@app.route('/api/backups', methods=['GET'])
def get_backups():
    session_id = session.get('session_id')
    if not session_id or session_id not in active_sessions:
        return jsonify({"error": "No autenticado"}), 401

    wa_backup = active_sessions[session_id]['wa_backup']
    try:
        raw_backups = list(wa_backup.backups())
        active_sessions[session_id]['backups_cache'] = raw_backups
        
        backups_info = [get_backup_info(b) for b in raw_backups]
        return jsonify({"backups": backups_info})
    except Exception as e:
        return jsonify({"error": str(e)}), 500

@app.route('/api/sync', methods=['POST'])
def sync():
    session_id = session.get('session_id')
    if not session_id or session_id not in active_sessions:
        return jsonify({"error": "No autenticado"}), 401

    data = request.json
    backup_id = data.get('backup_id')
    
    wa_backup = active_sessions[session_id]['wa_backup']
    backups = active_sessions[session_id]['backups_cache']
    
    selected_backup = next((b for b in backups if b['name'].split('/')[-1] == backup_id), None)
    if not selected_backup:
        return jsonify({"error": "Backup no encontrado"}), 404

    # Setup progress queue
    q = queue.Queue()
    progress_queues[session_id] = q

    def progress_callback(num_files, total_files, size):
        q.put({
            "num_files": num_files,
            "total_files": total_files,
            "size": size
        })

    def background_download():
        try:
            with open("md5sum.txt", "w", encoding="utf-8", buffering=1) as cksums:
                wa_backup.fetch_all(selected_backup, cksums, progress_callback)
            q.put({"done": True})
        except Exception as e:
            q.put({"error": str(e)})

    threading.Thread(target=background_download).start()
    return jsonify({"success": True})

@app.route('/api/progress')
def progress():
    session_id = session.get('session_id')
    if not session_id or session_id not in progress_queues:
        return jsonify({"error": "No progress stream"}), 404

    q = progress_queues[session_id]

    def generate():
        while True:
            try:
                msg = q.get(timeout=10)
                if 'done' in msg:
                    yield f"data: {json.dumps({'done': True})}\n\n"
                    break
                if 'error' in msg:
                    yield f"data: {json.dumps({'error': msg['error']})}\n\n"
                    break
                
                yield f"data: {json.dumps(msg)}\n\n"
            except queue.Empty:
                # Keep-alive
                yield f"data: {json.dumps({'keep_alive': True})}\n\n"

    return Response(generate(), mimetype='text/event-stream')


@app.route('/api/decrypt', methods=['POST'])
def decrypt_file():
    from decryptor import WhatsAppDecryptor
    import tempfile
    
    if 'file' not in request.files:
        return jsonify({'success': False, 'error': 'No se envió ningún archivo'})
        
    file = request.files['file']
    key_hex = request.form.get('key_hex', '')
    
    if not key_hex or len(key_hex) != 64:
        return jsonify({'success': False, 'error': 'Debes proporcionar una contraseña E2EE de 64 caracteres hexadecimales.'})
        
    if file.filename == '':
        return jsonify({'success': False, 'error': 'Archivo no seleccionado'})
        
    try:
        # Guardar archivo temporalmente
        temp_input = tempfile.NamedTemporaryFile(delete=False)
        file.save(temp_input.name)
        
        output_db = temp_input.name + ".db"
        
        decryptor = WhatsAppDecryptor(key_hex=key_hex)
        success, msg = decryptor.decrypt_crypt14_15(temp_input.name, output_db)
        
        if success:
            return send_file(output_db, as_attachment=True, download_name=f"decrypted_{file.filename}.db")
        else:
            return jsonify({'success': False, 'error': msg}), 400
            
    except Exception as e:
        return jsonify({'success': False, 'error': str(e)}), 500
    finally:
        # Clean up input temp file
        if os.path.exists(temp_input.name):
            os.remove(temp_input.name)

@app.route('/api/push_to_phone', methods=['POST'])
def push_to_phone():
    data = request.json
    backup_id = data.get('backup_id')
    if not backup_id:
        return jsonify({"success": False, "error": "No backup_id provided"})
        
    backup_path = os.path.join(os.getcwd(), backup_id, "files")
    if not os.path.exists(backup_path):
        return jsonify({"success": False, "error": f"La copia de seguridad no se ha descargado localmente aún. Por favor, descárgala primero."})

    try:
        import adbutils
        adb = adbutils.AdbClient(host="127.0.0.1", port=5037)
        devices = adb.device_list()
        
        if not devices:
            return jsonify({"success": False, "error": "No se detectó ningún celular conectado. Asegúrate de conectarlo por cable y activar la 'Depuración USB'."})
            
        device = devices[0]
        
        # Rutas de destino en Android
        paths = [
            "/sdcard/WhatsApp", # Android < 11 y Samsung
            "/sdcard/Android/media/com.whatsapp/WhatsApp" # Android >= 11
        ]
        
        # Subcarpetas a copiar
        subdirs_to_sync = ["Databases", "Backups"]
        
        logs = []
        for dest_base in paths:
            # Crear la ruta base en el celular si no existe
            device.shell(f"mkdir -p {dest_base}")
            
            for subdir in subdirs_to_sync:
                local_subdir = os.path.join(backup_path, subdir)
                if os.path.exists(local_subdir):
                    dest_subdir = f"{dest_base}/{subdir}"
                    device.shell(f"mkdir -p {dest_subdir}")
                    
                    # Push de los archivos individuales en ese directorio
                    for filename in os.listdir(local_subdir):
                        local_file = os.path.join(local_subdir, filename)
                        if os.path.isfile(local_file):
                            dest_file = f"{dest_subdir}/{filename}"
                            device.sync.push(local_file, dest_file)
                            logs.append(f"Copiado: {filename} a {dest_base}")
                            
        return jsonify({"success": True, "logs": logs})
        
    except Exception as e:
        return jsonify({"success": False, "error": f"Error ADB: {str(e)}"})

@app.route('/api/open_folder', methods=['POST'])
def open_folder():
    data = request.json
    backup_id = data.get('backup_id')
    if not backup_id:
        return jsonify({"success": False, "error": "No backup_id provided"}), 400
        
    # Prevenir Directory Traversal sanitizando la ruta
    base_dir = os.path.abspath(os.getcwd())
    backup_path = os.path.abspath(os.path.join(base_dir, backup_id))
    
    if not backup_path.startswith(base_dir):
        return jsonify({"success": False, "error": "Acceso no autorizado"}), 403
        
    if not os.path.exists(backup_path):
        return jsonify({"success": False, "error": "La copia de seguridad no se ha descargado todavía localmente."}), 404
        
    try:
        os.startfile(backup_path)
        return jsonify({"success": True})
    except Exception as e:
        return jsonify({"success": False, "error": str(e)}), 500

if __name__ == '__main__':
    # Detect if we should run in headless mode (e.g. inside Docker/Podman, or explicitly requested)
    run_headless = os.environ.get('HEADLESS', '0') == '1' or os.path.exists('/.dockerenv') or os.environ.get('CONTAINER', '') != ''
    
    if not run_headless:
        try:
            import webview
            window = webview.create_window('WhatsApp Google Drive Extractor', app, width=1000, height=700)
            webview.start()
        except Exception as e:
            print(f"No se pudo iniciar pywebview ({e}). Usando modo servidor web tradicional...")
            run_headless = True

    if run_headless:
        # Run Flask server directly (bind to 0.0.0.0 if in container, otherwise localhost for security)
        is_container = os.path.exists('/.dockerenv') or os.environ.get('CONTAINER', '') != ''
        host = "0.0.0.0" if is_container else "127.0.0.1"
        app.run(host=host, port=5000, debug=False)
