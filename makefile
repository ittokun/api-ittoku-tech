bash:
	docker-compose run web bash
db_setup:
	docker-compose run --rm web rails db:setup
db_reset:
	docker-compose run --rm web rails db:migrate:reset
