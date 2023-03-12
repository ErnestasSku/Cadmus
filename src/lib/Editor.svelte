<script lang="ts">
  import StoryInput from "./StoryInput.svelte";

  let storyBlocks = [];
  let connections = [];

  let moving: Boolean = false;
  let capturedMouse: Boolean = false;
  let translationX: number = 0;
  let translationY: number = 0;

  let windowWidth: number;
  let windowHeight: number;

  function addNew() {
    let newData = storyInputData();
    storyBlocks = [...storyBlocks, newData];
  }

  function storyInputData() {
    return {
      top: windowHeight / 2 - translationY,
      left: windowWidth / 2 - translationX,
      index: storyBlocks.length,
    };
  }

  const clamp = (num: number, min: number, max: number) =>
    Math.min(Math.max(num, min), max);

  function onMouseDown() {
    if (!capturedMouse) {
      moving = true;
    }
  }

  function onMouseMove(e: MouseEvent) {
    if (moving) {
      translationX += e.movementX / scale;
      translationY += e.movementY / scale;
    }
  }

  function onMouseUp() {
    moving = false;
  }

  let scale: number = 1;
  function onMouseWheel(e: WheelEvent) {
    // scale = clamp(scale + e.deltaY / 500, 0.5, 10);
  }

  function captureMouse() {
    capturedMouse = true;
  }

  function releaseMouse() {
    capturedMouse = false;
  }
</script>

<main>
  <div id="top-section">
    <button class="btn btn-accent" on:click={addNew}>Add new story</button>
  </div>

  <div id="canvas" on:mousedown={onMouseDown} style="--scale: {scale}">
    {#each storyBlocks as storyBlock}
      <!-- {...storyBlock} -->
      <StoryInput
        bind:top={storyBlock.top}
        bind:left={storyBlock.left}
        bind:index={storyBlock.index}
        bind:connections={storyBlock.connections}
        {translationX}
        {translationY}
        on:captureMouse={captureMouse}
        on:releaseMouse={releaseMouse}
      />
    {/each}
  </div>
</main>

<svelte:window
  bind:innerHeight={windowHeight}
  bind:innerWidth={windowWidth}
  on:mouseup={onMouseUp}
  on:mousemove={onMouseMove}
  on:wheel={onMouseWheel}
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
    transform: scale(calc(var(--scale) * 1));
    overflow: hidden;
  }

  #top-section {
    background-color: black;
    width: 100vw;
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
