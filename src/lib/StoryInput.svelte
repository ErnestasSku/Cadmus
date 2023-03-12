<script lang="ts">
  import ConnectionInput from "./ConnectionInput.svelte";
  import { createEventDispatcher } from "svelte";
  import { activeInputId } from "../stores/activeStoryInput";

  export let connections = [{}];
  export let top: number;
  export let left: number;
  export let index: number;
  export let translationX: number;
  export let translationY: number;

  let add: boolean = false;
  let active: boolean = false;
  let mouseCaptured: boolean = false;
  let moving: boolean = false;
  let cursor: string = "default";
  let headerColor: string = "#191d24";
  let textColor: string = "white";
  let zIndex: number = 0;

  const dispatch = createEventDispatcher();

  activeInputId.subscribe((value) => {
    active = value == index;
  });

  function addNewConnection(e: MouseEvent) {
    //TODO add logic to add only if it doesn't have any content
    connections = [...connections, {}];
  }

  function onmousedown(e: MouseEvent) {
    if (!mouseCaptured) {
      moving = true;
    }
    dispatch("captureMouse", {});
    if (!active) {
      activeInputId.set(index);
    }
  }

  function onmousemove(e: MouseEvent) {
    if (moving) {
      top += e.movementY;
      left += e.movementX;
    }
  }

  function onmouseup(e: MouseEvent) {
    moving = false;
    dispatch("releaseMouse", {});
  }

  function captureMouse(e: MouseEvent) {
    mouseCaptured = true;
  }

  function releaseMouse(e: MouseEvent) {
    mouseCaptured = false;
  }

  $: cursor = moving ? "move" : "default";
  $: headerColor = active ? "#36d399" : "#2a303c";
  $: textColor = active ? "black" : "white";
  $: zIndex = active ? 1 : 0;
</script>

<main
  style="--top: {top}; --left: {left}; --cursor: {cursor}; --translateX: {translationX}; --translateY: {translationY}; --zIndex: {zIndex}"
  on:mousedown={onmousedown}
>
  <div
    id="storyHeader"
    style="--headerColor: {headerColor}; --textColor: {textColor}"
  >
    <p class="text-center">Story block</p>
  </div>

  <div id="storyBody">
    <input
      class="input mt-5 w-full"
      placeholder="Story ID"
      on:mouseenter={captureMouse}
      on:mouseleave={releaseMouse}
    />
    <textarea
      class="textarea mt-1 w-full "
      placeholder="Story content"
      on:mouseenter={captureMouse}
      on:mouseleave={releaseMouse}
    />
    <hr class="line" />

    <div on:mouseenter={captureMouse} on:mouseleave={releaseMouse}>
      {#each connections as connection}
        <ConnectionInput {connection} />
      {/each}
    </div>

    <button
      on:click={addNewConnection}
      id="newConnection"
      class="btn btn-circle btn-primary "
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        ><path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M6 18L18 6M6 6l12 12"
        />
      </svg>
    </button>
  </div>
</main>

<svelte:window on:mousemove={onmousemove} on:mouseup={onmouseup} />

<style>
  div {
  }

  main {
    position: absolute;
    top: calc(var(--top) * 1px);
    left: calc(var(--left) * 1px);
    transform: translate(
      calc(var(--translateX) * 1px),
      calc(var(--translateY) * 1px)
    );
    z-index: var(--zIndex);

    width: 250px;
    height: auto;
    border: 2px solid #36d399;
    border-radius: 5px;

    cursor: var(--cursor);
    user-select: none;

    background-color: #191d24;
  }

  input:focus,
  textarea:focus {
    outline: 2px solid #f472b6;
    outline-offset: 2px;
  }

  .line {
    content: "";
    border-color: #36d399;
  }

  #newConnection {
    top: 0%;
    left: 50%;
    position: relative;
    transform: translate(-50%, 50%) !important;
  }

  #storyHeader {
    background-color: var(--headerColor);
    color: var(--textColor);
  }

  #storyBody {
    padding: 5px;
  }
</style>
