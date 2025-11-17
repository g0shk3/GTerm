import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';

let store = null;

async function initStore() {
  if (!store) {
    store = await load('privateKeys.json', { autoSave: true });
  }
  return store;
}

export async function getPrivateKeys() {
  const s = await initStore();
  const privateKeys = await s.get('privateKeys');
  return privateKeys || [];
}

export async function savePrivateKey(privateKey) {
  const s = await initStore();
  const privateKeys = await getPrivateKeys();
  const existingIndex = privateKeys.findIndex(pk => pk.id === privateKey.id);

  if (existingIndex >= 0) {
    privateKeys[existingIndex] = privateKey;
  } else {
    privateKeys.push(privateKey);
  }

  await s.set('privateKeys', privateKeys);
  await s.save();
  return privateKeys;
}

export async function deletePrivateKey(id) {
  const s = await initStore();
  const privateKeys = await getPrivateKeys();
  const filtered = privateKeys.filter(pk => pk.id !== id);
  await s.set('privateKeys', filtered);
  await s.save();
  return filtered;
}

export const privateKeysStore = writable([]);

export async function loadPrivateKeys() {
  const privateKeys = await getPrivateKeys();
  privateKeysStore.set(privateKeys);
  return privateKeys;
}

export async function addAndReloadPrivateKey(privateKey) {
  const privateKeys = await savePrivateKey(privateKey);
  privateKeysStore.set(privateKeys);
  return privateKeys;
}

export async function updateAndReloadPrivateKey(privateKey) {
  const privateKeys = await savePrivateKey(privateKey);
  privateKeysStore.set(privateKeys);
  return privateKeys;
}

export async function removeAndReloadPrivateKey(id) {
  const privateKeys = await deletePrivateKey(id);
  privateKeysStore.set(privateKeys);
  return privateKeys;
}
