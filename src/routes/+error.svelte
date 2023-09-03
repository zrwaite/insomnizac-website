<script>
	import { page } from '$app/stores'
	import Page404 from '$lib/pages/404.svelte'
	let is404 = $page.status === 404
</script>

{#if is404}
	<Page404 />
{:else}
	<h1>{$page.status}: {$page.error.message}</h1>
{/if}
