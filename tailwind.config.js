/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,rs}"],
  theme: {
    extend: {
      backgroundImage: {
              'stone-tile': "url('/_assets/tiles/stone_s.png')",
              'stone-tile-dark': "url('/_assets/tiles/stone_s_dark.png')",
              'ice-queen': "url('/_assets/chars/ice_queen.png')",
              'mage': "url('/_assets/chars/ice_queen.png')",
              'warrior': "url('/_assets/chars/warrior.png')",
              'rogue': "url('/_assets/chars/rogue.png')",
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

