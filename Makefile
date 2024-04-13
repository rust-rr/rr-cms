tw:
	cd client_dx/ && npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
ftx:
	cd client_dx/ && dx serve --hot-reload
fty:
	cd client_yew/ && trunk serve --open
bk:
	cd server/ && cargo run