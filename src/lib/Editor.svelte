<script lang="ts">
  import StoryInput from "./StoryInput.svelte";

  function addNew() {
    new StoryInput({
      target: document.getElementById("canvas"),
    });
  }

  let moving: Boolean = false;
  let x: number = 0;
  let y: number = 0;

  const clamp = (num: number, min: number, max: number) =>
    Math.min(Math.max(num, min), max);

  function onMouseDown() {
    // moving = true;
  }

  function onMouseMove(e: MouseEvent) {
    if (moving) {
      x += e.movementX / scale;
      y += e.movementY / scale;
    }
  }

  function onMouseUp() {
    moving = false;
  }

  let scale: number = 1;
  function onMouseWheel(e: WheelEvent) {
    // scale = clamp((scale + e.deltaY / 500), 0.5, 10);
  }
</script>

<main>
  <div id="top-section">
    <button class="btn btn-accent" on:click={addNew}>Press meh</button>
  </div>

  <div
    id="canvas"
    on:mousedown={onMouseDown}
    style="--x: {x}; --y: {y}; --scale: {scale}"
  >
    <!-- <div id="rect"></div> -->
  </div>
</main>

<svelte:window
  on:mouseup={onMouseUp}
  on:mousemove={onMouseMove}
  on:mousewheel={onMouseWheel}
/>

<style>
  main {
    width: 100%;
    height: 100%;
    position: fixed;
  }

  #canvas {
    position: fixed;
    height: 100vh;
    width: 100vw;
    transform: scale(calc(var(--scale) * 1))
      translate(calc(var(--x) * 1px), calc(var(--y) * 1px));
    overflow: hidden;
  }

  #top-section {
    background-color: black;
    width: 100vw;
  }

  button {
    /* border: 1px solid white; */
  }

  #rect {
    background-color: green;
    width: 10px;
    height: 10px;
    top: 50%;
    left: 50%;
    position: relative;
    transform: translate(-50%, -50%);
  }
</style>
