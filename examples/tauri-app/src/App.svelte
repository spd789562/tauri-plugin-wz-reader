<script lang="ts">
import FileSelect from './components/FileSelect.svelte';
import WzTreeRoot from './components/WzTreeRoot.svelte';
import Preview from './components/Preview.svelte';
import { apiClient } from './store/wz';
import { initAPIClient } from 'tauri-plugin-wz-reader-api';

async function init() {
  apiClient.set(await initAPIClient());
}

init();
</script>

<main class="container">
  <div>simple wz browser svelte</div>
  <FileSelect />
  <div class="row">
    <div class="flex-1">
      <WzTreeRoot />
    </div>
    <Preview />
  </div>
</main>
