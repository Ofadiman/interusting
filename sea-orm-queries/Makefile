# Refreshing migrations means that all tables will be dropped and all migrations will be run on a clear database.
.PHONY: migrate
migrate:
	sea-orm-cli migrate fresh --database-url postgres://user:password@localhost:5432/postgres

# SeaORM automatically generates Rust entities from database schemas.
.PHONY: entities
entities:
	sea-orm-cli generate entity --database-url postgres://user:password@localhost:5432/postgres --output-dir src/entities
