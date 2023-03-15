<script lang="ts">
  export let startX: number;
  export let startY: number;
  export let endX: number;
  export let endY: number;
  export let translationX: number;
  export let translationY: number;
  export let offsetX: number;
  export let offsetY: number;
  export let visible: boolean;

  let calculatedStartX: number;
  let calculatedStartY: number;
  let calculatedEndX: number;
  let calculatedEndY: number;
  let translationXStyle: string;
  let translationYStyle: string;
  let visibilityStyle: string;

  $: calculatedStartX = startX - offsetX;
  $: calculatedStartY = startY - offsetY;
  $: calculatedEndX = endX - offsetX;
  $: calculatedEndY = endY - offsetY;
  $: translationXStyle = translationX.toString() + "px";
  $: translationYStyle = translationY.toString() + "px";
  $: visibilityStyle = visible ? "visible" : "hidden";

  let curve;

  $: {
    //TODO. Initial implementation of curved connection line
    //Subject to change in the future
    let mx = (endX + startX) * 0.5;
    let my = (endY + startY) * 0.5;
    let theta = Math.atan2(endY - startY, endX - startX) - Math.PI / 2;
    let offset = 100;

    // location of control point:
    let c1x = mx - offset * Math.cos(theta);
    let c1y = my - offset * Math.sin(theta);

    curve =
      "M" +
      startX +
      " " +
      (startY - offsetY) +
      " Q " +
      c1x +
      " " +
      c1y +
      " " +
      endX +
      " " +
      (endY - offsetY);
  }
</script>

<main
  style="--translateX: {translationXStyle}; --translateY:{translationYStyle}; --visible: {visibilityStyle}"
>
  <svg style="position: absolute;">
    <path
      d="M {calculatedStartX} {calculatedStartY} {calculatedEndX} {calculatedEndY}"
      stroke="#22D3EE"
      stroke-width="3"
    />
    <!-- <path d={curve} stroke="red" stroke-width="2" fill="none" /> -->
  </svg>
</main>

<style>
  main,
  svg,
  path {
    position: absolute;
    user-select: none;
    pointer-events: none;
    z-index: -1;
  }

  svg {
    width: 100vw;
    height: 100vh;
  }

  path {
    transform: translate(var(--translateX), var(--translateY));
    visibility: var(--visible);
  }
</style>
