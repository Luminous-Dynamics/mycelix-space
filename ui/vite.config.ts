import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	define: {
		// Required for Cesium
		CESIUM_BASE_URL: JSON.stringify('/cesium')
	},
	optimizeDeps: {
		exclude: ['cesium']
	}
});
