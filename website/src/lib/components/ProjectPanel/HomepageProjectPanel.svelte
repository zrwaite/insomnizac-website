<script lang="ts">
	import type { ProjectType } from "src/types";


	export let project: ProjectType;

</script>

<div class="projectPanel">
	<div class="projectImage" style="flex:5">
		<img src={project.image} alt={project.name} />
	</div>
	<div class="projectDetails" style="flex:6">
		<h3>{project.name}</h3>
		<p>{project.description}</p>
	</div>
</div>

<style lang="scss">
	.projectPanel {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		max-width: 30rem;
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
		h3 {
			margin-bottom: 1rem;
		}
		p {
			font-size: 0.7rem;
		}
	}
</style>