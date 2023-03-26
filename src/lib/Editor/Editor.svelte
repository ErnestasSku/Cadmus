<script lang="ts">
  import StoryInput from "./StoryInput.svelte";
  import type { StoryBlock } from "src/typescript/interfaces";
  import Connector from "./Connector.svelte";
  import { onMount } from "svelte";
  import type { UpdateConnectionLinesEvent } from "src/typescript/events";
  import { saveFile } from "../../typescript/wrapper";

  let storyBlocks: StoryBlock[] = [];
  let moving: Boolean = false;
  let capturedMouse: Boolean = false;
  let translationX: number = 0;
  let translationY: number = 0;

  let windowWidth: number;
  let windowHeight: number;
  let canvas: HTMLElement;
  let canvasOffsetX: number;
  let canvasOffsetY: number;

  function addNew(): void {
    let newData = storyInputData();
    storyBlocks = [...storyBlocks, newData];
  }

  function storyInputData(): StoryBlock {
    return {
      top: windowHeight / 2 - translationY,
      left: windowWidth / 2 - translationX,
      connections: [],
      index: storyBlocks.length,
    };
  }

  const clamp = (num: number, min: number, max: number) =>
    Math.min(Math.max(num, min), max);

  function onMouseDown(): void {
    if (!capturedMouse) {
      moving = true;
    }
  }

  function onMouseMove(e: MouseEvent): void {
    if (moving) {
      translationX += e.movementX / scale;
      translationY += e.movementY / scale;
    }
  }

  function onMouseUp(): void {
    moving = false;
  }

  let scale: number = 1;
  function onMouseWheel(e: WheelEvent) {
    // scale = clamp(scale + e.deltaY / 500, 0.5, 10);
  }

  function captureMouse(): void {
    capturedMouse = true;
  }

  function releaseMouse(): void {
    capturedMouse = false;
  }

  async function testButton() {
    await saveFile(storyBlocks);
  }

  function updatedConnectionLines(
    e: CustomEvent<UpdateConnectionLinesEvent>
  ): void {
    let changedElement = e.detail.storyElementId;
    let changedLeft = e.detail.left;
    let changedTop = e.detail.top;

    for (let story of storyBlocks) {
      for (let connection of story.connections) {
        if (connection.connectedElementId == changedElement) {
          // When using top and left from Editor and Connector itself, the values differ based on offset.
          // Values from connector do not have offset value
          connection.endX = changedLeft + canvasOffsetX;
          connection.endY = changedTop + canvasOffsetY;
        }
      }
    }
  }

  onMount(() => {
    canvasOffsetX = canvas.offsetLeft;
    canvasOffsetY = canvas.offsetTop;
  });
</script>

<main>
  <div id="top-section">
    <button class="btn btn-accent" on:click={addNew}>Add new story</button>
    <button class="btn btn-secondary" on:click={testButton}>Test button</button>
  </div>

  <div
    id="canvas"
    bind:this={canvas}
    on:mousedown={onMouseDown}
    style="--scale: {scale}"
  >
    {#each storyBlocks as storyBlock}
      <StoryInput
        bind:top={storyBlock.top}
        bind:left={storyBlock.left}
        bind:index={storyBlock.index}
        bind:connections={storyBlock.connections}
        {translationX}
        {translationY}
        on:captureMouse={captureMouse}
        on:releaseMouse={releaseMouse}
        on:updatedConnectionLines={updatedConnectionLines}
      />

      {#each storyBlock.connections as connection}
        <Connector
          startX={connection.startX}
          startY={connection.startY}
          endX={connection.endX}
          endY={connection.endY}
          visible={connection.visible}
          {translationX}
          {translationY}
          offsetX={canvasOffsetX}
          offsetY={canvasOffsetY}
        />
      {/each}
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
</style>
