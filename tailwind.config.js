/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  darkMode: "class",
  theme: {
    extend: {
      screens: {
        sz900: "900px",
      },
      colors: {
        dark: {
          primary: "#16181C",
          code: "#222831",
          code2: "#464f5c",
        },
      },
    },
  },
};
