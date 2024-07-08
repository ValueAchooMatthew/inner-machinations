<script>
  import TitleHeader from "$lib/components/TitleHeader.svelte";
import LogIn from "./LogIn.svelte";
</script>
<div class="bg-gray-100 flex flex-col overflow-hidden gap-3 h-screen justify-between ">
  <TitleHeader />
  <main class="z-50">
    <LogIn />
  </main>
  <div class="min-h-64">
  </div>
</div>