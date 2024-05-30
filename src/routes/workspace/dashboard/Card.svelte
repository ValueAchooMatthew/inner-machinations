<script lang="ts">
  import { goto } from "$app/navigation";

    export let workspace_name;

    const handleClick = () => {
        document.cookie = "workspace_name=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
        document.cookie = "workspace_name" +"=" +workspace_name +"; path=/";
        goto("workbench");
    }

</script>
<button class="w-64 h-80 hover:w-80 hover:h-96 p-3 bg-gray-300 transition-all duration-300 rounded-md shadow-2xl flex flex-col justify-between" on:click={handleClick}>
    <div class="bg-white w-full h-full max-h-64">
    </div>
    <span class="self-center font-semibold text-orange-600 text-2xl">
        {workspace_name}
    </span>
</button>


