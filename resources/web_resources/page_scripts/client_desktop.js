'use strict';

// ── Desktop: keyboard shortcuts ───────────────────────────────────────────────

document.addEventListener('DOMContentLoaded', function () {
    document.addEventListener('keydown', function (e) {
        if (e.ctrlKey || e.metaKey || e.altKey) return;
        const tag = document.activeElement ? document.activeElement.tagName : '';
        if (tag === 'INPUT' || tag === 'SELECT' || tag === 'TEXTAREA') return;
        if (e.key === 'r') sendRefresh();
    });

    const logPanel = document.getElementById('logPanel');
    if (logPanel) {
        logPanel.addEventListener('toggle', function () {
            if (logPanel.open) {
                const logOutput = document.getElementById('logOutput');
                if (logOutput) { logOutput.scrollTop = logOutput.scrollHeight; }
                logPanel.scrollIntoView({ behavior: 'smooth', block: 'end' });
            }
        });
    }
});
