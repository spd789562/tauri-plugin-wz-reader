import { invoke } from '@tauri-apps/api/core';

export {
  initAPIClient,
  API_PATHES,
  WzReaderAPIClient,
} from './apiClient';

export enum WzType {
  File = 'File',
  Directory = 'Directory',
  Image = 'Image',
  Null = 'Null',

  Png = 'PNG',
  Sound = 'Sound',
  Property = 'Property',
  Convex = 'Convex',

  Short = 'Short',
  Int = 'Int',
  Long = 'Long',
  Float = 'Float',
  Double = 'Double',
  Vector = 'Vector',
  Uol = 'UOL',
  String = 'String',
  ParsedString = 'ParsedString',
  Lua = 'Lua',
  RawData = 'RawData',

  Unknown = 'Unknown',
}

export const MapWzType = {
  File: WzType.File,
  Directory: WzType.Directory,
  Image: WzType.Image,
  Null: WzType.Null,

  PNG: WzType.Png,
  Sound: WzType.Sound,
  Property: WzType.Property,
  Convex: WzType.Convex,

  Short: WzType.Short,
  Int: WzType.Int,
  Long: WzType.Long,
  Float: WzType.Float,
  Double: WzType.Double,
  Vector: WzType.Vector,
  UOL: WzType.Uol,
  String: WzType.String,
  ParsedString: WzType.ParsedString,
  Lua: WzType.Lua,
  RawData: WzType.RawData,
};

export function resolveWzType(t: string) {
  let data = {} as { type?: keyof typeof MapWzType };
  try {
    data = JSON.parse(t);
    return (data.type && MapWzType[data.type]) || WzType.Unknown;
  } catch (e) {
    return WzType.Unknown;
  }
}

export type InitOptions = {
  path: string;
  version?: string;
};

export type PathOptions = {
  path: string;
};

export interface WzNodeInfo {
  type: string;
  name: string;
  hasChild: boolean;
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

export async function parseNode(reqData: PathOptions): Promise<string> {
  return await invoke('plugin:wz-reader|parse_node', reqData);
}

export async function unparseNode(reqData: PathOptions): Promise<string> {
  return await invoke('plugin:wz-reader|unparse_node', reqData);
}

export async function getNodeInfo(reqData: PathOptions): Promise<WzNodeInfo> {
  return await invoke('plugin:wz-reader|get_node_info', reqData);
}

export async function getChildInfo(
  reqData: PathOptions,
): Promise<WzNodeInfo[]> {
  return await invoke('plugin:wz-reader|get_childs_info', reqData);
}
