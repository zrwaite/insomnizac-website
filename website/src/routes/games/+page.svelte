<script lang="ts">
	import { GAMES } from '../../data/games';
</script>

<article>
	{#each GAMES as game}
		<div>
			<a href={`/games/${game.slug}`}>{game.name}</a>
			<!-- <iframe height="100%" width="100%" title={game.name} src={game.src} frameborder="1" allowfullscreen></iframe> -->
		</div>
	{/each}
</article>
