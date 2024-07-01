<script lang="ts">
  import Menu from '../Menu.svelte';

</script>

<main>
  <!-- Import the menu-->
  <Menu />
  
  <!-- Main content -->
  <div class="container">
    <h1>Game Search</h1>

    <input type="text" placeholder="Search for a game" />
    <button>Search</button>

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