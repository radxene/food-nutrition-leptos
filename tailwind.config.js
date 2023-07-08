/** @type {import("tailwindcss").Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,css,html}"
  ],
  theme: {
    extend: {
      colors: {
        spill: {
          100: "#edf0f3",
        },
      }
    },
  },
  darkMode: "class",
  plugins: [
    require("@tailwindcss/typography"),
    require("@tailwindcss/forms")({ strategy: "class" }),
  ],
}

