import { sveltekit } from '@sveltejs/kit/vite'
import type { UserConfig } from 'vite'
import path from 'path'

const config: UserConfig = {
	plugins: [sveltekit()],
	resolve: {
		alias: {
			'@components': path.resolve('./src/lib/components'),
			'@lib': path.resolve('./src/lib'),
			'@utils': path.resolve('./src/lib/utils')
		}
	}
}

export default config
