<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/tauri";
  import { writable } from 'svelte/store';

  export let slug: string;
  let loading = true; // Step 1: Initialize loading state

  onMount(() => {
    invoke_get_torrents(slug);
  });

  // torrents store, list of lists
  type Torrent = [string, string]; // Assuming both elements are strings
  let torrents = writable<Torrent[]>([]);


  async function invoke_get_torrents(slug: string) {
    const result = await invoke("get_torrents", { gameName: slug });
    // Assert that result is of type Torrent[]
    torrents.set(result as Torrent[]);
    console.log($torrents); // Assuming you have a way to access the store value directly
  }

  async function invoke_download_torrent(link: Torrent) {
    console.log("Downloading torrent: ", link);
    await invoke("download_torrent", { name: link[0], game: link[0], url: link[1], uploader: "unknown" });
  }
</script>

<main>
  <h1>Download</h1>
  <p>Download torrents</p>
  <div class="torrnt-list">
    {#each $torrents as torrent}
      <div class="torrent-card">
        <a href={torrent[1]}>
          <button on:click={() => invoke_download_torrent(torrent)}>
            {torrent[0]}
          </button>
        </a>
      </div>
    {/each}
  </div>
</main>

<style>
</style>