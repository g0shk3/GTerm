import { writable } from 'svelte/store';

const storedSettings = localStorage.getItem('app_settings');
const defaultSettings = {
  autoStartLocalTerminal: false
};

const initialSettings = storedSettings ? JSON.parse(storedSettings) : defaultSettings;

export const settings = writable(initialSettings);

settings.subscribe(value => {
  localStorage.setItem('app_settings', JSON.stringify(value));
});

export function toggleAutoStartLocalTerminal() {
  settings.update(s => ({
    ...s,
    autoStartLocalTerminal: !s.autoStartLocalTerminal
  }));
}
