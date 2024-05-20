import { invoke } from '@tauri-apps/api/core';

export type InitOptions = {
  path: string;
  version?: string;
};

export interface WzNodeInfo {
  type: string;
  name: string;
  has_child: boolean;
}

export async function execute() {
  return await invoke('plugin:wz-reader|execute');
}

export async function init(options: InitOptions) {
  await invoke('plugin:wz-reader|init', options);
}

export async function getServerUrl(): Promise<string> {
  return await invoke('plugin:wz-reader|get_server_url');
}

export async function parseNode(): Promise<string> {
  return await invoke('plugin:wz-reader|parse_node');
}

export async function unparseNode(): Promise<string> {
  return await invoke('plugin:wz-reader|unparse_node');
}

export async function getNodeInfo(): Promise<WzNodeInfo> {
  return await invoke('plugin:wz-reader|get_node_info');
}

export async function getChildInfo(): Promise<WzNodeInfo[]> {
  return await invoke('plugin:wz-reader|get_child_info');
}
