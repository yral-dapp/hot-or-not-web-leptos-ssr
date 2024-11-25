/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./ssr/*.html", "./ssr/src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        primary: {
          50: "rgb(var(--color-primary-50))",
          100: "rgb(var(--color-primary-100))",
          200: "rgb(var(--color-primary-200))",
          300: "rgb(var(--color-primary-300))",
          400: "rgb(var(--color-primary-400))",
          500: "rgb(var(--color-primary-500))",
          600: "rgb(var(--color-primary-600))",
          700: "rgb(var(--color-primary-700))",
          800: "rgb(var(--color-primary-800))",
          900: "rgb(var(--color-primary-900))",
          950: "rgb(var(--color-primary-950))",
        },
      },
      keyframes: {
        "blink-colors": {
          "0%": { color: "red" },
          "25%": { color: "lime" },
          "50%": { color: "yellow" },
          "75%": { color: "fuchsia" },
          "100%": { color: "blue" },
        },
        "searching-a-1": {
          "0%, 49%": { opacity: 1 },
          "50%, 100%": { opacity: 0 },
        },
        "searching-a-2": {
          "0%, 49%": { opacity: 0 },
          "50%, 100%": { opacity: 1 },
        },
      },
      animation: {
        "blink-colors": "blink-colors 5s step-end infinite",
        "searching-a-1": "searching-a-1 2s infinite",
        "searching-a-2": "searching-a-2 2s infinite",
      },
    },
  },
  plugins: [],
};
