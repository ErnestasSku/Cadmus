<script lang="ts">
  export let index: number;
  export let empty: boolean = true;
  export let pathLabel: string;
  export let pathDescription: string;
  export let translationX: number;
  export let translationY: number;

  let dot;

  $: empty = pathLabel === "" && pathDescription === "";

  function onDotMouseDown() {}
  function onMouseMove() {}
  function onMouseUp() {}
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
