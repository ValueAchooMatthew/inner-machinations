<script lang="ts">
    export let default_connection_char;

    const handleCharChange = (event: Event) => {
        if(!(event instanceof InputEvent)){
            return;
        }
        const data = event.data;
        if(!data){
            return;
        }
        default_connection_char = event.data;
    }

</script>

<div class="bg-gray-200 p-4 text-center absolute w-min-fit w-96 h-min-fit flex flex-col rounded-r-md z-10">
    <div class="font-bold">
        Input Alphabet (works for DFA's only):
        <input on:input={handleCharChange} class="border-black border-2 rounded-md" type="text">
    </div>
    <div>
        Strict Checking (works for DFA's only):
    </div>
    <div>
        Specify default connection character (default: a):
        <input on:input={handleCharChange} class="border-black border-2 rounded-md" type="text">
    </div>

</div>