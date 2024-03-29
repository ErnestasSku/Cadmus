<script lang="ts">
  import ConnectionInput from "./ConnectionInput.svelte";
  import { createEventDispatcher } from "svelte";
  import {
    activeInputId,
    hoveredElement,
    hoveredElementId,
  } from "../../typescript/stores";
  import { onMount } from "svelte";
  import type { Connection } from "src/typescript/interfaces";
  import type { FoundLinkEvent } from "src/typescript/events";

  export let connections: Connection[] = [];
  export let top: number;
  export let left: number;
  export let index: number;
  export let translationX: number;
  export let translationY: number;
  export let initializing: boolean;
  export let storyId: string;
  export let storyContent: string;
  export let offsetX: number;
  export let offsetY: number;

  let StoryInput: HTMLElement;
  let add: boolean = false;
  let active: boolean = false;
  let linkHovered: boolean = false;
  let activeConnectionLine: number = null;
  let mouseCaptured: boolean = false;
  let moving: boolean = false;
  let cursor: string = "default";
  let headerColor: string = "#191d24";
  let textColor: string = "white";
  let zIndex: number = 0;
  let svgButtonRotation: string = "45deg";
  let clientWidth: number;
  let clientHeight: number;

  const dispatch = createEventDispatcher();

  $: cursor = moving ? "move" : "default";
  $: headerColor = active ? "#36d399" : "#2a303c";
  $: textColor = active ? "black" : "white";
  $: zIndex = active ? 1 : 0;
  $: active = $activeInputId == index;
  $: linkHovered = $hoveredElement === StoryInput;
  $: if (linkHovered) {
    hoveredElementId.set(index);
  }
  $: add = connections.find((element) => element.empty) == undefined;
  $: svgButtonRotation = add ? "45deg" : "0deg";
  // $: connections.forEach((elem) => (elem.startX = left));
  // $: connections.forEach((elem) => (elem.startY = top));

  function addNewConnection(e: MouseEvent) {
    let emptyIndex = connections.findIndex((element) => element.empty);
    if (emptyIndex === -1) {
      let newConnection = connectionData();

      connections = [...connections, newConnection];
    } else {
      connections.splice(emptyIndex, 1);
      connections = [...connections];
    }
  }

  function connectionData(): Connection {
    return {
      index: connections.length,
      connectedElementId: -1,
      empty: true,
      pathLabel: "",
      pathDescription: "",
      startX: 0,
      startY: 0,
      endX: 0,
      endY: 0,
      connected: false,
      visible: false,
    };
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

      for (let conn of connections) {
        conn.startX += e.movementX;
        conn.startY += e.movementY;
      }

      dispatch("updatedConnectionLines", {
        storyElementId: index,
        top: top,
        left: left,
      });
    }
  }

  function onmouseup(e: MouseEvent) {
    moving = false;
    dispatch("releaseMouse", {});

    if ($hoveredElement != null && activeConnectionLine != null) {
      let connectionIndex = connections.findIndex(
        (eleme) => eleme.index == activeConnectionLine
      );

      let canConnect = connections.every(
        (elem) => elem.connectedElementId != $hoveredElementId
      );

      if (connectionIndex !== -1 && canConnect) {
        connections[connectionIndex].endY =
          $hoveredElement.getBoundingClientRect().top - translationY;
        connections[connectionIndex].endX =
          $hoveredElement.getBoundingClientRect().left - translationX;
        connections[connectionIndex].connected = true;
        connections[connectionIndex].connectedElementId = $hoveredElementId;
      }

      hoveredElement.set(null);
      hoveredElementId.set(null);
      activeConnectionLine = null;
    }
  }

  function captureMouse(e: MouseEvent) {
    mouseCaptured = true;
  }

  function releaseMouse(e: MouseEvent) {
    mouseCaptured = false;
  }

  function adjustSize() {
    top = top - clientHeight / 2;
    left = left - clientWidth / 2;
  }

  function handleLink(e: CustomEvent<FoundLinkEvent>) {
    let foundElement: HTMLElement = e.detail.target;
    activeConnectionLine = e.detail.link;
    connections[activeConnectionLine].connectedElementId = -1;

    if (foundElement != StoryInput && foundElement != $hoveredElement) {
      hoveredElement.set(foundElement);
    }
  }

  function handleLinkLost(e: CustomEvent) {
    hoveredElement.set(null);
    activeConnectionLine = null;
  }

  onMount(() => {
    if (initializing) {
      adjustSize();
      initializing = false;
    }
  });
</script>

<main
  bind:clientHeight
  bind:clientWidth
  class="story"
  class:connecting={linkHovered}
  style="--top: {top}; --left: {left}; --cursor: {cursor}; --translateX: {translationX}; --translateY: {translationY}; --zIndex: {zIndex};"
  on:mousedown={onmousedown}
  bind:this={StoryInput}
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
      bind:value={storyId}
      on:mouseenter={captureMouse}
      on:mouseleave={releaseMouse}
    />
    <textarea
      class="textarea mt-1 w-full "
      placeholder="Story content"
      bind:value={storyContent}
      on:mouseenter={captureMouse}
      on:mouseleave={releaseMouse}
    />
    <hr class="line" />

    <div
      id="storyHeader"
      class="mt-1 mb-2"
      style="--headerColor: {headerColor}; --textColor: {textColor}"
    >
      <p class="text-center">Paths</p>
    </div>

    <div on:mouseenter={captureMouse} on:mouseleave={releaseMouse}>
      {#each connections as connection}
        <ConnectionInput
          index={connection.index}
          bind:empty={connection.empty}
          bind:connected={connection.connected}
          bind:pathLabel={connection.pathLabel}
          bind:pathDescription={connection.pathDescription}
          bind:startX={connection.startX}
          bind:startY={connection.startY}
          bind:endX={connection.endX}
          bind:endY={connection.endY}
          bind:visible={connection.visible}
          {translationX}
          {translationY}
          on:link={handleLink}
          on:linkLost={handleLinkLost}
        />
      {/each}
    </div>

    <button
      on:click={addNewConnection}
      id="newConnection"
      class="btn btn-circle btn-primary"
      style="--rotation: {svgButtonRotation}"
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
    transform: translate(-50%, 50%) rotate(var(--rotation)) !important;
  }

  #storyHeader {
    background-color: var(--headerColor);
    color: var(--textColor);
    border-radius: 0 0 5px 5px;
  }

  #storyBody {
    padding: 5px;
  }

  .connecting {
    filter: brightness(50%);
    border: 3px dashed white;
    border-radius: 5px;
  }
</style>
