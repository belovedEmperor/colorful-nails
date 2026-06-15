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
				sans: ["Inter", "ui-sans-serif", "system-ui", "sans-serif"],
				display: ["Playfair Display", "Georgia", "ui-serif", "serif"],
			},
			colors: {
				// ── Legacy tokens (backward compat) ───────────────────────────
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
				// ── Polish Collection palette ──────────────────────────────────
				// porcelain      #fbfbfe  — page background
				// midnight-ink   #050316  — body text
				// coral-lacquer  #E8524A  — primary CTA (booking)
				// violet-gloss   #7C3AED  — services / browse
				// gold-shimmer   #F59E0B  — info / hours / attention
				// teal-gel       #0891B2  — links / informational
				// blush-petal    #f9b4d7  — soft pink warmth
				// mint-frost     #dcf9e0  — success / availability
				// champagne-fizz #f6ffdb  — hover / accent surface
				// powder-rose    #f5e4ef  — card surfaces
				porcelain: "#fbfbfe",
				"midnight-ink": "#050316",
				"coral-lacquer": "#E8524A",
				"violet-gloss": "#7C3AED",
				"gold-shimmer": "#F59E0B",
				"teal-gel": "#0891B2",
				"blush-petal": "#f9b4d7",
				"mint-frost": "#dcf9e0",
				"champagne-fizz": "#f6ffdb",
				"powder-rose": "#f5e4ef",
			},
			fontSize: {
				title: ["3rem", { lineHeight: "1.1" }],
				subtitle: ["2rem", { lineHeight: "1.1" }],
				section: ["1.5rem", { lineHeight: "1.2" }],
				body: ["1rem", { lineHeight: "1.6" }],
			},
			keyframes: {
				shimmer: {
					"0%": { backgroundPosition: "200% center" },
					"100%": { backgroundPosition: "-200% center" },
				},
				"skeleton-sweep": {
					"0%": { backgroundPosition: "200% 0" },
					"100%": { backgroundPosition: "-200% 0" },
				},
			},
			animation: {
				shimmer: "shimmer 2.5s linear infinite",
				"skeleton-sweep": "skeleton-sweep 1.5s ease-in-out infinite",
			},
		},
	},
	plugins: [],
};
