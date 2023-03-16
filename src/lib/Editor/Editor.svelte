<script lang="ts">
  import StoryInput from "./StoryInput.svelte";
  import type { StoryBlock } from "src/typescript/interfaces";
  import Connector from "./Connector.svelte";
  import { onMount } from "svelte";
  import type { UpdateConnectionLinesEvent } from "src/typescript/events";
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    open as openFileDialog,
    save as saveFileDialog,
  } from "@tauri-apps/api/dialog";

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

  function addNew() {
    let newData = storyInputData();
    storyBlocks = [...storyBlocks, newData];
  }

  function storyInputData() {
    return {
      top: windowHeight / 2 - translationY,
      left: windowWidth / 2 - translationX,
      connections: [],
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

  async function testButton() {
    // translationX = translationX == 25 ? 0 : 25;
    // translationY = translationY == 25 ? 0 : 25;
    invoke("save_file", { storyBlocks: storyBlocks }).then((data: string) => {
      console.log(data);
      console.log(JSON.parse(data));
    });

    // const a = await saveFileDialog();
    // console.log(a);
  }

  function updatedConnectionLines(e: CustomEvent<UpdateConnectionLinesEvent>) {
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

  #rect {
    background-color: green;
    width: 10px;
    height: 10px;
    top: 50%;
    left: 50%;
    position: relative;
    transform: translate(-50%, -50%);
  }

  svg,
  path {
    position: absolute;
    width: 100vw;
    height: 100vh;
  }
</style>
