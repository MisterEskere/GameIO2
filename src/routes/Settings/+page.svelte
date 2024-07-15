<script lang="ts">
  import Menu from '../Menu.svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  async function get_api_key() {
    return await invoke('get_api_key');
  }

  async function set_api_key(api_key: string) {
    return await invoke('set_api_key', {apiKey: api_key});
  }
</script>

<main>
  <!-- Import the menu-->
  <Menu />
  
  <!-- Main content -->
  <div class="container">
    <h1>Settings</h1>

    <!-- API Key Input -->
    <div>
      <label for="api-key">API Key:</label>
      <input type="text" id="api-key" placeholder="Enter your API key" />
    </div>

    <!-- Save button -->
    <button>Save</button>

  </div>
</main>

<style>
  main {
    display: flex;
  }

  .container {
    flex: 1;
    padding: 20px;
  }

  input {
    padding: 10px;
    margin-right: 10px;
    margin-bottom: 20px; /* Added margin-bottom for spacing */
  }

  button {
    padding: 10px;
    background-color: #333;
    color: white;
    border: none;
  }

  button:hover {
    background-color: #555;
  }

  button:active {
    background-color: #777;
  }

</style>