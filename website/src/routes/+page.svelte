<script lang="ts">
	import { goto } from '$app/navigation'
</script>
<article>
	<h1>Zac Waite</h1>
	<button on:click={() => goto(`/projects`)}>
		Go to projects
	</button>
</article>