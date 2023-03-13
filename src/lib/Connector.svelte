<script lang="ts">
  export let startX: number;
  export let startY: number;
  export let endX: number;
  export let endY: number;
  export let translationX: number;
  export let translationY: number;
  export let offsetX: number;
  export let offsetY: number;

  let calculatedStartX: number;
  let calculatedStartY: number;
  let calculatedEndX: number;
  let calculatedEndY: number;

  $: calculatedStartX = startX - offsetX;
  $: calculatedStartY = startY - offsetY;
  $: calculatedEndX = endX - offsetX;
  $: calculatedEndY = endY - offsetY;

  // TODO implement translation
  $: console.log(translationX);
  $: console.log(translationY);
</script>

<main>
  <!-- /todo -->

  <svg style="position: absolute;">
    <path
      d="M {calculatedStartX} {calculatedStartY} {calculatedEndX} {calculatedEndY}"
      stroke="red"
      stroke-width="2"
      fill="none"
    />
  </svg>
</main>

<style>
  main,
  svg,
  path {
    position: absolute;
    user-select: none;
    pointer-events: none;
    z-index: 5;
  }

  svg {
    width: 100vw;
    height: 100vh;
  }
</style>
