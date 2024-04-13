ft:
	cd client/ && dx serve --hot-reload
tw:
	cd client/ && npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
cl:
	cd client/ && rm -rf dist
bk:
	cd server/ && cargo run