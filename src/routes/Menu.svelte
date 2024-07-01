<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
  
  
    // Define the menu structure
    const menuOptions = [
      { label: 'Home', href: '/Home.svelte' },
      { label: 'About', href: '#' },
      { label: 'Services', href: '#' },
      { label: 'Portfolio', href: '#' },
      { label: 'Contact', href: '#' },
    ];
  </script>
  
  <div class="app-layout">
    <!-- Sidebar menu -->
    <aside class="sidebar">
      <ul>
        {#each menuOptions as { label, href }}
          <li><a href={href}>{label}</a></li>
        {/each}
      </ul>
    </aside>
  
    <!-- Main content -->
    <div class="container">
      <!-- Existing content goes here -->
    </div>
  </div>
  
  <style>
  
  </style>