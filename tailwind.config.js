/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    // rust files
    "./src/**/*.rs",
    // html files
    "./templates/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        clifford: "#da373d",
      },
    },
  },
  plugins: [],
}

