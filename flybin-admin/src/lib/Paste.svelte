<script lang="ts">
	export let slug: string;
	export let content: string;
	export let created_at: string;
	export let expires_at: string;
	export let secret: string;
	export let ip_address: string;
	export let password: string | null;
	let copied = false;
	function copySlug() {
		navigator.clipboard.writeText(slug).then(() => {
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 1000);
		});
	}
</script>

<details>
	<summary>
		<span class="ip">{ip_address}</span>
		<span class="slug">{slug}</span>
	</summary>
	<div class="content">
		<button on:click={copySlug}>copy slug</button>
        {#if copied}
            <span class="copied">copied</span>
        {/if}
		<div>
			<time datetime={created_at}>created at {created_at.split('.')[0]}</time>
		</div>
		<div>
			<time datetime={expires_at}>expires at {expires_at.split('.')[0]}</time>
		</div>
		<code>secret {secret}</code>
		{#if password}
			<code>password {password}</code>
		{/if}
		<pre>{content}</pre>
	</div>
</details>

<style>
	details {
		background: inherit;
	}
	summary {
		padding: 0.5ch;
		background: parent;
		appearance: none;
		position: sticky;
		top: 0;
		cursor: pointer;
		outline: none;
	}
	summary:focus {
		outline: 2px solid #4a4b8b;
	}
	.ip {
		width: 15ch;
		font-family: inherit;
		background: #5555aa;
		color: #6bf377;
		padding: 0 1ch;
	}
	time {
		font-size: small;
	}
	.content {
		padding: 0.5ch;
		background: #13134a;
		color: #4a4b8b;
		overflow-x: auto;
	}

    .copied {
        color: #4a4b8b;
    }
	pre {
		color: #ad92ac;
	}
</style>
