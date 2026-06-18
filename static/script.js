// Initialize i18next
let i18nInitialized = false;

i18next
    .use(i18nextHttpBackend)
    .use(i18nextBrowserLanguageDetector)
    .init({
        fallbackLng: 'en',
        debug: false,
        backend: {
            loadPath: '/static/locales/{{lng}}/translation.json'
        }
    }).then(function(t) {
        i18nInitialized = true;
        updateContent();
        
        // Language switcher setup
        const langSwitch = document.getElementById('lang-switch');
        if (langSwitch) {
            // Find matched language prefix (e.g. es-US -> es)
            const currentLang = i18next.language.split('-')[0];
            const optionExists = Array.from(langSwitch.options).some(opt => opt.value === currentLang);
            if (optionExists) {
                langSwitch.value = currentLang;
            } else {
                langSwitch.value = 'en';
            }
            
            langSwitch.addEventListener('change', (e) => {
                i18next.changeLanguage(e.target.value).then(() => {
                    updateContent();
                    if (typeof window.loadBackups === 'function' && document.getElementById('backups-container')) {
                        // Refresh backups to update texts
                        window.loadBackups();
                    }
                });
            });
        }
    });

function updateContent() {
    document.querySelectorAll('[data-i18n]').forEach((element) => {
        const key = element.getAttribute('data-i18n');
        if (key.startsWith('[placeholder]')) {
            element.placeholder = i18next.t(key.replace('[placeholder]', ''));
        } else if (key.startsWith('[html]')) {
            element.innerHTML = i18next.t(key.replace('[html]', ''));
        } else {
            element.innerText = i18next.t(key);
        }
    });
}

// Helper for translation or fallback
function t(key, fallback) {
    if (!i18nInitialized) return fallback;
    return i18next.t(key);
}

// --- Rest of the application logic --- //

// Advanced Mode Toggle
const modeToggle = document.getElementById('advanced-mode-toggle');
if (modeToggle) {
    if (localStorage.getItem('advancedMode') === 'true') {
        document.body.classList.add('advanced-mode');
        modeToggle.checked = true;
    }
    
    modeToggle.addEventListener('change', (e) => {
        if (e.target.checked) {
            document.body.classList.add('advanced-mode');
            localStorage.setItem('advancedMode', 'true');
        } else {
            document.body.classList.remove('advanced-mode');
            localStorage.setItem('advancedMode', 'false');
        }
    });
}

// Theme Toggle
const themeBtn = document.getElementById('theme-btn');
const root = document.documentElement;
let isDark = localStorage.getItem('theme') === 'dark';

if(isDark) {
    root.setAttribute('data-theme', 'dark');
    if(themeBtn) themeBtn.textContent = '☀️';
}

if(themeBtn){
    themeBtn.addEventListener('click', () => {
        isDark = !isDark;
        if(isDark) {
            root.setAttribute('data-theme', 'dark');
            localStorage.setItem('theme', 'dark');
            themeBtn.textContent = '☀️';
        } else {
            root.removeAttribute('data-theme');
            localStorage.setItem('theme', 'light');
            themeBtn.textContent = '🌙';
        }
    });
}

// Login logic
const loginForm = document.getElementById('login-form');
const loginBtn = document.getElementById('login-btn');

if(loginForm) {
    loginForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        
        const errBox = document.getElementById('error-box');
        const gmail = document.getElementById('gmail').value.trim();
        
        const password = document.getElementById('password') ? document.getElementById('password').value.trim() : '';
        const android_id = document.getElementById('android_id') ? document.getElementById('android_id').value.trim() : '';
        const oauth_token = document.getElementById('oauth_token') ? document.getElementById('oauth_token').value.trim() : '';
        
        if (!gmail) {
            errBox.textContent = t('js.errEmail', 'Por favor introduce tu correo.');
            errBox.classList.remove('hidden');
            errBox.className = 'alert error';
            return;
        }

        if (!oauth_token && !password) {
            errBox.textContent = t('js.errTokenPass', 'Por favor introduce el Token OAuth o una Contraseña de Aplicación.');
            errBox.classList.remove('hidden');
            errBox.className = 'alert error';
            return;
        }

        loginBtn.textContent = t('login.connectingBtn', "Conectando...");
        loginBtn.disabled = true;
        errBox.classList.add('hidden');

        try {
            const payload = {
                gmail: gmail,
                password: password
            };
            
            if (android_id) {
                payload.android_id = android_id;
            }
            if (oauth_token) {
                payload.oauth_token = oauth_token;
            }

            const res = await fetch('/api/login', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify(payload)
            });
            const data = await res.json();
            
            if(data.success) {
                window.location.href = '/';
            } else {
                errBox.textContent = data.error || t('js.errLogin', 'Error al iniciar sesión');
                errBox.classList.remove('hidden');
                errBox.className = 'alert error';
                loginBtn.textContent = t('login.recoverBtn', "¡Recuperar Mis Chats!");
                loginBtn.disabled = false;
            }
        } catch (err) {
            errBox.textContent = t('js.errNetLogin', 'Error de red al conectar con el servidor');
            errBox.classList.remove('hidden');
            errBox.className = 'alert error';
            loginBtn.textContent = t('login.recoverBtn', "¡Recuperar Mis Chats!");
            loginBtn.disabled = false;
        }
    });
}

// Dashboard logic
const backupsContainer = document.getElementById('backups-container');
if(backupsContainer) {
    window.loadBackups = async function() {
        try {
            const res = await fetch('/api/backups');
            if(res.status === 401) {
                window.location.href = '/';
                return;
            }
            const data = await res.json();
            renderBackups(data.backups);
        } catch (e) {
            backupsContainer.innerHTML = `<div class="alert error">${t('js.errLoadBackups', 'Error al cargar backups')}</div>`;
        }
    }
    // Try to load backups if i18n is ready, otherwise wait.
    if(i18nInitialized){
        window.loadBackups();
    } else {
        i18next.on('initialized', () => window.loadBackups());
    }
}

function renderBackups(backups) {
    if(!backups || backups.length === 0) {
        backupsContainer.innerHTML = `<p>${t('js.noBackups', 'No se encontraron copias de seguridad.')}</p>`;
        return;
    }
    backupsContainer.innerHTML = '';
    const isAndroid = /android/i.test(navigator.userAgent);
    backups.forEach(b => {
        const div = document.createElement('div');
        div.className = 'glass-card backup-card';
        div.innerHTML = `
            <h3>${t('js.backupPrefix', 'Copia:')} ${b.id}</h3>
            <div class="stat-row"><span>${t('js.size', 'Tamaño')}</span><span>${b.backupSizeHuman}</span></div>
            <div class="stat-row"><span>${t('js.messages', 'Mensajes')}</span><span>${b.messages}</span></div>
            <div class="stat-row"><span>${t('js.media', 'Archivos Media')}</span><span>${b.mediaFiles}</span></div>
            <div class="stat-row"><span>${t('js.date', 'Fecha')}</span><span>${new Date(b.uploadTime).toLocaleString()}</span></div>
            <div class="backup-actions">
                <button class="btn primary" onclick="startSync('${b.id}')">${t('js.btnDownload', '⬇ Descargar')}</button>
                ${isAndroid ? '' : `<button class="btn success" onclick="pushToPhone('${b.id}')">${t('js.btnTransfer', '📲 Transferir al Celular')}</button>`}
                <button class="btn warning" onclick="openFolder('${b.id}')">${t('js.btnOpenFolder', '📂 Abrir Carpeta')}</button>
            </div>
        `;
        backupsContainer.appendChild(div);
    });
}

window.openFolder = async function(backupId) {
    try {
        const res = await fetch('/api/open_folder', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({ backup_id: backupId })
        });
        const data = await res.json();
        if (!data.success) {
            alert(data.error);
        }
    } catch (e) {
        alert(t('js.errOpenFolder', "Error al intentar abrir la carpeta."));
    }
}

window.pushToPhone = async function(backupId) {
    if (!confirm(t('js.confirmTransfer', "Asegúrate de que tu celular está conectado por USB y tiene la Depuración USB activada.\n\nEsta acción copiará la base de datos descargada al almacenamiento de tu celular.\n\n¿Deseas continuar?"))) return;
    
    document.getElementById('progress-modal').classList.remove('hidden');
    document.getElementById('progress-status').textContent = t('js.statusTransferring', "Conectando al dispositivo y transfiriendo archivos por favor espera...");
    document.getElementById('progress-status').style.color = "var(--text)";
    document.getElementById('progress-fill').style.width = "50%";
    
    try {
        const res = await fetch('/api/push_to_phone', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({ backup_id: backupId })
        });
        const data = await res.json();
        
        if (data.success) {
            document.getElementById('progress-status').textContent = t('js.statusTransferSuccess', "¡Transferencia Exitosa al celular!");
            document.getElementById('progress-status').style.color = "var(--primary)";
            document.getElementById('progress-fill').style.width = "100%";
            setTimeout(() => {
                document.getElementById('progress-modal').classList.add('hidden');
                alert(t('js.alertTransferDone', "¡Transferencia completada!\n\nRevisa la carpeta de WhatsApp en tu celular e instala WhatsApp para restaurar."));
            }, 2000);
        } else {
            document.getElementById('progress-status').textContent = t('js.errTransfer', "Error: ") + data.error;
            document.getElementById('progress-status').style.color = "var(--error)";
            setTimeout(() => {
                document.getElementById('progress-modal').classList.add('hidden');
                alert(t('js.errTransfer', "Hubo un error: ") + data.error);
            }, 4000);
        }
    } catch (e) {
        document.getElementById('progress-modal').classList.add('hidden');
        alert(t('js.errAdb', "Error al intentar comunicarse con el servidor local para la transferencia."));
    }
}

window.startSync = async function(backupId) {
    try {
        const res = await fetch('/api/sync', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({ backup_id: backupId })
        });
        const data = await res.json();
        if(data.success) {
            showProgressModal();
            listenToProgress();
        }
    } catch (e) {
        alert(t('js.errSync', "Error al iniciar descarga"));
    }
}

function showProgressModal() {
    document.getElementById('progress-modal').classList.remove('hidden');
    document.getElementById('progress-status').textContent = t('js.statusConnectingSync', "Conectando al servidor...");
    document.getElementById('progress-fill').style.width = "0%";
}

function listenToProgress() {
    const evtSource = new EventSource('/api/progress');
    const fill = document.getElementById('progress-fill');
    const filesCount = document.getElementById('files-count');
    const statusText = document.getElementById('progress-status');

    evtSource.onmessage = function(event) {
        const data = JSON.parse(event.data);
        if(data.keep_alive) return;
        
        if(data.done) {
            statusText.textContent = t('dashboard.modalDoneTitle', "¡Descarga Completada!");
            statusText.style.color = "var(--primary)";
            fill.style.width = "100%";
            evtSource.close();
            setTimeout(() => {
                document.getElementById('progress-modal').classList.add('hidden');
                alert(t('js.alertSyncDone', "¡Copia de seguridad descargada con éxito!"));
            }, 1500);
            return;
        }

        if(data.error) {
            statusText.textContent = t('js.errTransfer', "Error: ") + data.error;
            statusText.style.color = "var(--error)";
            evtSource.close();
            return;
        }

        if(data.total_files > 0) {
            const pct = (data.num_files / data.total_files) * 100;
            fill.style.width = pct + "%";
            filesCount.textContent = `${data.num_files} / ${data.total_files} archivos`;
            statusText.textContent = t('dashboard.modalDownloading', "Descargando contenido...");
        }
    };
    
    evtSource.onerror = function() {
        console.error("EventSource failed.");
    };
}

const logoutBtn = document.getElementById('logout-btn');
if(logoutBtn) {
    logoutBtn.addEventListener('click', async () => {
        await fetch('/api/logout', {method: 'POST'});
        window.location.href = '/';
    });
}

// Decryption Logic
const decryptForm = document.getElementById('decrypt-form');
if (decryptForm) {
    decryptForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const fileInput = document.getElementById('crypt-file');
        const keyInput = document.getElementById('crypt-key').value;
        const btn = document.getElementById('decrypt-btn');
        const errBox = document.getElementById('decrypt-error');

        if (!fileInput.files.length) return;

        btn.textContent = t('dashboard.decryptingBtn', "Procesando...");
        btn.disabled = true;
        errBox.classList.add('hidden');
        errBox.textContent = "";

        const formData = new FormData();
        formData.append('file', fileInput.files[0]);
        formData.append('key_hex', keyInput);

        try {
            const response = await fetch('/api/decrypt', {
                method: 'POST',
                body: formData
            });

            if (response.ok) {
                // Download the decrypted file
                const blob = await response.blob();
                const downloadUrl = window.URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = downloadUrl;
                a.download = fileInput.files[0].name + '.db';
                document.body.appendChild(a);
                a.click();
                a.remove();
                alert(t('js.decryptSuccess', "¡Desencriptación exitosa! Tu base de datos SQLite ha sido descargada."));
            } else {
                const errorData = await response.json();
                errBox.textContent = "Error: " + (errorData.error || t('js.decryptFail', "Fallo en la desencriptación"));
                errBox.classList.remove('hidden');
            }
        } catch (error) {
            errBox.textContent = t('js.errNetDecrypt', "Error de red al intentar desencriptar.");
            errBox.classList.remove('hidden');
        } finally {
            btn.textContent = t('js.btnDecryptReset', "Desencriptar .db");
            btn.disabled = false;
        }
    });
}
