<script lang="ts">
	import { goto } from '$app/navigation'
	import brainIcon from './icons/brain.svelte'
	const navItems: {name: string; link: string}[] = [
		{
			name: "Zac Waite",
			link: "/"
		},
		{
			name: "Projects",
			link: "/projects"
		},
		{
			name: "Resume",
			link: "/resume"
		}
	]
</script>

<nav>
	<section>
		{#each navItems as navItem}
			<div id="navItem" on:click={() => goto(navItem.link)}>
				<div id="navImg">
					<!-- <img src={`${navItem.img}`} alt={navItem.name}> -->
				</div>
				<h3>{navItem.name}</h3>
			</div>
		{/each}
	</section>
</nav>

<style lang="scss">
	nav {
		z-index: 100000;
		min-width: 4rem;
		width: 4rem;
		height: 100%;
		position: sticky;
		margin-right: 1.2rem;
		top: 0;
		section {
			padding: 0;
			width: 4rem;
			height: 100%;
			position: absolute;
			box-shadow: 0.2rem 0 1rem 0.2rem color('dark');
			background-color: color('background');
			transition: width 0.2s ease-out;
			overflow: hidden;
			&:hover {
				width: 15rem;
			}
			#navItem {
				display: flex;
				align-items: center;
				justify-content: flex-start;
				padding: 0.5rem;
				cursor: pointer;
				margin-bottom:1rem;
				// background-color: rgba(250, 250, 250, 0.15);
				&:hover {
					background-color: rgba(250, 250, 250, 0.2);
				}
				#navImg {
					min-width: 3rem;
					margin-right: 1rem;
					display: flex;
					flex-direction: row;
					justify-content: center;
					img {
						height: 3rem;
					}
				}
				h3 {
					min-width: 10rem;
					font-size: 1.5rem;
					font-weight: bold;
				}
			}
		}
	}
</style>