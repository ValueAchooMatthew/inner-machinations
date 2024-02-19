<script lang="ts">
import type { PageData } from './$types';
import { error } from '@sveltejs/kit';

export let data: PageData;

console.log(data)


</script>


<h1>{data.title}</h1>
<div>{@html data.content}</div>
