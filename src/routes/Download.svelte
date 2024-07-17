<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/tauri";

  onMount(() => {
    const slug = new URLSearchParams(window.location.search).get('slug');

  });

  interface torrents {
    "name": string,
    "link": string
  }
  let torrents = [] as torrents[];

  async function invoke_get_torrents(slug: string) {
    const torrents = await invoke("get_torrents", { slug: slug });
  }

</script>

<main>
  <h1>Download</h1>
  <p>Download torrents</p>
  <ul>
    {#each torrents as torrent}
      <li><a href={torrent.link}>{torrent.name}</a></li>
    {/each}
  </ul>
</main>

<style>
</style>
