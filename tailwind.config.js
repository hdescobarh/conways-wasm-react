// https://tailwindcss.com/docs/installation/using-postcss
// https://github.com/tailwindlabs/prettier-plugin-tailwindcss#resolving-your-tailwind-configuration
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js}", "./public/index.html"],
  theme: {
    extend: {},
  },
  plugins: [],
};
