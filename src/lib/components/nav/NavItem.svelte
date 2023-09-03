<script lang="ts">
	import { goto } from '$app/navigation'
	import { COLORS } from '../../styles/colors'
	export let overrideColor: boolean = false
	export let link: string | undefined = undefined
	export let icon: any
	export let name: string
	let hover = false
	$: fontColor = hover || overrideColor ? COLORS.primary : COLORS.font

	const mouseEnter = () => {
		hover = true
	}
	const mouseLeave = () => {
		hover = false
	}
</script>

<div
	style="--fontColor:{fontColor};"
	class={`navItem ${!link && 'disableHover'}`}
	on:click={link ? () => goto(link ?? '') : null}
	on:mouseleave={overrideColor ? null : mouseLeave}
	on:mouseenter={overrideColor ? null : mouseEnter}
>
	<div class="navImg">
		<svelte:component this={icon} color={fontColor} />
	</div>
	<h3>{name}</h3>
</div>

<style lang="scss">
	.navItem {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		padding: 0.5rem;
		cursor: pointer;
		margin-bottom: 1rem;
		.navImg {
			min-width: 2rem;
			padding: 0.5rem;
			margin-right: 1rem;
			display: flex;
			flex-direction: row;
			justify-content: center;
		}
		&:first-child {
			.navImg {
				min-width: 3rem;
				padding: 0;
			}
		}
		h3 {
			min-width: 10rem;
			font-size: 1.5rem;
			font-weight: bold;
			color: var(--fontColor);
		}
	}
	.disableHover {
		cursor: default;
	}
	@media screen and (max-width: 30rem) {
		.navItem {
			height: 4rem;
			max-width: 4rem;
			margin-bottom: 0;
			h3 {
				display: none;
			}
			&:first-child {
				display: none;
			}
			.navImg {
				height: 2rem;
			}
		}
	}
</style>
