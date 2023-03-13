<script lang="ts">
  export let startX: number;
  export let startY: number;
  export let endX: number;
  export let endY: number;
  export let translationX: number;
  export let translationY: number;
</script>

<main>
  <!-- /todo -->

  <svg style="position: absolute;">
    <path
      d="M {startY} {startX} {endX} {endY}"
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
