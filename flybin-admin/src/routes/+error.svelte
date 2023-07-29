<script>
    import { page } from '$app/stores';
</script>

<p>{$page.error?.message}</p>
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
	p {
		color: #ff3e00;
	}
</style>
