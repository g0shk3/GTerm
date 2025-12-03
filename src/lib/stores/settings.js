import { writable } from 'svelte/store';

const storedSettings = localStorage.getItem('app_settings');
const defaultSettings = {
  autoStartLocalTerminal: false,
  autoCopyOnSelect: false
};

const initialSettings = storedSettings ? JSON.parse(storedSettings) : defaultSettings;

export const settings = writable(initialSettings);

settings.subscribe(value => {
  localStorage.setItem('app_settings', JSON.stringify(value));
});
