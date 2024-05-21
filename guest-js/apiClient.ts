import { invoke } from '@tauri-apps/api/core';

export async function initAPIClient(): Promise<WzReaderAPIClient> {
  const host = await invoke<string>('plugin:wz-reader|get_server_url');
  return new WzReaderAPIClient(host);
}

export const API_PATHES = {
  MAPPING_SMAP: '/mapping/smap',
  MAPPING_ZMAP: '/mapping/zmap',
  MAPPING_IMAGE: '/mapping/image',

  NODE_PNG: '/node/image',
  NODE_SOUND: '/node/sound',
  NODE_JSON: '/node/json',
};

export class WzReaderAPIClient {
  host: string;
  constructor(host: string) {
    this.host = host;
  }

  async get<T>(path: string): Promise<T> {
    return fetch(`${this.host}${path}`).then((res) => res.json());
  }

  get_zmap() {
    return this.get<string[]>(API_PATHES.MAPPING_ZMAP);
  }
  get_smap() {
    return this.get<[string, string][]>(API_PATHES.MAPPING_SMAP);
  }
  get_image_mapping() {
    return this.get<string[]>(API_PATHES.MAPPING_IMAGE);
  }
  get_json(path: string, simple = false, force_parse = false) {
    const param = new URLSearchParams();
    if (simple) {
      param.append('simple', 'true');
    }
    if (force_parse) {
      param.append('force_parse', 'true');
    }
    return this.get(`${API_PATHES.NODE_JSON}/${path}?${param.toString()}`);
  }

  get_png_url(path: string, cache = 3600) {
    return `${this.host}${API_PATHES.NODE_PNG}/${path}?cache=${cache}`;
  }
  get_sound_url(path: string) {
    return `${this.host}${API_PATHES.NODE_SOUND}/${path}`;
  }
}
