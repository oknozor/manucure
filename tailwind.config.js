/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'class',
    content: [
        '**/*.{html,js,css}',
    ],
    plugins: [
        require('@tailwindcss/typography'),
        require('@tailwindcss/forms'),
    ],
}
