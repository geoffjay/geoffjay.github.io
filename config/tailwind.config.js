// const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  purge: {
    enabled: process.env.NODE_ENV === "production",
    mode: "all",
    content: [
      "../src/**/*.rs",
      "../public/index.html",
      "./tailwind.css",
    ]
  },
  darkMode: false,
    theme: {
      colors: {
        transparent: "transparent",
        black: "#000",
        white: "#fff",
        gray: {
          100: "#eceff4",
          200: "#e5e9f0",
          300: "#d8dee9",
          500: "#4c566a",
          600: "#434c5e",
          700: "#3b4252",
          800: "#2e3440",
        },
        blue: "#5e81ac",
        cyan: "#8fbcbb",
        lightblue: "#81a1c1",
        lightcyan: "#88c0d0",
        red: "#bf616a",
        orange: "#d08770",
        yellow: "#ebcb8b",
        green: "#a3be8c",
        purple: "#b48ead",
      },
    extend: {
      // fontFamily: {
      //   "sans": ["Source Sans Pro", ...defaultTheme.fontFamily.sans],
      //   "serif": ["Source Serif Pro", ...defaultTheme.fontFamily.serif],
      //   "mono": ["Source Code Pro", ...defaultTheme.fontFamily.mono],
      // },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
