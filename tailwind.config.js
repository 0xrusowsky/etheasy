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
        },
        minHeight: {
          110: "110px",
          130: "130px",
          150: "150px",
          170: "170px",
          190: "190px",
          210: "210px",
          230: "230px",
          250: "250px",
          270: "270px",
          290: "290px",
          310: "310px",
          330: "330px",
          350: "350px",
          370: "370px",
          390: "390px",
          410: "410px",
          430: "430px",
          450: "450px",
          470: "470px",
          490: "490px",
        },
      },
    },
  },
};
