<script lang="ts">
	import HomepageProjectPanel from '$lib/components/ProjectPanel/HomepageProjectPanel.svelte';
	import TitleRow from '$lib/components/TitleRow.svelte';

	import logo from '$lib/images/logo-nobackground.png';
	import Graphs from '$lib/pages/home/Graphs.svelte';
	import Languages from '$lib/pages/home/Languages.svelte';
	import type { HomeData } from './+page';
	export let data: HomeData;
	$: projects = data.projects;
</script>

<article>
	<section class="homeHeader">
		<div class="headerText">
			<h1>Hi, I'm Zac</h1>
			<h3 class="comment">//Full Stack Web Developer</h3>
			<h3 class="comment">//Robotics Programmer</h3>
		</div>
		<div class="headerLogo">
			<img src={logo} alt={'logo'} />
		</div>
	</section>
	<Languages />
	<Graphs />
	<TitleRow title={'Projects'} />
	<div class="projects">
		{#each projects as project}
			<HomepageProjectPanel {project} />
		{/each}
	</div>
</article>



<style lang="scss">
	article {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: flex-start;
		margin: 1rem;
	}
	.homeHeader {
		display: flex;
		width: 100%;
		max-width: 50rem;
		justify-content: space-around;
		margin: 1rem;
		h1 {
			font-size: 3rem;
			font-weight: bold;
		}
	}
	.headerText {
		* {
			width: 100%;
			text-align: left;
		}
	}
	.headerLogo {
		margin: 1rem 2rem;
	}
	.comment {
		font-style: italic;
		font-weight: 500;
		color: lightgrey;
	}
	.projects {
	display: flex;
	flex-direction: row;
	justify-content: center;
	align-items: center;
	flex-wrap: wrap;
}
</style>
