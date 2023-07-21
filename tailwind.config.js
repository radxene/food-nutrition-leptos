/** @type {import("tailwindcss").Config} */

const defaultTheme = require("tailwindcss/defaultTheme")

module.exports = {
  darkMode: "class",
  content: ["./index.html", "./src/**/*.{rs,css,html}"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", ...defaultTheme.fontFamily.sans],
      },
      transitionProperty: {
        "width-transform": "width, transform",
        "margin": "margin",
      },
      colors: {
        brand: { 500: "#BC1D35", 600: "#B01A31" },
        dark: {
          bg: "#151823",
          "vague-1": "#222738",
          "vague-2": "#2A2F42",
          "vague-3": "#2C3142",
        },
      },
      width: {
        20: "4.5rem",
      },
      margin: {
        20: "4.5rem",
      },
    },
  },
  safelist: ["lg:ml-20", "lg:ml-64"],
  plugins: [require("@tailwindcss/typography"), require("@tailwindcss/forms")({ strategy: "class" })],
};
