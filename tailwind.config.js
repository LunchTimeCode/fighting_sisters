/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,rs}"],
  theme: {
    extend: {
      backgroundImage: {
              'stone-tile': "url('/_assets/tiles/stone_s.png')",
              'portrait': "url('/_assets/png/portraits/silver_siren.png')",
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

