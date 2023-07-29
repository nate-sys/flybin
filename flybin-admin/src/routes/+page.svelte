<script lang="ts">
	import type { PageData } from './$types';
	import Paste from '$lib/Paste.svelte';
	export let data: PageData;
</script>

<nav>
	<h3>Pastes</h3>
    <form method="POST">
        <button>Logout</button>
    </form>
</nav>
{#await data.pastes.loaded}
	<p>loading...</p>
{:then loaded}
	<ul>
		{#each loaded as pastes}
			<li>
				<Paste {...pastes} />
			</li>
		{:else}
			<p>no pastes</p>
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
		max-width: 60ch;
		margin: auto;
	}
    :global( button ) {
        appearance: none;
        border: none;
        background: #4a4b8b;
        color: #fff;
        padding: 1ch;
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
