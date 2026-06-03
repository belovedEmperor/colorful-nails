/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./app/src/**/*.rs", "./frontend/src/**/*.rs", "./server/src/**/*.rs", "./index.html"],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
};
