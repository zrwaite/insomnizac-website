<script lang="ts">
	import type { ProjectType } from "src/types";


	export let project: ProjectType;

</script>

<div class="projectPanel">
	<div style="flex:1">
		<img class="projectImage" src={project.image} alt={project.name} />
	</div>
	<div style="flex:1">
		<h2>{project.name}</h2>
	</div>
</div>

<style lang="scss">
	.projectPanel {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		width: 22rem;
		height: 12rem;
		background-color: #fff;
		border-radius: 1rem;
		padding: 1.5rem;
		margin: 1rem;
		box-shadow: 0 0 1rem rgba(0, 0, 0, 0.1);
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
		width: 100%;
		height: 100%;
		border-radius: 1rem;
		object-fit: cover;
	}
</style>