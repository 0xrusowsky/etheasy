/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        dark: {
          primary: "#16181C",
          code: "#222831",
        },
      },
    },
  },
};
