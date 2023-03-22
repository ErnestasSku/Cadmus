<script lang="ts">
  import { fade } from "svelte/transition";
  import Waves from "./Waves.svelte";
  import {
    open as openFileDialog,
    save as saveFileDialog,
  } from "@tauri-apps/api/dialog";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  async function newStory() {
    const path = await saveFileDialog({
      filters: [{ extensions: ["cadmus"], name: "Story File" }],
    });

    if (typeof path === "string") {
      dispatch("openWorkspace", {
        new: true,
        path: path,
      });
    }

    console.log(path);
  }

  function openExisting() {}
</script>

<main class="container h-full">
  <div class="flex flex-col items-center">
    <div class="h-16" />
    <h1 class="text-center text-6xl mb-16 text-lime" style="color: #36D399;">
      Cadmus
    </h1>
    <button on:click={newStory} class="w-max m-5 button-pop-out"
      >New story</button
    >
    <button on:click={openExisting} class="w-max m-5 button-pop-out"
      >Open existing story</button
    >

    <div class="fixed footer-notice">
      *This is an early implementation of the UI. Everything is subject to
      change
    </div>
  </div>
</main>

<style lang="postcss">
  button {
    &:hover {
      filter: drop-shadow(0 0 2em #36d399);
      color: #36d399;
    }
  }
  .button-pop-out {
    @apply scale-100 hover:scale-110 duration-300 ease-linear transition-all;
  }

  .footer-notice {
    bottom: 50px;
  }
</style>
