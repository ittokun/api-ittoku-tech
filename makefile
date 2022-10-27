setup:
	docker-compose build
	docker-compose run --rm api rails db:setup

run:
	docker-compose up
