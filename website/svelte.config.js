import adapter from '@sveltejs/adapter-node'
import preprocess from 'svelte-preprocess'
import { dirname } from 'path'
import { fileURLToPath } from 'url'

/** @type {import('@sveltejs/kit').Config} */

const filePath = dirname(fileURLToPath(import.meta.url))
const sassPath = `${filePath}/src/lib/styles/`

const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess({
		scss: {
			prependData: `@import '${sassPath}_global-imports.scss';`
		}
	}),
	kit: {
		adapter: adapter()
	}
};

export default config
