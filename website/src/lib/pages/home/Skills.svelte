<script lang="ts">
	import TitleRow from '$lib/components/TitleRow.svelte';
	import type { SkillType } from '$lib/types';
	export let skills: SkillType[]
</script>

<TitleRow title={'Languages and Technologies'} />
<section class="languagesSection">
	{#each skills as skill}
		<img
			src={skill.image}
			alt={skill.name}
			height="50rem"
			width="50rem"
		/>
	{/each}
</section>

<style lang="scss">
	.languagesSection {
		margin: 0 1rem 1rem;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: center;
		img {
			margin: 0.2rem;
		}
	}
</style>
