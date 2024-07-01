<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }

  // Define the menu structure
  const menuOptions = [
    { label: 'Home', href: '#' },
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

  /* Existing styles... */
  .app-layout {
  display: flex;
  height: 100vh; /* Ensure the layout container fills the viewport height */
}

.sidebar {
  width: 200px;
  background-color: #f0f0f0;
  padding: 20px;
  height: 100vh; /* Make sidebar take all vertical space */
  box-sizing: border-box; /* Include padding in the height calculation */
}

.sidebar ul {
  list-style: none;
  padding: 0;
  margin: 0; /* Remove default margin */
  height: 100%; /* Optional: Ensure the list takes the full height of the sidebar */
  display: flex;
  flex-direction: column; /* Stack items vertically */
}

.sidebar ul li a {
  display: block;
  padding: 10px;
  text-decoration: none;
  color: #333;
}

.sidebar ul li a:hover {
  background-color: #ddd;
}

.container {
  flex-grow: 1;
  padding-left: 20px; /* Adjust padding to accommodate the sidebar */
}

</style>