/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			colors: {
				black: {
					100: '#d6d6d6',
					200: '#aeaeae',
					300: '#858585',
					400: '#5d5d5d',
					500: '#343434',
					600: '#2a2a2a',
					700: '#1f1f1f',
					800: '#151515',
					900: '#0a0a0a'
				},
				gray: {
					100: '#ededed',
					200: '#dcdcdc',
					300: '#cacaca',
					400: '#b9b9b9',
					500: '#a7a7a7',
					600: '#868686',
					700: '#646464',
					800: '#434343',
					900: '#212121'
				},
				blue: {
					100: '#ccedff',
					200: '#99daff',
					300: '#66c8ff',
					400: '#33b5ff',
					500: '#00a3ff',
					600: '#0082cc',
					700: '#006299',
					800: '#004166',
					900: '#002133'
				}
			}
		}
	},

	plugins: []
};
