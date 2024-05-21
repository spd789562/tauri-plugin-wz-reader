import { writable, derived } from 'svelte/store';
import { createWzTreeNodeView } from './wzTree';
import type { WzTreeNodeInfo } from './wzTree';

export interface PreviewNode extends WzTreeNodeInfo {
  path: string;
}

export const previewStore = writable<PreviewNode | undefined>(undefined);

export const hasPreview = derived(
  previewStore,
  ($preview) => $preview !== undefined,
);

export const isSelected = (path: string) =>
  derived(previewStore, ($preview) => {
    return $preview?.path === path;
  });

export const onClearPreview = () => previewStore.set(undefined);
export const onSelectPreview = (path: string, info?: WzTreeNodeInfo) => {
  if (info) {
    previewStore.set({ ...info, path: path || '' });
  } else {
    createWzTreeNodeView(path).subscribe((node) => {
      if (node) {
        previewStore.set({ ...node.info, path });
      }
    });
  }
};
export const onTogglePreview = (path: string) => {
  isSelected(path).subscribe((selected) => {
    if (selected) {
      onClearPreview();
    } else {
      onSelectPreview(path);
    }
  });
};
