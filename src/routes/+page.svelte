<script>
    import "../app.css";

</script>

<style lang="postcss">
    :global(html) {
      background-color: theme(colors.gray.100);
      padding: 2rem;
    }
  </style>

<link type="text/css" rel="stylesheet" href="https://www.gstatic.com/firebasejs/ui/6.1.0/firebase-ui-auth.css" />


<h1 class="text-4xl text-center gray-950 font-semibold">Inner Machinations</h1>
<h3 class="text-sm text-gray-800 text-center mt-2 italic">
    An application created to design your very own distinct (and distinguished) finite state machines.
</h3>
<div class="flex flex-col justify-center content-center justify-items-center text-center">
    <h2 >
        Click to sign in
    </h2>
    <button class="self-center bg-white rounded-md w-48 font-semibold text-sm px-3 py-1 shadow-2xl hover:bg-gray-300 transition-all duration-300 ease-in-out mt-4 mb-1">
        Sign in with Google
    </button>
    <button class="self-center bg-white rounded-md w-48 font-semibold text-sm px-3 py-1 shadow-2xl hover:bg-gray-300 transition-all duration-300 ease-in-out mb-4 mt-1">
        Sign in with Email
    </button>
    <h4>
        Or... sign up
    </h4>    
</div>
