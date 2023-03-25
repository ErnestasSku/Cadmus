<script lang="ts">
  import { onMount } from "svelte";
  import { createEventDispatcher } from "svelte";

  export let index: number;
  export let connected: boolean = false;
  export let empty: boolean = true;
  export let pathLabel: string;
  export let pathDescription: string;
  export let translationX: number;
  export let translationY: number;
  export let startX = 0;
  export let startY = 0;
  export let endX = 0;
  export let endY = 0;
  export let visible: boolean = false;

  let dot: HTMLElement;
  let linkingElement: HTMLElement;
  let dotSize: number = 25;
  let dotSizeStyle: string = "25px";
  let drawLine: boolean = false;

  const dispatch = createEventDispatcher();

  $: empty = pathLabel === "" && pathDescription === "" && !connected;
  $: dotSizeStyle = dotSize.toString() + "px";
  $: visible = drawLine || connected;

  function onDotMouseDown(e: MouseEvent) {
    drawLine = true;
    connected = false;

    endX = e.clientX - translationX;
    endY = e.clientY - translationY;
  }

  function onMouseMove(e: MouseEvent) {
    if (drawLine) {
      endX = e.clientX - translationX;
      endY = e.clientY - translationY;

      checkForLinks(e);
    }
  }

  function checkForLinks(e: MouseEvent) {
    let target = <Element>e.target;
    let element = <HTMLElement>target.closest(".story");
    if (element != null && element != linkingElement) {
      linkingElement = element;
      dispatch("link", {
        target: element,
        link: index,
      });
    }

    if (element == null && linkingElement != null) {
      linkingElement = null;
      dispatch("linkLost", {});
    }
  }

  function onMouseUp() {
    drawLine = false;
  }

  onMount(() => {
    let rect = dot.getBoundingClientRect();

    startX = rect.left + dotSize / 2 - translationX;
    startY = rect.top + dotSize / 2 - translationY;
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
    <span
      class="dot"
      bind:this={dot}
      on:mousedown={onDotMouseDown}
      style="--dotSize: {dotSizeStyle};"
    />
  </div>
</main>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} />

<style lang="postcss">
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
    height: var(--dotSize);
    width: var(--dotSize);
    background-color: #22d3ee;
    border-radius: 50%;
    margin-top: 0.5em;

    &:hover {
      @apply scale-100 hover:scale-125 duration-300 ease-linear transition-all;
    }
  }
</style>
