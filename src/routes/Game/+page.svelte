<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/tauri";

  onMount(() => {
    const idString = new URLSearchParams(window.location.search).get('id');
    const id = Number(idString);
    if (id) {
      game_details_invoke(id);
    }
});

  async function game_details_invoke(game_id: number) {
    game_details = await invoke("game_details", { gameId: game_id });
  }

  // Interface for the game object and telated search response for displaying the game details
  interface Game {
      "id": number,
      "slug": string,
      "name": string,
      "description": string,
      "background_image": string,
      "background_image_additional": string,
      "released": string,
      "genres": genres[],
  }

  // Interface for the genres object
  interface genres {
      "id": number,
      "name": string,
      "slug": string,
      "games_count": number,
      "image_background": string
  }

  let game_details: Game = {
      "id": 0,
      "slug": "",
      "name": "",
      "description": "",
      "background_image": "",
      "background_image_additional": "",
      "released": "",
      "genres": []
  };

</script>

<main>
  <div class="container">
    <div class="game-header">
      <h1>{game_details.name}</h1>
      <img src={game_details.background_image} alt={game_details.name} />
      <p class="game-released">Released: {game_details.released}</p>
    </div>
    <div class="game-description">
      <p>{game_details.description}</p>
    </div>
    <div class="game-genres">
      {#each game_details.genres as genre}
        <div class="genre-badge">{genre.name}</div>
      {/each}
    </div>
  </div>
</main>

<style>
  .container {
    max-width: 1200px;
    margin: auto;
    padding: 20px;
  }
  .game-header {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }
  .game-header img {
    max-width: 100%;
    height: auto;
    border-radius: 8px;
  }
  .game-released {
    margin-top: 10px;
    font-size: 18px;
    color: #888;
  }
  .game-description {
    margin-top: 20px;
  }
  .game-genres {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin-top: 20px;
  }
  .genre-badge {
    background-color: #007bff;
    color: white;
    padding: 5px 10px;
    border-radius: 20px;
    font-size: 14px;
  }
</style>