<script lang="ts">
import { getChildInfo, parseNode } from 'tauri-plugin-wz-reader-api';
import { message } from '@tauri-apps/plugin-dialog';
import {
  createWzTreeNodeView,
  updateWzTreeOnPath,
  fromWzNodeInfo,
} from 'store/wzTree';
import { isSelected, onClearPreview, onSelectPreview } from 'store/preview';

export let path = '';

let isOpen = false;
let isLoading = false;
let isLoaded = false;
let hasError = false;

$: currentInfo = createWzTreeNodeView(path);
$: isSelectedNode = isSelected(path);

$: expandable =
  $currentInfo?.info.hasChild || $currentInfo?.info.type === 'Image';

async function onExpand() {
  if (isOpen || isLoading || hasError) {
    isOpen = false;
    return;
  }
  if (isLoaded) {
    isOpen = true;
    return;
  }
  isLoading = true;
  try {
    if ($currentInfo?.info.type === 'Image') {
      await parseNode({ path });
    }
    const childs = await getChildInfo({ path });
    childs.sort((a, b) => (a.name > b.name ? 1 : -1));
    updateWzTreeOnPath(path, (node) => {
      node.children = childs.map((child) => fromWzNodeInfo(child));
    });
    isLoading = false;
    isLoaded = true;
    isOpen = true;
  } catch (error) {
    console.error(error);
    hasError = true;
    await message(JSON.stringify(error), {
      kind: 'error',
      title: 'Expand Node Error',
    });
  }
}

function onTogglePreview() {
  $isSelectedNode
    ? onClearPreview()
    : onSelectPreview(path, $currentInfo?.info);
}
</script>

<ul class="tree-list">
  <li>
    <div class="row">
      {#if expandable}
        <button class="button button-expand" on:click={onExpand}>{isOpen ? '-' : '+'}</button>
      {/if}
      <button
        class="tree-node-name" 
        class:tree-node-name__selected={$isSelectedNode} 
        on:click={onTogglePreview}
      >
        {$currentInfo?.info.name}
      </button>
    </div>
    <div>
      {#if isOpen}
        {#each ($currentInfo?.children || []) as child}
          <svelte:self path={`${path}/${child.info.name}`} />
        {/each}
      {/if}
    </div>
  </li>
</ul>