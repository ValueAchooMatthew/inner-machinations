<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { ReportedError } from "$lib/utils/backend";

  // A failed command has already shown the user why, so it needn't be logged as an unhandled error too
  onMount(() => {
    const ignoreReportedErrors = (event: PromiseRejectionEvent) => {
      if (event.reason instanceof ReportedError) {
        event.preventDefault();
      }
    };
    window.addEventListener("unhandledrejection", ignoreReportedErrors);
    return () => window.removeEventListener("unhandledrejection", ignoreReportedErrors);
  });
</script>

<slot />
