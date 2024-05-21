<script lang="typescript">
import { init, getChildInfo } from 'tauri-plugin-wz-reader-api';
import { open } from '@tauri-apps/plugin-dialog';

import { isInitialized } from 'store/wz';
import { wzTreeStore, fromWzNodeInfo } from 'store/wzTree';

let message = '';

async function selectWzFile() {
  const file = await open({
    multiple: false,
    directory: false,
    filters: [{ name: 'Base', extensions: ['wz'] }],
  });
  const path = file?.path;

  try {
    if (path) {
      await init({ path });

      let child = await getChildInfo({ path: '' });

      wzTreeStore.set({
        info: {
          name: 'Base',
          type: 'Root',
          hasChild: child.length > 0,
        },
        children: child
          .sort((a, b) => (a.name > b.name ? 1 : -1))
          .map((child) => fromWzNodeInfo(child)),
      });
      isInitialized.set(true);
    }
  } catch (error) {
    console.error(error);
    isInitialized.set(false);
    message = JSON.stringify(error);
  }
}
</script>

<div class="row">
  <button class="button" on:click={selectWzFile}>init from Base.wz</button>
  <span>{message}</span>
</div>