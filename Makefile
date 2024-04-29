fe_x:
	cd client_dx/ && dx serve --hot-reload
fe_y:
	cd client_yew/ && trunk serve --open
be_dev:
	cd server/ && make dev
be_test:
	cd server/ && make test