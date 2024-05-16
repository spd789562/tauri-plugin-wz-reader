import { invoke } from '@tauri-apps/api/core';

export type InitOptions = {
  path: string;
  version?: string;
};

export async function execute() {
  return await invoke('plugin:wz-reader|execute');
}

export async function init(options: InitOptions) {
  await invoke('plugin:wz-reader|init', options);
}

export async function get_server_url(): Promise<string> {
  return await invoke('plugin:wz-reader|get_server_url');
}
