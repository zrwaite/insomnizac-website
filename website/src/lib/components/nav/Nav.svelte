<script lang="ts">
	import HomeIcon from '../icons/home.svelte'
	import ComputerIcon from '../icons/computer.svelte'
	import ResumeIcon from '../icons/resume.svelte'
	import NodeIcon from '../icons/node.svelte'
	import NavItem from './NavItem.svelte'
	const navItems: {name: string; icon: any, link?: string, overrideColor?: boolean}[] = [
		{
			name: "ZW",
			icon: NodeIcon,
			overrideColor: true
		},
		{
			name: "Zac Waite",
			icon: HomeIcon,
			link: "/"
		},
		{
			name: "Projects",
			icon: ComputerIcon,
			link: "/projects"
		},
		{
			name: "Resume",
			icon: ResumeIcon,
			link: "/resume"
		}
	]
</script>

<nav>
	<section>
		{#each navItems as navItem}
			<NavItem {...navItem}/>
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
			.topNavItem {
				cursor: default;
				&:hover {
					background-color: transparent;
				}
				margin-bottom:0;
			}
		}
	}
</style>