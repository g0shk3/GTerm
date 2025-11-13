import { writable } from 'svelte/store';

const storedTheme = localStorage.getItem('theme') || 'dark';
export const theme = writable(storedTheme);

theme.subscribe(value => {
  localStorage.setItem('theme', value);
  if (typeof document !== 'undefined') {
    if (value === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }
});
