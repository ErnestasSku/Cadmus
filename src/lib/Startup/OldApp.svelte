<script lang="ts">
  // import { fade } from "svelte/types/runtime/transition";
  // import { flip } from 'svelte/animate';
  import { fade } from "svelte/transition";
  // import Waves from "./lib/Editor/Waves.svelte";

  let clientHeight;
  let innerHeight;

  function newStory() {
    // let a =  document.getElementsByTagName("")[0];
    let a = document.getElementById("app");
    // a.replaceChild
    for (let b in a.children) {
      a.removeChild(a.children[b]);
    }
    // a.classList.add("test");
  }

  function openExisting() {
    let a = document.getElementsByTagName("html")[0];
    a.classList.remove("test");
  }
</script>

<main class="container h-full" transition:fade>
  <div class="flex flex-col items-center">
    <div class="h-16" />
    <h1 class="text-center text-6xl mb-16 text-lime" style="color: #36D399;">
      Cadmus
    </h1>
    <button on:click={newStory} class="w-max m-1 button-pop-out"
      >New story</button
    >
    <button on:click={openExisting} class="w-max m-1 button-pop-out"
      >Open existing story</button
    >

    <!-- <Waves /> -->
  </div>
</main>

<style lang="postcss">
  button:hover {
    filter: drop-shadow(0 0 2em #36d399);
    color: #36d399;
  }
  .button-pop-out {
    @apply scale-100 hover:scale-110 duration-300 ease-linear transition-all;
  }
</style>
