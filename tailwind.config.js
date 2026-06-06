/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./app/src/**/*.rs",
    "./frontend/src/**/*.rs",
    "./server/src/**/*.rs",
    "./index.html",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: [
          "Libre Baskerville",
          "Inter",
          "ui-sans-serif",
          "system-ui",
          "sans-serif",
        ],
      },
      colors: {
        ink: "#050316",
        gray: "#0C0C0D",
        background: "#fbfbfe",
        primary: "#f9b4d7",
        secondary: "#dcf9e0",
        accent: "#f6ffdb",
        link: {
          DEFAULT: "#2563eb",
          hover: "#1e40af",
          visited: { DEFAULT: "#6b21a8", hover: "#581c87" },
        },
      },
      fontSize: {
        title: ["3rem", { lineHeight: "1.1" }],
        subtitle: ["2rem", { lineHeight: "1.1" }],
        section: ["1.5rem", { lineHeight: "1.2" }],
        body: ["1rem", { lineHeight: "1.6" }],
      },
    },
  },
  plugins: [],
};
