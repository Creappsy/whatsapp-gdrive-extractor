// Initialize Tauri APIs if running under Tauri
const invoke = window.__TAURI__ ? window.__TAURI__.core.invoke : async () => { return {}; };

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
        checkUpdates();
        setupDownloadListener();
        
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

// Load saved config on startup
async function loadSavedConfig() {
    try {
        const data = await invoke('get_config');
        
        const gmailField = document.getElementById('gmail');
        const androidIdField = document.getElementById('android_id');
        const oauthTokenField = document.getElementById('oauth_token');
        const passwordField = document.getElementById('password');

        if (data.gmail && gmailField) {
            gmailField.value = data.gmail;
        }
        if (data.android_id && androidIdField) {
            androidIdField.value = data.android_id;
        }
        if (data.has_oauth_token && oauthTokenField) {
            oauthTokenField.value = "••••••••••••";
        }
        if (data.has_password && passwordField) {
            passwordField.value = "••••••••••••";
        }
    } catch (e) {
        console.error("Error al cargar la configuración guardada:", e);
    }
}

// Clear bullet placeholders on focus/click to let user type new values
document.addEventListener('focusin', (e) => {
    if ((e.target.id === 'oauth_token' || e.target.id === 'password') && e.target.value === "••••••••••••") {
        e.target.value = "";
    }
});

// Run config loading on page load
if (document.getElementById('login-form')) {
    loadSavedConfig();
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
                password: password || null,
                oauthToken: oauth_token || null,
                androidId: android_id || null
            };

            await invoke('login', payload);
            window.location.href = 'dashboard.html';
        } catch (err) {
            errBox.textContent = err || t('js.errLogin', 'Error al iniciar sesión');
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
            const backups = await invoke('list_backups');
            renderBackups(backups);
        } catch (e) {
            console.error("No active session or error listing backups:", e);
            window.location.href = 'index.html';
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
    backupsContainer.innerHTML = '';
    
    if(!backups || backups.length === 0) {
        backupsContainer.innerHTML = `<div class="alert warning">${t('js.noBackups', 'No se encontraron copias de seguridad.')}</div>`;
        return;
    }

    backups.forEach(b => {
        const div = document.createElement('div');
        div.className = 'backup-card glass';
        
        // WhatsApp version check for showing Android vs PC restoration buttons
        const isAndroid = navigator.userAgent.toLowerCase().includes('android');
        
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
        await invoke('open_backup_folder', { backupId });
    } catch (e) {
        alert(e || t('js.errOpenFolder', "Error al intentar abrir la carpeta."));
    }
}

window.pushToPhone = async function(backupId) {
    if (!confirm(t('js.confirmTransfer', "Asegúrate de que tu celular está conectado por USB y tiene la Depuración USB activada.\n\nEsta acción copiará la base de datos descargada al almacenamiento de tu celular.\n\n¿Deseas continuar?"))) return;
    
    document.getElementById('progress-modal').classList.remove('hidden');
    document.getElementById('progress-status').textContent = t('js.statusTransferring', "Conectando al dispositivo y transfiriendo archivos por favor espera...");
    document.getElementById('progress-status').style.color = "var(--text)";
    document.getElementById('progress-fill').style.width = "50%";
    
    try {
        await invoke('push_to_phone', { backupId });
        
        document.getElementById('progress-status').textContent = t('js.statusTransferSuccess', "¡Transferencia Exitosa al celular!");
        document.getElementById('progress-status').style.color = "var(--primary)";
        document.getElementById('progress-fill').style.width = "100%";
        setTimeout(() => {
            document.getElementById('progress-modal').classList.add('hidden');
            alert(t('js.alertTransferDone', "¡Transferencia completada!\n\nRevisa la carpeta de WhatsApp en tu celular e instala WhatsApp para restaurar."));
        }, 2000);
    } catch (err) {
        document.getElementById('progress-modal').classList.add('hidden');
        alert(err || t('js.errAdb', "Error al intentar comunicarse con el servidor local para la transferencia."));
    }
}

window.startSync = async function(backupId) {
    showProgressModal();
    try {
        await invoke('download_backup', { backupId });
    } catch (e) {
        document.getElementById('progress-modal').classList.add('hidden');
        alert(e || t('js.errSync', "Error al iniciar descarga"));
    }
}

function showProgressModal() {
    document.getElementById('progress-modal').classList.remove('hidden');
    document.getElementById('progress-status').textContent = t('js.statusConnectingSync', "Conectando al servidor...");
    document.getElementById('progress-status').style.color = "var(--text)";
    document.getElementById('progress-fill').style.width = "0%";
    document.getElementById('progress-files-count').textContent = "";
}

// Set up background progress listener using Tauri events
let downloadUnlisten = null;
async function setupDownloadListener() {
    if (window.__TAURI__) {
        const { listen } = window.__TAURI__.event;
        downloadUnlisten = await listen('download_progress', (event) => {
            const data = event.payload;
            const statusText = document.getElementById('progress-status');
            const fill = document.getElementById('progress-fill');
            const filesCount = document.getElementById('progress-files-count');
            
            if(data.done) {
                statusText.textContent = t('dashboard.modalDoneTitle', "¡Descarga Completada!");
                statusText.style.color = "var(--primary)";
                fill.style.width = "100%";
                setTimeout(() => {
                    document.getElementById('progress-modal').classList.add('hidden');
                    alert(t('js.alertSyncDone', "¡Copia de seguridad descargada con éxito!"));
                }, 1500);
                return;
            }

            if(data.error) {
                statusText.textContent = t('js.errTransfer', "Error: ") + data.error;
                statusText.style.color = "var(--error)";
                return;
            }

            if(data.total_files > 0) {
                const pct = (data.num_files / data.total_files) * 100;
                fill.style.width = pct + "%";
                filesCount.textContent = `${data.num_files} / ${data.total_files} archivos`;
                statusText.textContent = t('dashboard.modalDownloading', "Descargando contenido...");
            }
        });
    }
}

const logoutBtn = document.getElementById('logout-btn');
if(logoutBtn) {
    logoutBtn.addEventListener('click', async () => {
        try {
            await invoke('logout');
        } catch (e) {}
        window.location.href = 'index.html';
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

        const file = fileInput.files[0];
        const reader = new FileReader();
        reader.onload = async function() {
            const base64Data = reader.result.split(',')[1];
            try {
                const decryptedBase64 = await invoke('decrypt_db', {
                    fileBase64: base64Data,
                    filename: file.name,
                    keyHex: keyInput
                });

                // Download the decrypted file
                const binaryString = window.atob(decryptedBase64);
                const len = binaryString.length;
                const bytes = new Uint8Array(len);
                for (let i = 0; i < len; i++) {
                    bytes[i] = binaryString.charCodeAt(i);
                }
                const blob = new Blob([bytes], {type: 'application/octet-stream'});
                const downloadUrl = window.URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = downloadUrl;
                a.download = file.name + '.db';
                document.body.appendChild(a);
                a.click();
                a.remove();
                alert(t('js.decryptSuccess', "¡Desencriptación exitosa! Tu base de datos SQLite ha sido descargada."));
            } catch (error) {
                errBox.textContent = "Error: " + (error || t('js.decryptFail', "Fallo en la desencriptación"));
                errBox.classList.remove('hidden');
            } finally {
                btn.textContent = t('js.btnDecryptReset', "Desencriptar .db");
                btn.disabled = false;
            }
        };
        reader.readAsDataURL(file);
    });
}

// Check updates checker
async function checkUpdates() {
    try {
        const data = await invoke('check_update');
        if (data.update_available) {
            const banner = document.createElement('div');
            banner.className = 'update-banner';
            banner.style.cssText = `
                background: linear-gradient(135deg, var(--primary, #007bff), #0056b3);
                color: white;
                text-align: center;
                padding: 0.75rem 1rem;
                position: sticky;
                top: 0;
                z-index: 10000;
                font-weight: 500;
                box-shadow: 0 4px 6px rgba(0,0,0,0.1);
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 10px;
            `;
            
            const message = document.createElement('span');
            let msgText = t('js.updateAvailable', "A new version is available! (v{{latest}}). Your current version is v{{current}}.");
            msgText = msgText.replace('{{latest}}', data.latest_version).replace('{{current}}', data.current_version);
            message.innerText = msgText;
            
            const link = document.createElement('a');
            link.href = data.url;
            link.target = '_blank';
            link.textContent = t('js.updateDownloadBtn', "Download");
            link.style.cssText = `
                color: white;
                text-decoration: underline;
                font-weight: bold;
                background: rgba(255, 255, 255, 0.2);
                padding: 0.25rem 0.75rem;
                border-radius: 4px;
                transition: background 0.2s;
            `;
            link.onmouseover = () => link.style.backgroundColor = 'rgba(255, 255, 255, 0.3)';
            link.onmouseout = () => link.style.backgroundColor = 'rgba(255, 255, 255, 0.2)';

            const closeBtn = document.createElement('button');
            closeBtn.textContent = '✕';
            closeBtn.style.cssText = `
                background: none;
                border: none;
                color: white;
                cursor: pointer;
                font-size: 1.1rem;
                margin-left: 15px;
            `;
            closeBtn.onclick = () => banner.remove();

            banner.appendChild(message);
            banner.appendChild(link);
            banner.appendChild(closeBtn);
            
            document.body.insertBefore(banner, document.body.firstChild);
        }
    } catch (error) {
        console.error("Error al comprobar actualizaciones:", error);
    }
}
