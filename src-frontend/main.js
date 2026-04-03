import { invoke } from '@tauri-apps/api/core';

document.addEventListener('DOMContentLoaded', () => {
    const slapBtn = document.getElementById('slap-btn');
    const statusText = document.getElementById('status-text');
    const licenseStatus = document.getElementById('license-status');
    const version = document.getElementById('version');

    // Load initial state
    async function init() {
        try {
            const state = await invoke('get_app_state');
            licenseStatus.textContent = state.licensed ? 'Licensed' : 'Unlicensed';
            licenseStatus.className = state.licensed ? 'licensed' : 'unlicensed';
            version.textContent = state.version;
        } catch (e) {
            console.error('Failed to load state:', e);
        }
    }

    slapBtn.addEventListener('click', async () => {
        slapBtn.disabled = true;
        statusText.textContent = 'Working...';
        statusText.className = '';

        try {
            const result = await invoke('perform_slap', {
                intensity: 'medium'
            });
            statusText.textContent = result.message || 'Done!';
            statusText.className = 'success';
        } catch (e) {
            statusText.textContent = e;
            statusText.className = 'warning';
        } finally {
            slapBtn.disabled = false;
        }
    });

    init();
});
