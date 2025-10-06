module.exports = {
  // specify the files Tailwind should scan for class names
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./**/*.html",
    "./README.md",
    "./input.css",
  ],
  // safelist classes that might be constructed dynamically or purged otherwise
  safelist: [],
  theme: {
    extend: {
      colors: {
        primary: "#3b82f6",
        secondary: "#22c55e",
      },
    },
  },
  plugins: [],
};
