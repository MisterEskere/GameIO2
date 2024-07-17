<script lang="ts">
  import Menu from "../Menu.svelte";
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/tauri";
  import Download from "../Download.svelte";

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

  let showDownload = false;

  function toggleDownload() {
    showDownload = !showDownload;
  }
</script>

<style>

</style>

<main>
  <Menu />

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
    <button on:click={toggleDownload}>Download</button>
    {#if showDownload}
    <button class="overlay" on:click={toggleDownload} type="button">Download</button>
      <div class="download-modal">
        <Download slug={game_details.slug} />
      </div>
    {/if}
  </div>
</main>