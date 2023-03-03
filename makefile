db-access:
	docker-compose exec db psql -h 0.0.0.0 -p 5432 -U root postgres

db-setup:
	docker-compose exec api diesel setup && docker-compose exec api diesel migration run

api-bash:
	docker-compose exec api bash
