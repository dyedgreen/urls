module.exports = {
  mode: "jit",
  darkMode: "media",
  purge: [
    "../server/templates/**/*.html",
    "./static/**/*.html",
    "./src/**/*.{js,jsx}",
  ],
};
