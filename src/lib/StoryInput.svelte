<script lang="ts">
  import ConnectionInput from "./ConnectionInput.svelte";

  export let connections = [{}];

  function addNewConnection(e: MouseEvent) {
    //TODO add logic to add only if it doesn't have any content
    connections = [...connections, {}];
  }
</script>

<main>
  <input class="input mt-1 w-full" placeholder="Story ID" />
  <textarea class="textarea mt-1 w-full " placeholder="Story content" />
  <hr class="line" />

  {#each connections as connection}
    <ConnectionInput conn={connection} />
  {/each}

  <button
    on:click={addNewConnection}
    id="newConnection"
    class="btn btn-circle btn-primary "
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-6 w-6"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      ><path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M6 18L18 6M6 6l12 12"
      /></svg
    >
  </button>
</main>

<style>
  div {
  }

  main {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);

    width: 250px;
    height: auto;
    border: 2px solid #36d399;
    border-radius: 5px;

    padding: 5px;
    background-color: #191d24;
  }

  input {
  }

  input:focus,
  textarea:focus {
    outline: 2px solid #f472b6;
    outline-offset: 2px;
  }

  .line {
    content: "";
    border-color: #36d399;
  }

  #newConnection {
    top: 0%;
    left: 50%;
    position: relative;
    transform: translate(-50%, 50%) !important;
  }
</style>
