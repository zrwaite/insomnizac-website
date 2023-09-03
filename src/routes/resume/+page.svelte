<script lang="ts">
	import Newtab from "$lib/components/icons/newtab.svelte"

	let resumeSrc = "https://storage.googleapis.com/insomnizac_public/dynamic/resume.pdf"
</script>
<article>
	<h1>Resume</h1>
	<a href={resumeSrc} target="_blank" class="resumeLink">
		<p>
			Open in new tab 
		</p>
		<div class="newtabIcon">
			<Newtab/>
		</div>
	</a>
	<iframe
		title="Zac Waite Resume"
		frameborder="0"
		src={resumeSrc}
	/>
</article>

<style lang="scss">
	article {
		display: flex;
		flex-direction: column;
		align-items: center;
		width: 100%;
	}
	iframe {
		align-self: center;
		width: 95%;
		max-width: 53rem;
		aspect-ratio: 3/4;
	}
	h1 {
		font-size: 4rem;
		margin: 1rem 0 1rem;
	}
	.resumeLink {
		display: flex;
		align-items: center;
		margin: 1rem;
		p {
			font-size: 1.5rem;
		}
		div {
			height: 1.5rem;
			margin-left: 0.5rem;
		}
	}
</style>
