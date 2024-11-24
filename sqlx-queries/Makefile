.PHONY:
up:
	docker compose up --detach --remove-orphans --timestamps	
	$(MAKE) logs

.PHONY:
stop:
	docker compose stop

.PHONY:
restart: stop up

.PHONY:
down: stop
	docker compose down --volumes --remove-orphans --rmi all

.PHONY:
logs:
	docker compose logs api --follow --timestamps

# make migrations_new name=create_users
.PHONY:
migrations_new:
	sqlx migrate add $(name)

.PHONY:
migrations_run:
	sqlx migrate run --database-url "postgres://user:password@localhost:5432/postgres"

.PHONY:
migrations_recreate:
	docker compose exec postgres psql -U "user" -d "postgres" -c "drop schema if exists public cascade;"
	docker compose exec postgres psql -U "user" -d "postgres" -c "create schema public;"
	$(MAKE) migrations_run
