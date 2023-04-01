<script lang="ts">
  import Editor from "./lib/Editor/Editor.svelte";
  import Welcome from "./lib/Startup/Welcome.svelte";
  import type { OpenWorkspaceEvent } from "./typescript/events";
  import type { StoryBlock } from "./typescript/interfaces";
  import { loadFile, updatePath } from "./typescript/wrapper";

  let currentPath: string = null;
  let startupScreen = true;
  let storyBlocks: StoryBlock[];

  $: startupScreen = currentPath == null;

  async function handleOpenWorkspace(
    e: CustomEvent<OpenWorkspaceEvent>
  ): Promise<void> {
    await updatePath(e.detail);

    if (!e.detail.new) {
      let data = await loadFile(e.detail.path).catch((err) => {
        console.log("Err: ", err);
        return [];
      });
      console.log("Correct ", data);
      storyBlocks = data;
    }
    currentPath = e.detail.path;
  }
</script>

<div>
  {#if startupScreen}
    <Welcome on:openWorkspace={handleOpenWorkspace} />
  {:else}
    <Editor {storyBlocks} />
  {/if}
</div>

<style>
</style>
