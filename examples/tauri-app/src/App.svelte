<script>
  import Greet from './lib/Greet.svelte'
  import { execute } from 'tauri-plugin-wz-reader-api'

	let response = ''

	function updateResponse(returnValue) {
    console.log(returnValue)
		response += `[${new Date().toLocaleTimeString()}]` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	}

	function _execute() {
		execute().then(updateResponse).catch(updateResponse)
	}
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>
  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
  </div>

  <div>
    <button on:click="{_execute}">Execute</button>
    <div>{@html response}</div>
  </div>

</main>
