<script lang="ts">
	import { goto } from '$app/navigation'
	import resumeImg from '$lib/images/resume.svg'
	import logoImg from '$lib/images/logo.png'
	const navItems: {name: string; img: string, link: string}[] = [
		{
			name: "Zac Waite",
			img: logoImg,
			link: "/"
		},
		{
			name: "Projects",
			img: "https://raw.githubusercontent.com/devicons/devicon/master/icons/github/github-original.svg",
			link: "/projects"
		},
		{
			name: "Resume",
			img: resumeImg,
			link: "/resume"
		}
	]
</script>

<nav>
	<section>
		{#each navItems as navItem}
			<div id="navItem" on:click={() => goto(navItem.link)}>
				<div id="navImg">
					<img src={`${navItem.img}`} alt={navItem.name}>
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
		section {
			padding: 0;
			width: 4rem;
			height: 100%;
			position: absolute;
			background-color: color('secondary');
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
					h3 {
					}
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