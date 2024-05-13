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
          code2: "#464f5c",
          gray350: "#B6BCC5",
          gray420: "#896F75",
        },
      },
    },
  },
};
