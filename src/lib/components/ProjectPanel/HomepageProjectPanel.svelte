<script lang="ts">
	import Skills from "$lib/pages/home/Skills.svelte"
	import type { ProjectType } from "$lib/types"

	export let project: ProjectType

	const link = project.devpost_link || project.github_link
</script>

<a class="projectPanel" href={link} target="_blank">
	<div class="projectImage" style="flex:5">
		<img src={project.image} alt={project.name} />
	</div>
	<div class="projectDetails" style="flex:6">
		<h3>{project.name}</h3>
		<p>{project.description}</p>
		<Skills skills={project.skills} size={"30rem"} max={8} />	
	</div>
</a>

<style lang="scss">
	.projectPanel {
		cursor: pointer;
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		max-width: 30rem;
		width: calc(100% - 4rem);
		aspect-ratio: 22/12;
		background: linear-gradient(180deg, #28372E 0%,#131B1B00 100%);
		border-radius: 1rem;
		padding: 1rem;
		margin: 1rem;
		box-shadow: 0 0 1rem rgba(0, 0, 0, 0.5);
		transition: all 0.2s ease-in-out;
		&:hover {
			transform: translateY(-5px);
			box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
		}
		div {
			height: 100%;
			width: 100%;
		}
	}	
	.projectImage {
		img {
			width: 100%;
			height: 100%;
			border-radius: 1rem;
			object-fit: cover;
		}
		margin-right: 1rem;
	}
	.projectDetails {
		width: 100%;
		h3 {
			margin-bottom: 1rem;
		}
		p {
			font-size: 0.7rem;
		}
	}
</style>