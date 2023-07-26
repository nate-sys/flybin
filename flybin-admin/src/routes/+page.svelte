<script lang="ts">
	import type { PageData } from './$types';
	import Paste from '$lib/Paste.svelte';
	export let data: PageData;
</script>

{#await data.posts.loaded}
	<p>loading...</p>
{:then loaded}
	<ul>
		{#each loaded as post}
			<li>
				<Paste {...post} />
			</li>
		{/each}
	</ul>
{:catch error}
	<p>{error.message}</p>
{/await}

<style>
	:global(*) {
		box-sizing: border-box;
	}
	:global(body) {
		font-family: sans-serif;
        background: #0a023e;
        color: #ffffff;
        max-width: 80ch;
        margin: auto;
	}
	ul {
		list-style: none;
		padding: 0;
	}
	li:nth-child(even) {
        background: #1a1a3a;
	}
	li:nth-child(odd) {
        background: #3a3264;
	}
</style>
