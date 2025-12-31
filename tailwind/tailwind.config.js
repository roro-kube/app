import typography from '@tailwindcss/typography'

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "../gui/src/**/*.rs",
    "../web-docs/src/**/*.{js,jsx,ts,tsx}",
  ],
  darkMode: "class",
  theme: {
    extend: {
      // Custom colors and theme extensions can be added here
    },
  },
  plugins: [typography],
}

