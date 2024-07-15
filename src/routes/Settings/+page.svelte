<script lang="ts">
  import Menu from '../Menu.svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  // At the start of the page, we will call the get_api_key_invoke function
  // This will return the API key
  onMount(() => {
    get_api_key_invoke();
  });

  // Function used to get the API key
  async function get_api_key_invoke() {
    api_key = await invoke('get_api_key');
  }

  // Function used to set the API key
  async function set_api_key_invoke(api_key: string) {
    await invoke('set_api_key', { apiKey: api_key });
    await get_api_key_invoke();
  }

  // api_key variable binded to the input field of the API key
  let api_key = '';

</script>

<main>
  <!-- Import the menu-->
  <Menu />
  
  <!-- Main content -->
  <div class="container">
    <h1>Settings</h1>

    <!-- API Key Input -->
    <input type="text" placeholder="API Key" bind:value={api_key} />


    <!-- Save button -->
    <button on:click={() => set_api_key_invoke(api_key)}>Save</button>

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