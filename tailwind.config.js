/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        ruso: {
          dark: "#16181C",
          dark_code: "#222831", //"#282A38",
        },
      },
    },
  },
};
