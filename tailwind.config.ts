import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        lime: {
          DEFAULT: "#A4E62D",
          dark: "#7AB520",
        },
        coral: {
          DEFAULT: "#FF6B6B",
          dark: "#FF4D4D",
        },
        cream: "#FFF8E7",
        dark: {
          DEFAULT: "#1A1A2E",
          light: "#212144",
        },
      },
      fontFamily: {
        sans: ["var(--font-inter)"],
        hand: ["var(--font-patrick)"],
      },
    },
  },
  plugins: [],
};
export default config;
