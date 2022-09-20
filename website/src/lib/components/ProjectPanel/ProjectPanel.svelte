<script lang="ts">
	import type { ProjectType } from "$lib/types";

	export let project: ProjectType;

	const link = project.devpostLink || project.githubLink;
</script>

<a class="projectPanel" href={link} target="_blank" style='--backgroundImage:url("{project.image}")'>
	<div class="projectImage" >
		<div>
			<p>{project.description}</p>
		</div>
		<!-- <img class={"backgroundImage"} src={project.image} alt={project.name} /> -->
	</div>
	<div class="projectDetails" >
		<h3>{project.name}</h3>
	</div>
</a>

<style lang="scss">
	.projectPanel {
		cursor: pointer;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		max-width: 30rem;
		width: calc(100% - 4rem);
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
		position: relative;
		width: 100%;
		aspect-ratio: 16/9;
		background-image: var(--backgroundImage);
		background-size: cover;
		border-radius: 1rem;
		div {
			padding: 1rem;
			border-radius: 1rem;
			box-sizing: border-box;
			background-color:rgba(50, 50, 50, 0.5);
			opacity:0;
			transition: opacity 0.2s ease-in-out;
		}
		&:hover {
		div {
				opacity: 1;
			}
		}
	}
	.projectDetails {
		h3 {
			margin-top: 0.5rem;
		}
	}
</style>