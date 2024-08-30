<script>
  import { goto } from "$app/navigation";
  import { Store } from "tauri-plugin-store-api";

  const handleClick = async () => {
    const store = new Store(".settings.dat");
    await store.clear();
    document.cookie = "email=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    goto("/");
  };
</script>

<button class="bg-orange-500 text-white flex w-fit px-4 py-4 rounded-md
text-3xl transition-all duration-300 hover:-translate-y-4 font-semibold gap-2"
  on:click={handleClick}>
  <svg
    class="w-10 h-10"
    data-slot="icon"
    fill="none"
    stroke-width="1.5"
    stroke="currentColor"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
    aria-hidden="true">
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z">
    </path>
  </svg>
  <div class="w-fit">Sign Out</div>
</button>
