<script>
  import { invoke } from "@tauri-apps/api/core"
  import { init } from 'tauri-plugin-wz-reader-api'

  let path = "";
  let message = "";

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    try {
      await init({ path });
      message = "success"
    } catch (e) {
      message = "fail"
    }
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter wz path" bind:value={path} />
    <button on:click={greet}>
      Init Wz
    </button>
  </div>
  <p>{message}</p>
</div>

