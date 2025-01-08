/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,rs}"],
  theme: {
    extend: {},
  },
  plugins: [  
    require('daisyui'),
  ],
  daisyui: {
     themes: ["light", "dark", "cupcake"],
   },
}

