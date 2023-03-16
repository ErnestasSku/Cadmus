<script lang="ts">
  import Editor from "./lib/Editor/Editor.svelte";
  import Welcome from "./lib/Startup/Welcome.svelte";

  let startupScreen = true;

  function handleOpenWorkspace(e: CustomEvent) {
    startupScreen = false;
    //TODO
    console.log(e);
  }
</script>

<div>
  {#if startupScreen}
    <Welcome on:openWorkspace={handleOpenWorkspace} />
  {:else}
    <Editor />
  {/if}
</div>

<style>
</style>
