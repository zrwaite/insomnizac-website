<script lang="ts">
	import TitleRow from '$lib/components/TitleRow.svelte';

	import construction from '$lib/images/construction_zac.png';
	import Graphs from '$lib/pages/home/Graphs.svelte';
	import { defaultHomeData, type HomeData } from './+page';
	import ProjectsGrid from '$lib/components/ProjectPanel/ProjectsGrid.svelte';
	import Button from '$lib/components/Button.svelte';
	import { goto } from '$app/navigation';
	import Skills from '$lib/pages/home/Skills.svelte';
	export let data: HomeData = defaultHomeData;
	$: projects = data.projects.slice(0, 6);
	$: skills = data.skills
</script>

<article>
	<section class="homeHeader">
		<div class="headerText">
			<h1>Hi, I'm Zac</h1>
			<h3 class="comment">//Full Stack Web Developer</h3>
			<h3 class="comment">//Robotics Programmer</h3>
			<div class="headerDescription">
				<p>This website is under it's initial construction!</p>
				<p>Much more to come!</p>
			</div>
		</div>
		<div class="headerLogo">
			<img src={construction} alt={'logo'} />
		</div>
	</section>
	<TitleRow title={'Languages and Technologies'} />
	<Skills skills={skills} />
	<Graphs />
	<TitleRow title={'Projects'} />
	<ProjectsGrid projects={projects} panelType={"homepage"}/>
	<div class="buttonMargin">
		<Button onClick={() => goto("/projects")} >
			<p class={"allProjectsButtonText"}>View all Projects</p>
		</Button>
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
		margin: 0 1rem;
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
	
	.headerLogo {
		margin: 0 2rem;
		height: 14rem;
		width: 9rem;
		min-width: 9rem;
		img {
			height: 100%;
			width: 100%;
		}
	}
	.headerDescription {
		margin: 1rem 0;
	}
	.allProjectsButtonText {
		color: color('primary');
	}
	.buttonMargin {
		margin: 1rem 0 2rem;
	}
	@media screen and (max-width: 40rem) {
		.homeHeader {
			flex-direction: column;
			align-items: center;
			.headerLogo {
				margin: 1rem 0;
			}
		}
		.headerDescription {
			p {
				text-align: center;
			}
		}
		.buttonMargin {
			padding: 1rem 0;
		}
	}

</style>
