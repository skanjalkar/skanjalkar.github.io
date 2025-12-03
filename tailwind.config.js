/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index_leptos.html",
    "./static/**/*.html",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // Dark theme colors matching your current site
        'bg-dark': 'rgb(10, 10, 10)',
        'bg-light': 'rgb(255, 255, 255)',
        'card-dark': 'rgb(20, 20, 20)',
        'card-light': 'rgb(245, 245, 245)',
        'blog-gray': 'rgb(180, 180, 180)',
      },
      fontFamily: {
        'sans': ['Poppins', 'sans-serif'],
        'display': ['Questrial', 'sans-serif'],
      },
      backgroundImage: {
        'hero-gradient': 'linear-gradient(90deg, rgba(10, 10, 10, 0.3), rgb(10, 10, 10, 1))',
        'hero-gradient-mobile': 'linear-gradient(0deg, rgba(10, 10, 10, 1), rgba(10, 10, 10, 0))',
        'blog-overlay': 'linear-gradient(0deg, rgba(10, 10, 10, 1), rgba(10, 10, 10, 0.1))',
      },
      animation: {
        'spin-slow': 'spin 1s linear infinite',
        'fade-in': 'fadeIn 0.5s ease-in-out',
        'fade-out': 'fadeOut 0.5s ease-in-out',
        'slide-in': 'slideIn 0.3s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        fadeOut: {
          '0%': { opacity: '1' },
          '100%': { opacity: '0' },
        },
        slideIn: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
      scale: {
        '103': '1.03',
      },
    },
  },
  plugins: [],
}