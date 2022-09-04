<script lang="ts">
	import { goto } from '$app/navigation'
	import logo from '$lib/images/logo-nobackground.png'
</script>
<article>
	<section class="homeHeader">
		<div class="headerText">
			<h1>Hi, I'm Zac</h1>
			<h3 class="comment">//Full Stack Web Developer</h3>
			<h3 class="comment">//Robotics Programmer</h3>
		</div>
		<div class="headerLogo">
			<img src={logo} alt={'logo'}/>
		</div>
	</section>
	<h1>Zac Waite</h1>
	<button on:click={() => goto(`/projects`)}>
		Go to projects
	</button>
</article>
<style lang="scss">
	article {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: flex-start;
	}
	.homeHeader {
		display: flex;
		width: 100%;
		justify-content:space-around;
		margin: 2rem;
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
</style>