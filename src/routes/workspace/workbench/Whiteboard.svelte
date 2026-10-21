<script lang="ts">
  import { onMount } from "svelte";
  import { drawAutomaton } from "$lib/utils/drawingFuncs";
  import { automaton, connection_start, selection } from "$lib/utils/automataStores";
  import { stateById, type Point } from "$lib/workbench/automaton";
  import { clickCanvas, doubleClickCanvas, dragCanvas, pressCanvas, pressKey, releaseCanvas } from "$lib/workbench/actions";
  import { saveNow } from "$lib/workbench/autosave";

  // The state a string's traversal has reached, while it is being shown
  export let highlighted_state_id: number | null = null;

  let canvas: HTMLCanvasElement | null;
  let context: CanvasRenderingContext2D;
  let width = 900;
  let height = 900;

  // Where the pointer is, for drawing a connection that has been started but not finished
  let pointer: Point | null = null;

  $: connection_preview_start = $connection_start === null ? undefined : stateById($automaton, $connection_start);

  $: if (context) {
    drawAutomaton(context, width, height, $automaton, {
      selection: $selection,
      highlightedStateId: highlighted_state_id,
      connectionPreview: connection_preview_start && pointer ? { from: connection_preview_start.position, to: pointer } : null,
    });
  }

  onMount(() => {
    width = window.screen.availWidth - 200;
    height = window.screen.height - 300;
    const ctx = canvas?.getContext("2d");
    if (!ctx) {
      return;
    }
    context = ctx;
    context.imageSmoothingQuality = "high";
  });

  // Where the event happened on the canvas, in the canvas's own pixels
  const canvasPoint = (event: MouseEvent): Point => {
    const bounds = canvas?.getBoundingClientRect();
    if (!canvas || !bounds) {
      return { x: event.offsetX, y: event.offsetY };
    }
    return {
      x: Math.round(((event.clientX - bounds.left) * canvas.width) / bounds.width),
      y: Math.round(((event.clientY - bounds.top) * canvas.height) / bounds.height),
    };
  };

  const handlePointerDown = (event: PointerEvent): void => {
    // Keep receiving the pointer's movements while dragging, even outside the canvas
    canvas?.setPointerCapture(event.pointerId);
    pressCanvas(canvasPoint(event));
  };

  const handlePointerMove = (event: PointerEvent): void => {
    const point = canvasPoint(event);
    pointer = point;
    dragCanvas(point);
  };

  // Keys typed into a text box are for the text box
  const isTextEntry = (target: EventTarget | null): boolean =>
    target instanceof HTMLElement && (target.isContentEditable || ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName));

  const handleKeyDown = (event: KeyboardEvent): void => {
    if ((event.ctrlKey || event.metaKey) && event.key === "s") {
      event.preventDefault();
      saveNow().catch(() => {});
      return;
    }
    if (!isTextEntry(event.target)) {
      pressKey(event);
    }
  };
</script>

<svelte:window on:keydown={handleKeyDown}/>
<canvas class="border-black border-2 rounded-md mx-2 my-2 bg-white mr-0 shrink-0 touch-none"
  draggable="false"
  style={`width: ${width}px; height: ${height}px;`}
  {width}
  {height}
  bind:this={canvas}
  on:click={(event) => clickCanvas(canvasPoint(event))}
  on:dblclick={(event) => doubleClickCanvas(canvasPoint(event))}
  on:pointerdown={handlePointerDown}
  on:pointermove={handlePointerMove}
  on:pointerup={releaseCanvas}
  on:pointercancel={releaseCanvas}
  on:pointerleave={() => (pointer = null)}>
</canvas>
