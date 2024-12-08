/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./ssr/*.html", "./ssr/src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        kumbh: ["Kumbh Sans", "sans-serif"],
      },
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
        shimmer: {
          to: {
            backgroundPositionX: "0%",
          },
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
        shimmer: "shimmer 1s infinite linear",
        "searching-a-1": "searching-a-1 2s infinite",
        "searching-a-2": "searching-a-2 2s infinite",
      },
    },
  },
  plugins: [],
};
