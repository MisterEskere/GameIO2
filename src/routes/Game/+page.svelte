<script lang="ts">
  import Menu from '../Menu.svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  // value of the search input
  let inputValue = '';

  // response from the search
  interface Game {
    title: string;
    link: string;
  }
  let searchResponse: Game[] = [];

  // response from the game
  let gameResponse = '';

  async function fitgirl_game_invoke(games_page: string) {
    gameResponse = await invoke('fitgirl_game', {gamesPage: games_page});
  }
</script>

<main>
  <!-- Import the menu-->
  <Menu />
  
  <!-- Main content -->
  <div class="container">
    <h1>Game Search</h1>

    <input type="text" placeholder="Search for a game" bind:value={inputValue} />
  
    <button on:click={() => fitgirl_search_invoke(inputValue)}>Download</button>


    <!-- Create a new entry for each game and make it recall the game_detail_invoke-->
    
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