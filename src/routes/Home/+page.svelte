<script lang="ts">
  import Menu from "../Menu.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { writable } from 'svelte/store';

  // Interface for the game object and telated search response for displaying the games
  interface Game {
    id: number;
    name: string;
    slug: string;
    background_image: string;
  }

  // Name of the game to search for binded to the input field of the search bar
  let game_name = "";

  // Assuming search_response is a Svelte store
  let search_response = writable<Game[]>([]);

  // At the start of the page, we will call the games_list_invoke function with an empty string
  // This will return the popular games
  onMount(() => {
    games_list_invoke("");
  });

  // Functin used to search for games, it will be called when the search button is clicked
  async function games_list_invoke(game_name: string) {
    let search_response_list = await invoke("games_list", { gameName: game_name }) as Game[];
    search_response.set(search_response_list);
  }

  // Update game
</script>

<main>
  <Menu />

  <div class="container">
    <h1>Game Search</h1>

    <!-- Search bar for searching for games -->
    <input type="text" placeholder="Search for a game" bind:value={game_name} />

    <!-- Search button to search for games -->
    <button on:click={() => games_list_invoke(game_name)}>Search</button>

    <!-- Grid for displaying the games -->
    <div class="game-grid">
      {#each $search_response as item} <!-- Note the $ sign to access store value -->
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

  input,
  button {
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
