import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';

let store = null;

async function initStore() {
  if (!store) {
    store = await load('snippets.json', { autoSave: true });
  }
  return store;
}

export async function getSnippets() {
  const s = await initStore();
  const snippets = await s.get('snippets');
  return snippets || [];
}

export async function saveSnippet(snippet) {
  const s = await initStore();
  const snippets = await getSnippets();
  const existingIndex = snippets.findIndex(sn => sn.id === snippet.id);

  if (existingIndex >= 0) {
    snippets[existingIndex] = snippet;
  } else {
    snippets.push(snippet);
  }

  await s.set('snippets', snippets);
  await s.save();
  return snippets;
}

export async function deleteSnippet(id) {
  const s = await initStore();
  const snippets = await getSnippets();
  const filtered = snippets.filter(sn => sn.id !== id);
  await s.set('snippets', filtered);
  await s.save();
  return filtered;
}

export const snippetsStore = writable([]);

export async function loadSnippets() {
  const snippets = await getSnippets();
  snippetsStore.set(snippets);
  return snippets;
}

export async function addAndReloadSnippet(snippet) {
  const snippets = await saveSnippet(snippet);
  snippetsStore.set(snippets);
  return snippets;
}

export async function updateAndReloadSnippet(snippet) {
  const snippets = await saveSnippet(snippet);
  snippetsStore.set(snippets);
  return snippets;
}

export async function removeAndReloadSnippet(id) {
  const snippets = await deleteSnippet(id);
  snippetsStore.set(snippets);
  return snippets;
}
