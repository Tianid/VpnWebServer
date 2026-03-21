'use strict';

// ── Location view (full-screen) ───────────────────────────────────────────────

function openLocationView() {
    document.getElementById('locationView').removeAttribute('hidden');
    document.getElementById('locationSearch').focus();
}

function closeLocationView() {
    document.getElementById('locationView').setAttribute('hidden', '');
}

// ── Log panel toggle ──────────────────────────────────────────────────────────

function toggleLogPanel() {
    const panel = document.getElementById('logPanel');
    panel.classList.toggle('open');
    if (panel.classList.contains('open')) {
        const logOutput = document.getElementById('logOutput');
        if (logOutput) { logOutput.scrollTop = logOutput.scrollHeight; }
    }
}

// ── selectLocation override: close location view after selection ──────────────

var _origSelectLocation = selectLocation;
selectLocation = function (city) {
    _origSelectLocation(city);
    closeLocationView();
};

// ── applyTranslations override: keep trigger button in sync with language ──────

var _origApplyTranslations = applyTranslations;
applyTranslations = function () {
    _origApplyTranslations();
    var btn = document.getElementById('locationSheetBtn');
    if (btn) btn.textContent = tr('location_panel_title');
};
