db-access:
	docker-compose exec db psql -h 0.0.0.0 -p 5432 -U root ittoku_api

api-bash:
	docker-compose exec api bash

api-test:
	docker-compose exec api cargo test
