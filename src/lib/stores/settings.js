import { writable } from 'svelte/store';

const storedSettings = localStorage.getItem('app_settings');

// Define default settings, including the new scrollback option
const defaultSettings = {
  autoStartLocalTerminal: false,
  autoCopyOnSelect: false,
  scrollback: 10000, // Default value
  openTabsNextToActive: false,
  searchDirection: 'bottomToTop' // 'bottomToTop' or 'topToBottom'
};

// Merge stored settings with defaults to ensure all keys are present
const initialSettings = storedSettings
  ? { ...defaultSettings, ...JSON.parse(storedSettings) }
  : defaultSettings;

export const settings = writable(initialSettings);

settings.subscribe(value => {
  localStorage.setItem('app_settings', JSON.stringify(value));
});
