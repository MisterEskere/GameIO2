<script lang="ts">
  import Menu from '../Menu.svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  onMount(() => {
    games_list_invoke("");
  });

  let inputValue = '';

  interface Game {
    "id": number,
    "name": string,
    "slug": string,
    "background_image": string
  }
  let searchResponse: Game[] = [];

  async function games_list_invoke(game_name: string) {
    searchResponse = await invoke('games_list', {gameName: game_name});
  }

</script>

<main>
  <Menu />
  
  <div class="container">
    <h1>Game Search</h1>

    <input type="text" placeholder="Search for a game" bind:value={inputValue} />
  
    <button on:click={() => games_list_invoke(inputValue)}>Search</button>

    <div class="game-grid">
      {#each searchResponse as item}
        <div class="game-card">
          <a href="/Game?id={item.id}">
            <img class="game-image" src={item.background_image} alt={item.name} />
            <div class="game-info">
              <h2 class="game-title">{item.name}</h2>
            </div>
          </a>
        </div>
      {/each}
    </div>
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

  input, button {
    padding: 10px;
    margin: 5px 0;
  }

  button {
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

  .game-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 20px;
    margin-top: 20px;
  }

  .game-card {
    display: flex;
    flex-direction: column;
    background-color: #202020;
    border-radius: 8px;
    overflow: hidden;
    cursor: pointer;
    transition: transform 0.2s;
  }

  .game-card:hover {
    transform: scale(1.05);
  }

  .game-image {
    width: 100%;
    height: 140px;
    object-fit: cover;
  }

  .game-info {
    padding: 10px;
    text-align: center;
  }

  .game-title {
    margin: 10px 0;
    color: #fff;
  }
</style>