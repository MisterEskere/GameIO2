<script lang="ts">
  import { onMount } from 'svelte';

  let id: Number = 0;

  onMount(() => {
    id = new URLSearchParams(window.location.search).get('id');
  });
</script>

<h1>Game ID: {id}</h1>