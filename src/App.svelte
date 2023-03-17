<script lang="ts">
  import Editor from "./lib/Editor/Editor.svelte";
  import Welcome from "./lib/Startup/Welcome.svelte";
  import type { OpenWorkspaceEvent } from "./typescript/events";
  import { updatePath } from "./typescript/wrapper";

  let currentPath: string = null;
  let startupScreen = true;

  $: startupScreen = currentPath == null;

  async function handleOpenWorkspace(e: CustomEvent<OpenWorkspaceEvent>) {
    currentPath = e.detail.path;
    await updatePath(e.detail);
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
