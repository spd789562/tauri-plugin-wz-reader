import { writable, derived } from 'svelte/store';
import { resolveWzType } from 'tauri-plugin-wz-reader-api';
import type { WzNodeInfo, WzType } from 'tauri-plugin-wz-reader-api';

export interface WzTreeNode {
  info: WzTreeNodeInfo;
  children: WzTreeNode[];
}

export interface WzTreeNodeInfo extends Omit<WzNodeInfo, 'type'> {
  data: WzNodeInfo['type'];
  type: WzType;
}

export function fromWzNodeInfo(info: WzNodeInfo): WzTreeNode {
  return {
    info: {
      ...info,
      data: info.type,
      type: resolveWzType(info.type) || 'unknown',
    },
    children: [],
  };
}

const getWzTreeNode = (
  node: WzTreeNode,
  path: string[],
): WzTreeNode | undefined => {
  const currentPath = path[0];

  if (!currentPath) {
    return node;
  }

  const target = node.children.find((child) => child.info.name === currentPath);

  if (!target) {
    return undefined;
  }

  return getWzTreeNode(target, path.slice(1));
};

export const wzTreeStore = writable<WzTreeNode | undefined>(undefined);

export const updateWzTreeOnPath = (
  path: string,
  callback: (node: WzTreeNode) => void,
) => {
  wzTreeStore.update(($wzTreeStore) => {
    if (!$wzTreeStore) {
      return $wzTreeStore;
    }
    const target = getWzTreeNode($wzTreeStore, path.split('/'));
    if (target) {
      callback(target);
      return $wzTreeStore;
    }
    return $wzTreeStore;
  });
};

export const createWzTreeNodeView = (path: string) => {
  return derived(
    wzTreeStore,
    ($wzTreeStore) =>
      $wzTreeStore && getWzTreeNode($wzTreeStore, path.split('/')),
  );
};
