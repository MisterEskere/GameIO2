<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
</script>

<div class="app-layout">
  <!-- Sidebar menu -->
  <aside class="sidebar">
    <ul>
      <li><a href="/Home">Home</a></li>
      <li><a href="/Library">Library</a></li>
      <li><a href="/Downloads">Downloads</a></li>
      <li><a href="/Settings">Settings</a></li>
      <li><a href="/About">About</a></li>
    </ul>
  </aside>

  <!-- Main content -->
  <div class="container">
    <!-- Existing content goes here -->
  </div>
</div>

<style>
</style>
