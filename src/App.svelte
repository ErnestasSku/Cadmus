<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Editor from "./lib/Editor/Editor.svelte";
  import Welcome from "./lib/Startup/Welcome.svelte";

  let startupScreen = true;

  function handleOpenWorkspace(e: CustomEvent) {
    startupScreen = false;
    console.log(JSON.stringify(e.detail));
    invoke("update_path", { newPath: e.detail.path });
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
