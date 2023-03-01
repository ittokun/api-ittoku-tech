db-access:
	docker-compose exec db psql -h 0.0.0.0 -p 5432 -U postgres postgres
