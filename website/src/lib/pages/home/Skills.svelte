<script lang="ts">
	import Dots from '$lib/components/icons/dots.svelte';
	import type { SkillType } from '$lib/types';
	export let skills: SkillType[]
	export let size = "50rem"
	export let max: number|undefined = undefined
</script>

<section class="languagesSection">
	{#each skills as skill}
		{#if !max || skills.indexOf(skill) < max}
			<img
				src={skill.image}
				alt={skill.name}
				height={size}
				width={size}
			/>
		{/if}
	{/each}
	{#if max && skills.length >= max -1}
		<img
			src="https://img.icons8.com/ios-filled/50/000000/more.png"
			alt="More"
			height={size}
			width={size}
		/>
	{/if}
</section>

<style lang="scss">
	.languagesSection {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: center;
		img {
			margin: 0.2rem;
		}
	}
</style>
