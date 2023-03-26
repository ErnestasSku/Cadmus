<script lang="ts">
  import Editor from "./lib/Editor/Editor.svelte";
  import Welcome from "./lib/Startup/Welcome.svelte";
  import type { OpenWorkspaceEvent } from "./typescript/events";
  import { loadFile, updatePath } from "./typescript/wrapper";

  let currentPath: string = null;
  let startupScreen = true;

  $: startupScreen = currentPath == null;

  async function handleOpenWorkspace(
    e: CustomEvent<OpenWorkspaceEvent>
  ): Promise<void> {
    currentPath = e.detail.path;
    await updatePath(e.detail);

    if (!e.detail.new) {
      let a = await loadFile(e.detail.path).catch((err) =>
        console.log("Err: ", err)
      );
      console.log("Correct ", a);
    }
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
