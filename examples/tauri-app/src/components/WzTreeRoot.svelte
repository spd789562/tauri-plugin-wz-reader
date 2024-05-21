<script lang="ts">
import { wzTreeStore } from 'store/wzTree';
import WzTreeNode from './WzTreeNode.svelte';
import { isInitialized } from 'store/wz';
</script>


{#if $isInitialized}
  <ul class="tree-container">
    {#each ($wzTreeStore?.children || []) as child}
      <li>
        <WzTreeNode path={child.info.name} />
      </li>
    {/each}
  </ul>
{/if}
