<script>
  import LogIn from "./LogIn.svelte";
</script>
<div class="bg-gray-100 flex flex-col overflow-hidden gap-3 h-screen justify-between ">
  <header class="relative z-10 w-full h-fit min-h-64">
    <div class=" text-white text-center p-12">
      <h1 class="z-50 lg:text-5xl text-3xl font-bold">
        Inner Machinations
      </h1>
      <h3 class="z-50 lg:text-3xl text-xl font-semibold italic">
        A desktop application to create your very own DFA's and NFA's
      </h3>
    </div>
    <svg class="absolute top-0 -z-10 min-h-64 left-1/2 -translate-x-1/2 min-w-full"
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 1440 320">
      <path
      fill="#ff5500"
      fill-opacity="1"
      d="M0,256L60,234.7C120,213,240,171,360,170.7C480,171,600,213,720,213.3C840,213,960,171,1080,176C1200,181,1320,235,1380,261.3L1440,288L1440,0L1380,0C1320,0,1200,0,1080,0C960,0,840,0,720,0C600,0,480,0,360,0C240,0,120,0,60,0L0,0Z">
      </path>
    </svg>
  </header>
  <main class="z-50">
    <LogIn />
  </main>
  <div class="min-h-64">
  </div>
</div>