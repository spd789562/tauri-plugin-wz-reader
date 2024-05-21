<script lang="ts">
import { previewStore } from 'store/preview';
import { apiClient } from 'store/wz';
import { WzType } from 'tauri-plugin-wz-reader-api';

let data = {};

$: {
  try {
    data = JSON.parse($previewStore?.data || '{}');
  } catch (error) {
    console.error(error);
    data = {};
  }
}
$: isPng = $previewStore?.type === WzType.Png;
$: isVec2 = $previewStore?.type === WzType.Vector;
$: isSound = $previewStore?.type === WzType.Sound;
</script>

{#if $previewStore}
  <div class="flex-1 preview-container">
    <span>path: {$previewStore?.path}</span>
    <h3>Type: {$previewStore?.type}</h3>
    {#if data?.data !== undefined}
      {#if isPng}
        <div>
          width: {data?.data.width}
        </div>
        <div>
          height: {data?.data.height}
        </div>
        <div>
          image:
        </div>
        <img src={$apiClient?.get_png_url($previewStore?.path)} alt={$previewStore?.path} />
      {:else if isVec2}
        <div>
          x: {data?.data[0]}
        </div>
        <div>
          y: {data?.data[1]}
        </div>
      {:else if isSound}
          <div>
            duration: {Math.floor(data?.data.duration / 1000 / 60)}m {Math.floor(data?.data.duration / 1000 % 60)}s
          </div>
          <audio controls>
            <source src={$apiClient?.get_sound_url($previewStore?.path)} type="audio/mpeg" />
            Your browser does not support the audio element.
          </audio>
      {:else}
        <div>Value: {data?.data}</div>
      {/if}
    {/if}
  </div>
{/if}