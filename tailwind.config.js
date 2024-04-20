/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        ruso: {
          dark: "#16181C",
          dark_code: "#282A38",
        },
      },
    },
  },
};
