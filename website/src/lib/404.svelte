<script lang="ts">
	import gif404 from '$lib/images/404.gif'
	import { onMount } from 'svelte';

	let images = [
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/typescript/typescript-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/javascript/javascript-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/python/python-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/go/go-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/php/php-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/cplusplus/cplusplus-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/c/c-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/ruby/ruby-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/html5/html5-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/css3/css3-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/rust/rust-plain.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/java/java-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/swift/swift-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/nodejs/nodejs-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/graphql/graphql-plain.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/react/react-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/mongodb/mongodb-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/postgresql/postgresql-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/docker/docker-original.svg",
		"https://raw.githubusercontent.com/devicons/devicon/master/icons/digitalocean/digitalocean-original.svg",
	];

	let confetti = new Array(100).fill(null)
		.map((_, i) => {
			return {
				image: images[i % images.length],
				x: Math.random() * 100,
				y: -20 - Math.random() * 100,
				r: 0.4 + Math.random() * 2
			};
		})

	onMount(() => {
		let frame:number

		function loop() {
			frame = requestAnimationFrame(loop);

			confetti = confetti.map(emoji => {
				emoji.y += 0.2 * emoji.r;
				if (emoji.y > 120) {
					emoji.y = -50;
					emoji.x = Math.random() * 100
					emoji.r = 0.4 + Math.random() * 2
				}
				return emoji;
			});
		}

		loop();

		return () => cancelAnimationFrame(frame);
	});
</script>
<section id="container404">
	<h1>Not found</h1>
	{#each confetti as img}
		<span style="left: calc({img.x}% + 4rem); top: {img.y}%; transform: scale({img.r}); z-index: {Math.round(1000*img.r)}">
			<img height={40} width={40} src={img.image} alt="404"/>
		</span>
	{/each}
	<img id="gif404" style="z-index: 10000;" src={gif404} alt='not found'/>
</section>


<style>
	#container404 {
		position: relative;
		overflow: hidden;
		height: 100vh;
	}

	span {
		position: absolute;
		font-size: 5vw;
		user-select: none;
	} 
	#gif404 {
		position: absolute;
		bottom: 0;
		left: 50%;
		transform: translateX(calc(-50% + 4rem));
		height: 80%;

	}
</style>