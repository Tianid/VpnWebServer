'use strict';

// ── i18n ──────────────────────────────────────────────────────────────────────

const i18n = {
    en: {
        title:               'VPN Control Panel',
        status_connected:    'Connected',
        status_disconnected: 'Disconnected',
        status_connecting:   'Connecting...',
        status_disconnecting:'Disconnecting...',
        status_reconnecting: 'Reconnecting...',
        btn_connect:         'Connect',
        btn_disconnect:      'Disconnect',
        btn_wifi:            'Reconnect WiFi',
        btn_restart:         'Restart',
        btn_refresh:         'Refresh',
        location_placeholder:'Search locations...',
        location_auto:       'Fastest (auto)',
        location_ping:       'ms',
        lang_toggle:         'RU',
        loading_locations:   'Loading locations...',
        error_ws:            'Connection to server lost. Reconnecting...',
        log_panel_title:     'Server Log',
        log_clear:           'Clear',
        location_panel_title:'Locations',
        confirm_wifi_title:  'Reconnect WiFi',
        confirm_wifi_msg:    'Reconnect the device to the current Wi-Fi network?',
        confirm_restart_title:'Restart Device',
        confirm_restart_msg: 'Restart the device now? The connection will be briefly interrupted.',
        btn_confirm:         'Confirm',
        btn_cancel:          'Cancel',
    },
    ru: {
        title:               'Панель управления VPN',
        status_connected:    'Подключено',
        status_disconnected: 'Отключено',
        status_connecting:   'Подключение...',
        status_disconnecting:'Отключение...',
        status_reconnecting: 'Переподключение...',
        btn_connect:         'Подключиться',
        btn_disconnect:      'Отключиться',
        btn_wifi:            'Переподключить WiFi',
        btn_restart:         'Перезагрузить',
        btn_refresh:         'Обновить',
        location_placeholder:'Поиск локации...',
        location_auto:       'Быстрейшая (авто)',
        location_ping:       'мс',
        lang_toggle:         'EN',
        loading_locations:   'Загрузка локаций...',
        error_ws:            'Соединение с сервером потеряно. Переподключение...',
        log_panel_title:     'Лог сервера',
        log_clear:           'Очистить',
        location_panel_title:'Локации',
        confirm_wifi_title:  'Переподключить WiFi',
        confirm_wifi_msg:    'Переподключить устройство к текущей Wi-Fi сети?',
        confirm_restart_title:'Перезагрузка',
        confirm_restart_msg: 'Перезагрузить устройство? Соединение будет кратковременно прервано.',
        btn_confirm:         'Подтвердить',
        btn_cancel:          'Отмена',
    }
};

// ── Language ──────────────────────────────────────────────────────────────────

function detectLanguage() {
    const saved = sessionStorage.getItem('lang');
    if (saved) return saved;
    return navigator.language.toLowerCase().startsWith('ru') ? 'ru' : 'en';
}

let currentLang = detectLanguage();

function tr(key) {
    return i18n[currentLang][key] || key;
}

function toggleLanguage() {
    currentLang = currentLang === 'ru' ? 'en' : 'ru';
    sessionStorage.setItem('lang', currentLang);
    applyTranslations();
}

function applyTranslations() {
    document.title = tr('title');
    document.getElementById('langToggle').textContent = tr('lang_toggle');
    document.getElementById('refreshBtn').textContent = tr('btn_refresh');
    document.getElementById('wifiBtn').textContent = tr('btn_wifi');
    document.getElementById('restartBtn').textContent = tr('btn_restart');
    document.getElementById('modalCancelBtn').textContent  = tr('btn_cancel');
    document.getElementById('modalConfirmBtn').textContent = tr('btn_confirm');
    document.getElementById('locationSearch').placeholder = tr('location_placeholder');
    const logTitle = document.querySelector('[data-i18n="log_panel_title"]');
    if (logTitle) logTitle.textContent = tr('log_panel_title');
    const logClear = document.querySelector('[data-i18n="log_clear"]');
    if (logClear) logClear.textContent = '\uD83D\uDDD1 ' + tr('log_clear');
    const locTitle = document.querySelector('[data-i18n="location_panel_title"]');
    if (locTitle) locTitle.textContent = tr('location_panel_title');
    updateStatusUI(currentVpnState, currentLocation);
    renderLocationTable();
}

// ── State ─────────────────────────────────────────────────────────────────────

let currentVpnState = null;
let currentLocation  = null;
let allLocations     = [];
let selectedCity     = null;

// ── WebSocket ─────────────────────────────────────────────────────────────────

const RECONNECT_DELAYS = [1000, 2000, 4000, 8000, 16000];
let reconnectAttempt = 0;
let ws = null;

function connect() {
    const host = window.location.hostname;
    const port = window.location.port;
    ws = new WebSocket('ws://' + host + ':' + port + '/ws');

    ws.addEventListener('open', function () {
        reconnectAttempt = 0;
        hideError();
    });

    ws.addEventListener('message', function (event) {
        try {
            handleMessage(JSON.parse(event.data));
        } catch (e) {
            console.warn('Failed to parse server message:', event.data);
        }
    });

    ws.addEventListener('close', function () {
        setStatusUnknown();
        showError(tr('error_ws'));
        scheduleReconnect();
    });

    ws.addEventListener('error', function () {
        console.error('WebSocket error');
    });
}

function scheduleReconnect() {
    const delay = RECONNECT_DELAYS[Math.min(reconnectAttempt, RECONNECT_DELAYS.length - 1)];
    reconnectAttempt++;
    setTimeout(connect, delay);
}

function sendMessage(obj) {
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify(obj));
    }
}

// ── Incoming message dispatch ─────────────────────────────────────────────────

function handleMessage(msg) {
    switch (msg.type) {
        case 'StatusUpdate':
            currentVpnState = msg.state;
            if (msg.location) {
                currentLocation = msg.location;
            } else if (msg.state === 'Connected') {
                currentLocation = currentLocation || selectedCity || null;
            } else {
                currentLocation = null;
            }
            updateStatusUI(msg.state, currentLocation);
            renderLocationTable();
            break;
        case 'LocationList':
            allLocations = msg.locations || [];
            renderLocationTable();
            if (currentVpnState === 'Connected' && currentLocation) {
                const canon = allLocations.find(function(l) {
                    return l.city.toLowerCase() === currentLocation.toLowerCase();
                });
                if (canon) currentLocation = canon.city;
            }
            updateLocationLabel(currentLocation, currentVpnState);
            break;
        case 'LogLine':
            appendLogLine(msg);
            break;
        case 'LogLevelChanged':
            document.getElementById('logLevelSelect').value = msg.level;
            break;
        case 'Error':
            console.error('Server error [' + msg.code + ']: ' + msg.message);
            showError(msg.code + ': ' + msg.message);
            break;
        default:
            console.warn('Unknown server message type:', msg.type);
    }
}

// ── Status UI ─────────────────────────────────────────────────────────────────

const STATE_CSS = {
    Connected:    'status-connected',
    Disconnected: 'status-disconnected',
    Connecting:   'status-connecting',
    Disconnecting:'status-disconnecting',
    Reconnecting: 'status-reconnecting',
};

const STATE_TEXT_KEY = {
    Connected:    'status_connected',
    Disconnected: 'status_disconnected',
    Connecting:   'status_connecting',
    Disconnecting:'status_disconnecting',
    Reconnecting: 'status_reconnecting',
};

function updateStatusUI(state, locationName) {
    const indicator = document.getElementById('statusIndicator');
    const statusEl  = document.getElementById('statusText');
    const toggleBtn = document.getElementById('toggleBtn');

    indicator.className = 'status-indicator ' + (STATE_CSS[state] || 'status-unknown');

    var text = state ? tr(STATE_TEXT_KEY[state] || 'status_disconnected') : '\u2014';
    statusEl.textContent = text;
    updateLocationLabel(locationName, state);

    const inTransition = state === 'Connecting' || state === 'Disconnecting' || state === 'Reconnecting';
    if (inTransition) {
        toggleBtn.textContent = state ? tr(STATE_TEXT_KEY[state]) : '\u2026';
        toggleBtn.disabled = true;
        toggleBtn.classList.remove('danger');
    } else if (state === 'Connected') {
        toggleBtn.textContent = tr('btn_disconnect');
        toggleBtn.disabled = false;
        toggleBtn.classList.add('danger');
    } else {
        toggleBtn.textContent = tr('btn_connect');
        toggleBtn.disabled = (state === null);
        toggleBtn.classList.remove('danger');
    }
}

function setStatusUnknown() {
    currentVpnState = null;
    currentLocation  = null;
    const indicator = document.getElementById('statusIndicator');
    indicator.className = 'status-indicator status-unknown';
    document.getElementById('statusText').textContent = '\u2014';
    updateLocationLabel(null, null);
    const toggleBtn = document.getElementById('toggleBtn');
    toggleBtn.textContent = '\u2026';
    toggleBtn.disabled = true;
    toggleBtn.classList.remove('danger');
}

function updateLocationLabel(locationName, state) {
    const locLabel = document.getElementById('locationLabel');
    if (!locLabel) return;
    if (state === 'Connected' && locationName) {
        const canon = allLocations.find(function(l) {
            return l.city.toLowerCase() === locationName.toLowerCase();
        });
        if (canon) {
            locLabel.innerHTML = '';
            const flag = isoToFlag(canon.id);
            if (flag) {
                const flagEl = document.createElement('span');
                flagEl.className = 'loc-label-flag';
                flagEl.textContent = flag + '\u00a0';
                locLabel.appendChild(flagEl);
            }
            const countryEl = document.createElement('span');
            countryEl.className = 'loc-label-country';
            countryEl.textContent = canon.country;
            locLabel.appendChild(countryEl);
            locLabel.appendChild(document.createTextNode(' \u00b7 '));
            const cityEl = document.createElement('span');
            cityEl.className = 'loc-label-city';
            cityEl.textContent = canon.city;
            locLabel.appendChild(cityEl);
        } else {
            locLabel.textContent = locationName;
        }
    } else {
        locLabel.innerHTML = '';
    }
}

// ── Location table ────────────────────────────────────────────────────────────

function filterLocations() {
    renderLocationTable();
}

function renderLocationTable() {
    const query = document.getElementById('locationSearch').value.toLowerCase().trim();
    const body  = document.getElementById('locationBody');

    const filtered = allLocations.filter(function (loc) {
        return loc.city.toLowerCase().includes(query) ||
               loc.country.toLowerCase().includes(query) ||
               loc.id.toLowerCase().includes(query);
    });

    if (allLocations.length === 0) {
        body.innerHTML = '<tr><td colspan="2" class="location-empty">' +
            escapeHtml(tr('loading_locations')) + '</td></tr>';
        return;
    }

    if (filtered.length === 0) {
        body.innerHTML = '<tr><td colspan="2" class="location-empty">' +
            escapeHtml(tr('location_auto')) + '</td></tr>';
        return;
    }

    body.innerHTML = '';
    filtered.forEach(function (loc) {
        const isActive = currentVpnState === 'Connected' &&
            loc.city.toLowerCase() === (currentLocation || '').toLowerCase();
        const row = document.createElement('tr');
        if (isActive) row.classList.add('connected');
        if (loc.city === selectedCity) row.classList.add('selected');

        const tdName = document.createElement('td');
        const info = document.createElement('div');
        info.className = 'loc-info';
        const flag = isoToFlag(loc.id);
        if (flag) {
            const flagEl = document.createElement('span');
            flagEl.className = 'loc-flag';
            flagEl.textContent = flag;
            info.appendChild(flagEl);
        }
        const textDiv = document.createElement('div');
        const countryDiv = document.createElement('div');
        countryDiv.className = 'loc-country';
        countryDiv.textContent = loc.country;
        const cityDiv = document.createElement('div');
        cityDiv.className = 'loc-city';
        cityDiv.textContent = loc.city;
        textDiv.appendChild(countryDiv);
        textDiv.appendChild(cityDiv);
        info.appendChild(textDiv);
        tdName.appendChild(info);

        const tdPing = document.createElement('td');
        tdPing.className = 'loc-ping';
        const ms = loc.ping_ms;
        const pingSpan = document.createElement('span');
        pingSpan.textContent = ms >= 0 ? ms + '\u00a0' + tr('location_ping') : '\u2014';
        pingSpan.style.color = pingColor(ms);
        tdPing.appendChild(pingSpan);
        if (isActive) {
            const check = document.createElement('span');
            check.className = 'loc-check';
            check.textContent = '\u00a0\u2713';
            tdPing.appendChild(check);
        }

        row.appendChild(tdName);
        row.appendChild(tdPing);
        row.addEventListener('click', function () { selectLocation(loc.city); });
        body.appendChild(row);
    });
}

function selectLocation(city) {
    const inTransition = currentVpnState === 'Connecting' || currentVpnState === 'Disconnecting' || currentVpnState === 'Reconnecting';
    if (inTransition) return;
    if (currentVpnState === 'Connected' &&
        (currentLocation || '').toLowerCase() === city.toLowerCase()) return;
    selectedCity = city;
    currentVpnState = 'Connecting';
    updateStatusUI('Connecting', null);
    renderLocationTable();
    sendMessage({ type: 'Connect', location: city });
}

// ── Outgoing senders ──────────────────────────────────────────────────────────

function onToggleClick() {
    if (currentVpnState === 'Connected') {
        currentVpnState = 'Disconnecting';
        updateStatusUI('Disconnecting', null);
        renderLocationTable();
        sendMessage({ type: 'Disconnect' });
    } else {
        currentVpnState = 'Connecting';
        updateStatusUI('Connecting', null);
        renderLocationTable();
        const msg = { type: 'Connect' };
        if (selectedCity) msg.location = selectedCity;
        sendMessage(msg);
    }
}

function sendWifi()    { sendMessage({ type: 'ReconnectWifi' }); }
function sendRestart() { sendMessage({ type: 'Restart' }); }
function sendRefresh() { sendMessage({ type: 'RefreshLocations' }); }

// ── Confirmation modal ────────────────────────────────────────────────────────

let _confirmCallback = null;

function showConfirm(titleKey, msgKey, onConfirm) {
    document.getElementById('modalTitle').textContent   = tr(titleKey);
    document.getElementById('modalMessage').textContent = tr(msgKey);
    document.getElementById('modalCancelBtn').textContent  = tr('btn_cancel');
    document.getElementById('modalConfirmBtn').textContent = tr('btn_confirm');
    _confirmCallback = onConfirm;
    document.getElementById('confirmModal').classList.add('active');
    document.getElementById('modalConfirmBtn').focus();
}

function hideConfirm() {
    document.getElementById('confirmModal').classList.remove('active');
    _confirmCallback = null;
}

function confirmWifi()    { showConfirm('confirm_wifi_title',    'confirm_wifi_msg',    sendWifi); }
function confirmRestart() { showConfirm('confirm_restart_title', 'confirm_restart_msg', sendRestart); }

// ── Log panel ─────────────────────────────────────────────────────────────────

const MAX_LOG_LINES = 500;

function appendLogLine(msg) {
    const el   = document.getElementById('logOutput');
    const line = document.createElement('div');
    line.className   = 'log-' + msg.level.toLowerCase();
    line.textContent = msg.timestamp.slice(11) + ' ' +
                       msg.level.padEnd(5) + ' [' + msg.tag + '] ' + msg.message;
    el.appendChild(line);
    if (el.scrollHeight - el.scrollTop <= el.clientHeight + 30) {
        el.scrollTop = el.scrollHeight;
    }
    while (el.children.length > MAX_LOG_LINES) {
        el.removeChild(el.firstChild);
    }
}

function clearLog() {
    document.getElementById('logOutput').innerHTML = '';
}

function setLogLevel(level) {
    sendMessage({ type: 'SetLogLevel', level: level });
}

// ── Error banner ──────────────────────────────────────────────────────────────

function showError(msg) {
    const banner = document.getElementById('errorBanner');
    banner.textContent = msg;
    banner.style.display = 'block';
}

function hideError() {
    document.getElementById('errorBanner').style.display = 'none';
}

// ── Utility ───────────────────────────────────────────────────────────────────

function escapeHtml(str) {
    return str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;');
}

function isoToFlag(iso) {
    if (!iso || iso.length !== 2) return '';
    const up = iso.toUpperCase();
    const base = 0x1F1E6 - 65;
    return String.fromCodePoint(up.charCodeAt(0) + base) +
           String.fromCodePoint(up.charCodeAt(1) + base);
}

function pingColor(ms) {
    if (ms < 0)   return '#999';
    if (ms < 80)  return '#28a745';
    if (ms < 180) return '#fd7e14';
    return '#dc3545';
}

// ── Bootstrap ─────────────────────────────────────────────────────────────────

document.addEventListener('DOMContentLoaded', function () {
    applyTranslations();
    connect();

    document.getElementById('modalCancelBtn').addEventListener('click', hideConfirm);
    document.getElementById('modalConfirmBtn').addEventListener('click', function () {
        const cb = _confirmCallback;
        hideConfirm();
        if (cb) cb();
    });
    document.getElementById('confirmModal').addEventListener('click', function (e) {
        if (e.target === this) hideConfirm();
    });
    document.addEventListener('keydown', function (e) {
        if (e.key === 'Escape') hideConfirm();
    });
});

