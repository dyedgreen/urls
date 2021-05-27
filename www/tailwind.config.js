module.exports = {
  mode: "jit",
  purge: [
    "../server/templates/**/*.html",
    "./static/**/*.html",
    "./src/**/*.{js,jsx}",
  ],
};
