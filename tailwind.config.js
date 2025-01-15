/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,rs}"],
  theme: {
    extend: {
      backgroundImage: {
              'stone-tile': "url('/_assets/tiles/stone_s.png')",
              'stone-tile-dark': "url('/_assets/tiles/stone_s_dark.png')",
              'ice-queen': "url('/_assets/chars/ice_queen.png')",
            }
    },
  },
  plugins: [  
    require('daisyui'),
  ],
  daisyui: {
    base: false,
    darkTheme: "dark",
    themes: false,
   },
}

