import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';

let store = null;

async function initStore() {
  if (!store) {
    store = await load('hosts.json', { autoSave: true });
  }
  return store;
}

export async function getHosts() {
  const s = await initStore();
  const hosts = await s.get('hosts');
  return hosts || [];
}

export async function saveHost(host) {
  const s = await initStore();
  const hosts = await getHosts();
  const existingIndex = hosts.findIndex(h => h.id === host.id);

  if (existingIndex >= 0) {
    hosts[existingIndex] = host;
  } else {
    hosts.push(host);
  }

  await s.set('hosts', hosts);
  await s.save();
  return hosts;
}

export async function deleteHost(id) {
  const s = await initStore();
  const hosts = await getHosts();
  const filtered = hosts.filter(h => h.id !== id);
  await s.set('hosts', filtered);
  await s.save();
  return filtered;
}

export const hostsStore = writable([]);

export async function loadHosts() {
  const hosts = await getHosts();
  hostsStore.set(hosts);
  return hosts;
}

export async function addAndReloadHost(host) {
  const hosts = await saveHost(host);
  hostsStore.set(hosts);
  return hosts;
}

export async function removeAndReloadHost(id) {
  const hosts = await deleteHost(id);
  hostsStore.set(hosts);
  return hosts;
}

export async function updateAndReloadHost(host) {
  const hosts = await saveHost(host);
  hostsStore.set(hosts);
  return hosts;
}

export async function updateHosts(updatedHosts) {
  const s = await initStore();
  await s.set('hosts', updatedHosts);
  await s.save();
  hostsStore.set(updatedHosts);
  return updatedHosts;
}
