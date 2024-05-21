import { writable } from 'svelte/store';
import type { WzReaderAPIClient } from 'tauri-plugin-wz-reader-api';

export const isInitialized = writable(false);
export const apiClient = writable<WzReaderAPIClient | undefined>(undefined);
