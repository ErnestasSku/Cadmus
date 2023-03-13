<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import Connector from "./Connector.svelte";

  export let index: number;
  export let empty: boolean = true;
  export let pathLabel: string;
  export let pathDescription: string;
  export let translationX: number;
  export let translationY: number;

  export let startX = 0;
  export let startY = 0;
  export let endX = 0;
  export let endY = 0;
  // let translationX = 0;
  // let translationY = 0;

  let dot;
  let drawLine: boolean = false;
  let connection;

  const dispatch = createEventDispatcher();

  $: empty = pathLabel === "" && pathDescription === "";

  function onDotMouseDown() {
    drawLine = true;

    // dispatch("drawLineStarted", {
    //   id: id,
    // });
  }
  function onMouseMove(e: MouseEvent) {
    if (drawLine) {
      endX = e.clientX;
      endY = e.clientY;
    }
  }
  function onMouseUp() {
    drawLine = false;
  }

  onMount(() => {
    let rect = dot.getBoundingClientRect();

    startX = rect.top;
    startY = rect.left;
  });
</script>

<main class="mt-1">
  <div>
    <input class="input" placeholder="Label" bind:value={pathLabel} />
    <vl />
    <input
      class="input"
      placeholder="Description"
      bind:value={pathDescription}
    />
    <vl />
    <span class="dot" bind:this={dot} on:mousedown={onDotMouseDown} />
  </div>
</main>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} />

<style>
  div {
    display: flex;
    justify-content: center;
  }

  input {
    width: 48%;
  }

  input:focus,
  textarea:focus {
    outline: 2px solid #f472b6;
    outline-offset: 2px;
  }

  vl {
    width: 2%;
  }

  .dot {
    height: 25px;
    width: 25px;
    background-color: #22d3ee;
    border-radius: 50%;
    margin-top: 0.5em;
  }

  .dot:hover {
    @apply scale-100 hover:scale-125 duration-300 ease-linear transition-all;
  }
</style>
