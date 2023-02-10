/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx}",
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // "gray": {
        "dark": {
          100: "#D6D6D6",
          200: "#BBBBBB",
          300: "#707070",
          400: "#505050",
          500: "#3E3E3E",
          600: "#2E2E2E",
          700: "#282828",
          800: "#232323",
          900: "#1C1C1C",
        },
        // "green": {
        "contrast": {
          // 100: "#40CF8E",
          // 200: "#24B47E"
          "light": "#40CF8E",
          "dark": "#24B47E"
        },
      },
    },
  },
  plugins: [],
}
