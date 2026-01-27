/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // Status colors matching tray icons
        status: {
          loading: "#eab308", // yellow
          recording: "#ef4444", // red
          processing: "#3b82f6", // blue
          ready: "#22c55e", // green
        },
      },
    },
  },
  plugins: [],
};
